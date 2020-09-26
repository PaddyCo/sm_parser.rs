pub mod simfile;
use simfile::{BPMDisplayType, Chart, ChartDifficulty, DisplayBPM, NoteType, Simfile, Stop, BPM, BgChange};
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimfileParseError {
    BufReadError,
    FailedToParseBPMs,
    FailedToParseStops,
    TooManyValuesInDisplayBPM,
    EmptyNotesSection,
    InvalidChartFormat,
    InvalidBgChangeFormat,
    UnknownChartDifficulty,
    FailedToParseChartMeter,
    UnsupportedNoteType,
    FailedToParseRadarValues,
}

// TODO: Check if having a semicolon in the middle of a value is supported
// in Stepmania e.g "#TITLE: This is; a title;", if it does: make it work.
// TODO: Check if having a comment at the end of fields like title is supported
// in Stepmania e.g "#TITLE: This is a //very cool title"
// TODO: Handle non-UTF-8 streams! (at the moment they will return BufReadError)
pub fn parse_simfile<R: BufRead>(reader: &mut R) -> Result<Simfile, SimfileParseError> {
    let mut sim = Simfile::new();

    // Clean data by removing comments and unneccesary whitespace
    let mut cleaned_data = String::new();
    loop {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(count) => {
                if count == 0 {
                    break;
                }

                // Ignore comments
                let line = match buf.find("//") {
                    Some(i) => &buf[..i],
                    None => &buf,
                };

                cleaned_data.push_str(line.trim());
                cleaned_data.push('\r');
                cleaned_data.push('\n');
            }
            Err(e) => {
                return Err(SimfileParseError::BufReadError);
            }
        }
    }

    let mut reader = BufReader::new(cleaned_data.as_bytes());

    loop {
        let mut buf = vec![];
        match reader.read_until(b';', &mut buf) {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                let section = std::string::String::from_utf8_lossy(&buf);
                parse_section(&mut sim, &section)?;
            }
            Err(e) => {
                return Err(SimfileParseError::BufReadError);
            }
        };
    }

    return Ok(sim);
}

fn parse_section(simfile: &mut Simfile, section: &str) -> Result<(), SimfileParseError> {
    // Get start of the section (#KEY: value;)
    let section_start_index = match section.find('#') {
        Some(i) => i + 1,
        None => return Ok(()),
    };
    let section = &section[section_start_index..];

    // Get the end of the key
    let key_end_index = match section.find(':') {
        Some(i) => i,
        None => return Ok(()),
    };

    let value_end_index = section.len() - 1;

    let key = &section[..key_end_index];
    let val = &section[key_end_index + 1..value_end_index].trim();
    let value = if val.trim().len() > 0 {
        Some(val.to_string())
    } else {
        None
    };

    // TODO: Parse BGCHANGE section
    // TODO: Parse FGCHANGE section
    match key {
        "TITLE" => simfile.title = value,
        "SUBTITLE" => simfile.subtitle = value,
        "ARTIST" => simfile.artist = value,
        "TITLETRANSLIT" => simfile.title_translit = value,
        "SUBTITLETRANSLIT" => simfile.subtitle_translit = value,
        "ARTISTTRANSLIT" => simfile.artist_translit = value,
        "GENRE" => simfile.genre = value,
        "CREDIT" => simfile.credit = value,
        "BANNER" => simfile.banner_path = value,
        "JACKET" => simfile.jacket_path = value,
        "BACKGROUND" => simfile.background_path = value,
        "LYRICSPATH" => simfile.lyrics_path = value,
        "CDTITLE" => simfile.cd_title_path = value,
        "MUSIC" => simfile.music_path = value,
        "OFFSET" => simfile.offset = parse_float(value),
        "SAMPLESTART" => simfile.sample_start = parse_float(value),
        "SAMPLELENGTH" => simfile.sample_length = parse_float(value),
        "SELECTABLE" => simfile.selectable = parse_bool(value),
        "BGCHANGES" => simfile.bg_changes = parse_bg_changes(value)?,
        "BPMS" => {
            simfile.bpms = {
                match parse_key_value_list(value) {
                    Ok(i) => i
                        .into_iter()
                        .map(|x| BPM {
                            beat: x.key,
                            bpm: x.value,
                        })
                        .collect(),
                    Err(_) => {
                        return Err(SimfileParseError::FailedToParseBPMs);
                    }
                }
            }
        }
        "DISPLAYBPM" => simfile.display_bpm = parse_display_bpm(value)?,
        "STOPS" => {
            simfile.stops = {
                match parse_key_value_list(value) {
                    Ok(i) => i
                        .into_iter()
                        .map(|x| Stop {
                            beat: x.key,
                            time: x.value,
                        })
                        .collect(),
                    Err(_) => {
                        return Err(SimfileParseError::FailedToParseStops);
                    }
                }
            }
        }
        "NOTES" => match parse_chart(value) {
            Ok(chart) => simfile.charts.push(chart),
            Err(e) => return Err(e),
        },
        _ => {}
    };

    return Ok(());
}

fn parse_float(value: Option<String>) -> Option<f32> {
    match value.unwrap_or_default().as_str().parse() {
        Ok(i) => Some(i),
        _ => None,
    }
}

fn parse_bool(value: Option<String>) -> Option<bool> {
    match value {
        Some(i) => match i.as_ref() {
            "YES" => Some(true),
            "NO" => Some(false),
            _ => None,
        },
        None => None,
    }
}

fn parse_display_bpm(value: Option<String>) -> Result<Option<DisplayBPM>, SimfileParseError> {
    let value = match value {
        Some(i) => i,
        None => return Ok(None),
    };

    let values: Vec<&str> = value.trim().split(":").collect();

    // Check how many values (1 = Single/Random, 2 = Range)
    let display_bpm = match values.len() {
        1 => match values[0].parse() {
            // Could parse value as float, assume its a Single
            Ok(i) => DisplayBPM {
                display_type: BPMDisplayType::Single,
                value: i,
                value2: 0.0,
            },
            // Could NOT parse value as float, assume its a Random
            Err(_) => DisplayBPM {
                display_type: BPMDisplayType::Random,
                value: 0.0,
                value2: 0.0,
            },
        },
        2 => DisplayBPM {
            display_type: BPMDisplayType::Range,
            value: values[0].parse().unwrap(),
            value2: values[1].parse().unwrap(),
        },
        _ => return Err(SimfileParseError::TooManyValuesInDisplayBPM),
    };

    return Ok(Some(display_bpm));
}

fn parse_chart(value: Option<String>) -> Result<Chart, SimfileParseError> {
    let value = match value {
        Some(v) => v,
        None => return Err(SimfileParseError::EmptyNotesSection),
    };
    let value = value.trim();
    let values: Vec<&str> = value.split(":").map(|v| v.trim()).collect();

    if values.len() != 6 {
        return Err(SimfileParseError::InvalidChartFormat);
    }

    let chart = Chart {
        chart_type: values[0].to_string(),
        author: match values[1].is_empty() {
            true => None,
            false => Some(values[1].to_string()),
        },
        difficulty: match values[2] {
            "Beginner" => ChartDifficulty::Beginner,
            "Easy" => ChartDifficulty::Easy,
            "Medium" => ChartDifficulty::Medium,
            "Hard" => ChartDifficulty::Hard,
            "Challenge" => ChartDifficulty::Challenge,
            "Edit" => ChartDifficulty::Edit,
            _ => {
                return Err(SimfileParseError::UnknownChartDifficulty);
            }
        },
        meter: match values[3].parse() {
            Ok(i) => i,
            Err(_) => return Err(SimfileParseError::FailedToParseChartMeter),
        },
        radar_values: parse_radar_values(values[4])?,
        note_data: parse_chart_data(values[5])?,
    };

    Ok(chart)
}

fn parse_bg_changes(value: Option<String>) -> Result<Vec<BgChange>, SimfileParseError> {
    let value = match value {
        Some(v) => v,
        None => return Ok(Vec::new()),
    };
    let bg_changes: Vec<Result<BgChange, SimfileParseError>> =
        value.replace("\n", "")
             .replace("\r", "")
             .split(',')
             .collect::<Vec<&str>>()
             .into_iter()
             .map(parse_bg_change)
             .collect();

    let errors: Vec<SimfileParseError> = bg_changes.clone().into_iter().filter(|b| b.is_err()).map(|b| b.err().unwrap()).collect();

    if !errors.is_empty() {
        Err(errors[0])
    } else {
        Ok(bg_changes.into_iter().map(|b| b.unwrap() ).collect())
    }
}

fn parse_bg_change(value: &str) -> Result<BgChange, SimfileParseError> {
    // Split into values
    let values: Vec<&str> = value.split('=').collect();

    // TODO: Find out how many values are REQUIRED by StepMania
    // NOTE: From the SM wiki: "The set of entries is between the colon and the semicolon. Each entry is separated from the next by a comma. Each entry is composed of 1 to 11 values separated by equals."

    if values.len() < 6 {
        return Err(SimfileParseError::InvalidBgChangeFormat);
    }

    let start_beat = match values[0].parse::<f32>() {
        Ok(v) => v,
        Err(_) => return Err(SimfileParseError::InvalidBgChangeFormat)
    };

    let file_name = String::from(values[1]);

    let play_rate = match values[2].parse::<f32>() {
        Ok(v) => v,
        Err(_) => return Err(SimfileParseError::InvalidBgChangeFormat)
    };

    let transition_type = match values[3].parse::<i8>() {
        Ok(v) => v,
        Err(_) => return Err(SimfileParseError::InvalidBgChangeFormat)
    };

    let effect_flag = match values[4].parse::<i8>() {
        Ok(v) => v,
        Err(_) => return Err(SimfileParseError::InvalidBgChangeFormat)
    };

    let second_effect_flag = match values[5].parse::<i8>() {
        Ok(v) => v,
        Err(_) => return Err(SimfileParseError::InvalidBgChangeFormat)
    };

    // TODO: Add support for the rest of the fields
    //       in BGCHANGES (effect_file, second_effect_file, transition_file, color_string, second_color_string)


    Ok(BgChange {
        start_beat,
        file_name,
        play_rate,
        transition_type,
        effect_flag,
        second_effect_flag,
        effect_file: None,
        second_effect_file: None,
        transition_file: None,
        color_string: None,
        second_color_string: None,
    })
}

fn parse_radar_values(data: &str) -> Result<Vec<f32>, SimfileParseError> {
    let values: Vec<&str> = data.split(",").collect();
    let values: Vec<f32> = values
        .into_iter()
        .map(|v| match v.trim().parse() {
            Ok(v) => v,
            Err(_) => 0.0,
        })
        .collect();

    Ok(values)
}

// TODO: Figure out what 'H' corresponds to, and if Stepmania recognizes this.
fn parse_chart_data(data: &str) -> Result<Vec<Vec<NoteType>>, SimfileParseError> {
    let measures: Vec<&str> = data.split(",").collect();
    let mut chart: Vec<Vec<NoteType>> = vec![];

    for data in measures {
        let mut measure: Vec<NoteType> = vec![];

        // Ignore comments
        let end_index = data.find("//");
        let data = match end_index {
            Some(i) => &data[..i],
            None => data,
        };

        for note in data.trim().chars() {
            let note = match note {
                '0' => NoteType::None,
                '1' => NoteType::Normal,
                '2' => NoteType::HoldHead,
                '3' => NoteType::HoldOrRollTail,
                '4' => NoteType::RollHead,
                'M' => NoteType::Mine,
                'K' => NoteType::AutomaticKeysound,
                'L' => NoteType::LiftNote,
                'F' => NoteType::FakeNote,
                ' ' => continue,
                '\r' => continue,
                '\n' => continue,
                _ => NoteType::InvalidNote,
            };
            measure.push(note);
        }
        chart.push(measure);
    }

    Ok(chart)
}

struct KeyValue {
    key: f32,
    value: f32,
}

enum ParseKeyValueError {
    FailedToParseKeyValue,
    FailedToParseFloat,
}

// TODO: Check how Stepmania handles empty values in a keyvalue list:
// E.g (#BPMS:0.0=120.0;;10.0=150.0)
fn parse_key_value_list(value: Option<String>) -> Result<Vec<KeyValue>, ParseKeyValueError> {
    let value = match value {
        Some(i) => i,
        None => return Ok(vec![]),
    };

    let mut list: Vec<KeyValue> = vec![];

    // Split into values
    let values = value.split(",");

    // Extract key/values
    for value in values {
        let key_value: Vec<&str> = value.split("=").collect();
        if key_value.len() != 2 {
            return Err(ParseKeyValueError::FailedToParseKeyValue);
        }
        let key = match key_value[0].trim().parse() {
            Ok(i) => i,
            Err(_) => return Err(ParseKeyValueError::FailedToParseFloat),
        };
        let value = match key_value[1].trim().parse() {
            Ok(i) => i,
            Err(_) => return Err(ParseKeyValueError::FailedToParseFloat),
        };
        let key_value = KeyValue {
            key: key,
            value: value,
        };
        list.push(key_value);
    }

    Ok(list)
}

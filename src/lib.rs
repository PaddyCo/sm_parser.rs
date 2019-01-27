pub mod simfile;
use simfile::{BPMDisplayType, Chart, ChartDifficulty, DisplayBPM, Simfile, Stop, BPM};
use std::io::BufRead;

#[derive(Debug, PartialEq)]
pub enum SimfileParseError {
    BufReadError,
    FailedToParseBPMs,
    FailedToParseStops,
    TooManyValuesInDisplayBPM,
    EmptyNotesSection,
    InvalidChartFormat,
    UnknownChartDifficulty,
    FailedToParseChartMeter,
}

pub fn parse_simfile<R: BufRead>(reader: &mut R) -> Result<Simfile, SimfileParseError> {
    let mut sim = Simfile::new();

    loop {
        let mut buf = vec![];
        // TODO: Check if having a semicolon in the middle of a value is supported
        // in Stepmania e.g "#TITLE: This is; a title;", if it does: make it work.
        match reader.read_until(b';', &mut buf) {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                let section = std::string::String::from_utf8_lossy(&buf);
                parse_section(&mut sim, &section)?;
            }
            Err(_) => {
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
    let val = &section[key_end_index + 1..value_end_index];
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
        "BACKGROUND" => simfile.background_path = value,
        "LYRICSPATH" => simfile.lyrics_path = value,
        "CDTITLE" => simfile.cd_title_path = value,
        "MUSIC" => simfile.music_path = value,
        "OFFSET" => simfile.offset = parse_float(value),
        "SAMPLESTART" => simfile.sample_start = parse_float(value),
        "SAMPLELENGTH" => simfile.sample_length = parse_float(value),
        "SELECTABLE" => simfile.selectable = parse_bool(value),
        "BPMS" => {
            simfile.bpms = {
                match parse_key_value_list(value) {
                    Ok(i) => i
                        .into_iter()
                        .map(|x| BPM {
                            row: x.key,
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
                            row: x.key,
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
        radar_values: vec![],
        note_data: vec![vec![]],
    };

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

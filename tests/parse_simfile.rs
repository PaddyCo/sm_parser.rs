extern crate sm_parser;

use sm_parser::simfile::{BPMDisplayType, ChartDifficulty, NoteType, Simfile, Stop, BPM};
#[cfg(test)]
use sm_parser::{parse_simfile, SimfileParseError};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

// TODO: Test BGCHANGES/FGCHANGES parsing

fn load_and_parse_simfile(filename: &str) -> Result<Simfile, SimfileParseError> {
    // Load example file
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(format!("example_files/{}", filename));
    let file = File::open(d).unwrap();

    // Parse it!
    parse_simfile(&mut BufReader::new(file))
}

fn parse_string_as_simfile(data: &str) -> Result<Simfile, SimfileParseError> {
    parse_simfile(&mut BufReader::new(data.as_bytes()))
}

fn assert_bpm(bpm: &BPM, parsed_beat: f32, parsed_bpm: f32) {
    assert_eq!(bpm.beat, parsed_beat);
    assert_eq!(bpm.bpm, parsed_bpm);
}

#[test]
fn parses_title() {
    let sim = parse_string_as_simfile("#TITLE:The Title;").unwrap();
    assert_eq!(sim.title, Some("The Title".to_string()));
}

#[test]
fn parses_empty_title() {
    let sim = parse_string_as_simfile("#TITLE:;").unwrap();
    assert_eq!(sim.title, None);
}

#[test]
fn parses_i18n_title() {
    // This is the only test where we explicitly test for
    // Kanji/Hiragana characters, just to make sure it's handling
    // non-ascii characters correctly in general
    let sim = parse_string_as_simfile("#TITLE:虹を編めたら;").unwrap();
    assert_eq!(sim.title, Some("虹を編めたら".to_string()));
}

#[test]
fn parses_subtitle() {
    let sim = parse_string_as_simfile("#SUBTITLE:The Subtitle;").unwrap();
    assert_eq!(sim.subtitle, Some("The Subtitle".to_string()));
}

#[test]
fn parses_empty_subtitle() {
    let sim = parse_string_as_simfile("#SUBTITLE:;").unwrap();
    assert_eq!(sim.subtitle, None);
}

#[test]
fn parses_artist() {
    let sim = parse_string_as_simfile("#ARTIST:The Artist;").unwrap();
    assert_eq!(sim.artist, Some("The Artist".to_string()));
}

#[test]
fn parses_empty_artist() {
    let sim = parse_string_as_simfile("#ARTIST:;").unwrap();
    assert_eq!(sim.artist, None);
}

#[test]
fn parses_title_translit() {
    let sim = parse_string_as_simfile("#TITLETRANSLIT:The Title;").unwrap();
    assert_eq!(sim.title_translit, Some("The Title".to_string()));
}

#[test]
fn parses_empty_title_translit() {
    let sim = parse_string_as_simfile("#TITLETRANSLIT:;").unwrap();
    assert_eq!(sim.title_translit, None);
}

#[test]
fn parses_subtitle_translit() {
    let sim = parse_string_as_simfile("#SUBTITLETRANSLIT:The Subtitle;").unwrap();
    assert_eq!(sim.subtitle_translit, Some("The Subtitle".to_string()));
}

#[test]
fn parses_empty_subtitle_translit() {
    let sim = parse_string_as_simfile("#SUBTITLETRANSLIT:;").unwrap();
    assert_eq!(sim.subtitle_translit, None);
}

#[test]
fn parses_artist_translit() {
    let sim = parse_string_as_simfile("#ARTISTTRANSLIT:The Artist;").unwrap();
    assert_eq!(sim.artist_translit, Some("The Artist".to_string()));
}

#[test]
fn parses_empty_artist_translit() {
    let sim = parse_string_as_simfile("#ARTISTTRANSLIT:;").unwrap();
    assert_eq!(sim.artist_translit, None);
}

#[test]
fn parses_genre() {
    let sim = parse_string_as_simfile("#GENRE:The Genre;").unwrap();
    assert_eq!(sim.genre, Some("The Genre".to_string()));
}

#[test]
fn parses_empty_genre() {
    let sim = parse_string_as_simfile("#GENRE:;").unwrap();
    assert_eq!(sim.genre, None);
}

#[test]
fn parses_credit() {
    let sim = parse_string_as_simfile("#CREDIT:The Credit;").unwrap();
    assert_eq!(sim.credit, Some("The Credit".to_string()));
}

#[test]
fn parses_empty_credit() {
    let sim = parse_string_as_simfile("#CREDIT:;").unwrap();
    assert_eq!(sim.credit, None);
}

#[test]
fn parses_banner() {
    let sim = parse_string_as_simfile("#BANNER:TheBanner.png;").unwrap();
    assert_eq!(sim.banner_path, Some("TheBanner.png".to_string()));
}

#[test]
fn parses_empty_banner() {
    let sim = parse_string_as_simfile("#BANNER:;").unwrap();
    assert_eq!(sim.banner_path, None);
}

#[test]
fn parses_jacket() {
    let sim = parse_string_as_simfile("#JACKET:TheJacket.png;").unwrap();
    assert_eq!(sim.jacket_path, Some("TheJacket.png".to_string()));
}

#[test]
fn parses_empty_jacket() {
    let sim = parse_string_as_simfile("#JACKET:;").unwrap();
    assert_eq!(sim.jacket_path, None);
}

#[test]
fn parses_background() {
    let sim = parse_string_as_simfile("#BACKGROUND:TheBackground.png;").unwrap();
    assert_eq!(sim.background_path, Some("TheBackground.png".to_string()));
}

#[test]
fn parses_empty_background() {
    let sim = parse_string_as_simfile("#BACKGROUND:;").unwrap();
    assert_eq!(sim.background_path, None);
}

#[test]
fn parses_lyrics_path() {
    let sim = parse_string_as_simfile("#LYRICSPATH:TheLyrics.lrc;").unwrap();
    assert_eq!(sim.lyrics_path, Some("TheLyrics.lrc".to_string()));
}

#[test]
fn parses_empty_lyrics_path() {
    let sim = parse_string_as_simfile("#LYRICSPATH:;").unwrap();
    assert_eq!(sim.lyrics_path, None);
}

#[test]
fn parses_cd_title() {
    let sim = parse_string_as_simfile("#CDTITLE:TheCdTitle.png;").unwrap();
    assert_eq!(sim.cd_title_path, Some("TheCdTitle.png".to_string()));
}

#[test]
fn parses_empty_cd_title() {
    let sim = parse_string_as_simfile("#CDTITLE:;").unwrap();
    assert_eq!(sim.cd_title_path, None);
}

#[test]
fn parses_music() {
    let sim = parse_string_as_simfile("#MUSIC:TheMusic.ogg;").unwrap();
    assert_eq!(sim.music_path, Some("TheMusic.ogg".to_string()));
}

#[test]
fn parses_empty_music() {
    let sim = parse_string_as_simfile("#MUSIC:;").unwrap();
    assert_eq!(sim.music_path, None);
}

#[test]
fn parses_offset() {
    let sim = parse_string_as_simfile("#OFFSET:43.053;").unwrap();
    assert_eq!(sim.offset, Some(43.053));
}

#[test]
fn parses_empty_offset() {
    let sim = parse_string_as_simfile("#OFFSET:;").unwrap();
    assert_eq!(sim.offset, None);
}

#[test]
fn parses_sample_start() {
    let sim = parse_string_as_simfile("#SAMPLESTART:43.053;").unwrap();
    assert_eq!(sim.sample_start, Some(43.053));
}

#[test]
fn parses_empty_sample_start() {
    let sim = parse_string_as_simfile("#SAMPLESTART:;").unwrap();
    assert_eq!(sim.sample_start, None);
}

#[test]
fn parses_sample_length() {
    let sim = parse_string_as_simfile("#SAMPLELENGTH:43.053;").unwrap();
    assert_eq!(sim.sample_length, Some(43.053));
}

#[test]
fn parses_empty_sample_length() {
    let sim = parse_string_as_simfile("#SAMPLELENGTH:;").unwrap();
    assert_eq!(sim.sample_length, None);
}

#[test]
fn parses_true_selectable() {
    let sim = parse_string_as_simfile("#SELECTABLE:YES;").unwrap();
    assert_eq!(sim.selectable, Some(true));
}

#[test]
fn parses_false_selectable() {
    let sim = parse_string_as_simfile("#SELECTABLE:NO;").unwrap();
    assert_eq!(sim.selectable, Some(false));
}

#[test]
fn parses_empty_selectable() {
    let sim = parse_string_as_simfile("#SELECTABLE:;").unwrap();
    assert_eq!(sim.selectable, None);
}

#[test]
fn parses_bpms() {
    let sim = parse_string_as_simfile(
        "#BPMS:0.000=132.000,237.000=33.000,237.125=66.000,237.250=132.000;",
    )
    .unwrap();

    // Is it parsed correctly?
    assert_eq!(sim.bpms.len(), 4);
    assert_bpm(&sim.bpms[0], 0.000, 132.000);
    assert_bpm(&sim.bpms[1], 237.000, 33.000);
    assert_bpm(&sim.bpms[2], 237.125, 66.000);
    assert_bpm(&sim.bpms[3], 237.250, 132.000);
}

#[test]
fn parses_display_bpm_none() {
    let sim = parse_string_as_simfile("#DISPLAYBPM:;").unwrap();
    assert_eq!(sim.display_bpm.is_none(), true);
}

#[test]
fn parses_display_bpm_single() {
    let sim = parse_string_as_simfile("#DISPLAYBPM:66.000;").unwrap();
    let display_bpm = sim.display_bpm.unwrap();
    assert_eq!(display_bpm.display_type, BPMDisplayType::Single);
    assert_eq!(display_bpm.value, 66.0);
}

#[test]
fn parses_display_bpm_range() {
    let sim = parse_string_as_simfile("#DISPLAYBPM:66.000:132.000;").unwrap();
    let display_bpm = sim.display_bpm.unwrap();
    assert_eq!(display_bpm.display_type, BPMDisplayType::Range);
    assert_eq!(display_bpm.value, 66.0);
    assert_eq!(display_bpm.value2, 132.0);
}

#[test]
fn parses_display_bpm_random() {
    let sim = parse_string_as_simfile("#DISPLAYBPM:*;").unwrap();
    let display_bpm = sim.display_bpm.unwrap();
    assert_eq!(display_bpm.display_type, BPMDisplayType::Random);
}

#[test]
fn parses_stops() {
    let sim = parse_string_as_simfile(
        "#STOPS:236.000=0.227,236.500=0.228,238.000=0.227,238.500=0.227,239.000=0.114;",
    )
    .unwrap();

    fn assert_stop(stop: &Stop, parsed_beat: f32, parsed_time: f32) {
        assert_eq!(stop.beat, parsed_beat);
        assert_eq!(stop.time, parsed_time);
    }

    // Is it parsed correctly?
    assert_stop(&sim.stops[1], 236.500, 0.228);
    assert_stop(&sim.stops[0], 236.000, 0.227);
    assert_eq!(sim.stops.len(), 5);
    assert_stop(&sim.stops[2], 238.000, 0.227);
    assert_stop(&sim.stops[3], 238.500, 0.227);
    assert_stop(&sim.stops[4], 239.000, 0.114);
}

const TEST_CHART: &str = "
#NOTES:
     dance-single:
     CondorTalon:
     Challenge:
     11:
     0.779,0.891,0.620,0.091,0.863:
0041
103K
2L1M
31ZF
;
";

#[test]
fn parses_chart_type() {
    let sim = parse_string_as_simfile(TEST_CHART).unwrap();
    assert_eq!(sim.charts[0].chart_type, "dance-single");
}

#[test]
fn parses_chart_author() {
    let sim = parse_string_as_simfile(TEST_CHART).unwrap();
    assert_eq!(sim.charts[0].author, Some("CondorTalon".to_string()));
}

#[test]
fn parses_chart_difficulty() {
    let sim = parse_string_as_simfile(TEST_CHART).unwrap();
    assert_eq!(sim.charts[0].difficulty, ChartDifficulty::Challenge);
}

#[test]
fn parses_chart_meter() {
    let sim = parse_string_as_simfile(TEST_CHART).unwrap();
    assert_eq!(sim.charts[0].meter, 11);
}

#[test]
fn parses_chart_radar_values() {
    let sim = parse_string_as_simfile(TEST_CHART).unwrap();
    assert_eq!(
        sim.charts[0].radar_values,
        vec![0.779, 0.891, 0.620, 0.091, 0.863]
    );
}

#[test]
fn parses_empty_bg_changes() {
    let sim = parse_string_as_simfile("#BGCHANGES:;").unwrap();
    assert_eq!(sim.bg_changes.len(), 0);
}

#[test]
fn parses_long_bg_changes() {
    let sim = parse_string_as_simfile("#BGCHANGES:5.400=Diamond Happy.mp4=1.000=1=0=0=StretchNoLoop==CrossFade==;").unwrap();
    assert_eq!(sim.bg_changes.len(), 1);
    let bg_changes = &sim.bg_changes[0];
    assert_eq!(bg_changes.start_beat, 5.400);
    assert_eq!(bg_changes.file_name, String::from("Diamond Happy.mp4"));
    assert_eq!(bg_changes.play_rate, 1.000);
    assert_eq!(bg_changes.transition_type, 1);
    assert_eq!(bg_changes.effect_flag, 0);
    assert_eq!(bg_changes.second_effect_flag, 0);
    return; // TODO: Remove this once we implement support for these fields
    assert_eq!(bg_changes.effect_file, Some(String::from("StretchNoLoop")));
    assert_eq!(bg_changes.second_effect_file, None);
    assert_eq!(bg_changes.transition_file, Some(String::from("CrossFade")));
    assert_eq!(bg_changes.color_string, None);
    assert_eq!(bg_changes.second_color_string, None);
}

#[test]
fn parses_bg_changes_with_multiple_entries_and_comment() {
    let sim = parse_string_as_simfile(
        "
        #BGCHANGES:6.167=Shuffle! On the Stage OP Video.avi=1.000=1=0=0,
        259.000=Shuffle! On The Stage BG2.jpg=1.000=1=0=0,
        99999=-nosongbg-=1.000=0=0=0 // don't automatically add -songbackground-
        ;
        ",
    ).unwrap();
    let bg_changes = &sim.bg_changes;

    assert_eq!(bg_changes.len(), 3);

    assert_eq!(bg_changes[0].start_beat, 6.167);
    assert_eq!(bg_changes[0].file_name, String::from("Shuffle! On the Stage OP Video.avi"));
    assert_eq!(bg_changes[0].play_rate, 1.000);
    assert_eq!(bg_changes[0].transition_type, 1);
    assert_eq!(bg_changes[0].effect_flag, 0);
    assert_eq!(bg_changes[0].second_effect_flag, 0);
    assert_eq!(bg_changes[0].effect_file, None);
    assert_eq!(bg_changes[0].second_effect_file, None);
    assert_eq!(bg_changes[0].transition_file, None);
    assert_eq!(bg_changes[0].color_string, None);
    assert_eq!(bg_changes[0].second_color_string, None);

    assert_eq!(bg_changes[1].start_beat, 259.000);
    assert_eq!(bg_changes[1].file_name, String::from("Shuffle! On The Stage BG2.jpg"));
    assert_eq!(bg_changes[1].play_rate, 1.000);
    assert_eq!(bg_changes[1].transition_type, 1);
    assert_eq!(bg_changes[1].effect_flag, 0);
    assert_eq!(bg_changes[1].second_effect_flag, 0);
    assert_eq!(bg_changes[1].effect_file, None);
    assert_eq!(bg_changes[1].second_effect_file, None);
    assert_eq!(bg_changes[1].transition_file, None);
    assert_eq!(bg_changes[1].color_string, None);
    assert_eq!(bg_changes[1].second_color_string, None);

    assert_eq!(bg_changes[2].start_beat, 99999.0);
    assert_eq!(bg_changes[2].file_name, String::from("-nosongbg-"));
    assert_eq!(bg_changes[2].play_rate, 1.000);
    assert_eq!(bg_changes[2].transition_type, 0);
    assert_eq!(bg_changes[2].effect_flag, 0);
    assert_eq!(bg_changes[2].second_effect_flag, 0);
    assert_eq!(bg_changes[2].effect_file, None);
    assert_eq!(bg_changes[2].second_effect_file, None);
    assert_eq!(bg_changes[2].transition_file, None);
    assert_eq!(bg_changes[2].color_string, None);
    assert_eq!(bg_changes[2].second_color_string, None);
}

#[test]
fn parses_chart_measures() {
    let sim = parse_string_as_simfile(TEST_CHART).unwrap();
    assert_eq!(
        sim.charts[0].note_data[0],
        vec![
            // First row
            NoteType::None,
            NoteType::None,
            NoteType::RollHead,
            NoteType::Normal,
            // Second row
            NoteType::Normal,
            NoteType::None,
            NoteType::HoldOrRollTail,
            NoteType::AutomaticKeysound,
            // Third row
            NoteType::HoldHead,
            NoteType::LiftNote,
            NoteType::Normal,
            NoteType::Mine,
            // Fourth row
            NoteType::HoldOrRollTail,
            NoteType::Normal,
            NoteType::InvalidNote,
            NoteType::FakeNote,
        ]
    )
}

#[test]
fn parses_multiple_chart_measures() {
    // Also testing some weird edge case stuff with comments
    let sim = parse_string_as_simfile(
        "
        #NOTES:dance-single:CondorTalon:Challenge:11:0,0,0,0,0:
        1010, // Measure 1
        // Between measures
        0101  // Measure 2
        ;
        ",
    )
    .unwrap();

    assert_eq!(sim.charts[0].note_data.len(), 2);
    assert_eq!(
        sim.charts[0].note_data[0],
        vec![
            NoteType::Normal,
            NoteType::None,
            NoteType::Normal,
            NoteType::None
        ]
    );
    assert_eq!(
        sim.charts[0].note_data[1],
        vec![
            NoteType::None,
            NoteType::Normal,
            NoteType::None,
            NoteType::Normal
        ]
    );
}

#[test]
fn parsing_bpm_with_too_many_values_returns_error() {
    let sim = parse_string_as_simfile("#BPMS:133.000=210.0000=300.0;");
    assert_eq!(sim.err().unwrap(), SimfileParseError::FailedToParseBPMs);
}

#[test]
fn parsing_bpm_with_non_numerical_values_returns_error() {
    let sim = parse_string_as_simfile("#BPMS:AA=210.0000;");
    assert_eq!(sim.err().unwrap(), SimfileParseError::FailedToParseBPMs);
}

#[test]
fn parsing_stops_with_too_many_values_returns_error() {
    let sim = parse_string_as_simfile("#STOPS:133.000=210.0000=300.0;");
    assert_eq!(sim.err().unwrap(), SimfileParseError::FailedToParseStops);
}

#[test]
fn parsing_stops_with_non_numerical_values_returns_error() {
    let sim = parse_string_as_simfile("#STOPS:AA=210.0000;");
    assert_eq!(sim.err().unwrap(), SimfileParseError::FailedToParseStops);
}

#[test]
fn parsing_display_bpm_with_too_many_values_returns_error() {
    let sim = parse_string_as_simfile("#DISPLAYBPM:66.000:132.000:64.00;");
    assert_eq!(
        sim.err().unwrap(),
        SimfileParseError::TooManyValuesInDisplayBPM
    );
}

#[test]
fn parsing_bg_changes_with_too_few_values_returns_error() {
    let sim = parse_string_as_simfile("#BGCHANGES:5.400=Diamond Happy.mp4=;");
    assert!(sim.is_err());
    assert_eq!(
        sim.err().unwrap(),
        SimfileParseError::InvalidBgChangeFormat
    );
}

#[test]
fn parsing_empty_chart_returns_error() {
    let sim = parse_string_as_simfile("#NOTES:;");
    assert_eq!(sim.err().unwrap(), SimfileParseError::EmptyNotesSection);
}


#[test]
fn parsing_chart_with_invalid_header_returns_error() {
    let sim = parse_string_as_simfile("
        #NOTES:
             dance-single:
             :
             Challenge:
             0.733800,0.772920,0.048611,0.850698,0.060764,634.000000,628.000000,6.000000,105.000000,8.000000,0.000000,0.733800,0.772920,0.048611,0.850698,0.060764,634.000000,628.000000,6.000000,105.000000,8.000000,0.000000:
        0000
        0000
        0000
        0000;
    ");
    assert_eq!(sim.err().unwrap(), SimfileParseError::InvalidChartFormat);
}

#[test]
fn parsing_chart_with_undefined_difficulty_returns_error() {
    let sim = parse_string_as_simfile("
        #NOTES:
             dance-single:
             :
             NotADifficulty:
             10:
             0.733800,0.772920,0.048611,0.850698,0.060764,634.000000,628.000000,6.000000,105.000000,8.000000,0.000000,0.733800,0.772920,0.048611,0.850698,0.060764,634.000000,628.000000,6.000000,105.000000,8.000000,0.000000:
        0000
        0000
        0000
        0000;
    ");

    assert_eq!(
        sim.err().unwrap(),
        SimfileParseError::UnknownChartDifficulty
    );
}

#[test]
fn parsing_chart_with_non_numeric_meter_returns_error() {
    let sim = parse_string_as_simfile("
        #NOTES:
             dance-single:
             :
             Challenge:
             AMeter:
             0.733800,0.772920,0.048611,0.850698,0.060764,634.000000,628.000000,6.000000,105.000000,8.000000,0.000000,0.733800,0.772920,0.048611,0.850698,0.060764,634.000000,628.000000,6.000000,105.000000,8.000000,0.000000:
        0000
        0000
        0000
        0000;
    ");

    assert_eq!(
        sim.err().unwrap(),
        SimfileParseError::FailedToParseChartMeter
    );
}

#[test]
fn parse_simfile_parses_correctly() {
    let sim = load_and_parse_simfile("goin_under.sm").unwrap();

    // Is it parsed correctly?
    assert_eq!(sim.title, Some("Goin' Under".to_string()));
    assert_eq!(sim.artist, Some("NegaRen".to_string()));
    assert_eq!(sim.genre, Some("Raggacore".to_string()));
    assert_eq!(sim.banner_path, Some("bn.png".to_string()));
    assert_eq!(sim.background_path, Some("bg.png".to_string()));
    assert_eq!(sim.music_path, Some("Goin' Under.ogg".to_string()));
    assert_eq!(sim.offset, Some(0.0));
    assert_eq!(sim.sample_start, Some(45.714001));
    assert_eq!(sim.sample_length, Some(13.714000));
    assert_eq!(sim.selectable, Some(true));
    assert_eq!(sim.bpms.len(), 1);

    assert_bpm(&sim.bpms[0], 0.0, 210.0);

    println!("{:#?}", sim);

    // TODO: BG changes
    // TODO: Test chart
}

#[test]
fn parse_simfile_ignores_comments() {
    let sim = parse_string_as_simfile(
        "#TITLE:This is a//very cool title
        ;",
    )
    .unwrap();
    assert_eq!(sim.title, Some("This is a".to_string()));
}

#[test]
fn parse_simfile_parse_comment_issue() {
    // This should not panic
    let sim = parse_string_as_simfile(
        "
            // This is a comment: 23
            #TITLE:A title;
        ",
    )
    .unwrap();
    assert_eq!(sim.title, Some("A title".to_string()));
}

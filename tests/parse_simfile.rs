extern crate sm_parser;

#[cfg(test)]
use sm_parser::parse_simfile;
use sm_parser::simfile::{BPMDisplayType, Simfile, Stop, BPM};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

// TODO: Test errors
// TODO: Test chart parsing
// TODO: Test BGCHANGES/FGCHANGES parsing

fn load_and_parse_simfile(filename: &str) -> Simfile {
    // Load example file
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(format!("example_files/{}", filename));
    let file = File::open(d).unwrap();

    // Parse it!
    parse_simfile(&mut BufReader::new(file))
}

fn parse_string_as_simfile(data: &str) -> Simfile {
    parse_simfile(&mut BufReader::new(data.as_bytes()))
}

fn assert_bpm(bpm: &BPM, parsed_row: f32, parsed_bpm: f32) {
    assert_eq!(bpm.row, parsed_row);
    assert_eq!(bpm.bpm, parsed_bpm);
}

#[test]
fn parses_title() {
    let sim = parse_string_as_simfile("#TITLE:The Title;");
    assert_eq!(sim.title, Some("The Title".to_string()));
}

#[test]
fn parses_empty_title() {
    let sim = parse_string_as_simfile("#TITLE:;");
    assert_eq!(sim.title, None);
}

#[test]
fn parses_i18n_title() {
    // This is the only test where we explicitly test for
    // Kanji/Hiragana characters, just to make sure it's handling
    // non-ascii characters correctly in general
    let sim = parse_string_as_simfile("#TITLE:虹を編めたら;");
    assert_eq!(sim.title, Some("虹を編めたら".to_string()));
}

#[test]
fn parses_subtitle() {
    let sim = parse_string_as_simfile("#SUBTITLE:The Subtitle;");
    assert_eq!(sim.subtitle, Some("The Subtitle".to_string()));
}

#[test]
fn parses_empty_subtitle() {
    let sim = parse_string_as_simfile("#SUBTITLE:;");
    assert_eq!(sim.subtitle, None);
}

#[test]
fn parses_artist() {
    let sim = parse_string_as_simfile("#ARTIST:The Artist;");
    assert_eq!(sim.artist, Some("The Artist".to_string()));
}

#[test]
fn parses_empty_artist() {
    let sim = parse_string_as_simfile("#ARTIST:;");
    assert_eq!(sim.artist, None);
}

#[test]
fn parses_title_translit() {
    let sim = parse_string_as_simfile("#TITLETRANSLIT:The Title;");
    assert_eq!(sim.title_translit, Some("The Title".to_string()));
}

#[test]
fn parses_empty_title_translit() {
    let sim = parse_string_as_simfile("#TITLETRANSLIT:;");
    assert_eq!(sim.title_translit, None);
}

#[test]
fn parses_subtitle_translit() {
    let sim = parse_string_as_simfile("#SUBTITLETRANSLIT:The Subtitle;");
    assert_eq!(sim.subtitle_translit, Some("The Subtitle".to_string()));
}

#[test]
fn parses_empty_subtitle_translit() {
    let sim = parse_string_as_simfile("#SUBTITLETRANSLIT:;");
    assert_eq!(sim.subtitle_translit, None);
}

#[test]
fn parses_artist_translit() {
    let sim = parse_string_as_simfile("#ARTISTTRANSLIT:The Artist;");
    assert_eq!(sim.artist_translit, Some("The Artist".to_string()));
}

#[test]
fn parses_empty_artist_translit() {
    let sim = parse_string_as_simfile("#ARTISTTRANSLIT:;");
    assert_eq!(sim.artist_translit, None);
}

#[test]
fn parses_genre() {
    let sim = parse_string_as_simfile("#GENRE:The Genre;");
    assert_eq!(sim.genre, Some("The Genre".to_string()));
}

#[test]
fn parses_empty_genre() {
    let sim = parse_string_as_simfile("#GENRE:;");
    assert_eq!(sim.genre, None);
}

#[test]
fn parses_credit() {
    let sim = parse_string_as_simfile("#CREDIT:The Credit;");
    assert_eq!(sim.credit, Some("The Credit".to_string()));
}

#[test]
fn parses_empty_credit() {
    let sim = parse_string_as_simfile("#CREDIT:;");
    assert_eq!(sim.credit, None);
}

#[test]
fn parses_banner() {
    let sim = parse_string_as_simfile("#BANNER:TheBanner.png;");
    assert_eq!(sim.banner_path, Some("TheBanner.png".to_string()));
}

#[test]
fn parses_empty_banner() {
    let sim = parse_string_as_simfile("#BANNER:;");
    assert_eq!(sim.banner_path, None);
}

#[test]
fn parses_background() {
    let sim = parse_string_as_simfile("#BACKGROUND:TheBackground.png;");
    assert_eq!(sim.background_path, Some("TheBackground.png".to_string()));
}

#[test]
fn parses_empty_background() {
    let sim = parse_string_as_simfile("#BACKGROUND:;");
    assert_eq!(sim.background_path, None);
}

#[test]
fn parses_lyrics_path() {
    let sim = parse_string_as_simfile("#LYRICSPATH:TheLyrics.lrc;");
    assert_eq!(sim.lyrics_path, Some("TheLyrics.lrc".to_string()));
}

#[test]
fn parses_empty_lyrics_path() {
    let sim = parse_string_as_simfile("#LYRICSPATH:;");
    assert_eq!(sim.lyrics_path, None);
}

#[test]
fn parses_cd_title() {
    let sim = parse_string_as_simfile("#CDTITLE:TheCdTitle.png;");
    assert_eq!(sim.cd_title_path, Some("TheCdTitle.png".to_string()));
}

#[test]
fn parses_empty_cd_title() {
    let sim = parse_string_as_simfile("#CDTITLE:;");
    assert_eq!(sim.cd_title_path, None);
}

#[test]
fn parses_music() {
    let sim = parse_string_as_simfile("#MUSIC:TheMusic.ogg;");
    assert_eq!(sim.music_path, Some("TheMusic.ogg".to_string()));
}

#[test]
fn parses_empty_music() {
    let sim = parse_string_as_simfile("#MUSIC:;");
    assert_eq!(sim.music_path, None);
}

#[test]
fn parses_offset() {
    let sim = parse_string_as_simfile("#OFFSET:43.053;");
    assert_eq!(sim.offset, Some(43.053));
}

#[test]
fn parses_empty_offset() {
    let sim = parse_string_as_simfile("#OFFSET:;");
    assert_eq!(sim.offset, None);
}

#[test]
fn parses_sample_start() {
    let sim = parse_string_as_simfile("#SAMPLESTART:43.053;");
    assert_eq!(sim.sample_start, Some(43.053));
}

#[test]
fn parses_empty_sample_start() {
    let sim = parse_string_as_simfile("#SAMPLESTART:;");
    assert_eq!(sim.sample_start, None);
}

#[test]
fn parses_sample_length() {
    let sim = parse_string_as_simfile("#SAMPLELENGTH:43.053;");
    assert_eq!(sim.sample_length, Some(43.053));
}

#[test]
fn parses_empty_sample_length() {
    let sim = parse_string_as_simfile("#SAMPLELENGTH:;");
    assert_eq!(sim.sample_length, None);
}

#[test]
fn parses_true_selectable() {
    let sim = parse_string_as_simfile("#SELECTABLE:YES;");
    assert_eq!(sim.selectable, Some(true));
}

#[test]
fn parses_false_selectable() {
    let sim = parse_string_as_simfile("#SELECTABLE:NO;");
    assert_eq!(sim.selectable, Some(false));
}

#[test]
fn parses_empty_selectable() {
    let sim = parse_string_as_simfile("#SELECTABLE:;");
    assert_eq!(sim.selectable, None);
}

#[test]
fn parses_bpms() {
    let sim = parse_string_as_simfile(
        "#BPMS:0.000=132.000,237.000=33.000,237.125=66.000,237.250=132.000;",
    );

    // Is it parsed correctly?
    assert_eq!(sim.bpms.len(), 4);
    assert_bpm(&sim.bpms[0], 0.000, 132.000);
    assert_bpm(&sim.bpms[1], 237.000, 33.000);
    assert_bpm(&sim.bpms[2], 237.125, 66.000);
    assert_bpm(&sim.bpms[3], 237.250, 132.000);
}

#[test]
fn parses_display_bpm_none() {
    let sim = parse_string_as_simfile("#DISPLAYBPM:;");
    assert_eq!(sim.display_bpm.is_none(), true);
}

#[test]
fn parses_display_bpm_single() {
    let sim = parse_string_as_simfile("#DISPLAYBPM:66.000;");
    let display_bpm = sim.display_bpm.unwrap();
    assert_eq!(display_bpm.display_type, BPMDisplayType::Single);
    assert_eq!(display_bpm.value, 66.0);
}

#[test]
fn parses_display_bpm_range() {
    let sim = parse_string_as_simfile("#DISPLAYBPM:66.000:132.000;");
    let display_bpm = sim.display_bpm.unwrap();
    assert_eq!(display_bpm.display_type, BPMDisplayType::Range);
    assert_eq!(display_bpm.value, 66.0);
    assert_eq!(display_bpm.value2, 132.0);
}

#[test]
fn parses_display_bpm_random() {
    let sim = parse_string_as_simfile("#DISPLAYBPM:*;");
    let display_bpm = sim.display_bpm.unwrap();
    assert_eq!(display_bpm.display_type, BPMDisplayType::Random);
}

#[test]
fn parses_stops() {
    let sim = parse_string_as_simfile(
        "#STOPS:236.000=0.227,236.500=0.228,238.000=0.227,238.500=0.227,239.000=0.114;",
    );

    fn assert_stop(stop: &Stop, parsed_row: f32, parsed_time: f32) {
        assert_eq!(stop.row, parsed_row);
        assert_eq!(stop.time, parsed_time);
    }

    // Is it parsed correctly?
    assert_eq!(sim.stops.len(), 5);
    assert_stop(&sim.stops[0], 236.000, 0.227);
    assert_stop(&sim.stops[1], 236.500, 0.228);
    assert_stop(&sim.stops[2], 238.000, 0.227);
    assert_stop(&sim.stops[3], 238.500, 0.227);
    assert_stop(&sim.stops[4], 239.000, 0.114);
}

#[test]
fn parse_simfile_parses_correctly() {
    let sim = load_and_parse_simfile("goin_under.sm");

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

    // TODO: Test chart
}

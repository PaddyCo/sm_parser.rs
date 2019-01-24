extern crate sm_parser;

#[cfg(test)]
use sm_parser::parse_simfile;
use sm_parser::simfile::{BPMDisplayType, Simfile, Stop, BPM};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

// TODO: Add test for simfile that sets every possible value
// TODO: Add test for simfile that sets the least possible amount of values
// TODO: Add test for simfile with japanese properties etc
// TODO: Make better tests for properties (Look at the Display BPM tests)
// TODO: Test errors

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

fn assert_stop(stop: &Stop, parsed_row: f32, parsed_time: f32) {
    assert_eq!(stop.row, parsed_row);
    assert_eq!(stop.time, parsed_time);
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

#[test]
fn parses_multiple_bpms_correctly() {
    let sim = load_and_parse_simfile("news_39.sm");

    // Is it parsed correctly?
    assert_eq!(sim.bpms.len(), 35);
    assert_bpm(&sim.bpms[0], 0.000, 132.000);
    assert_bpm(&sim.bpms[1], 237.000, 33.000);
    assert_bpm(&sim.bpms[2], 237.125, 66.000);
    assert_bpm(&sim.bpms[3], 237.250, 132.000);
    assert_bpm(&sim.bpms[4], 237.500, 66.000);
    assert_bpm(&sim.bpms[5], 238.000, 132.000);
    assert_bpm(&sim.bpms[6], 239.000, 99.000);
    assert_bpm(&sim.bpms[7], 240.000, 132.000);
    assert_bpm(&sim.bpms[8], 252.000, 66.000);
    assert_bpm(&sim.bpms[9], 252.250, 132.000);
    assert_bpm(&sim.bpms[10], 252.500, 264.000);
    assert_bpm(&sim.bpms[11], 253.000, 132.000);
    assert_bpm(&sim.bpms[12], 262.500, -132.001);
    assert_bpm(&sim.bpms[13], 266.500, 132.000);
    assert_bpm(&sim.bpms[14], 270.500, 264.000);
    assert_bpm(&sim.bpms[15], 271.500, 132.000);
    assert_bpm(&sim.bpms[16], 282.500, 264.000);
    assert_bpm(&sim.bpms[17], 284.000, 396.000);
    assert_bpm(&sim.bpms[18], 284.750, 44.000);
    assert_bpm(&sim.bpms[19], 285.000, 132.000);
    assert_bpm(&sim.bpms[20], 290.500, 396.000);
    assert_bpm(&sim.bpms[21], 290.875, 44.000);
    assert_bpm(&sim.bpms[22], 291.000, 396.000);
    assert_bpm(&sim.bpms[23], 291.375, 44.000);
    assert_bpm(&sim.bpms[24], 291.500, 396.000);
    assert_bpm(&sim.bpms[25], 291.688, 44.000);
    assert_bpm(&sim.bpms[26], 291.750, 396.000);
    assert_bpm(&sim.bpms[27], 291.938, 44.000);
    assert_bpm(&sim.bpms[28], 292.000, 132.000);
    assert_bpm(&sim.bpms[29], 296.500, 264.000);
    assert_bpm(&sim.bpms[30], 298.500, 132.000);
    assert_bpm(&sim.bpms[31], 303.000, 264.000);
    assert_bpm(&sim.bpms[32], 303.500, 132.000);
    assert_bpm(&sim.bpms[33], 303.625, 264.000);
    assert_bpm(&sim.bpms[34], 304.000, 132.000);
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
fn parses_multiple_stops() {
    let sim = load_and_parse_simfile("news_39.sm");

    // Is it parsed correctly?
    assert_eq!(sim.stops.len(), 27);
    assert_stop(&sim.stops[0], 236.000, 0.227);
    assert_stop(&sim.stops[1], 236.500, 0.228);
    assert_stop(&sim.stops[2], 238.000, 0.227);
    assert_stop(&sim.stops[3], 238.500, 0.227);
    assert_stop(&sim.stops[4], 239.000, 0.114);
    assert_stop(&sim.stops[5], 239.375, 0.114);
    assert_stop(&sim.stops[6], 239.750, 0.075);
    assert_stop(&sim.stops[7], 270.500, 0.227);
    assert_stop(&sim.stops[8], 282.500, 0.057);
    assert_stop(&sim.stops[9], 282.750, 0.057);
    assert_stop(&sim.stops[10], 283.000, 0.113);
    assert_stop(&sim.stops[11], 283.500, 0.114);
    assert_stop(&sim.stops[12], 296.500, 0.057);
    assert_stop(&sim.stops[13], 296.750, 0.057);
    assert_stop(&sim.stops[14], 297.000, 0.057);
    assert_stop(&sim.stops[15], 297.250, 0.057);
    assert_stop(&sim.stops[16], 297.500, 0.038);
    assert_stop(&sim.stops[17], 297.667, 0.038);
    assert_stop(&sim.stops[18], 297.833, 0.038);
    assert_stop(&sim.stops[19], 298.000, 0.028);
    assert_stop(&sim.stops[20], 298.125, 0.028);
    assert_stop(&sim.stops[21], 298.250, 0.028);
    assert_stop(&sim.stops[22], 298.375, 0.028);
    assert_stop(&sim.stops[23], 303.000, 0.057);
    assert_stop(&sim.stops[24], 303.250, 0.057);
    assert_stop(&sim.stops[25], 303.625, 0.057);
    assert_stop(&sim.stops[26], 303.875, 0.028);
}

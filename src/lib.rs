mod simfile;
use simfile::SMSimfile;
use std::io::BufRead;

pub fn parse_simfile<R: BufRead>(reader: &mut R) -> SMSimfile {
    let mut sim = SMSimfile::new();

    // TODO: Instead of going line-by-line, check if we can split it up in sections using the
    // semicolon as the delimiter? This would lead to better behaviour when parsing multi-line
    // properties (BPMs, Notechart etc)
    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(count) => {
                if count == 0 {
                    break;
                }
                parse_line(&mut sim, &line);
            }
            Err(e) => {
                // TODO: Return error instead of printing to console
                println!("ERROR: {}", e);
                break;
            }
        };
    }

    return sim;
}

fn parse_line(simfile: &mut SMSimfile, line: &str) {
    let key_end_index = match line.find(':') {
        Some(i) => i,
        None => return,
    };

    let value_end_index = match line.find(';') {
        Some(i) => i,
        None => return,
    };

    let key = &line[1..key_end_index];
    let val = &line[key_end_index + 1..value_end_index];
    let value = if val.len() > 0 {
        Some(val.to_string())
    } else {
        None
    };

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
        _ => {}
    }

    return;
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

#[cfg(test)]
mod tests {
    use crate::simfile::SMSimfile;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    // TODO: Add test for simfile that sets every possible value
    // TODO: Add test for simfile that sets the least possible amount of values

    #[test]
    fn parse_simfile_parses_correctly() {
        // Load example file
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("example_files/goin_under.sm");
        let file = File::open(d).unwrap();

        // Parse it!
        let sim = crate::parse_simfile(&mut BufReader::new(file));

        println!("{:?}", sim);

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
        assert_eq!(sim.bpms[0].row, 0.0);
        assert_eq!(sim.bpms[0].bpm, 210.0);

        // TODO: Test chart
    }
}

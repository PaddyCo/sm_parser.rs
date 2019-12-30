use std::fmt;

#[derive(Debug, Clone)]
pub struct BPM {
    /// The row where the BPM gets set
    pub row: f32,
    /// The BPM to set
    pub bpm: f32,
}

#[derive(Debug, Clone)]
pub struct Stop {
    /// The row the stop occurs
    pub row: f32,
    /// How long the stop lasts
    pub time: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BPMDisplayType {
    /// Only a single value will be displayed
    Single,
    /// A range will be displayed
    Range,
    /// Will randomly change
    Random,
}

#[derive(Debug, Clone)]
pub struct DisplayBPM {
    /// Sets how the BPM should be displayed
    pub display_type: BPMDisplayType,
    /// The first value (Used if `display_type` is `SINGLE` or `RANGE`)
    pub value: f32,
    /// The second value (Used if `display_type` is `RANGE`)
    pub value2: f32,
}

#[derive(Debug, Clone)]
pub struct BgChange {
    /// Start beat for bg change
    pub start_beat: f32,
    /// File or folder name
    pub file_name: String,
    /// Play rate for effect
    pub play_rate: f32,
    /// Backward compatible transition type, CrossFade is used if this is not 0.
    pub transition_type: i8,
    /// Backward compatible effect flag, StretchRewind is used if this is not 0.
    pub effect_flag: i8,
    /// Backward compatible effect flag, StretchNoLoop is used if this is not 0.
    pub second_effect_flag: i8,
    /// name of the effect file to use. The BackgroundEffects folder will be searched for a match.
    pub effect_file: Option<String>,
    /// name of the second effect file to use. The BackgroundEffects folder will be searched for a match.
    pub second_effect_file: Option<String>,
    /// name of the the transition file to use. The BackgroundTransitions folder will be searched for a match.
    pub transition_file: Option<String>,
    /// Color string in either "1.0^0.5^0.75^0.25" or "#ff7fcf3f" form. The fourth channel is optional.
    pub color_string: Option<String>,
    /// Second color string in either "1.0^0.5^0.75^0.25" or "#ff7fcf3f" form. The fourth channel is optional.
    pub second_color_string: Option<String>
}

#[derive(Debug, Clone)]
pub struct FgChange {
    pub start_beat: f32,
    /// File or folder name
    pub path: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChartDifficulty {
    Beginner,
    Easy,
    Medium,
    Hard,
    Challenge,
    Edit,
}

impl fmt::Display for ChartDifficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NoteType {
    None,
    Normal,
    HoldHead,
    HoldOrRollTail,
    RollHead,
    Mine,
    AutomaticKeysound,
    LiftNote,
    FakeNote,
    InvalidNote,
}

#[derive(Debug, Clone)]
pub struct Chart {
    pub chart_type: String,
    pub author: Option<String>,
    pub difficulty: ChartDifficulty,
    /// The numerical difficulty level of the chart
    pub meter: u16,
    // TODO: Figure out what all these radar values actually are
    pub radar_values: Vec<f32>,
    /// Note data is defined in terms of "measures", every "measure" contains the notes for that
    /// measures, which is represented as a flat vector of notes, where there is one entry for
    /// every lane and line in the measure.
    pub note_data: Vec<Vec<NoteType>>,
}

/// Represents a parsed Stepmania stepfile (.sm)
#[derive(Debug, Clone)]
pub struct Simfile {
    /// The primary title of the song.
    pub title: Option<String>,
    /// The subtitle of the song.
    pub subtitle: Option<String>,
    /// The artist of the song.
    pub artist: Option<String>,
    /// The transliterated primarty title of the song.
    pub title_translit: Option<String>,
    /// The transliterated subtitle of the song.
    pub subtitle_translit: Option<String>,
    /// The transliterated artist of the song.
    pub artist_translit: Option<String>,
    /// The genre of the song.
    pub genre: Option<String>,
    /// The Creator/Credits.
    pub credit: Option<String>,
    /// Path to the Banner. (Relative from the Song's directory.)
    pub banner_path: Option<String>,
    /// Path to the Background. (Relative from the Song's directory.)
    pub background_path: Option<String>,
    /// Path to the Preview Video. (Relative from the Song's directory.)
    pub preview_video_path: Option<String>,
    /// Path to the Jacket. (Relative from the Song's directory.)
    pub jacket_path: Option<String>,
    /// Path to the CD Image. (Relative from the Song's directory.)
    pub lyrics_path: Option<String>,
    /// The Song's CD Title image, small image meant to show the origin of the song.
    pub cd_title_path: Option<String>,
    /// Path to the Audio file for the Song. (Relative from the Song's directory.)
    pub music_path: Option<String>,
    /// The Song's Offset. (Effects the timing of the start of the Notes.)
    pub offset: Option<f32>,
    /// The Song's Sample Start Time
    pub sample_start: Option<f32>,
    /// The Song's Sample Length
    pub sample_length: Option<f32>,
    /// Is this song selectable?
    pub selectable: Option<bool>,
    /// The Song's Beats Per Minute's at certain times. (Can have multiple changes.)
    pub bpms: Vec<BPM>,
    /// The displayed BPM shown in-game
    pub display_bpm: Option<DisplayBPM>,
    /// The Song's Stops. (Can have multiple changes.)
    pub stops: Vec<Stop>,
    /// Is used to control what backgrounds are loaded by the simfile and when they appear.
    pub bg_changes: Vec<BgChange>,
    /// Is used to control what foregrounds are loaded by the simfile and when they appear.
    pub fg_changes: Vec<FgChange>,
    /// The charts of the simfile
    pub charts: Vec<Chart>,
}

impl Simfile {
    pub fn new() -> Simfile {
        Simfile {
            title: None,
            subtitle: None,
            artist: None,
            title_translit: None,
            subtitle_translit: None,
            artist_translit: None,
            genre: None,
            credit: None,
            banner_path: None,
            background_path: None,
            preview_video_path: None,
            jacket_path: None,
            lyrics_path: None,
            cd_title_path: None,
            music_path: None,
            offset: None,
            sample_start: None,
            sample_length: None,
            selectable: None,
            bpms: Vec::new(),
            display_bpm: None,
            stops: Vec::new(),
            bg_changes: Vec::new(),
            fg_changes: Vec::new(),
            charts: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Simfile;

    #[test]
    fn sm_simfile_new_initializes_empty() {
        let new_simfile = Simfile::new();

        assert_eq!(new_simfile.title.is_none(), true);
        assert_eq!(new_simfile.subtitle.is_none(), true);
        assert_eq!(new_simfile.artist.is_none(), true);
        assert_eq!(new_simfile.title_translit.is_none(), true);
        assert_eq!(new_simfile.subtitle_translit.is_none(), true);
        assert_eq!(new_simfile.artist_translit.is_none(), true);
        assert_eq!(new_simfile.genre.is_none(), true);
        assert_eq!(new_simfile.credit.is_none(), true);
        assert_eq!(new_simfile.banner_path.is_none(), true);
        assert_eq!(new_simfile.background_path.is_none(), true);
        assert_eq!(new_simfile.preview_video_path.is_none(), true);
        assert_eq!(new_simfile.jacket_path.is_none(), true);
        assert_eq!(new_simfile.lyrics_path.is_none(), true);
        assert_eq!(new_simfile.cd_title_path.is_none(), true);
        assert_eq!(new_simfile.music_path.is_none(), true);
        assert_eq!(new_simfile.offset.is_none(), true);
        assert_eq!(new_simfile.sample_start.is_none(), true);
        assert_eq!(new_simfile.sample_length.is_none(), true);
        assert_eq!(new_simfile.selectable.is_none(), true);
        assert_eq!(new_simfile.bpms.len(), 0);
        assert_eq!(new_simfile.display_bpm.is_none(), true);
        assert_eq!(new_simfile.stops.len(), 0);
        assert_eq!(new_simfile.bg_changes.len(), 0);
        assert_eq!(new_simfile.fg_changes.len(), 0);
        assert_eq!(new_simfile.charts.len(), 0);
    }
}

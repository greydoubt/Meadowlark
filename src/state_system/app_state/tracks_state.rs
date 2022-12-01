use super::PaletteColor;

pub static DEFAULT_TRACK_LANE_HEIGHT: f32 = 60.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackType {
    Audio,
    Synth,
    //Folder, // TODO
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrackRouteType {
    ToMaster,
    ToTrackWithIndex(usize),
    None,
}

#[derive(Debug, Clone)]
pub struct TrackState {
    pub name: String,
    pub color: PaletteColor,
    pub lane_height: f32,
    pub type_: TrackType,
    pub volume_normalized: f32,
    pub pan_normalized: f32,

    pub routed_to: TrackRouteType,
    //pub parent_track_index: Option<usize>, // TODO
}

#[derive(Debug, Clone)]
pub struct TracksState {
    pub master_track_color: PaletteColor,
    pub master_track_lane_height: f32,
    pub master_track_volume_normalized: f32,
    pub master_track_pan_normalized: f32,

    pub tracks: Vec<TrackState>,
}

impl TracksState {
    pub fn new() -> Self {
        Self {
            master_track_color: PaletteColor::Unassigned,
            master_track_lane_height: DEFAULT_TRACK_LANE_HEIGHT,
            master_track_volume_normalized: 1.0,
            master_track_pan_normalized: 0.5,

            tracks: vec![
                TrackState {
                    name: "Spicy Synth".into(),
                    color: PaletteColor::Color0,
                    lane_height: DEFAULT_TRACK_LANE_HEIGHT,
                    type_: TrackType::Synth,
                    volume_normalized: 1.0,
                    pan_normalized: 0.5,
                    routed_to: TrackRouteType::ToMaster,
                },
                TrackState {
                    name: "Drum Hits".into(),
                    color: PaletteColor::Color1,
                    lane_height: DEFAULT_TRACK_LANE_HEIGHT,
                    type_: TrackType::Audio,
                    volume_normalized: 1.0,
                    pan_normalized: 0.5,
                    routed_to: TrackRouteType::ToMaster,
                },
            ],
        }
    }
}

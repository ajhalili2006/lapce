use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum PanelPosition {
    LeftTop,
    LeftBottom,
    BottomLeft,
    BottomRight,
    RightTop,
    RightBottom,
}

impl PanelPosition {
    pub fn is_bottom(&self) -> bool {
        matches!(self, PanelPosition::BottomLeft | PanelPosition::BottomRight)
    }

    pub fn peer(&self) -> PanelPosition {
        match &self {
            PanelPosition::LeftTop => PanelPosition::LeftBottom,
            PanelPosition::LeftBottom => PanelPosition::LeftTop,
            PanelPosition::BottomLeft => PanelPosition::BottomRight,
            PanelPosition::BottomRight => PanelPosition::BottomLeft,
            PanelPosition::RightTop => PanelPosition::RightBottom,
            PanelPosition::RightBottom => PanelPosition::RightTop,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum PanelContainerPosition {
    Left,
    Bottom,
    Right,
}

impl PanelContainerPosition {
    pub fn is_bottom(&self) -> bool {
        matches!(self, PanelContainerPosition::Bottom)
    }

    pub fn first(&self) -> PanelPosition {
        match self {
            PanelContainerPosition::Left => PanelPosition::LeftTop,
            PanelContainerPosition::Bottom => PanelPosition::BottomLeft,
            PanelContainerPosition::Right => PanelPosition::RightTop,
        }
    }

    pub fn second(&self) -> PanelPosition {
        match self {
            PanelContainerPosition::Left => PanelPosition::LeftBottom,
            PanelContainerPosition::Bottom => PanelPosition::BottomRight,
            PanelContainerPosition::Right => PanelPosition::RightBottom,
        }
    }
}
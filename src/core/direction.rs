use serde::Serialize;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Direction {
    Up,
    Down,
    North,
    East,
    South,
    West,
    In,
    Out,
}

impl Direction {
    pub fn from(input: &str) -> Option<Direction> {
        let d = match input.to_uppercase().as_ref() {
            "UP" => Self::Up,
            "DOWN" => Self::Down,
            "NORTH" => Self::North,
            "EAST" => Self::East,
            "SOUTH" => Self::South,
            "WEST" => Self::West,
            "IN" => Self::In,
            "OUT" => Self::Out,
            _ => return None,
        };

        Some(d)
    }
}

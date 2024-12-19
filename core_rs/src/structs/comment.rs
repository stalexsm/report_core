use super::coordinate::Coordinate;

#[derive(Clone, Default, Debug)]
pub struct Comment {
    coordinate: Coordinate,
    author: Box<str>,
    text: String,
}

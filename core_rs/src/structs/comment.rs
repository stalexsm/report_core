use serde::Serialize;

use super::coordinate::Coordinate;

#[derive(Clone, Default, Debug, Serialize)]
pub struct Comment {
    #[serde(flatten)]
    coordinate: Coordinate,
    author: Box<str>,
    text: String,
}

impl Comment {
    pub fn new(coordinate: Coordinate, author: &str) -> Self {
        Self {
            coordinate,
            author: Box::from(author),
            ..Default::default()
        }
    }

    #[inline]
    pub fn get_coordinate(&self) -> &Coordinate {
        &self.coordinate
    }

    #[inline]
    pub fn get_author(&self) -> &str {
        &self.author
    }

    #[inline]
    pub fn set_author<S: Into<String>>(&mut self, value: S) -> &mut Self {
        self.author = value.into().into_boxed_str();
        self
    }

    #[inline]
    pub fn get_text(&self) -> &str {
        &self.text
    }

    #[inline]
    pub fn set_text(&mut self, value: &str) -> &mut Self {
        self.text = value.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_comment() {
        let comment = Comment::new(Coordinate::new(1, 1), "A.C");

        assert_eq!(comment.get_author(), "A.C");
        assert_eq!(comment.get_coordinate(), &Coordinate::new(1, 1));
        assert_eq!(comment.get_text(), "");
    }

    #[test]
    fn author() {
        let mut comment = Comment::new(Coordinate::new(1, 1), "A.C");
        comment.set_author("A.A.C");

        assert_eq!(comment.get_author(), "A.A.C");
    }

    #[test]
    fn text() {
        let mut comment = Comment::new(Coordinate::new(1, 1), "A.C");
        comment.set_text("Вот комментарий!");

        assert_eq!(comment.get_text(), "Вот комментарий!");
    }
}

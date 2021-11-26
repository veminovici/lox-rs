use std::fmt::Debug;

/// The line in the source stream
#[derive(Clone, Copy)]
pub struct Line(usize);

/// The column in the source stream
#[derive(Clone, Copy)]
pub struct Column(usize);

/// The position in the stream
pub struct Span {
    start_line: Line,
    start_col: Column,
    end_line: Line,
    end_col: Column,
}

impl Span {
    /// Creates a new one-char span
    pub fn new(l: Line, c: Column) -> Self {
        Span {
            start_line: l,
            start_col: c,
            end_line: l,
            end_col: c,
        }
    }

    /// Returns true if the span is a one-line one.
    #[inline]
    pub fn is_one_line(&self) -> bool {
        self.start_line.0 == self.end_line.0
    }

    /// Returns true if the span is one-char one.
    #[inline]
    pub fn is_one_char(&self) -> bool {
        self.is_one_line() && self.start_col.0 == self.end_col.0
    }

    /// Returns true if the span is a multi-line one.
    #[inline]
    pub fn is_multi_line(&self) -> bool {
        !self.is_one_line()
    }

    /// Increments the column of a span
    pub fn incr_col(&mut self) {
        self.end_col = Column(self.end_col.0 + 1);
    }

    /// Increment the line of a span
    pub fn incr_line(&mut self) {
        self.end_line = Line(self.end_line.0 + 1);
        self.end_col = Column(0);
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_one_char() {
            write!(f, "{}:{}", self.start_line.0, self.start_col.0)
        } else if self.is_one_line() {
            write!(
                f,
                "{}:{}-{}",
                self.start_line.0, self.start_col.0, self.end_col.0
            )
        } else {
            write!(
                f,
                "{}:{}-{}:{}",
                self.start_line.0, self.start_col.0, self.end_line.0, self.end_col.0
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let s = Span::new(Line(10), Column(100));
        assert_eq!(10, s.start_line.0);
        assert_eq!(100, s.start_col.0);
        assert_eq!(10, s.end_line.0);
        assert_eq!(100, s.end_col.0);
        assert!(s.is_one_char())
    }

    #[test]
    fn test_one_char_incr_col_incr_line() {
        let mut s = Span::new(Line(10), Column(100));

        s.incr_col();
        s.incr_col();
        assert!(s.is_one_line());
        assert_eq!(s.start_line.0, 10);
        assert_eq!(s.start_col.0, 100);
        assert_eq!(s.end_line.0, 10);
        assert_eq!(s.end_col.0, 102);

        s.incr_line();
        assert!(s.is_multi_line());
        assert_eq!(s.start_line.0, 10);
        assert_eq!(s.start_col.0, 100);
        assert_eq!(s.end_line.0, 11);
        assert_eq!(s.end_col.0, 0);

        s.incr_col();
        s.incr_col();
        assert!(s.is_multi_line());
        assert_eq!(s.start_line.0, 10);
        assert_eq!(s.start_col.0, 100);
        assert_eq!(s.end_line.0, 11);
        assert_eq!(s.end_col.0, 2);

        s.incr_line();
        assert!(s.is_multi_line());
        assert_eq!(s.start_line.0, 10);
        assert_eq!(s.start_col.0, 100);
        assert_eq!(s.end_line.0, 12);
        assert_eq!(s.end_col.0, 0);
    }

    #[test]
    fn test_one_char_incr_line() {
        let mut s = Span::new(Line(10), Column(100));

        s.incr_line();
        assert!(s.is_multi_line());
        assert_eq!(s.start_line.0, 10);
        assert_eq!(s.start_col.0, 100);
        assert_eq!(s.end_line.0, 11);
        assert_eq!(s.end_col.0, 0);
    }
}

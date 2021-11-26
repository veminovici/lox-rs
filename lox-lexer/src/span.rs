use std::fmt::Debug;

/// The line in the source stream
#[derive(Clone, Copy, PartialEq)]
pub struct Line(pub usize);

/// The column in the source stream
#[derive(Clone, Copy, PartialEq)]
pub struct Column(pub usize);

/// The position in the stream
#[derive(Clone, Copy, PartialEq)]
pub struct Span {
    start_line: Line,
    pub(crate) start_col: Column,
    end_line: Line,
    pub(crate) end_col: Column,
}

impl Default for Span {
    fn default() -> Self {
        Self {
            start_line: Line(1),
            start_col: Column(0),
            end_line: Line(1),
            end_col: Column(0),
        }
    }
}

impl Span {
    /// Creates a new one-char span
    pub fn new(l: Line, c: Column) -> Self {
        Span {
            start_line: l,
            start_col: c,
            end_line: l,
            end_col: Column(c.0 + 1),
        }
    }

    /// Returns true if the span is a one-line one.
    #[inline]
    pub fn is_one_line(&self) -> bool {
        self.start_line.0 == self.end_line.0
    }

    /// Returns true if the span is on the same line and has n characters.
    pub fn is_n_chars(&self, n: usize) -> bool {
        self.is_one_line() && self.end_col.0 - self.start_col.0 == n
    }

    /// Returns true if the span is a two-chars one.
    #[inline]
    pub fn is_two_chars(&self) -> bool {
        self.is_n_chars(2)
    }

    /// Returns true if the span is one-char one.
    #[inline]
    pub fn is_one_char(&self) -> bool {
        self.is_n_chars(1)
    }

    /// Returns true if the span is a multi-line one.
    #[inline]
    pub fn is_multi_line(&self) -> bool {
        !self.is_one_line()
    }

    /// Increments the coumn of a span
    #[inline]
    pub fn incr_col_n(&mut self, n: usize) {
        self.end_col = Column(self.end_col.0 + n);
    }

    /// Increments the column of a span
    #[inline]
    pub fn incr_col(&mut self) {
        self.incr_col_n(1)
    }

    /// Increment the line of a span
    pub fn incr_line(&mut self) {
        self.end_line = Line(self.end_line.0 + 1);
        self.end_col = Column(0);
    }

    /// Completes a span and starts a new one.
    pub fn complete(&mut self) -> Self {
        let s = *self;

        self.start_line = self.end_line;
        let e = self.end_col;
        self.start_col = e;
        self.end_col = e;

        s
    }
}

impl Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_one_line() {
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
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let s = Span::new(Line(10), Column(100));
        assert_eq!(10, s.start_line.0);
        assert_eq!(100, s.start_col.0);
        assert_eq!(10, s.end_line.0);
        assert_eq!(101, s.end_col.0);
        assert!(s.is_one_char())
    }

    #[test]
    fn test_one_char_incr_col_incr_line() {
        let mut s = Span::new(Line(10), Column(100));

        s.incr_col();
        s.incr_col();
        assert!(s.is_one_line());
        assert_eq!(10, s.start_line.0);
        assert_eq!(100, s.start_col.0);
        assert_eq!(10, s.end_line.0);
        assert_eq!(103, s.end_col.0);

        s.incr_line();
        assert!(s.is_multi_line());
        assert_eq!(10, s.start_line.0);
        assert_eq!(100, s.start_col.0);
        assert_eq!(11, s.end_line.0);
        assert_eq!(0, s.end_col.0);

        s.incr_col();
        s.incr_col();
        assert!(s.is_multi_line());
        assert_eq!(10, s.start_line.0);
        assert_eq!(100, s.start_col.0);
        assert_eq!(11, s.end_line.0);
        assert_eq!(2, s.end_col.0);

        s.incr_line();
        assert!(s.is_multi_line());
        assert_eq!(10, s.start_line.0);
        assert_eq!(100, s.start_col.0);
        assert_eq!(12, s.end_line.0);
        assert_eq!(0, s.end_col.0);
    }

    #[test]
    fn test_one_char_incr_line() {
        let mut s = Span::new(Line(10), Column(100));

        s.incr_line();
        assert!(s.is_multi_line());
        assert_eq!(10, s.start_line.0);
        assert_eq!(100, s.start_col.0);
        assert_eq!(11, s.end_line.0);
        assert_eq!(0, s.end_col.0);
    }

    #[test]
    fn test_complete_one_char() {
        let mut s = Span::new(Line(10), Column(100));
        let s1 = s.complete();

        assert!(s1.is_one_char());
        assert_eq!(10, s1.start_line.0);
        assert_eq!(100, s1.start_col.0);
        assert_eq!(10, s1.end_line.0);
        assert_eq!(101, s1.end_col.0);

        assert_eq!(10, s.start_line.0);
        assert_eq!(101, s.start_col.0);
        assert_eq!(10, s.end_line.0);
        assert_eq!(101, s.end_col.0);
    }
}

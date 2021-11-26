pub(crate) const CHAR_NEWLINE: char = '\n';

pub(crate) const CHAR_LEFT_PAREN: char = '(';
pub(crate) const CHAR_RIGHT_PAREN: char = ')';
pub(crate) const CHAR_LEFT_BRACE: char = '{';
pub(crate) const CHAR_RIGHT_BRACE: char = '}';
pub(crate) const CHAR_COMMA: char = ',';
pub(crate) const CHAR_DOT: char = '.';
pub(crate) const CHAR_PLUS: char = '+';
pub(crate) const CHAR_MINUS: char = '-';
pub(crate) const CHAR_SEMICOLON: char = ';';
pub(crate) const CHAR_STAR: char = '*';
pub(crate) const CHAR_BANG: char = '!';
pub(crate) const CHAR_EQUAL: char = '=';
pub(crate) const CHAR_GREATER: char = '>';
pub(crate) const CHAR_LESS: char = '<';
pub(crate) const CHAR_SLASH: char = '/';

pub(crate) const CHAR_WHITESPACE: char = ' ';
pub(crate) const CHAR_CARRIAGE_RETURN: char = '\r';
pub(crate) const CHAR_TAB: char = '\t';

pub(crate) const CHAR_DOUBLE_QUOTE: char = '"';

pub(crate) const CHAR_0: char = '0';
pub(crate) const CHAR_9: char = '9';

pub(crate) const CHAR_LOWERCASE_A: char = 'a';
pub(crate) const CHAR_LOWERCASE_Z: char = 'z';
pub(crate) const CHAR_UPPERCASE_A: char = 'A';
pub(crate) const CHAR_UPPERCASE_Z: char = 'Z';
pub(crate) const CHAR_UNDERSCORE: char = '_';

#[inline]
pub(crate) fn is_whitespace(c: char) -> bool {
    c == CHAR_WHITESPACE || c == CHAR_TAB || c == CHAR_CARRIAGE_RETURN
}

#[inline]
pub(crate) fn is_digit(c: char) -> bool {
    c >= CHAR_0 && c <= CHAR_9
}

#[inline]
pub(crate) fn is_alpha(c: char) -> bool {
    c >= CHAR_LOWERCASE_A && c <= CHAR_LOWERCASE_Z
        || c >= CHAR_UPPERCASE_A && c <= CHAR_UPPERCASE_Z
        || c == CHAR_UNDERSCORE
}

#[inline]
pub(crate) fn is_alphanum(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

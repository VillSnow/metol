pub fn is_line_break_char(c: char) -> bool {
    matches!(
        c,
        '\r' | '\n' | '\u{000B}' | '\u{000C}' | '\u{0085}' | '\u{2028}' | '\u{2029}'
    )
}

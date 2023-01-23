//! Escaping and unescaping functions.

use crate::constants::Encoding;

// TODO: Replace with nom and differentiate by property, param, and value types when needed.
pub fn escape(str: &str) -> String {
    let mut string = String::new();

    for char in str.chars() {
        match char {
            Encoding::UNESCAPED_BACKSLASH => string.push_str(Encoding::ESCAPED_BACKSLASH),
            Encoding::UNESCAPED_COMMA => string.push_str(Encoding::ESCAPED_COMMA),
            Encoding::UNESCAPED_LF => string.push_str(Encoding::ESCAPED_LF),
            Encoding::UNESCAPED_SEMICOLON => string.push_str(Encoding::ESCAPED_SEMICOLON),
            Encoding::UNESCAPED_TAB => string.push_str(Encoding::ESCAPED_TAB),
            _ => string.push(char),
        }
    }

    string
}

// TODO: Replace with nom and differentiate by property, param, and value types when needed.
pub fn unescape(str: &str) -> String {
    let mut string = String::new();

    let mut chars = str.chars().peekable();
    while let Some(char) = chars.next() {
        match char {
            Encoding::UNESCAPED_BACKSLASH => match chars.next() {
                Some(Encoding::UNESCAPED_BACKSLASH) => match chars.peek() {
                    Some('n') => {
                        string.push(Encoding::UNESCAPED_LF);
                        chars.next();
                    }
                    Some('t') => {
                        string.push(Encoding::UNESCAPED_TAB);
                        chars.next();
                    }
                    _ => string.push(char),
                },
                Some(Encoding::UNESCAPED_COMMA) => string.push(Encoding::UNESCAPED_COMMA),
                Some(Encoding::UNESCAPED_LF) => string.push(Encoding::UNESCAPED_LF),
                Some(Encoding::UNESCAPED_SEMICOLON) => string.push(Encoding::UNESCAPED_SEMICOLON),
                _ => continue,
            },
            _ => string.push(char),
        }
    }

    string
}

#[cfg(test)]
mod tests {
    use crate::parse::encoding::{escape, unescape};

    #[test]
    fn parse_encoding() {
        assert_eq!(unescape(r"\\"), "\\");
        assert_eq!(escape("\\"), r"\\");
        assert_eq!(unescape(r"\,"), ",");
        assert_eq!(escape(","), r"\,");
        assert_eq!(unescape(r"\\n"), "\n");
        assert_eq!(escape("\n"), r"\\n");
        assert_eq!(unescape(r"\;"), ";");
        assert_eq!(escape(";"), r"\;");

        let text = r#"
            This is multiline text,
            with commas,,,
            with semi-colons;;;
            and \backslashes\,
            and multiple \\backslashes\\.
        "#;
        assert_eq!(unescape(escape(text).as_str()), text);
    }
}

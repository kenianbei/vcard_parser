use std::fmt::{Display, Formatter};

use crate::parse::encoding::{escape, unescape};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueTextListData {
    pub delimiter: char,
    pub value: Vec<String>,
}

impl Default for ValueTextListData {
    fn default() -> Self {
        Self {
            delimiter: ';',
            value: Vec::new(),
        }
    }
}

impl From<(&str, char)> for ValueTextListData {
    fn from((str, delimiter): (&str, char)) -> Self {
        let mut value = Vec::new();

        fn chars_to_unescaped_string(chars: Vec<char>) -> String {
            unescape(chars.into_iter().collect::<String>().as_str())
        }

        let mut chars = str.chars().peekable();
        if let Some(mut prev) = chars.next() {
            let mut text = Vec::new();

            // Deal with first char based on whether it's a delimiter.
            if prev == delimiter {
                value.push(String::new());
                if chars.peek() == None {
                    value.push(String::new());
                }
            } else if chars.peek() == None {
                value.push(chars_to_unescaped_string(Vec::from([prev])));
            } else {
                text.push(prev);
            }

            while let Some(char) = chars.next() {
                // End loop if on last char.
                if chars.peek() == None {
                    if char == delimiter {
                        value.push(chars_to_unescaped_string(text));
                        value.push(String::new());
                    } else {
                        text.push(char);
                        value.push(chars_to_unescaped_string(text));
                    }
                    break;
                }

                // Add text to textlist when there is a non-escaped delimiter.
                if char == delimiter && prev != '\\' {
                    value.push(chars_to_unescaped_string(text));
                    text = Vec::new();
                    continue;
                }

                text.push(char);
                prev = char;
            }
        } else {
            value.push(String::new())
        }

        Self { delimiter, value }
    }
}

impl Display for ValueTextListData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.iter().map(|s| { escape(s) }).collect::<Vec<String>>().join(self.delimiter.to_string().as_str()))
    }
}

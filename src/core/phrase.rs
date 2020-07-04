#[derive(Debug)]
pub struct Phrase {
    verb: String,
    args: Vec<String>,
}

impl Phrase {
    pub fn verb(&self) -> &str {
        &self.verb
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }

    pub fn from(input: &str) -> Option<Self> {
        // guard on empty input
        if input.is_empty() {
            return None;
        }

        // split up the string
        let mut args = tokenize(input);
        let verb = args.remove(0);

        Some(Self { verb, args })
    }
}

const QUOTES: [char; 2] = ['\'', '"'];

fn tokenize(input: &str) -> Vec<String> {
    let mut output = vec![];
    let mut buffer = String::new();
    let mut quoted: Option<char> = None;

    for c in input.chars() {
        if let Some(quote_char) = quoted {
            // if we're at the next quote_char
            if c == quote_char {
                output.push(buffer.clone());
                buffer.clear();
                quoted = None;
            } else {
                // if we're not as the next quote_char, buffer
                buffer.push(c);
            }
        } else {
            // outside of quotes
            if c.is_whitespace() {
                if !buffer.is_empty() {
                    // we've reached the end of a word
                    output.push(buffer.clone());
                    buffer.clear();
                }
            } else if QUOTES.contains(&c) {
                quoted = Some(c);
            } else if c.is_alphanumeric() {
                buffer.push(c);
            }
        }
    }

    if !buffer.is_empty() {
        output.push(buffer);
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let src = "One, TWO thrEE! 'four five' \"six 'Seven\"";
        let tokenized = tokenize(src);
        assert_eq!(
            tokenized,
            vec![
                "One".to_owned(),
                "TWO".to_owned(),
                "thrEE".to_owned(),
                "four five".to_owned(),
                "six 'Seven".to_owned(),
            ]
        );
    }
}

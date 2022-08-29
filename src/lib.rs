use std::io::{BufRead, BufReader, Lines, Read};

#[derive(Debug, PartialEq, Eq)]
pub struct Parser {
    has_header: bool,
    delimiter: char,
}

impl Parser {
    pub fn new() -> Self {
        Builder::new().build()
    }

    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn parse<R: Read>(&self, r: R) -> Rows<R> {
        let mut lines = BufReader::new(r).lines();
        if self.has_header {
            lines.next();
        }

        Rows {
            iter: lines,
            delimiter: self.delimiter,
        }
    }
}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Rows<R> {
    iter: Lines<BufReader<R>>,
    delimiter: char,
}

impl<R: Read> Iterator for Rows<R> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iter.next() {
            Some(Ok(row)) => Some(
                row.split(self.delimiter)
                    .map(str::to_owned)
                    .collect::<Vec<_>>(),
            ),
            _ => None,
        }
    }
}

pub struct Builder {
    has_header: bool,
    delimiter: char,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            has_header: true,
            delimiter: ',',
        }
    }

    pub fn with_header(mut self, has_header: bool) -> Self {
        self.has_header = has_header;
        self
    }

    pub fn with_delimiter(mut self, delimiter: char) -> Self {
        self.delimiter = delimiter;
        self
    }

    pub fn build(&self) -> Parser {
        Parser {
            has_header: self.has_header,
            delimiter: self.delimiter,
        }
    }
}

impl Default for Builder {
    fn default() -> Self {
        Builder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_builder() {
        assert_eq!(
            Parser {
                has_header: false,
                delimiter: 'd'
            },
            Parser::builder()
                .with_header(false)
                .with_delimiter('d')
                .build()
        );
    }

    #[test]
    fn default_parser() {
        assert_eq!(Parser::new(), Parser::builder().build());
    }

    #[test]
    fn parse_no_headers() {
        let parser = Parser::builder().with_header(false).build();
        let rows: Vec<Vec<String>> = parser.parse("this,is,the,song".as_bytes()).collect();

        assert_eq!(
            vec![vec![
                String::from("this"),
                String::from("is"),
                String::from("the"),
                String::from("song")
            ]],
            rows
        );
    }

    #[test]
    fn parse_custom_delimiter() {
        let parser = Parser::builder()
            .with_header(false)
            .with_delimiter('\t')
            .build();
        let rows: Vec<Vec<String>> = parser.parse("this\tis\tthe\tsong".as_bytes()).collect();

        assert_eq!(
            vec![vec![
                String::from("this"),
                String::from("is"),
                String::from("the"),
                String::from("song")
            ]],
            rows
        );
    }

    #[test]
    fn parse_with_headers() {
        let data = &["these,are,the,headers", "this,is,the,song"].join("\n");
        let rows: Vec<Vec<String>> = Parser::new().parse(data.as_bytes()).collect();

        assert_eq!(
            vec![vec![
                String::from("this"),
                String::from("is"),
                String::from("the"),
                String::from("song")
            ]],
            rows
        );
    }

    #[test]
    fn parse_multiple_lines() {
        let data = &[
            "this,is,the,song",
            "that,never,ends,yes",
            "it,goes,on,and",
            "on,my,friends,:)",
        ]
        .join("\n");
        let rows: Vec<Vec<String>> = Parser::builder()
            .with_header(false)
            .build()
            .parse(data.as_bytes())
            .collect();
        assert_eq!(
            vec![
                vec![
                    String::from("this"),
                    String::from("is"),
                    String::from("the"),
                    String::from("song")
                ],
                vec![
                    String::from("that"),
                    String::from("never"),
                    String::from("ends"),
                    String::from("yes")
                ],
                vec![
                    String::from("it"),
                    String::from("goes"),
                    String::from("on"),
                    String::from("and")
                ],
                vec![
                    String::from("on"),
                    String::from("my"),
                    String::from("friends"),
                    String::from(":)")
                ],
            ],
            rows
        );
    }
}

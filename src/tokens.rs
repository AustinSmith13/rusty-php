pub enum TokenType {
    Output(String),
    ShortOpenTag, /* Can be disabled in php.ini --disable-short-tags */
    EchoOpenTag,
    OpenTag,
    CloseTag,
    String(String),
    Echo,
}

pub struct PhpToken {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

pub fn create_php_token(token: TokenType, input: &str, remaining: &str) -> PhpToken {
    let (line, column) = calculate_position(input, remaining);
    PhpToken {
        token_type: token,
        line: line,
        column: column,
    }
}

pub fn calculate_position(input: &str, remaining: &str) -> (usize, usize) {
    let processed = &input[..input.len() - remaining.len()];
    let lines: Vec<&str> = processed.lines().collect();
    let line = lines.len();
    let column = lines.last().map(|line| line.len()).unwrap_or(0) + 1;

    (line, column)
}

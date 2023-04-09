mod ast;
mod tokens;

use nom::{branch::alt, bytes::complete::tag, multi::many0, IResult};
use tokens::{create_php_token, PhpToken, TokenType};

const PHP_SOURCE: &str = "<?php echo \"hello, world\" ?>";

struct AST {}

fn parse_echo(input: &str) -> IResult<&str, PhpToken> {
    let (new_input, _) = tag("echo")(input)?;
    Ok((
        new_input,
        create_php_token(TokenType::Echo, input, new_input),
    ))
}

fn parse_php(input: &str) -> IResult<&str, AST> {
    Ok((input, AST {}))
}

fn main() {
    match parse_php(PHP_SOURCE) {
        Ok((input, ast)) => {
            println!("{}", input);
        }
        Err(e) => {
            println!("{}", e.to_owned().to_string())
        }
    }
}

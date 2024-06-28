use crate::header_types::Control;
use crate::parser::{parse_identifier, parse_key_value, parse_value};
use nom::{
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{newline, space1},
    multi::many0,
    IResult,
};
use std::path::PathBuf;

fn is_alphanumeric_underscore(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn variable_name(input: &str) -> IResult<&str, &str> {
    take_while1(is_alphanumeric_underscore)(input)
}

fn variable_value(input: &str) -> IResult<&str, &str> {
    take_while1(char::is_alphanumeric)(input)
}
pub fn parse_variable(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = tag("$")(input)?;
    let (input, var_name) = variable_name(input)?;
    let (input, _) = space1(input)?;
    let (input, var_value) = variable_value(input)?;
    Ok((input, (var_name, var_value)))
}
pub fn parse_define_value(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (remaining, output) = newline(input)?;
    let (remaining, output) = take_while(|c: char| c.is_whitespace())(remaining)?;
    let (remaining, output) = tag("#define")(remaining)?;
    // let (remaining, output) = take_while(|c: char| c.is_whitespace())(remaining)?;
    let (remaining, output) = newline(input)?;
    let (remaining, output) = many0(parse_variable)(remaining)?;
    // let (var_name, var_val) = output;
    Ok((remaining, output))
}
pub fn parse_default_path(input: &str) -> IResult<&str, &str> {
    let (remaining, output) = parse_identifier(&input)?;
    let (remaining, output) = parse_value(remaining)?;
    let (remaining, output) = parse_key_value(remaining)?;
    
    
    let (_, default_path) = output;
    Ok((remaining, default_path))
}

pub fn parse_control(input: &str) -> IResult<&str, Control> {
    let mut control_header = Control::new();
    let (remaining, output) = parse_identifier(input)?;
    let (remaining, default_path) = parse_default_path(remaining)?;
    
    control_header.default_path = PathBuf::from(default_path);
    let (remaining, output) = newline(input)?;
    let (remaining, output) = take_while(|c: char| c.is_whitespace())(remaining)?;
    let (remaining, output) = many0(parse_define_value)(remaining)?;
    // let (define_directive, directive_value) = output;
    // control_header.define_directives.insert(define_directive.into(), directive_value.into());
    println!("Remaining: {remaining}, Output: {:?}", output);
    Ok((remaining, control_header))
}
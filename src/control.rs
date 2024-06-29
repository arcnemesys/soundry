use crate::header_types::Control;
use crate::parser::{parse_identifier, parse_key_value, parse_value};
use nom::{
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{newline, space1},
    multi::many1,
    IResult,
};
use std::path::PathBuf;

fn is_alphanumeric_underscore(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}


fn variable_name(input: &str) -> IResult<&str, &str> {
    let (input, _) = tag("$")(input)?;
    take_while1(is_alphanumeric_underscore)(input)
}

fn variable_value(input: &str) -> IResult<&str, &str> {
    take_while1(char::is_alphanumeric)(input)
}


fn parse_define_line(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, _) = tag("#define")(input)?;
    let (input, _) = space1(input)?;
    let (input, var_name) = variable_name(input)?;
    let (input, _) = space1(input)?;
    let (input, var_value) = variable_value(input)?;
    let (input, _) = newline(input)?;
    let (input, _) = space1(input)?;
    Ok((input, (var_name, var_value)))
}

fn parse_defines(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many1(parse_define_line)(input)
}

pub fn parse_default_path(input: &str) -> IResult<&str, &str> {
    let (remaining, _) = parse_identifier(&input)?;
    let (remaining, _) = parse_value(remaining)?;
    let (remaining, output) = parse_key_value(remaining)?;

    let (_, default_path) = output;
    Ok((remaining, default_path))
}

pub fn parse_control(input: &str) -> IResult<&str, Control> {
    let mut control_header = Control::new();
    let (remaining, _) = parse_identifier(input)?;
    let (remaining, default_path) = parse_default_path(remaining)?;
    control_header.default_path = PathBuf::from(default_path);
    let (remaining, _) = newline(remaining)?;
    let (remaining, _) = take_while(|c: char| c.is_whitespace())(remaining)?;
    let (remaining, directives) = parse_defines(remaining)?;

    add_define_directives(&mut control_header, directives);
    Ok((remaining, control_header))
}

pub fn add_define_directives(control_header: &mut Control, directives: Vec<(&str, &str)>) {
    for (define_var, define_value) in directives {
        control_header
            .define_directives
            .insert(define_var.to_owned(), define_value.to_owned());
    }
}

use crate::header_types::Control;
use crate::parser::{parse_identifier, parse_key_value, parse_value};
use nom::bytes::complete::escaped;
use nom::character::complete::{alpha0, one_of};
use nom::{
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{newline, space1},
    multi::many1,
    IResult,
};
use serde::de;
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

pub fn add_define_directives(control_header: &mut Control, directives: Vec<(&str, &str)>) {
    for (define_var, define_value) in directives {
        control_header
            .define_directives
            .insert(define_var.to_owned(), define_value.to_owned());
    }
}

use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not},
    character::complete::{alpha1, char},
    combinator::value,
    sequence::delimited,
};

fn parser(input: &str) -> IResult<&str, String> {
    escaped_transform(
        is_not("\\\""), // match any character except \ and "
        '\\',           // escape character
        alt((
            value("\\", tag("\\")),
            value("\"", tag("\"")),
            value("\n", tag("n")),
        )),
    )(input)
}

fn esc(s: &str) -> IResult<&str, &str> {
    escaped(alpha0, '\\', one_of(r#"\"#))(s)
}

fn dequote(i: &str) -> IResult<&str, &str> {
    let remaining = char('"')(i)?;

    let (include_var, _) = remaining;

    let (remaining, include_directive) = take_while1(|c: char| c.is_alphanumeric() || c == '\\' || c == '.' && c != '\"')(include_var)?;
    Ok((remaining, include_directive))
}

pub fn parse_include_line(input: &str) -> IResult<&str, &str> {
    let (remaining, output) = tag("#include")(input)?;

    let (remaining, output) = space1(remaining)?;
    let (remaining, include_directive) = dequote(remaining)?;
    let (remaining, _) = char('"')(remaining)?;
    println!("From `parse_include_line` - Remaining: {remaining}, incl_dir: {include_directive}");
    // let (remaining, output) = parser(remaining)?;
    // let include_directive = include_directive.to_string();
    // let (remaining, _) = esc(remaining)?;
    // let (remaining, _) = space1(remaining)?;
    // println!("From `parse_include_line` - Remaining: {remaining}.");
    // println!("From `parse_include_line` - Remaining: {remaiwning}.");
    // let (input, output) = newline(input)?;
    // Ok((input, (var_name, var_value)))
    Ok((remaining, include_directive))
}
fn parse_includes(input: &str) -> IResult<&str, Vec<&str>> {
    many1(parse_include_line)(input)
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
    let (remaining, include_directives) = parse_includes(remaining)?;
    println!("Remaining: {remaining}, Includes: {:?}", include_directives);
    // add_include_directives(&mut control_header, include_directives);
    // control_header.include_directives.push(include_directive.to_owned());
    Ok((remaining, control_header))
}

pub fn add_include_directives(control_header: &mut Control, directives:Vec<&str>) {

    for i in directives {
        control_header.include_directives.push(i.to_string());
    }
}

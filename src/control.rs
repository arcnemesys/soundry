use crate::header_types::Control;
use crate::parser::{take_to_newline, parse_identifier, parse_key_value, parse_value};
use nom::character::complete::alphanumeric1;
use nom::{
    sequence::tuple,
    IResult,
    multi::{many0, many1},
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{alpha1, newline, space1, char, multispace0},
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

pub fn add_define_directives(control_header: &mut Control, directives: Vec<(&str, &str)>) {
    for (define_var, define_value) in directives {
        control_header
            .define_directives
            .insert(define_var.to_owned(), define_value.to_owned());
    }
}

fn dequote(i: &str) -> IResult<&str, &str> {
    let remaining = char('"')(i)?;

    let (include_var, _) = remaining;

    let (remaining, include_directive) =
        take_while1(|c: char| c.is_alphanumeric() || c == '\\' || c == '.' && c != '\"')(
            include_var,
        )?;
    Ok((remaining, include_directive))
}

pub fn parse_include_line(input: &str) -> IResult<&str, &str> {
    let (remaining, _) = tag("#include")(input)?;

    let (remaining, _) = space1(remaining)?;
    let (remaining, include_directive) = dequote(remaining)?;
    let (remaining, _) = char('"')(remaining)?;
    let (remaining, _) = multispace0(remaining)?;

    Ok((remaining, include_directive))
}
fn parse_includes(input: &str) -> IResult<&str, Vec<&str>> {
    let (remaining, include_directives) = many0(parse_include_line)(input)?;

    Ok((remaining, include_directives))
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
    add_include_directives(&mut control_header, include_directives);
    let (remaining, output) = parse_cc_labels(remaining)?;
    // println!("Remaining: {remaining}, CC_labels: {:?}.", output);
    Ok((remaining, control_header))
}

pub fn add_include_directives(control_header: &mut Control, directives: Vec<&str>) {
    for i in directives {
        control_header.include_directives.push(i.to_string());
    }
}

pub fn parse_cc_label(input: &str) -> IResult<&str, (&str, &str, &str)> {
    let mut midi_labels: Vec<(u32, String)> = Vec::new();

    let (remaining, _) = take_while(|c: char| c.is_whitespace())(input)?;

    let (remaining, label_cc) = tag("label_cc")(remaining)?;
    let (remaining, (label_number, _, label_value)) = tuple((alphanumeric1, tag("="), take_to_newline))(remaining)?;

    println!("Remaining: {remaining}, label_number: {label_number}, label_value: {label_value}.");

    Ok((remaining, (label_cc, label_number, label_value)))
}

pub fn parse_cc_labels(input: &str) -> IResult<&str, Vec<(&str, &str, &str)>> {
    let (remaining, cc_labels) = many1(parse_cc_label)(input)?;

    Ok((remaining, cc_labels))
}
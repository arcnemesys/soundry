use crate::header_types::Control;
use crate::parser::{white_space, take_to_newline, parse_identifier, parse_key_value, parse_value};
use nom::{
    sequence::tuple,
    IResult,
    multi::{many0, many1},
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{newline, space1, char, multispace0},
};

use std::path::PathBuf;

fn is_alphanumeric_underscore(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

fn variable_name(sfz_source: &str) -> IResult<&str, &str> {
    let (remaining, _) = tag("$")(sfz_source)?;
    take_while1(|c: char| c.is_alphanumeric() || c == '_')(remaining)
}

fn variable_value(sfz_source: &str) -> IResult<&str, &str> {
    take_while1(char::is_alphanumeric)(sfz_source)
}

fn parse_define_line(sfz_source: &str) -> IResult<&str, (&str, &str)> {
    let (remaining, _) = tag("#define")(sfz_source)?;
    let (remaining, _) = space1(remaining)?;
    let (remaining, var_name) = variable_name(remaining)?;
    let (remaining, _) = space1(remaining)?;
    let (remaining, var_value) = variable_value(remaining)?;
    let (remaining, _) = newline(remaining)?;
    let (remaining, _) = space1(remaining)?;
    Ok((remaining, (var_name, var_value)))
}

fn parse_defines(sfz_source: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many1(parse_define_line)(sfz_source)
}

pub fn parse_default_path(sfz_source: &str) -> IResult<&str, &str> {
    let (remaining, _) = parse_identifier(&sfz_source)?;
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

fn dequote(sfz_source: &str) -> IResult<&str, &str> {
    let remaining = char('"')(sfz_source)?;

    let (include_var, _) = remaining;

    let (remaining, include_directive) =
        take_while1(|c: char| c.is_alphanumeric() || c == '\\' || c == '.' && c != '\"')(
            include_var,
        )?;
    Ok((remaining, include_directive))
}

pub fn parse_include_line(sfz_source: &str) -> IResult<&str, &str> {
    let (remaining, _) = tag("#include")(sfz_source)?;

    let (remaining, _) = space1(remaining)?;
    let (remaining, include_directive) = dequote(remaining)?;
    let (remaining, _) = char('"')(remaining)?;
    let (remaining, _) = multispace0(remaining)?;

    Ok((remaining, include_directive))
}
fn parse_includes(sfz_source: &str) -> IResult<&str, Vec<&str>> {
    let (remaining, include_directives) = many0(parse_include_line)(sfz_source)?;

    Ok((remaining, include_directives))
}

pub fn parse_control(sfz_source: &str) -> IResult<&str, Control> {
    let mut control_header = Control::new();
    let (remaining, _) = parse_identifier(sfz_source)?;
    let (remaining, default_path) = parse_default_path(remaining)?;
    control_header.default_path = PathBuf::from(default_path);


    let (remaining, _) = newline(remaining)?;
    let (remaining, _) = take_while(|c: char| c.is_whitespace())(remaining)?;
    let (remaining, directives) = parse_defines(remaining)?;

    add_define_directives(&mut control_header, directives);

    let (remaining, include_directives) = parse_includes(remaining)?;
    add_include_directives(&mut control_header, include_directives);
    let (remaining, cc_labels) = parse_cc_labels(remaining)?;

    add_label_ccns(&mut control_header, cc_labels);
    let (remaining, output) = parse_set_ccns(remaining)?;
    println!("output: {:?}", output);
    Ok((remaining, control_header))
}

pub fn add_include_directives(control_header: &mut Control, directives: Vec<&str>) {
    for i in directives {
        control_header.include_directives.push(i.to_string());
    }
}

pub fn add_set_ccns(control_header: &mut Control, set_ccns: Vec<(&str, &str)>) {
    for i in set_ccns {
        control_header.set_ccn.insert(i.0.to_owned(), i.1.to_owned());
    }
}

pub fn add_label_ccns(control_header: &mut Control, label_ccns: Vec<(&str, &str, &str)>) {
    for cc_tuple in label_ccns {

        let (label, label_number, label_value) = cc_tuple;

        let cc_label = format!("{}{}", label, label_number);

        let mut instrument = String::new();
        let mut modulation = String::new();

        if label_value.contains(" ") {

            let mut label_value_iter = label_value.split_whitespace();

            instrument = label_value_iter.next().unwrap().to_owned();
            modulation = label_value_iter.next().unwrap().to_owned();
            
            control_header.label_ccn.insert(cc_label.clone(), (instrument, Some(modulation)));
        }

        control_header.label_ccn.insert(cc_label, (label_value.to_owned(), None));

    }
}

pub fn parse_cc_label(sfz_source: &str) -> IResult<&str, (&str, &str, &str)> {
    let (remaining, _) = take_while(|c: char| c.is_whitespace())(sfz_source)?;

    let (remaining, label_cc) = tag("label_cc")(remaining)?;
    let (remaining, (label_number, label_value)) = parse_cc_var(remaining)?;

    Ok((remaining, (label_cc, label_number, label_value)))
}

pub fn parse_cc_labels(sfz_source: &str) -> IResult<&str, Vec<(&str, &str, &str)>> {
    let (remaining, cc_labels) = many1(parse_cc_label)(sfz_source)?;

    Ok((remaining, cc_labels))
}

pub fn parse_cc_var(sfz_source: &str) -> IResult<&str, (&str, &str), > {
    let (remaining, (var_name, _,  var_value)) = tuple((parse_identifier, tag("="), take_to_newline))(sfz_source)?;

    Ok((remaining, (var_name, var_value)))

}

pub fn parse_set_ccn(sfz_source: &str) -> IResult<&str, (&str, &str)> {
    let (remaining, _) = white_space(sfz_source)?;
    let (remaining, (set_number, set_value, )) = parse_cc_var(remaining)?;
    println!("set_number: {set_number}, set_value: {set_value}");
    Ok((remaining, (set_number, set_value)))
}

pub fn parse_set_ccns(sfz_source: &str) -> IResult<&str, Vec<(&str, &str)>> {
    let (remaining, set_cc_vars) = many0(parse_set_ccn)(sfz_source)?;

    Ok((remaining, set_cc_vars))
}

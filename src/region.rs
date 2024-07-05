use crate::parser::parse_key_value;
use nom::{bytes::complete::tag, character::complete::space0, multi::many0, IResult};
use std::collections::HashMap;
#[derive(Debug, PartialEq, Clone)]
pub struct Region {
    low_velocity: u32,
    high_velocity: u32,
    volume: f32,
    region_label: String,
    sample: (String, String),
    offset: u32,
    parameters: HashMap<String, String>,
}

pub struct SFZFile {
    elements: Vec<Region>,
}

pub fn parse_region(sfz_source: &str) -> IResult<&str, Region> {
    let (remaining, _) = tag("<region>")(sfz_source)?;
    let (remaining, _) = space0(remaining)?;
    let (remaining, params) = nom::multi::many0(parse_key_value)(remaining)?;

    for i in &params {
        println!("i is: {:?}", *i);
    }

    let mut parameters = HashMap::new();

    for (key, value) in params {
        // match the key, then match the value if it
        // satisfies the refinement predicate, then move on to
        // the line below.
        parameters.insert(key.to_string(), value.to_string());
    }

    Ok((sfz_source, Region { 
        parameters,
        low_velocity: 0,
        high_velocity: 0,
        volume: 0f32,
        region_label: String::new(),
        sample: (String::new(), String::new()),
        offset: 0
    }))
}

pub fn parse_sfz(sfz_source: &str) -> IResult<&str, SFZFile> {
    let (remaining, elements) = many0(parse_region)(sfz_source)?;
    Ok((remaining, SFZFile { elements }))
}

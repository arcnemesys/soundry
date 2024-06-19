use std::collections::HashMap;
use nom::{bytes::complete::tag, character::complete::space0, multi::many0, IResult};
use crate::parser::parse_key_value;
#[derive(Debug, PartialEq, Clone)]
pub struct Region {
    parameters: HashMap<String, String>,
}

pub struct SFZFile {
    elements: Vec<Region>
}


pub fn parse_region(sfz_source: &str) -> IResult<&str, Region> {
    let (sfz_source, _) = tag("<region>")(sfz_source)?;
    let (sfz_source, _) = space0(sfz_source)?;
    let (sfz_source, params) = nom::multi::many0(parse_key_value)(sfz_source)?;

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

    Ok((sfz_source, Region {parameters}))
}

pub fn parse_sfz(input: &str) -> IResult<&str, SFZFile> {
    let (input, elements) = nom::multi::many0(parse_region)(input)?;
    Ok((input, SFZFile { elements }))
}

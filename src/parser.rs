use nom::{
    bytes::complete::{tag, take_until, take_while, take_while1}, character::complete::char, combinator::recognize, multi::{many0, many0_count}, sequence::{pair, tuple}, IResult
};

pub fn parse_identifier(inpt: &str) -> IResult<&str, &str> {
    let (remaining, output) = take_while(|c: char| c.is_whitespace())(inpt)?;
    // println!("Remaining: {remaining}, Output: {output}");
    take_while(|c: char| c.is_alphanumeric() || c == '_')(remaining)
    // `take_while1` receives a condition, that implements the `Fn`
    // trait, and returns a boolean value which is satisfied by the
    // closure, that checks whether or not a character is alphanumeric,
    // a dash, or an underscore.
    // This is going to grab the various opcodes that can be found in
    // region headers, such as `sample` or `ampeg_release`.
    
}

pub fn parse_value(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace() && c != '=')(input)
}

pub fn parse_key_value(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, (key, _, value)) = tuple((parse_identifier, tag("="), parse_value))(input)?;
    Ok((input, (key, value)))
}


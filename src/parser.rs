use nom::{
    bytes::complete::{tag, take_until1, take_while, take_while1}, sequence::tuple, IResult
};

pub fn parse_identifier(inpt: &str) -> IResult<&str, &str> {
    let (remaining, _) = take_while(|c: char| c.is_whitespace())(inpt)?;
    take_while(|c: char| c.is_alphanumeric() || c == '_')(remaining)

}

pub fn parse_value(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace() && c != '=')(input)
}

pub fn parse_key_value(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, (key, _, value)) = tuple((parse_identifier, tag("="), parse_value))(input)?;
    Ok((input, (key, value)))
}

pub fn take_to_newline(input: &str) -> IResult<&str, &str> {
    let (remaining, output) = take_until1("\n")(input)?;
    Ok((remaining, output))
}
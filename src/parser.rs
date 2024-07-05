use nom::{
    bytes::complete::{tag, take_until1, take_while, take_while1}, sequence::tuple, IResult
};

pub fn parse_identifier(sfz_source: &str) -> IResult<&str, &str> {
    let (remaining, _) = take_while(|c: char| c.is_whitespace())(sfz_source)?;
    take_while(|c: char| c.is_alphanumeric() || c == '_')(remaining)

}


pub fn parse_value(sfz_source: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace() && c != '=')(sfz_source)
}

pub fn parse_key_value(sfz_source: &str) -> IResult<&str, (&str, &str)> {
    let (remaining, (key, _, value)) = tuple((parse_identifier, tag("="), parse_value))(sfz_source)?;
    Ok((remaining, (key, value)))
}

pub fn take_to_newline(sfz_source: &str) -> IResult<&str, &str> {
    let (remaining, output) = take_until1("\n")(sfz_source)?;
    Ok((remaining, output))
}
pub fn white_space(sfz_source: &str) -> IResult<&str, ()> {
   let (remaining, _) = take_while(|c: char| c.is_whitespace())(sfz_source)?;

   Ok((remaining, ()))
}
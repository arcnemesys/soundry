use nom::{IResult, bytes::complete::tag};
// The first object we want to look for is a control header, which is
// `<`[a-z]`>`, and our nom tag will be `<control>`.

// Once we encounter that, we want to look for `default_path=`.

fn control_parser(sfz_source: &str) -> IResult<&str, &str> {
    tag("<control>")(sfz_source)
}
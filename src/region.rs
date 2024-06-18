use nom::{
    bytes::complete::{tag, take_while, take_while1}, character::complete::space0, combinator::map, sequence::tuple, IResult
};

use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub struct Region {
    parameters: HashMap<String, String>,
}


#[derive(Debug, PartialEq)]
pub struct SFZFile {
    elements: Vec<Region>,
}





// #[derive(Clone, Debug, Serialize, Deserialize)]
// enum InputControl {
//     LoChan(u8),
//     HiChan(u8),
//     LoKey(u8),
//     HiKey(u8),
//     Key,
//     LoVel(u8),
//     HiVel(u8),
//     LoCC(u8),
//     HiCC(u8),
//     LoBend(u8),
//     HiBend(u8),
//     LoChanAft(u8),
//     HiChanAft(u8),
//     LoPolyAft(u8),
//     HiPolyAft(u8),
//     LoRand(f32),
//     HiRand(f32),
//     LoHpm(f32),
//     HiBpm(f32),
//     SeqLength(u8),
//     SeqPosition(u8),
//     SwLoKey(u8),
//     SwHiKey(u8),
//     SwLast(u8),
//     SwDown(u8),
//     SwUp(u8),
//     SwPrevious(u8),
//     SwVel(String),   // Can be either current or previous.
//     Trigger(String), // Can be attack, release, first or legato.
//     Group(u8),
//     OffBy(u32),
//     OffMode(String), // Can be fast or normal
//     OnLoCC(u8),
//     OnHiCC(u8),
// }

// #[derive(Clone, Debug, Serialize, Deserialize)]
// enum PerformanceParameter {
//     SamplePlayer(SamplePlayerParameter),
//     Pitch(PitchParameter),
//     PitchEG(PitchEGParameter),
//     PitchLFO(PitchLFOParameter),
//     Filter(FilterParameter),
//     FilterEG(FilterEGParameter),
//     FilterLFO(FilterLFOParameter),
//     Amplifier(AmplifierParameter),
//     AmplifierEG(AmplifierEGParameter),
//     AmplifierLFO(AmplifierLFOParameter),
//     Equalizer(EqualizerParameter),
// }

// #[derive(Clone, Debug, )]
// pub struct Region {
//     /// The sample to be played for this specific region.
//     /// Defines when the sample in a region will play.
//     // input_controls: Vec<InputControl>,
//     // perf_params: Vec<PerformanceParameter>,
//     parameters: HashMap<String, String>
// }


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

pub fn parse_value(inpt: &str) -> IResult<&str, &str> {
    take_while1(|c: char| !c.is_whitespace() && c != '=')(inpt)
    // println!("Remaining: {remaining}, Output: {output}");
    // This function then
}

pub fn parse_key_value(inpt: &str) -> IResult<&str, (&str, &str)> {
    let (input, (key, _, value)) = tuple((parse_identifier, tag("="), parse_value))(inpt)?;
    Ok((input, (key, value)))
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
        parameters.insert(key.to_string(), value.to_string());
    }

    Ok((sfz_source, Region {parameters}))
}

pub fn parse_sfz(input: &str) -> IResult<&str, SFZFile> {
    let (input, elements) = nom::multi::many0(parse_region)(input)?;
    Ok((input, SFZFile { elements }))
}

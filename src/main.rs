// Thanks to Anders Danhielson
// Reference Material
// http://drealm.info/sfz/plj-sfz.xhtml
// https://sfzformat.com/headers/
// Opcode List: https://www.linuxsampler.org/sfz/
// https://sfzformat.com/legacy/
// SFZ Tutorial/Intro: https://sfzformat.com/tutorials/basics/
// MIDI CC Message List: https://atherproducer.com/online-tools-for-musicians/midi-cc-list/
// https://www.sustainable-music.org/demystifying-sfz-a-guide-to-understanding-the-sfz-format-for-sound-libraries/
// https://github.com/sfz/tests/tree/master
// https://sfzlab.github.io/sfz-website/
// https://edrums.github.io/en/linuxsampler/sfz/#Effects

// TODO: Dedupe EG/LFO variants to be defined once and reused.
// TODO [in progress]: Explore refinement types.
// TODO [in progress]: Implement parsing with nom.
// TODO: Add Cakewalk specific codes.
// TODO: Add v2 opcodes for
// Sample Playback
// Voice LifeCycle
// Midi Conditions
// Internal Conditions
// Triggers
// Amplifier
// EQ
// Filter
// Pitch
// LFO
// Curves
// Effects
// Loading
// Wavetable Oscillator

#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use crate::opcode_types::{EffectType, BusOption,};
use crate::region::{parse_sfz, Region, parse_region,  parse_value, parse_key_value, parse_identifier};
mod parser;
mod region;
mod refinements;
mod opcode_types;
// Eq
// PartialEq
// Ord
// PartialOrd
// Hash
// Display

#[derive(Clone, Debug, )]
struct Control {
    /// Defines the SFZ header type of this struct.
    header_type: HeaderType,
    /// Contains the `#define` directives (variables) for the SFZ  control header.
    directives: HashMap<String, u32>,
    /// Path to samples, should be relative for Cakewalk,
    /// but can be relative or absolute for ARIA, Bassmidi and sfizz.
    /// The value of `default_path` is reset by a new control header,
    /// in ARIA, but t in Cakewalk.
    default_path: PathBuf,
    /// Informs an SFZ player to offset incoming MIDI notes
    /// by a specified number of semitones.
    note_offset: u8,
    /// Informs an SFZ player to offset incoming MIDI notes
    /// by a specified number of octaves.
    octave_offset: u8, // Refactor: use https://docs.rs/refinement/latest/refinement/#
    /// Creates labels for MIDI control changes for altering parameters
    /// on MIDI-enabled devices.
    label_ccn: Vec<(u32, String)>, // Refactor: Choose better representation.
    /// Sets default values for MIDI CC number N.
    set_ccn: Vec<(u32, String)>,
}

impl Control {
    fn new() -> Self {
        Self {
            header_type: HeaderType::Control,
            directives: HashMap::new(),
            default_path: PathBuf::new(),
            note_offset: 0,
            octave_offset: 0,
            label_ccn: vec![],
            set_ccn: vec![],
        }
    }
}

impl Default for Control {
    fn default() -> Self {
        Self {
            header_type: HeaderType::Control,
            directives: HashMap::new(),
            default_path: PathBuf::new(),
            note_offset: 0,
            octave_offset: 0,
            label_ccn: vec![],
            set_ccn: vec![],
        }
    }
}
// Eq
// PartialEq
// Ord
// PartialOrd
// Hash
// Display
// Default
#[derive(Clone, Debug)]
struct Group<T> {
    /// The regions to which the common parameters will be applied.
    regions: Vec<Region>,
    /// The common parameters for the regions associated with the group.
    common_params: HashMap<String, T>,
}
// Eq
// PartialEq
// Ord
// PartialOrd
// Hash
// Display
// Default

// #[derive(Clone, Debug, )]
// struct Region {
//     /// Defines the Sfz header type of this struct.
//     header_type: HeaderType,
//     /// The sample to be played for this specific region.
//     sample: PathBuf,
//     /// Defines when the sample in a region will play.
//     input_controls: Vec<InputControl>,
//     perf_params: Vec<PerformanceParameter>,
// }


// pub fn match_input_control<T>(input_control: (&str, T)) -> InputControl {
//     match input_control {
//         ("lovel", val) => {

//         }
//     }
// }


// Copy
// Clone
// Eq
// PartialEq
// Ord
// PartialOrd
// Hash
// Debug
// Display
// Default
#[derive(Clone, Debug, )]
struct Global<T> {
    common_params: HashMap<String, T>,
}
// Copy
// Clone
// Eq
// PartialEq
// Ord
// PartialOrd
// Hash
// Debug
// Display
// Default
#[derive(Clone, Debug, )]
struct Curve<T> {
    index: u32,
    values: Vec<(String, T)>,
}


// Copy
// Clone
// Eq
// PartialEq
// Ord
// PartialOrd
// Hash
// Debug
// Display
// Default
#[derive(Clone, Debug, )]
struct Effect {
    effect_type: EffectType,
    param_offset: u32,
    effect_one: f32,   // Reverb in Cakewalk
    effect_two: f32,   // Chorus in Cakewalk
    effect_three: f32, // Gain of regions send tracks into 3rd effect bus.
    effect_four: f32,  // Gain of regions send tracks into 4th effect bus.
    bus: BusOption,
    dsp_order: u8,
}

// Copy
// Clone
// Eq
// PartialEq
// Ord
// PartialOrd
// Hash
// Debug
// Display
// Default
#[derive(Clone, Debug, )]
struct Sample {
    name: PathBuf,
    data: Vec<u8>,
}
// Copy
// Eq
// PartialEq
// Ord
// PartialOrd
// Hash
// Display
// Default

#[derive(Clone, Debug, )]
struct Master<T> {
    op_codes: Vec<(String, T)>,
    groups: Option<Vec<Group<T>>>,
}
// Copy
// Eq
// PartialEq
// Ord
// PartialOrd
// Hash
// Display
// Default

#[derive(Clone, Debug, )]
struct SfzInstrument<T> {
    global: Vec<Global<T>>,
    control: Vec<Control>,
    group: Vec<Group<T>>,
    master: Option<Vec<Master<T>>>,
    region: Vec<Region>,
    effect: Vec<Effect>,
}

#[derive(Clone, Debug, )]
enum HeaderType {
    Region,
    Group,
    Control,
    Global,
    Curve,
    Effect,
    Master,
    Midi,
    Sample,
}

use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take_until},
    character::complete::multispace0,
    character::complete::newline,
    error::ParseError,
    sequence::delimited,
    sequence::separated_pair,
    IResult, Parser,
};

// The first object we want to look for is a control header, which is
// `<`[a-z]`>`, and our nom tag will be `<control>`.

// Once we encounter that, we want to look for a newline, `\n`, then`default_path=`.

fn control_header_parser(sfz_source: &str) -> IResult<&str, &str> {
    tag("<control>")(sfz_source)
}

fn define_directive(sfz_source: &str) -> IResult<&str, &str> {
    tag("#define")(sfz_source)
}

fn default_path(sfz_source: &str) -> IResult<&str, &str> {
    take_until("default_path=")(sfz_source)
}


fn default_path_parser(sfz_source: &str) -> IResult<&str, &str> {
    tag("default_path=")(sfz_source)
}

fn global_header_parser(sfz_source: &str) -> IResult<&str, &str> {
    tag("<global>")(sfz_source)
}
fn until_newline(sfz_source: &str) -> IResult<&str, &str> {
    take_until("\n")(sfz_source)
}
fn main() -> Result<(), Box<dyn Error>> {
    // let sfz_file = "<control>
    // default_path=samples/
    // <global>";
    // let (remaining, output) = control_header_parser(sfz_file)?;

    // assert_eq!(output, "<control>");
    // assert_eq!(remaining, "\n    default_path=samples/\n    <global>");

    // let (remaining_next, output) = default_path(remaining)?;

    // let (remaining_next, output) = default_path_parser(remaining_next)?;

    // assert_eq!(output, "default_path=");
    // assert_eq!(remaining_next, "samples/\n    <global>");

    // let (remaining_next, output) = until_newline(remaining_next)?;

    // println!("{}", remaining_next);

    let sfz_content = r#"<region>
    sample=kick.wav
    pitch_keycenter=60
    lovel=0
    hivel=127
    "#;

    let (remaining, output) = parse_region(sfz_content)?;
    // let (remaining, output) = parse_value(remaining)?;
    // let (remaining, output) = parse_key_value(remaining)?;

    println!("Remaining: {remaining}, Output: {:?}", output);
    

    // match parse_sfz(sfz_content) {
    //     Ok((_, sfz_file)) => println!("{:#?}", sfz_file),
    //     Err(err) => eprintln!("Failed to parse SFZ file: {:?}", err),
    // }
    Ok(())
}

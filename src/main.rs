#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::error::Error;

use crate::opcode_types::{EffectType, BusOption,};
use crate::header_types::{Control, Global, Group, Master, Effect};
use crate::parser::{parse_identifier, parse_key_value, parse_value};
use crate::region::{Region, parse_region};
mod parser;
mod region;
mod refinements;
mod opcode_types;
mod header_types;
mod control;
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

    let sfz_content = r#"
    
    
    <global>loop_mode=one_shot seq_length=4

<group>key=36 hivel=31 amp_velcurve_31=1
<region>seq_position=1 sample=kick_vl1_rr1.wav
<region>seq_position=2 sample=kick_vl1_rr2.wav
<region>seq_position=3 sample=kick_vl1_rr3.wav
<region>seq_position=4 sample=kick_vl1_rr4.wav


    "#;

    // let (remaining, output) = parse_identifier(sfz_content)?;
    // let (remaining, output) = parse_value(remaining)?;
    // let (remaining, output) = parse_key_value(remaining)?;

    // println!("Remaining: {remaining}, Output: {:?}", output);
    
    let control_header = "
    <control>
    default_path=Samples\
";

    let (remaining, output) = parse_identifier(&control_header)?;
    // println!("Remaining: {remaining}, Output: {:?}", output);
    let (remaining, output) = parse_value(remaining)?;
    // println!("Remaining: {remaining}, Output: {:?}", output);
    // let (remaining, output) = parse_key_value(remaining)?;
    
    println!("Remaining: {remaining}, Output: {:?}", output);

    // match parse_sfz(sfz_content) {
    //     Ok((_, sfz_file)) => println!("{:#?}", sfz_file),
    //     Err(err) => eprintln!("Failed to parse SFZ file: {:?}", err),
    // }
    Ok(())
}

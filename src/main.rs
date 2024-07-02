#![allow(dead_code)]
use control::{parse_control, parse_include_line};
use serde::{Deserialize, Serialize};
use std::boxed::Box;
use std::error::Error;

use crate::header_types::{Control, Effect, Global, Group, Master};
use crate::opcode_types::{BusOption, EffectType};
use crate::region::{parse_region, Region};
mod control;
mod header_types;
mod opcode_types;
mod parser;
mod refinements;
mod region;
// Copy
// Eq
// PartialEq
// Ord
// PartialOrd
// Hash
// Display
// Default

#[derive(Clone, Debug)]
struct SfzInstrument<T> {
    global: Vec<Global<T>>,
    control: Vec<Control>,
    group: Vec<Group<T>>,
    master: Option<Vec<Master<T>>>,
    region: Vec<Region>,
    effect: Vec<Effect>,
}

fn main() -> Result<(), Box<dyn Error>> {

    let control_header = r#"


    <control>
    default_path=Samples\

    #define $EXT wav
    #define $KICKKEY 36
    #define $SNAREKEY 38
    #define $HATKEY 42
    #include "data\control.sfz"
    #include "data\multiout.sfz"
    #include "data\global.sfz"
    #include "data\kick.sfz"
    #include "data\snare.sfz"
    #include "data\tom1.sfz"
    #include "data\tom2.sfz"
    #include "data\hihat.sfz"
    #include "data\ride.sfz"
    #include "data\crash.sfz"
    label_cc30=Bass vol
    label_cc31=Bass pan
    label_cc32=Tune
    label_cc33=Mute
    set_cc40=127
    set_cc100=30"#;

    let cmplx_control_header = r#"#include \"data\global.sfz\"
#include \"data\kick.sfz\"
#include \"data\snare.sfz\"
#include \"data\tom1.sfz\"
#include \"data\tom2.sfz\"
#include \"data\hihat.sfz\"
#include \"data\ride.sfz\"
#include \"data\crash.sfz\"#;

    let simple_control_header = "<control>
    default_path=Samples/
    #define $EXT flac
    #define $EXTTWO wav
    #include \"data/control.sfz\"";

    // let (remaining, output) = parse_identifier(simple_control_header)?;
    // println!("Remaining: {remaining}, Output: {:?}", output);
    // let (remaining, output) = parse_default_path(remaining)?;
    // println!("Remaining: {remaining}, Output: {:?}", output);

    // let (remaining, output) = parse_define_value(remaining)?;
    // println!("Remaining: {remaining}, Output: {:?}", output);
    let (remaining, control_header) = parse_control(control_header)?;
    // let (remaining, output) = parse_include_line(cmplx_control_header)?;

    // println!("Remaining: {remaining}, Control Header: {:?}", control_header);
    for i in control_header.label_ccn {
        println!("label_ccn: {:?}", i);
    }

    Ok(())
}

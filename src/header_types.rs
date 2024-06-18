use std::path::PathBuf;
use std::collections::HashMap;
use crate::region::Region;
use crate::opcode_types::{EffectType, BusOption};
#[derive(Clone, Debug)]
pub struct Group<T> {
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
pub struct Global<T> {
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
pub struct Curve<T> {
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
pub struct Effect {
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
pub struct Sample {
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
pub struct Master<T> {
    op_codes: Vec<(String, T)>,
    groups: Option<Vec<Group<T>>>,
}

#[derive(Clone, Debug, )]
pub struct Control {
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

#[derive(Clone, Debug, )]
pub enum HeaderType {
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

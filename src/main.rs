// Reference Material
// http://drealm.info/sfz/plj-sfz.xhtml
// https://sfzformat.com/headers/
// Opcode List: https://www.linuxsampler.org/sfz/
// https://sfzformat.com/legacy/
// SFZ Tutorial/Intro: https://sfzformat.com/tutorials/basics/
// MIDI CC Message List: https://atherproducer.com/online-tools-for-musicians/midi-cc-list/
// https://www.sustainable-music.org/demystifying-sfz-a-guide-to-understanding-the-sfz-format-for-sound-libraries/
// https://github.com/sfz/tests/tree/master

// SFZ files are sub-divided into headers.
// The region header is the most essential, and is the
// unit that instruments are comprised of.

// A `group` is an optional organizational level containing
// one or more regions

// The gLobal header contains opcodes which apply to all
// regions in the file.

// The master header is an extra level between groups and regions.

// The hierarchy can be:
// global -> group -> region
// global -> master -> group -> region

// The control header should be at the beginning of the file.
// The curve headers, are usually at the end of files.

// TODO: Dedupe EG/LFO variants to be defined once and reused.

#![allow(dead_code)]
use std::collections::HashMap;
use std::path::PathBuf;

struct Control {
    /// Defines the SFZ header type of this struct.
    header_type: HeaderType,
    /// Contains the `#define` directives (variables) for the SFZ  control header.
    define_directives: HashMap<String, u32>,
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
    fn new(
        define_directives: HashMap<String, u32>,
        default_path: PathBuf,
        note_offset: u8,
        octave_offset: u8,
        label_ccn: Vec<(u32, String)>,
        set_ccn: Vec<(u32, String)>,
    ) -> Self {
        Self {
            header_type: HeaderType::Control,
            define_directives,
            default_path,
            note_offset,
            octave_offset,
            label_ccn,
            set_ccn,
        }
    }
}

struct Group<T> {
    /// The regions to which the common parameters will be applied.
    regions: Vec<Region>,
    /// The common parameters for the regions associated with the group.
    common_params: HashMap<String, T>,
}

struct Region {
    /// Defines the Sfz header type of this struct.
    header_type: HeaderType,
    /// The sample to be played for this specific region.
    sample: PathBuf,
    /// Defines when the sample in a region will play.
    input_controls: Vec<InputControl>,
    perf_params: Vec<String>,
}

// These seem to map on to the SFZ opcode categories:
// Key Mapping
// Midi Conditions
// Internal Conditions
enum InputControl {
    LoChan(u8),
    HiChan(u8),
    LoKey(u8),
    HiKey(u8),
    Key,
    LoVel(u8),
    HiVel(u8),
    LoCC(u8),
    HiCC(u8),
    LoBend(u8),
    HiBend(u8),
    LoChanAft(u8),
    HiChanAft(u8),
    LoPolyAft(u8),
    HiPolyAft(u8),
    LoRand(f32),
    HiRand(f32),
    LoHpm(f32),
    HiBpm(f32),
    SeqLength(u8),
    SeqPosition(u8),
    SwLoKey(u8),
    SwHiKey(u8),
    SwLast(u8),
    SwDown(u8),
    SwUp(u8),
    SwPrevious(u8),
    SwVel(String),   // Can be either current or previous.
    Trigger(String), // Can be attack, release, first or legato.
    Group(u8),
    OffBy(u32),
    OffMode(String), // Can be fast or normal
    OnLoCC(u8),
    OnHiCC(u8),
}

// Performance parameters are all sound modifiers including:
// Pitch
// Amplifier
// Filter
// EQ
enum PerformanceParameter {
    SamplePlayer(SamplePlayerParameter),
    Pitch(PitchParameter),
    PitchEG(PitchEGParameter),
    PitchLFO,
    Filter,
    FilterEG, 
    FilterLFO,
    Amplifier,
    AmplifierEG,
    AmplifierLFO,
    Equalizer,
}

struct Global<T> {
    common_params: HashMap<String, T>,
}

struct Curve<T> {
    index: u32,
    values: Vec<(String, T)>,
}

enum AriaEffect {
    Limiter,
    Overdrive,
    Leslie,
    RingMod,
    Delay,
    Bandisto,
    Ambience,
    DubDelay,
    Detune,
    Dither,
    Combo,
    Degrade,
    SubSynth,
    RezFilter,
}

enum SfzEffect {
    Apan,
    Comp,
    Delay,
    Disto,
    Eq,
    Filter,
    Fverb,
    Gate,
    Limiter,
    Lofi,
    Mverb,
    Phaser,
    Static,
    Strings,
    Tdfir,
}
enum EffectType {
    AriaEffects(AriaEffect),
    SfzEffects(SfzEffect),
}
struct Effect {
    effect_type: EffectType,
    param_offset: u32,
    effect_one: f32,   // Reverb
    effect_two: f32,   // Chorus
    effect_three: f32, // Gain of regions send tracks into 3rd effect bus.
    effect_four: f32,  // Gain of regions send tracks into 4th effect bus.
    bus: BusOption,
    dsp_order: u8,
}

enum BusOption {
    Main,
    Aux1,
    Aux2,
    Aux3,
    Aux4,
    Aux5,
    Aux6,
    Aux7,
    Aux8,
    Fx1,
    Fx2,
    Fx3,
    Fx4,
    Midi,
}

enum SamplePlayerParameter {
    Delay(f32),
    DelayRandom(f32),
    DelayCC(f32),
    Offset(u32),
    OffsetRandom(u16),
    OffsetCC(u16),
    End(u32),
    Count(u32),
    LoopMode(String), // Can be no_loop, one_shot, loop_continuous, loop_sustain
    LoopStart(u32),
    LoopEnd(u32),
    SyncBeats(f32),
    SyncOffset(f32),
}

enum PitchParameter {
    Transpose(u8),
    Tune(u8),
    KeyCenter(u8),
    KeyTrack(u16),
    VelTrack(u16),
    Random(u16),
    BendUp(u16),
    BendDown(u16),
    BendStep(u16),
}

enum PitchEGParameter {
    Delay(f32),
    Start(f32),
    Attack(f32),
    Hold(f32),
    Decay(f32),
    Sustain(f32),
    Release(f32),
    Depth(f32),
    Vel2Delay(f32),
    Vel2Attack(f32),
    Vel2Hold(f32),
    Vel2Decay(f32),
    Vel2Sustain(f32),
    Vel2Release(f32),
    Vel2Depth(u32),
}

enum PitchLFO {
    Delay(f32),
    Fade(f32),
    Freq(f32),
    Depth(u16),
    DepthCC(u16), // For DepthCCN
    DepthChanAft(u16),
    DepthPolyAft(u16),
    FreqCC(u8), // For FreqCCN
    FreqChanAft(f32),
    FreqPolyAft(f32),
}
enum Filter {
    FilType(String),
    Cutoff(f32),
    CutoffCC(u16), // For CutoffCCN
    CutoffChanAft(u16),
    CutoffPolyAft(u16),
    Resonance(f32),
    KeyTrack(u16),
    KeyCenter(u8),
    VelTrack(u16),
    Random(u16),
}


enum FilterEG {
    Delay(f32),
    Start(f32),
    Attack(f32),
    Hold(f32),
    Decay(f32),
    Sustain(f32),
    Release(f32),
    Depth(u16),
    Vel2Delay(f32),
    Vel2Attack(f32),
    Vel2Hold(f32),
    Vel2Decay(f32),
    Vel2Sustain(f32),
    Vel2Release(f32),
    Vel2Depth(u16),
}
enum FilterLFO {
    Delay(f32),
    Fade(f32),
    Freq(f32),
    Depth(u16),
    DepthCC(u16), // For DepthCCN
    DepthChanAft(u16),
    DepthPolyAft(u16),
    FreqCC(u8), // For FreqCCN
    FreqChanAft(f32),
    FreqPolyAft(f32),
}


enum Amplifier {
    Volume,
    Pan,
    Width,
    Position,
    KeyTrack,
    KeyCenter,
    VelTrack,
    AmpVelCurve(u8), // For VelCurveN
    Random,
    RtDecay,
    Output,
    GainCC(u8), // For GainCCN
    CfInLowKey,
    CfInHighKey,
    CfOutLowKey,
    CfOutHighKey,
    CfKeyCurve,
    CfInLowVel,
    CfInHighVel,
    CfOutLowVel,
    CfOutHighVel,
    CfVelCurve,
    CfInLowCC(u8), // For CrossfadeInCCN
    CfInHighCC(u8), // For CrossfadeInCCN
    CfOutLowCC(u8), // For CrossfadeOutCCN
    CfOutHighCC(u8), // For CrossfadeOutCCN
    CfCCCurve,
}


enum AmplifierEG {
    Delay,
    Start,
    Attack,
    Hold,
    Decay,
    Sustain,
    Release,
    Vel2Delay,
    Vel2Attack,
    Vel2Hold,
    Vel2Decay,
    Vel2Sustain,
    Vel2Release,
    DelayCC(u8), // For DelayCCN
    StartCC(u8), // For StartCCN
    AttackCC(u8), // For AttackCCN
    HoldCC(u8), // For HoldCCN
    DecayCC(u8), // For DecayCCN
    SustainCC(u8), // For SustainCCN
    ReleaseCC(u8), // For ReleaseCCN
}


enum AmplifierLFO {
    Delay,
    Fade,
    Freq,
    Depth,
    DepthCC(u8), // For DepthCCN
    DepthChanAft,
    DepthPolyAft,
    FreqCC(u8), // For FreqCCN
    FreqChanAft,
    FreqPolyAft,
}


enum Equalizer {
    Eq1Freq,
    Eq2Freq,
    Eq3Freq,
    Eq1FreqCC(u8), // For Eq1FreqCCN
    Eq2FreqCC(u8), // For Eq2FreqCCN
    Eq3FreqCC(u8), // For Eq3FreqCCN
    Eq1Vel2Freq,
    Eq2Vel2Freq,
    Eq3Vel2Freq,
    Eq1Bandwidth,
    Eq2Bandwidth,
    Eq3Bandwidth,
    Eq1BandwidthCC(u8), // For Eq1BandwidthCCN
    Eq2BandwidthCC(u8), // For Eq2BandwidthCCN
    Eq3BandwidthCC(u8), // For Eq3BandwidthCCN
    Eq1Gain,
    Eq2Gain,
    Eq3Gain,
    Eq1GainCC(u8), // For Eq1GainCCN
    Eq2GainCC(u8), // For Eq2GainCCN
    Eq3GainCC(u8), // For Eq3GainCCN
    Eq1Vel2Gain,
    Eq2Vel2Gain,
    Eq3Vel2Gain,
}



struct Sample {
    name: String,
    data: Vec<u8>,
}

struct Master<T> {
    op_codes: Vec<(String, T)>,
    groups: Option<Vec<Group<T>>>,
}

struct Sfz {
    region: Region,
}

enum HeaderType {
    Region,
    Group,
    Control,
    GLobal,
    Curve,
    Effect,
    Master,
    Midi,
    Sample,
}
fn main() {
    println!("HelLo, world!");
}

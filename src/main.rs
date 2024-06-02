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

// TODO: Dedupe EG/LFO variants to be defined once and reused.
// TODO: Explore refinement types.
// TODO: Implement parsing with nom.
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
    PitchLFO(PitchLFOParameter),
    Filter(FilterParameter),
    FilterEG(FilterEGParameter),
    FilterLFO(FilterLFOParameter),
    Amplifier(AmplifierParameter),
    AmplifierEG(AmplifierEGParameter),
    AmplifierLFO(AmplifierLFOParameter),
    Equalizer(EqualizerParameter),
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
    effect_one: f32,   // Reverb in Cakewalk
    effect_two: f32,   // Chorus in Cakewalk
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

enum PitchLFOParameter {
    Delay(f32),
    Fade(f32),
    Freq(f32),
    Depth(u16),
    DepthCC(u16),
    DepthChanAft(u16),
    DepthPolyAft(u16),
    FreqCC(u8),
    FreqChanAft(f32),
    FreqPolyAft(f32),
}
enum FilterParameter {
    FilType(String),
    Cutoff(f32),
    CutoffCC(u16),
    CutoffChanAft(u16),
    CutoffPolyAft(u16),
    Resonance(f32),
    KeyTrack(u16),
    KeyCenter(u8),
    VelTrack(u16),
    Random(u16),
}

enum FilterEGParameter {
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
enum FilterLFOParameter {
    Delay(f32),
    Fade(f32),
    Freq(f32),
    Depth(u16),
    DepthCC(u16),
    DepthChanAft(u16),
    DepthPolyAft(u16),
    FreqCC(u8),
    FreqChanAft(f32),
    FreqPolyAft(f32),
}

enum AmplifierParameter {
    Volume(f32),
    Pan(f32),
    Width(f32),
    Position(f32),
    KeyTrack(f32),
    KeyCenter(u8),
    VelTrack(f32),
    AmpVelCurve(f32),
    Random(f32),
    RtDecay(f32),
    Output(u16),
    GainCC(f32),
    CfInLowKey(u8),
    CfInHighKey(u8),
    CfOutLowKey(u8),
    CfOutHighKey(u8),
    CfKeyCurve(String), // gain or power
    CfInLowVel(f32),
    CfInHighVel(f32),
    CfOutLowVel(f32),
    CfOutHighVel(f32),
    CfVelCurve(String), // gain or power
    CfInLowCC(u8),
    CfInHighCC(u8),
    CfOutLowCC(u8),
    CfOutHighCC(u8),
    CfCCCurve(String),
}

enum AmplifierEGParameter {
    Delay(f32),
    Start(f32),
    Attack(f32),
    Hold(f32),
    Decay(f32),
    Sustain(f32),
    Release(f32),
    Vel2Delay(f32),
    Vel2Attack(f32),
    Vel2Hold(f32),
    Vel2Decay(f32),
    Vel2Sustain(f32),
    Vel2Release,
    DelayCC(f32),
    StartCC(f32),
    AttackCC(f32),
    HoldCC(f32),
    DecayCC(f32),
    SustainCC(f32),
    ReleaseCC(f32),
}

enum AmplifierLFOParameter {
    Delay(f32),
    Fade(f32),
    Freq(f32),
    Depth(u16),
    DepthCC(u16),
    DepthChanAft(u16),
    DepthPolyAft(u16),
    FreqCC(u8),
    FreqChanAft(f32),
    FreqPolyAft(f32),
}

enum EqualizerParameter {
    Eq1Freq(f32),
    Eq2Freq(f32),
    Eq3Freq(f32),
    Eq1FreqCC(f32),
    Eq2FreqCC(f32),
    Eq3FreqCC(f32),
    Eq1Vel2Freq(f32),
    Eq2Vel2Freq(f32),
    Eq3Vel2Freq(f32),
    Eq1Bandwidth(f32),
    Eq2Bandwidth(f32),
    Eq3Bandwidth(f32),
    Eq1BandwidthCC(f32),
    Eq2BandwidthCC(f32),
    Eq3BandwidthCC(f32),
    Eq1Gain(f32),
    Eq2Gain(f32),
    Eq3Gain(f32),
    Eq1GainCC(f32),
    Eq2GainCC(f32),
    Eq3GainCC(f32),
    Eq1Vel2Gain(f32),
    Eq2Vel2Gain(f32),
    Eq3Vel2Gain(f32),
}

struct Sample {
    name: PathBuf,
    data: Vec<u8>,
}

struct Master<T> {
    op_codes: Vec<(String, T)>,
    groups: Option<Vec<Group<T>>>,
}

struct SfzInstrument<T> {
    global: Vec<Global<T>>,
    control: Control,
    group: Vec<Group<T>>,
    master: Option<Vec<Master<T>>>,
    region: Vec<Region>,
    effect: Effect,


}

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
fn main() {
    println!("HelLo, world!");
}

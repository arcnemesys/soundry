// STATUS: [] Real-Time Instrument Script
// STATUS: [Done] Sample Playback
// STATUS: [Done] Instrument Settings
// STATUS: [Done] Voice Lifecycle
// STATUS: [Done] Key Mapping
// STATUS: [DONE] MIDI Conditions
// STATUS: [Done] Internal Conditions
// STATUS: [Done] Triggers
// STATUS: [] Amplifier
// STATUS: [] EQ
// STATUS: [] Filter
// STATUS: [] Pitch
// STATUS: [] Envelope Generators
// STATUS: [] LFO
// STATUS: [] Curves
// STATUS: [] Effects
// STATUS: [] Loading
// STATUS: [] Wavetable Oscillator

// ccN -> CC
// onccN ->  OnCC
// lo -> Low
// hi -> High
//
#[derive(Clone, Debug)]
pub enum InputControl {}

pub enum Triggers {
    Trigger(String), // Can be attack, release, first or legato.
    OnHighCC(u8),
    OnHighHdCC(f32),
    OnLowCC(u8),
    OnLowHdCC(f32),
    StartHighCC(u8),
    StartHighHdCC(f32),
    StartLowCC(u8),
    StartLowHdCC(f32),
    StopHighCC(i32),
    StopHighHdCC(f32),
    StopLowCC(u32),
    StopLowHdCC(f32),
}
pub enum SamplePlayback {
    Count(u32),
    DelayBeatsCurve(u8),
    DelayBeatsOn(String),
    DelayBeatsRandom(f32),
    DelayBeats(f32),
    DelayCC(f32),
    DelayCurve(u8),
    DelayOn(f32),
    DelayRandom(f32),
    DelaySamplesOn(u32),
    DelaySamples(u32),
    Delay(f32),
    Direction(String),
    End(u32),
    LoopCount(u32),
    LoopCrossfade(f32),
    LoopLengthOn(String),
    LoopLengthCC(String),
    LoopMode(String), // Can be no_loop, one_shot, loop_continuous, loop_sustain
    LoopStart(u32),
    LoopStartCC(u32),
    LoopEnd(u32),
    LoopTune(f32),
    LoopType(String),
    Md5(String),
    OffsetMode(String),
    Offset(u32),
    OffsetRandom(u32),
    OffsetCC(u32),
    OffsetOn(u16),
    ReverseHighCC(u8),
    ReverseLowCC(u8),
    SampleDynParamNOnCC(f32),
    SampleDynParam(f32),
    SampleFadeout(f32),
    Sample(String),
    StopBeats(f32),
    SyncBeats(f32),
    SyncOffset(f32),
    WaveGuide(String),
}

pub enum InstrumentSettings {
    Mod(String),
    Default(String),
    Define(DefineDirective),
    GlobalLabel(String),
    GroupLabel(String),
    Hint,
    Include(String),
    LabelCcn(String),
    LabelKey(String),
    LabelOutput(String),
    MasterLabel(String),
    NoteOffset(i8),
    OctaveOffset(i8),
    RegionLabel(String),
    SetCcn(u8),
    SetHdCC(f32),
    SetRealCC(f32),
    SwNoteOffset(i8),
    SwOctaveOffset(i8),
}

pub enum VoiceLifecycle {
    Group(i32),
    NotePolyphony(u32),
    NoteSelfmask(String),
    OffBy(i32),
    OffMode(String), // Can be fast or normal
    OffCurve(i8),
    OffShape(f32),
    OffTime(f32),
    Output(u16),
    PolyphonyGroup(i32),
    PolyphonyStealing(u32),
    RtDead(String),
}

pub enum KeyMapping {
    HiKey(u8),
    HiVel(u8),
    Key,
    LoVel(u8),
    LoKey(u8),
}

pub enum MidiConditions {
    HighBend(i16),
    HighCC(u8),
    HighChan(u8),
    HighHdCC(f32),
    HighProg(u8),
    LowBend(i16),
    LowCC(u8),
    LowChan(u8),
    LowHdCC(f32),
    SwLowKey(u8),
    SwLast(u8),
    SwDown(u8),
    SwHighKey(u8),
    SwUp(u8),
    SwPrevious(u8),
    SwVel(String),
    SostenutoCC(u8),
    SostenutoLow(f32),
    SustainCC(u8),
    SustainLow(f32),
    SustainSw(String),
    SwDefault(u8),
    SwHighLast(u8),
    SwLabel(String),
    SwLowLast(u8),
    VarNN(String),
    VarNNCurveCC(u8),
    VarNNMod(String),
    VarNNOnCC(f32),
}

pub enum InternalConditions {
    HighChanAft(u8),
    HighPolyAft(u8),
    HighBpm(f32),
    HighRand(f32),
    HighTimer(f32),
    LowChanAft(u8),
    LowPolyAft(u8),
    LowRand(f32),
    LowBpm(f32),
    LowTimer(f32),
    SeqLength(u8),
    SeqPosition(u8),
}
#[derive(Clone, Debug)]
pub enum BusOption {
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
#[derive(Clone, Debug)]
pub enum SamplePlayerParameter {
    Delay(f32),
    DelayRandom(f32),
    Count(u32),
    Offset(u32),
    OffsetRandom(u16),
    OffsetCC(u16),
    SyncBeats(f32),
    SyncOffset(f32),
}
#[derive(Clone, Debug)]
pub enum PitchParameter {
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
#[derive(Clone, Debug)]
pub enum PitchEGParameter {
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
#[derive(Clone, Debug)]

pub enum PitchLFOParameter {
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
#[derive(Clone, Debug)]
pub enum FilterParameter {
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
#[derive(Clone, Debug)]

pub enum FilterEGParameter {
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
#[derive(Clone, Debug)]
pub enum FilterLFOParameter {
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
#[derive(Clone, Debug)]
pub enum AmplifierParameter {
    AmpKeyCenter(u8),
    AmpKeyTrack(f32),
    AmpVelTrackCC(f32),
    AmpRandom(f32),
    AmpVelCurve(f32),
    AmplVelTrackCurveCC(u8),
    AmplVelTrackCurveOnCC(u8),
    AmpVelTrackRandom(String),
    AmpVelTrack(f32),
    AmplitudeCC(f32),
    AmplitudeCurveCC(u8),
    AmplitudeOnCC(f32),
    AmplitudeSmoothCC(String),
    Amplitude(f32),
    GainOnCC(f32),
    GainCC(f32),
    GainRandom(String),
    GlobalAmplitude(f32),
    GlobalVolume(f32),
    GroupAmplitude(f32),
    GroupVolume(f32),
    MasterAmplitude(f32),
    MasterVolume(f32),
    PanCC(String),
    PanCurveCC(f32),
    PanKeyCenter(u8),
    PanKeyTrack(f32),
    PanLaw(String),
    PanOnCC(String),
    PanRandom(f32),
    PanSmoothCC(String),
    PanStepCC(String),
    PanVelTrack(f32),
    Pan(f32),
    Phase(String),
    PositionCurveCC(u32),
    PositionKeyCenter(String),
    PositionKeyTrack(String),
    PositionOnCC(String),
    PositionRandom(f32),
    PositionSmoothCC(String),
    PositionStepCC(String),
    PositionVelTrack(String),
    Position(f32),
    RtDecay(f32),
    RtDecayNTime(f32),
    RTDecayN(f32),
    VolumeCurveCC(u32),
    VolumeOnCC(f32),
    VolumeSmoothCC(String),
    VolumeStepCC(String),
    Volume(f32),
    WidthCurveCC(u32),
    WidthOnCC(String),
    WidthStepCC(String),
    Width(f32),
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
#[derive(Clone, Debug)]
pub enum AmplifierEGParameter {
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
#[derive(Clone, Debug)]
pub enum AmplifierLFOParameter {
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
#[derive(Clone, Debug)]

pub enum EqualizerParameter {
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

#[derive(Clone, Debug)]
pub enum AriaEffect {
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
#[derive(Clone, Debug)]
pub enum SfzEffect {
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
#[derive(Clone, Debug)]
pub enum EffectType {
    // These variants are going to have to be
    // refactored as this is a confusing naming.
    AriaEffects(AriaEffect),
    SfzEffects(SfzEffect),
}

// Performance parameters are all sound modifiers including:
// Pitch
// Amplifier
// Filter
// EQ
#[derive(Clone, Debug)]
pub enum PerformanceParameter {
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

pub struct DefineDirective {
    define_name: String,
    define_value: String,
}

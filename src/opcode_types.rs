// These seem to map on to the SFZ opcode categories:
// Key Mapping
// Midi Conditions
// Internal Conditions
#[derive(Clone, Debug, )]
pub enum InputControl {
    LoChan(u8),
    HiChan(u8),
    LoKey(u8),
    HiKey(u8),
    Key,
    LoVel(u8),
    HiVel(u8),
    LoCC(u8),
    HiCC(u8),
    LoBend(i16),
    HiBend(i16),
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
#[derive(Clone, Debug, )]
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
#[derive(Clone, Debug, )]
pub enum SamplePlayerParameter {
    Delay(f32),
    DelayRandom(f32),
    DelayCC(f32),
    Offset(u32),
    OffsetRandom(u16),
    OffsetCC(u16),
    End(i32),
    Count(u32),
    LoopMode(String), // Can be no_loop, one_shot, loop_continuous, loop_sustain
    LoopStart(u32),
    LoopEnd(u32),
    SyncBeats(f32),
    SyncOffset(f32),
}
#[derive(Clone, Debug, )]
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
#[derive(Clone, Debug, )]
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
#[derive(Clone, Debug, )]

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
#[derive(Clone, Debug, )]
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
#[derive(Clone, Debug, )]

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
#[derive(Clone, Debug, )]
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
#[derive(Clone, Debug, )]
pub enum AmplifierParameter {
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
#[derive(Clone, Debug, )]
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
#[derive(Clone, Debug, )]
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
#[derive(Clone, Debug, )]

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

#[derive(Clone, Debug, )]
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
#[derive(Clone, Debug, )]
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
#[derive(Clone, Debug, )]
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
#[derive(Clone, Debug, )]
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
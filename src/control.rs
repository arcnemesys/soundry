use nom::{bytes::complete::tag, character::complete::space0, IResult};



fn control_header_control() -> &'static str {
    let control_header = "
    <control>
    default_path=Samples\
    #define $EXT wav
    #define $KICKKEY 36
    #define $SNAREKEY 38
    #define $HATKEY 42
    #include \"data\\control.sfz\"
    #include \"data\\multiout.sfz\"
    #include \"data\\global.sfz/\"
    #include \"data\\kick.sfz\"
    #include \"data\\snare.sfz\"
    #include \"data\\tom1.sfz\"
    #include \"data\\tom2.sfz\"
    #include \"data\\hihat.sfz\"
    #include \"data\\ride.sfz\"
    #include \"data\\crash.sfz\"
    label_cc30=Bass vol
    label_cc31=Bass pan
    label_cc32=Tune
    label_cc33=Mute
    set_cc40=127
    set_cc100=30";

    control_header
}

// We'll just make default_path a tag and store whatever comes after the equal sign. 
// fn parse_control(sfz_source: &str) -> IResult<&str, &str> {
//     let (sfz_source, _) = tag("<control>")(sfz_source)?;
//     let (sfz_source, _) = space0(sfz_source)?;

// }
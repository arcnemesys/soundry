use refinement::{Predicate, Refinement};

// https://docs.rs/nom/latest/nom/combinator/fn.verify.html
// This might come in handy for applying our refinements.
struct ZeroToSixteen;
type RangeZeroToSixteen = Refinement<u8, ZeroToSixteen>;
impl Predicate<u8> for ZeroToSixteen {
    fn test(x: &u8) -> bool {
        *x <= 16
    }
}

struct ZeroToOneTwentySeven;
type RangeZeroToOneTwentySeven = Refinement<u8, ZeroToOneTwentySeven>;
impl Predicate<u8> for ZeroToOneTwentySeven {
    fn test(x: &u8) -> bool {
        *x <= 16
    }
}

struct EightThousandOneNinetyTwoNegToPos;
type RangeEightThousandOneNinetyTwoNegToPos = Refinement<u16, EightThousandOneNinetyTwoNegToPos>;
impl Predicate<i16> for EightThousandOneNinetyTwoNegToPos {
    fn test(x: &i16) -> bool {
        -8192 <= *x && *x >= 8192
    }
}

struct FloatZeroToOne;
type RangeFloatZeroToOne = Refinement<f32, FloatZeroToOne>;
impl Predicate<f32> for FloatZeroToOne {
    fn test(x: &f32) -> bool {
        0.0 <= *x && *x >= 1.0
    }
}

struct FloatZeroToFiveHundred;
type RangeFloatZeroToFiveHundredw = Refinement<f32, FloatZeroToFiveHundred>;
impl Predicate<f32> for FloatZeroToFiveHundred {
    fn test(x: &f32) -> bool {
        0.0 <= *x && *x >= 500.0
    }
}

struct OneToOneHundred;
type RangeOneToOneHundred = Refinement<u8, OneToOneHundred>;
impl Predicate<u8> for OneToOneHundred {
    fn test(x: &u8) -> bool {
        *x <= 100
    }
}
struct ZeroToU32BitMax;
type RangeZeroToU32BitMax = Refinement<u32, ZeroToU32BitMax>;
impl Predicate<u32> for ZeroToU32BitMax {
    fn test(x: &u32) -> bool {
        *x <= u32::MAX
    }
}
struct FloatZeroToOneHundred;
type RangeFloatZeroToOneHundred = Refinement<f32, FloatZeroToOneHundred>;
impl Predicate<f32> for FloatZeroToOneHundred {
    fn test(x: &f32) -> bool {
        0.0 <= *x && *x >= 100.0
    }
}

struct NegOneToU32BitMax;
type RangeNegOneToU32BitMax = Refinement<u32, NegOneToU32BitMax>;
impl Predicate<i32> for NegOneToU32BitMax {
    fn test(x: &i32) -> bool {
        -1 <= *x && *x >= i32::MAX
    }
}

struct FloatZeroToThirtyTwo;
type RangeFloatZeroToThirtyTwo = Refinement<f32, FloatZeroToThirtyTwo>;
impl Predicate<f32> for FloatZeroToThirtyTwo {
    fn test(x: &f32) -> bool {
        0.0 <= *x && *x >= 32.0
    }
}

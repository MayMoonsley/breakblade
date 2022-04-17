use std::ops::Add;

pub trait Threshold: Add + Sized + PartialEq + Into<f64> {
    // returns this number type's "zero crossing"
    fn zero_crossing() -> Self;

    // returns this number type's maximum value
    fn max_val() -> Self;

    // returns whether or not this is the zero crossing
    fn is_zero(self) -> bool {
        self == Self::zero_crossing()
    }

    // converts a sample to its corresponding dBFS value
    fn to_dbfs(self) -> f64 {
        let val: f64 = self.into();
        let max: f64 = Self::max_val().into();
        20.0 * (val.abs() / max).log10()
    }
}

impl Threshold for u8 {
    fn zero_crossing() -> u8 {
        // per https://stackoverflow.com/questions/44415863/what-is-the-byte-format-of-an-8-bit-monaural-wav-file
        // (cannot verify for myself since audacity won't export 8-bit WAV)
        127
    }

    fn max_val() -> u8 {
        u8::MAX
    }
}

impl Threshold for i16 {
    fn zero_crossing() -> i16 {
        0
    }

    fn max_val() -> i16 {
        i16::MAX
    }
}

impl Threshold for i32 {
    fn zero_crossing() -> i32 {
        0
    }

    fn max_val() -> i32 {
        i32::MAX
    }
}

impl Threshold for f32 {
    fn zero_crossing() -> f32 {
        0.0
    }

    fn max_val() -> f32 {
        // the maximum value for dbfs calculating purposes
        1.0
    }
}
pub mod conversions {
    #[inline(always)]
    pub const fn u8_to_i8(value: u8) -> i8 {
        (value as i16 - 128) as i8
    }

    #[inline(always)]
    pub const fn i8_to_u8(value: i8) -> u8 {
        (value as i16 + 128) as u8
    }

    #[inline(always)]
    pub const fn f32_to_i32(value: f32) -> i32 {
        (value * (i32::MAX as f32)) as i32
    }

    #[inline(always)]
    pub const fn i32_to_f32(value: i32) -> f32 {
        (value as f32) / i32::MAX as f32
    }

    #[inline(always)]
    pub const fn f64_to_i32(value: f64) -> i32 {
        (value * (i32::MAX as f64)) as i32
    }

    #[inline(always)]
    pub const fn i32_to_f64(value: i32) -> f64 {
        (value as f64) / i32::MAX as f64
    }
}

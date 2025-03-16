pub mod decoder;
pub mod encoder;
pub mod error;
pub mod sample;
pub mod wav;

pub type Result<T> = std::result::Result<T, error::Error>;

#[derive(Default, Debug, Clone, Copy)]
pub struct AudioInfo {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub sample_type: Option<sample::SampleType>,
}

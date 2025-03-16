use super::{
    AudioInfo,
    sample::{Sample, SampleType},
};
use crate::reader::LgReader;
use std::fmt::Debug;
use std::marker::PhantomData;

pub mod decoder;
pub mod encoder;
pub mod reader;
pub mod writer;

pub use decoder::LgWavDecoder;
pub use encoder::LgWavEncoder;

// ------------------------- WAVE FORMATS --------------------------
const WAVE_FORMAT_PCM: u16 = 0x0001;
const WAVE_FORMAT_IEEE_FLOAT: u16 = 0x0003;
const WAVE_FORMAT_ALAW: u16 = 0x0006;
const WAVE_FORMAT_MULAW: u16 = 0x0007;
const WAVE_FORMAT_EXTENSIBLE: u16 = 0xFFFE;

#[allow(non_camel_case_types)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub enum WavFmtTag {
    #[default]
    WAVE_FORMAT_PCM,
    WAVE_FORMAT_IEEE_FLOAT,
    WAVE_FORMAT_ALAW,
    WAVE_FORMAT_MULAW,
    WAVE_FORMAT_EXTENSIBLE,
    OTHER(u16),
}
impl From<u16> for WavFmtTag {
    fn from(value: u16) -> Self {
        match value {
            WAVE_FORMAT_PCM => Self::WAVE_FORMAT_PCM,
            WAVE_FORMAT_IEEE_FLOAT => Self::WAVE_FORMAT_IEEE_FLOAT,
            WAVE_FORMAT_ALAW => Self::WAVE_FORMAT_ALAW,
            WAVE_FORMAT_MULAW => Self::WAVE_FORMAT_MULAW,
            WAVE_FORMAT_EXTENSIBLE => Self::WAVE_FORMAT_EXTENSIBLE,
            _ => Self::OTHER(value),
        }
    }
}
impl From<WavFmtTag> for u16 {
    fn from(val: WavFmtTag) -> u16 {
        match val {
            WavFmtTag::WAVE_FORMAT_PCM => WAVE_FORMAT_PCM,
            WavFmtTag::WAVE_FORMAT_IEEE_FLOAT => WAVE_FORMAT_IEEE_FLOAT,
            WavFmtTag::WAVE_FORMAT_ALAW => WAVE_FORMAT_ALAW,
            WavFmtTag::WAVE_FORMAT_MULAW => WAVE_FORMAT_MULAW,
            WavFmtTag::WAVE_FORMAT_EXTENSIBLE => WAVE_FORMAT_EXTENSIBLE,
            WavFmtTag::OTHER(value) => value,
        }
    }
}

// ------------------------- CHUNKS --------------------------
pub(super) enum WavChunks {
    Fmt(AudioInfo),
    /// Not used.
    Fact,
    /// Chunk size
    Data(u32),
}

// ------------------------- SAMPLE --------------------------

pub struct LgWavSampleIter<'si, R, S: Sample>
where
    R: LgReader,
{
    bits_per_sample: u16,
    sample_type: SampleType,
    reader: &'si mut R,
    _phantom: PhantomData<S>,
}
impl<'si, R, S: Sample> LgWavSampleIter<'si, R, S>
where
    R: LgReader,
{
    fn new(reader: &'si mut R, sample_type: SampleType, bits_per_sample: u16) -> Self {
        Self {
            sample_type,
            bits_per_sample,
            reader,
            _phantom: PhantomData,
        }
    }
}
impl<R, S: Sample> Iterator for LgWavSampleIter<'_, R, S>
where
    R: LgReader<Error = super::error::Error>,
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        S::read(self.reader, self.sample_type, self.bits_per_sample).ok()
    }
}

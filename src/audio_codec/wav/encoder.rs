use super::writer::LgWavWriter;
use super::{
    super::{
        Result,
        encoder::LgEncoder,
        sample::{Sample, SampleType},
    },
    AudioInfo,
};
use std::{fs, io, path};

pub struct LgWavEncoder<W: io::Write + io::Seek> {
    pub(super) info: AudioInfo,
    writer: LgWavWriter<W>,
}
impl LgWavEncoder<io::BufWriter<fs::File>> {
    pub fn new(path: impl AsRef<path::Path>, info: AudioInfo) -> Result<Self> {
        let file = fs::File::create(path)?;
        let writer = LgWavWriter::new(io::BufWriter::new(file), &info)?;

        Ok(Self { info, writer })
    }

    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }

    pub fn finish(mut self) -> Result<()> {
        self.writer.finish()
    }
}
impl<W: io::Write + io::Seek> LgEncoder for LgWavEncoder<W> {
    #[inline(always)]
    fn info(&self) -> AudioInfo {
        self.info
    }

    #[inline(always)]
    fn encode_sample<S: Sample>(&mut self, sample: S) -> Result<()> {
        let sample_type = match self.info.sample_type {
            Some(st) => st,
            None => SampleType::INT,
        };

        self.writer
            .write_sample(sample, sample_type, self.info.bits_per_sample)
    }

    #[inline(always)]
    fn encoded_samples(&self) -> usize {
        self.writer.data_bytes_written as usize * self.info.bits_per_sample as usize
    }

    #[inline(always)]
    fn duration(&self) -> usize {
        self.len() / self.info.channels as usize / self.info.sample_rate as usize
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.writer.data_bytes_written as usize / self.info.bits_per_sample as usize
    }
}

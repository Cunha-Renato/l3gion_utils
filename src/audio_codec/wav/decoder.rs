use super::{
    super::{AudioInfo, Result, decoder::LgDecoder, error::Error},
    SampleType,
};
use super::{LgWavSampleIter, WavChunks, reader::LgWavReader};
use std::{fmt, fs, io, path};

pub struct LgWavDecoder<R: io::Read> {
    info: AudioInfo,
    sample_len: usize,

    reader: LgWavReader<R>,
}
impl<R: io::Read> fmt::Debug for LgWavDecoder<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LgWavDecoder")
            .field("info", &self.info)
            .field("sample_len", &self.sample_len)
            .finish()
    }
}
impl LgWavDecoder<io::BufReader<fs::File>> {
    pub fn new(path: impl AsRef<path::Path>) -> Result<Self> {
        let file = fs::File::open(path)?;
        // Already checks the header.
        let mut reader = LgWavReader::new(io::BufReader::new(file))?;

        // Just in case the fmt chunk is not present.
        let mut info = Err(Error::WrongFmt);
        let sample_len;

        loop {
            let chunk = reader.read_next_chunk();
            match chunk? {
                WavChunks::Fmt(wav_info) => info = Ok(wav_info),
                WavChunks::Fact => (),
                WavChunks::Data(d_len) => {
                    match &mut info {
                        Ok(info) => {
                            sample_len = (d_len / (info.bits_per_sample as u32 / 8)) as usize
                        }
                        Err(_) => return Err(Error::WrongFmt),
                    }

                    break;
                }
            }
        }

        Ok(Self {
            info: info?,
            sample_len,
            reader,
        })
    }
}
impl<R: io::Read> LgDecoder for LgWavDecoder<R> {
    #[inline(always)]
    fn info(&self) -> AudioInfo {
        self.info
    }

    #[inline(always)]
    fn samples<S: super::Sample>(&mut self) -> impl Iterator<Item = S> {
        let sample_type = match self.info.sample_type {
            Some(st) => st,
            None => SampleType::INT,
        };

        LgWavSampleIter::new(&mut self.reader, sample_type, self.info.bits_per_sample)
    }

    #[inline(always)]
    fn duration(&self) -> usize {
        self.sample_len / self.info.channels as usize / self.info.sample_rate as usize
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.sample_len
    }

    #[inline(always)]
    fn is_empty(&self) -> bool {
        self.sample_len == 0
    }
}

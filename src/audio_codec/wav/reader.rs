use super::super::AudioInfo;
use super::super::error::Error;
use super::super::sample::SampleType;
use super::WavFmtTag;
use crate::reader::LgReader;
use std::io;

use super::WavChunks;

pub struct LgWavReader<R: io::Read> {
    pub(super) reader: R,
    max_size: usize,
    cursor: usize,
}
impl<R: io::Read> LgReader for LgWavReader<R> {
    type Error = super::super::error::Error;

    fn read_into(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.move_cursor(buffer.len())?;

        Ok(self.reader.read_into(buffer)?)
    }

    fn read_next_bytes<const N: usize>(&mut self) -> Result<[u8; N], Self::Error> {
        self.move_cursor(N)?;

        Ok(self.reader.read_next_bytes()?)
    }

    fn skip_next_bytes<const N: usize>(&mut self) -> Result<(), Self::Error> {
        self.move_cursor(N)?;

        Ok(self.reader.skip_next_bytes::<N>()?)
    }

    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        self.move_cursor(1)?;

        Ok(self.reader.read_u8()?)
    }

    fn read_le_u16(&mut self) -> Result<u16, Self::Error> {
        self.move_cursor(2)?;

        Ok(self.reader.read_le_u16()?)
    }

    fn read_le_u32(&mut self) -> Result<u32, Self::Error> {
        self.move_cursor(4)?;

        Ok(self.reader.read_le_u32()?)
    }

    fn read_le_i8(&mut self) -> Result<i8, Self::Error> {
        self.move_cursor(1)?;

        Ok(self.reader.read_le_i8()?)
    }

    fn read_le_i16(&mut self) -> Result<i16, Self::Error> {
        self.move_cursor(2)?;

        Ok(self.reader.read_le_i16()?)
    }

    fn read_le_i32(&mut self) -> Result<i32, Self::Error> {
        self.move_cursor(4)?;

        Ok(self.reader.read_le_i32()?)
    }

    fn read_le_i32_24(&mut self) -> Result<i32, Self::Error> {
        self.move_cursor(3)?;

        Ok(self.reader.read_le_i32_24()?)
    }

    fn read_le_f32(&mut self) -> Result<f32, Self::Error> {
        self.move_cursor(4)?;

        Ok(self.reader.read_le_f32()?)
    }

    fn read_le_f64(&mut self) -> Result<f64, Self::Error> {
        self.move_cursor(8)?;

        Ok(self.reader.read_le_f64()?)
    }
}
impl<R: io::Read> LgWavReader<R> {
    pub(super) fn new(reader: R) -> Result<Self, Error> {
        Self::read_header(reader)
    }

    pub(super) fn read_header(mut reader: R) -> Result<Self, Error> {
        if b"RIFF" != &reader.read_next_bytes()? {
            return Err(Error::WrongHeader);
        }

        let ck_size = reader.read_le_u32()? - 4;

        if b"WAVE" != &reader.read_next_bytes()? {
            return Err(Error::WrongHeader);
        }

        Ok(Self {
            reader,
            max_size: ck_size as usize,
            cursor: 0,
        })
    }

    pub(super) fn read_next_chunk(&mut self) -> Result<WavChunks, Error> {
        Ok(match &self.read_next_bytes()? {
            b"fmt " => WavChunks::Fmt(self.read_fmt_chunk()?),
            b"fact" => {
                self.read_fact_chunk()?;
                WavChunks::Fact
            }
            b"data" => {
                // Some files will have metadata in them after the data chunk.
                // We don't want that to be marked as a sample, so we make sure we only read the rest of the data.
                let data_ck_size = self.read_le_u32()?;
                self.max_size = data_ck_size as usize;
                self.cursor = 0;

                WavChunks::Data(data_ck_size)
            }

            _ => {
                return Err(Error::WrongFmtInfo(
                    "Currently do not support more chunks other than fmt and data!".to_string(),
                ));
            }
        })
    }

    pub(super) fn read_fmt_chunk(&mut self) -> Result<AudioInfo, Error> {
        let ck_size = self.read_le_u32()? as usize;

        if !(16..=40).contains(&ck_size) {
            return Err(Error::WrongFmt);
        }

        let fmt_tag: WavFmtTag = self.read_le_u16()?.into();
        let channels = self.read_le_u16()?;
        let samples_per_sec = self.read_le_u32()?;
        let _avg_bytes_per_sec = self.read_le_u32()?;
        let _block_align = self.read_le_u16()?;
        let bits_per_sample = self.read_le_u16()?;

        let mut info = AudioInfo {
            channels,
            sample_rate: samples_per_sec,
            bits_per_sample,
            sample_type: Some(match fmt_tag {
                WavFmtTag::WAVE_FORMAT_PCM
                | WavFmtTag::WAVE_FORMAT_ALAW
                | WavFmtTag::WAVE_FORMAT_MULAW
                | WavFmtTag::WAVE_FORMAT_EXTENSIBLE
                | WavFmtTag::OTHER(_) => SampleType::INT,
                WavFmtTag::WAVE_FORMAT_IEEE_FLOAT => SampleType::FLOAT,
            }),
        };

        // Time to check if the info is ok.
        check_fmt(&info)?;

        match (fmt_tag, ck_size) {
            (WavFmtTag::WAVE_FORMAT_PCM, ck_size) => self.read_check_fmt_pcm(ck_size, &info)?,
            (WavFmtTag::WAVE_FORMAT_IEEE_FLOAT, ck_size) => {
                self.read_check_fmt_ieee_float(ck_size, &info)?
            }
            (WavFmtTag::WAVE_FORMAT_ALAW, ck_size) => self.read_check_fmt_alaw(ck_size, &info)?,
            (WavFmtTag::WAVE_FORMAT_MULAW, ck_size) => self.read_check_fmt_mulaw(ck_size, &info)?,
            (WavFmtTag::WAVE_FORMAT_EXTENSIBLE, ck_size) => {
                self.read_check_fmt_extensible(ck_size, &mut info)?
            }

            _ => return Err(Error::WrongFmt),
        };

        // 4 bytes for the ck_id.
        // 4 bytes for the ck_size.
        let bytes_to_skip = (ck_size + 8) - self.cursor;
        self.cursor += bytes_to_skip;

        // 4 bytes for the ck_id.
        // 4 bytes for the ck_size.
        assert_eq!(self.cursor, 8 + ck_size);

        Ok(info)
    }

    fn read_check_fmt_pcm(&mut self, ck_size: usize, fmt: &AudioInfo) -> Result<(), Error> {
        // If ck_size is 16, that means that all the fmt was read.
        if ck_size == 16 {
            return Ok(());
        }

        // If this executes then it means that is a WAVEFORMATEX.
        // Dealing with cb_size.
        self.skip_next_bytes::<2>()?;

        // Dealing with bits_per_sample.
        if fmt.bits_per_sample > 24 || fmt.bits_per_sample < 8 {
            return Err(Error::WrongFmtInfo(
                "Invalid bits_per_sample for PCM format!".to_string(),
            ));
        }

        Ok(())
    }

    fn read_check_fmt_ieee_float(&mut self, ck_size: usize, _: &AudioInfo) -> Result<(), Error> {
        // If ck_size is 16, that means that all the fmt was read.
        if ck_size == 16 {
            return Ok(());
        }
        if ck_size != 18 {
            return Err(Error::WrongFmtInfo(
                "IEEE_FLOAT does not alow for ck_size > 18!".to_string(),
            ));
        }

        // Dealing with cb_size.
        if self.read_le_u16()? != 0 {
            return Err(Error::WrongFmtInfo(
                "IEEE_FLOAT must have cb_size of 0!".to_string(),
            ));
        }

        Ok(())
    }

    #[allow(unused)]
    fn read_check_fmt_alaw(&mut self, ck_size: usize, fmt: &AudioInfo) -> Result<(), Error> {
        todo!()
    }

    #[allow(unused)]
    fn read_check_fmt_mulaw(&mut self, ck_size: usize, fmt: &AudioInfo) -> Result<(), Error> {
        todo!()
    }

    fn read_check_fmt_extensible(
        &mut self,
        ck_size: usize,
        fmt: &mut AudioInfo,
    ) -> Result<(), Error> {
        if ck_size < 40 {
            return Err(Error::WrongFmtInfo(
                "WAVE_FORMAT_EXTENSIBLE must have ck_size of 40!".to_string(),
            ));
        }

        // Dealing with cb_size.
        if self.read_le_u16()? != 22 {
            return Err(Error::WrongFmtInfo(
                "WAVE_FORMAT_EXTENSIBLE must have cb_size of 22!".to_string(),
            ));
        }

        let valid_bits_per_sample = self.read_le_u16()?;
        // Skip channel_mask.
        self.skip_next_bytes::<4>()?;
        // GUID
        let _sub_format: [u8; 16] = self.read_next_bytes()?;

        // TODO: Support different GUIDs.

        if valid_bits_per_sample > 0 {
            fmt.bits_per_sample = valid_bits_per_sample;
        }

        Ok(())
    }

    fn read_fact_chunk(&mut self) -> Result<(), Error> {
        let ck_size = self.read_le_u32()? as usize;
        let mut _skip_bytes = vec![0u8; ck_size];

        self.read_into(&mut _skip_bytes)?;

        Ok(())
    }
}
impl<R: io::Read> LgWavReader<R> {
    fn move_cursor(&mut self, n: usize) -> Result<(), Error> {
        if self.cursor + n > self.max_size + 1 {
            return Err(Error::Io(io::Error::new::<String>(
                io::ErrorKind::UnexpectedEof,
                "".into(),
            )));
        }

        self.cursor += n;

        Ok(())
    }
}

fn check_fmt(fmt: &AudioInfo) -> Result<(), Error> {
    if fmt.channels == 0 {
        return Err(Error::WrongFmtInfo("fmt.channels must be > 0!".to_string()));
    }

    if fmt.bits_per_sample % 8 != 0 || fmt.bits_per_sample == 0 {
        return Err(Error::WrongFmtInfo(
            "bits_per_sample must be non 0 and a multiple of 8!".to_string(),
        ));
    }

    Ok(())
}

use std::{io, usize};
use super::super::error::Error;
use super::super::Result;
use super::super::sample::Sample;
use crate::writer::LgWriter;
use super::WavFmtTag;
use super::super::AudioInfo;
use super::super::sample::SampleType;
use super::{WAVE_FORMAT_EXTENSIBLE, WAVE_FORMAT_IEEE_FLOAT, WAVE_FORMAT_PCM};

const RIFF_CK_SIZE_POSITION: usize = 4;

pub struct LgWavWriter<W: io::Write + io::Seek> {
    pub(super) writer: W,
    pub(super) data_bytes_written: u32,
    pub(super) data_ck_size_position: usize,
}
impl<W: io::Write + io::Seek> Drop for LgWavWriter<W> {
    fn drop(&mut self) {
        let _ = self.finish();
    }
}
impl<W: io::Write + io::Seek> LgWavWriter<W> {
    pub fn new(writer: W, info: &AudioInfo) -> Result<Self> {
        let mut result = Self {
            writer,
            data_bytes_written: 0,
            data_ck_size_position: 0,
        };
        
        result.write_header()?;
        result.write_fmt_chunk(info)?;
        result.writer.write(b"data")?;
        result.writer.write_le_u32(0)?;

        Ok(result)
    }
    
    #[inline(always)]
    pub fn write_sample<S: Sample>(&mut self, sample: S, sample_type: SampleType, bits_per_sample: u16) -> Result<()> {
        sample.write(&mut self.writer, sample_type, bits_per_sample)?;
        self.data_bytes_written += bits_per_sample as u32 / 8;
        
        Ok(())
    }

    pub fn flush(&mut self) -> Result<()> {
        let current_pos = self.writer.seek(io::SeekFrom::Current(0))?;
        self.update_headers()?;
        self.writer.flush()?;
        self.writer.go_to(current_pos as usize)?;
        
        Ok(())
    }
    
    pub fn finish(&mut self) -> Result<()> {
        self.update_headers()?;
        self.writer.flush()?;
        
        Ok(())
    }
}
impl<W: io::Write + io::Seek> LgWavWriter<W> {
    fn write_header(&mut self) -> Result<()> {
        self.writer.write(b"RIFF")?;

        // Empty for now. (ck_size) - position 4.
        self.writer.write_le_u32(0)?;
        self.writer.write(b"WAVE")?;

        Ok(())
    }

    fn write_fmt_chunk(&mut self, info: &AudioInfo) -> Result<()> {
        self.writer.write(b"fmt ")?;

        match info.sample_type {
            Some(SampleType::INT) 
            | None => if info.channels > 2 {
                self.write_check_pcm_ex_fmt(info)
            }
            else {
                self.write_check_pcm_fmt(info)
            },

            Some(SampleType::FLOAT) => self.write_check_ieee_float_fmt(info),
        }
    }
    
    fn write_check_pcm_fmt(&mut self, info: &AudioInfo) -> Result<()> {
        // Header + fmt header + fmt data + data tag.
        self.data_ck_size_position = 12 + 8 + 16 + 4;
        
        // ck_size of 16.
        self.writer.write_le_u32(16)?;

        // fmt_tag.
        self.writer.write_le_u16(WAVE_FORMAT_PCM)?;

        self.write_fmt(info)
    }

    fn write_check_pcm_ex_fmt(&mut self, info: &AudioInfo) -> Result<()> {
        // Header + fmt header + fmt data + data tag.
        self.data_ck_size_position = 12 + 8 + 40 + 4;
        
        // ck_size of 40.
        self.writer.write_le_u32(40)?;

        // fmt_tag.
        self.writer.write_le_u16(WAVE_FORMAT_EXTENSIBLE)?;

        self.write_fmt(info)?;
            
        // cb_size.
        self.writer.write_le_u16(22)?;
        
        // valid_bits_per_sample.
        self.writer.write_le_u16(info.bits_per_sample)?;
        
        // channel_mask.
        let channels = if info.channels > 18 { 18 } else { info.channels };
        self.writer.write_le_u32(channels as u32)?;
        
        // sub_format.
        self.writer.write(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80,
            0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71])?;
        
        Ok(())
    }

    fn write_check_ieee_float_fmt(&mut self, info: &AudioInfo) -> Result<()> {
        // Header + fmt header + fmt data + data tag.
        self.data_ck_size_position = 12 + 8 + 18 + 4;

        // ck_size of 18.
        self.writer.write_le_u32(18)?;
        
        // fmt_tag.
        self.writer.write_le_u16(WAVE_FORMAT_IEEE_FLOAT)?;
        
        self.write_fmt(info)?;
        
        // cb_size.
        self.writer.write_le_u16(0)?;
        
        Ok(())
    }
    
    fn write_fmt(&mut self, info: &AudioInfo) -> Result<()> {
        // n_channels.
        self.writer.write_le_u16(info.channels)?;

        // samples_per_sec.
        self.writer.write_le_u32(info.sample_rate)?;
        
        // avg_bytes_per_sec.
        let bytes_per_sec = info.sample_rate
            * (info.bits_per_sample / 8) as u32
            * info.channels as u32;

        self.writer.write_le_u32(bytes_per_sec)?;
        
        // block_align.
        self.writer.write_le_u16((bytes_per_sec / info.sample_rate) as u16)?;
            
        // bits_per_sample.
        self.writer.write_le_u16(info.bits_per_sample)?;
        
        Ok(())
    }

    fn update_headers(&mut self) -> Result<()> {
        let file_size = self.data_bytes_written + self.data_ck_size_position as u32 - 4;
        
        // RIFF ck_size.
        self.writer.go_to(RIFF_CK_SIZE_POSITION)?;
        self.writer.write_le_u32(file_size)?;
        
        // Data ck_size.
        self.writer.go_to(self.data_ck_size_position)?;
        self.writer.write_le_u32(self.data_bytes_written)?;

        Ok(())
    }
}
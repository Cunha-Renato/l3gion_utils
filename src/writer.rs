use std::io;

pub trait LgWriter {
    type Error;

    fn go_to(&mut self, position: usize) -> Result<usize, Self::Error>;

    fn write_u8(&mut self, data: u8) -> Result<usize, Self::Error>;

    fn write_le_u16(&mut self, data: u16) -> Result<usize, Self::Error>;

    fn write_le_u32(&mut self, data: u32) -> Result<usize, Self::Error>;

    fn write_le_i8(&mut self, data: i8) -> Result<usize, Self::Error>;

    fn write_le_i16(&mut self, data: i16) -> Result<usize, Self::Error>;

    fn write_le_i32(&mut self, data: i32) -> Result<usize, Self::Error>;

    fn write_le_i32_24(&mut self, data: i32) -> Result<(), Self::Error>;

    fn write_le_f32(&mut self, data: f32) -> Result<(), Self::Error>;

    fn write_le_f64(&mut self, data: f64) -> Result<(), Self::Error>;
}
impl<W: io::Write + io::Seek> LgWriter for W {
    type Error = std::io::Error;

    fn go_to(&mut self, position: usize) -> Result<usize, Self::Error> {
        Ok(self.seek(io::SeekFrom::Start(position as u64))? as usize)
    }

    fn write_u8(&mut self, data: u8) -> Result<usize, Self::Error> {
        self.write(&[data])
    }

    fn write_le_u16(&mut self, data: u16) -> Result<usize, Self::Error> {
        self.write(&data.to_le_bytes())
    }

    fn write_le_u32(&mut self, data: u32) -> Result<usize, Self::Error> {
        self.write(&data.to_le_bytes())
    }

    fn write_le_i8(&mut self, data: i8) -> Result<usize, Self::Error> {
        let data = crate::bytes::conversions::i8_to_u8(data);

        self.write_u8(data)
    }

    fn write_le_i16(&mut self, data: i16) -> Result<usize, Self::Error> {
        self.write(&data.to_le_bytes())
    }

    fn write_le_i32(&mut self, data: i32) -> Result<usize, Self::Error> {
        self.write(&data.to_le_bytes())
    }

    fn write_le_i32_24(&mut self, data: i32) -> Result<(), Self::Error> {
        let buf = data.to_le_bytes();
        self.write_all(&[buf[0], buf[1], buf[2]])
    }

    fn write_le_f32(&mut self, data: f32) -> Result<(), Self::Error> {
        self.write_all(&data.to_le_bytes())
    }

    fn write_le_f64(&mut self, data: f64) -> Result<(), Self::Error> {
        self.write_all(&data.to_le_bytes())
    }
}

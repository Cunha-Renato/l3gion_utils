use std::io;

pub trait LgReader {
    type Error;

    fn read_into(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error>;

    fn read_next_bytes<const N: usize>(&mut self) -> Result<[u8; N], Self::Error>;

    fn skip_next_bytes<const N: usize>(&mut self) -> Result<(), Self::Error>;

    #[inline]
    fn read_exact_n<const N: usize>(&mut self) -> Result<[u8; N], Self::Error> {
        let mut buf = [8; N];
        self.read_into(&mut buf)?;

        Ok(buf)
    }

    #[inline]
    fn read_u8(&mut self) -> Result<u8, Self::Error> {
        Ok(self.read_exact_n::<1>()?[0])
    }

    #[inline]
    fn read_le_u16(&mut self) -> Result<u16, Self::Error> {
        Ok(u16::from_le_bytes(self.read_exact_n()?))
    }

    #[inline]
    fn read_le_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(u32::from_le_bytes(self.read_exact_n()?))
    }

    #[inline]
    fn read_le_i8(&mut self) -> Result<i8, Self::Error> {
        Ok(crate::bytes::conversions::u8_to_i8(
            self.read_exact_n::<1>()?[0],
        ))
    }

    #[inline]
    fn read_le_i16(&mut self) -> Result<i16, Self::Error> {
        Ok(i16::from_le_bytes(self.read_exact_n()?))
    }

    #[inline]
    fn read_le_i32(&mut self) -> Result<i32, Self::Error> {
        Ok(i32::from_le_bytes(self.read_exact_n()?))
    }

    #[inline]
    fn read_le_i32_24(&mut self) -> Result<i32, Self::Error> {
        let buf: [u8; 3] = self.read_exact_n()?;

        Ok(i32::from_le_bytes([
            buf[0],
            buf[1],
            buf[2],
            if buf[2] & 0x80 != 0 { 0xFF } else { 0x00 }, // Sign extend if needed
        ]))
    }

    #[inline]
    fn read_le_f32(&mut self) -> Result<f32, Self::Error> {
        Ok(f32::from_le_bytes(self.read_exact_n()?))
    }

    #[inline]
    fn read_le_f64(&mut self) -> Result<f64, Self::Error> {
        Ok(f64::from_le_bytes(self.read_exact_n()?))
    }

    #[inline]
    fn read_be_u16(&mut self) -> Result<u16, Self::Error> {
        Ok(u16::from_be_bytes(self.read_exact_n()?))
    }

    #[inline]
    fn read_be_u32(&mut self) -> Result<u32, Self::Error> {
        Ok(u32::from_be_bytes(self.read_exact_n()?))
    }

    #[inline]
    fn read_be_i16(&mut self) -> Result<i16, Self::Error> {
        Ok(i16::from_be_bytes(self.read_exact_n()?))
    }

    #[inline]
    fn read_be_i32(&mut self) -> Result<i32, Self::Error> {
        Ok(i32::from_be_bytes(self.read_exact_n()?))
    }

    #[inline]
    fn read_be_f32(&mut self) -> Result<f32, Self::Error> {
        Ok(f32::from_be_bytes(self.read_exact_n()?))
    }

    #[inline]
    fn read_be_f64(&mut self) -> Result<f64, Self::Error> {
        Ok(f64::from_be_bytes(self.read_exact_n()?))
    }
}
impl<R: io::Read> LgReader for R {
    type Error = std::io::Error;

    fn read_into(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error> {
        self.read_exact(buffer)
    }

    fn read_next_bytes<const N: usize>(&mut self) -> Result<[u8; N], Self::Error> {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;

        Ok(buf)
    }

    fn skip_next_bytes<const N: usize>(&mut self) -> Result<(), Self::Error> {
        self.read_exact(&mut [0; N])
    }
}

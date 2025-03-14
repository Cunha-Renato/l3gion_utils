use std::io;

pub trait LgReader {
    type Error;

    fn read_into(&mut self, buffer: &mut [u8]) -> Result<(), Self::Error>;

    fn read_next_bytes<const N: usize>(&mut self) -> Result<[u8; N], Self::Error>;

    fn skip_next_bytes<const N: usize>(&mut self) -> Result<(), Self::Error>;

    fn read_le_u8(&mut self) -> Result<u8, Self::Error>;

    fn read_le_u16(&mut self) -> Result<u16, Self::Error>;
    
    fn read_le_u32(&mut self) -> Result<u32, Self::Error>;

    fn read_le_i8(&mut self) -> Result<i8, Self::Error>;
    
    fn read_le_i16(&mut self) -> Result<i16, Self::Error>;
    
    fn read_le_i32(&mut self) -> Result<i32, Self::Error>;

    fn read_le_i32_24(&mut self) -> Result<i32, Self::Error>;
    
    fn read_le_f32(&mut self) -> Result<f32, Self::Error>;
    
    fn read_le_f64(&mut self) -> Result<f64, Self::Error>;
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

    fn read_le_u8(&mut self) -> Result<u8, Self::Error> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        
        Ok(buf[0])
    }

    fn read_le_u16(&mut self) -> Result<u16, Self::Error> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        
        Ok(u16::from_le_bytes(buf))
    }
    
    fn read_le_u32(&mut self) -> Result<u32, Self::Error> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        
        Ok(u32::from_le_bytes(buf))
    }

    fn read_le_i8(&mut self) -> Result<i8, Self::Error> {
        let mut buf = [0];
        self.read_exact(&mut buf)?;
        
        Ok(crate::bytes::conversions::u8_to_i8(buf[0]))
    }
    
    fn read_le_i16(&mut self) -> Result<i16, Self::Error> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        
        Ok(i16::from_le_bytes(buf))
    }
    
    fn read_le_i32(&mut self) -> Result<i32, Self::Error> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        
        Ok(i32::from_le_bytes(buf))
    }

    fn read_le_i32_24(&mut self) -> Result<i32, Self::Error> {
        let mut buf = [0; 3];
        self.read_exact(&mut buf)?;

        Ok(i32::from_le_bytes([
            buf[0], 
            buf[1], 
            buf[2], 
            if buf[2] & 0x80 != 0 { 0xFF } else { 0x00 } // Sign extend if needed
        ]))
    }
    
    fn read_le_f32(&mut self) -> Result<f32, Self::Error> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        
        Ok(f32::from_le_bytes(buf))
    }
    
    fn read_le_f64(&mut self) -> Result<f64, Self::Error> {
        let mut buf = [0; 8];
        self.read_exact(&mut buf)?;
        
        Ok(f64::from_le_bytes(buf))
    }
}
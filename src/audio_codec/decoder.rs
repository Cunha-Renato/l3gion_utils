use super::{AudioInfo, sample::Sample};

pub trait LgDecoder: Sized {
    fn info(&self) -> AudioInfo;
    
    /// Iterator over the samples.
    /// Once you iterate over the elements, calling this again will not be on the start of 
    /// the samples, so it is recommended that you store the samples in a container if 
    /// you need to reuse them.
    fn samples<S: Sample>(&mut self) -> impl Iterator<Item = S>;
    
    /// Duration of the audio in seconds.
    fn duration(&self) -> usize;
    
    /// Length of the samples.
    fn len(&self) -> usize;
}
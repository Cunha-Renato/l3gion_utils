use super::{AudioInfo, Result, sample::Sample};

pub trait LgEncoder {
    fn info(&self) -> AudioInfo;

    fn encode_sample<S: Sample>(&mut self, sample: S) -> Result<()>;

    /// Number of samples encoded so far.
    fn encoded_samples(&self) -> usize;

    fn duration(&self) -> usize;

    fn len(&self) -> usize;
}

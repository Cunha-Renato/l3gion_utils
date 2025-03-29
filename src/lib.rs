#[cfg(feature = "bytes")]
pub mod bytes;

#[cfg(feature = "reader")]
pub mod reader;

#[cfg(feature = "writer")]
pub mod writer;

#[cfg(feature = "audio_codec")]
pub mod audio_codec;

#[cfg(feature = "atlas_gen")]
pub mod atlas_gen;

// Crate level.

#[cfg(all(
    any(feature = "reader", feature = "writer", feature = "audio_codec",),
    not(feature = "bytes")
))]
pub(crate) mod bytes;

#[cfg(all(feature = "audio_codec", not(feature = "reader")))]
pub(crate) mod reader;

#[cfg(all(feature = "audio_codec", not(feature = "writer")))]
pub(crate) mod writer;

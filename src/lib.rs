//! An implementation of [FLAC](https://xiph.org/flac), free lossless audio
//! codec, written in Rust.
//!
//! # Examples
//!
//! Basic decoding from a file.
//!
//! ```
//! match flac::Stream::from_file("path/to/file.flac") {
//!   Ok(mut stream) => {
//!     // Copy of `StreamInfo` to help convert to a different audio format.
//!     let info = stream.info();
//!
//!     for sample in stream.iter() {
//!       // Iterate over each decoded sample
//!     }
//!   }
//!   Err(error)     => println!("{}", error),
//! }
//! ```

#[macro_use]
extern crate nom;

#[macro_use]
mod utility;
mod frame;
mod subframe;
pub mod metadata;
pub mod stream;

pub use metadata::Metadata;
pub use stream::Stream;
pub use utility::{StreamProducer, ReadStream, ByteStream};

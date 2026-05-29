//! Pure-Rust, clean-room **AIFF / AIFF-C (AIFC)** container parser
//! for the OxideAV framework.
//!
//! AIFF is an application of the **EA IFF 85** chunked-file standard:
//! an outermost `FORM` chunk holds a `COMM` (format) chunk, an `SSND`
//! (sample data) chunk, and any number of optional metadata chunks.
//! It is the big-endian Macintosh counterpart to Microsoft's
//! little-endian RIFF/WAVE.
//!
//! This crate parses the two form types defined by Apple:
//!
//! * **`FORM/AIFF`** — the original uncompressed format (Apple AIFF
//!   v1.3, 1989). 80-bit IEEE-extended sample rate, big-endian
//!   two's-complement PCM samples.
//! * **`FORM/AIFC`** — the compressed-capable extension (Apple
//!   AIFF-C draft 8/26/91). Adds a `FVER` chunk and extends `COMM`
//!   with a `compressionType` FourCC + Pascal-string compression
//!   name.
//!
//! The clean-room layout summary the parser is built against is in
//! `docs/audio/aiff/aiff-aifc-format.md`, derived from the staged
//! Apple specs (`aiff-1.3.pdf`, `aiff-c.pdf`, `aiff-c.txt`).
//!
//! ## Initial coverage (round 185)
//!
//! * The EA IFF 85 chunk walker [`chunk::ChunkIter`] — ckID + ckSize
//!   + odd-size pad byte handling.
//! * 80-bit IEEE 754 extended-precision sample-rate decode
//!   ([`extended::decode_sample_rate`] / [`extended::decode_extended`]).
//! * `COMM` parser ([`common::parse_common`]) for both AIFF and
//!   AIFF-C forms, including the AIFF-C `compressionType` + Pascal-
//!   string compression name.
//! * `FVER` + `SSND` + the top-level FORM walker ([`form::parse`]).
//! * PCM compression-flavour readers ([`pcm::decode_pcm`]) for the
//!   uncompressed AIFF-C `compressionType` FourCCs **`NONE`**,
//!   **`twos`**, **`sowt`**, **`raw `**, **`fl32`**/`FL32`, and
//!   **`fl64`**/`FL64`. Integer flavours promote to left-justified
//!   `i32`; float flavours stay in their native precision.
//!
//! Codec-bearing AIFF-C `compressionType` FourCCs (`ima4`, `ulaw`,
//! `alaw`, `MAC3`, `MAC6`, `GSM `, `G722`, `G726`, `G728`, `QDMC`,
//! `QDM2`, `Qclp`, …) are recognised in the chunk parser but routed
//! through sibling codec crates (`oxideav-adpcm` for `ima4`,
//! `oxideav-g711` for `ulaw` / `alaw`, etc.) — they are NOT decoded
//! here. [`pcm::decode_pcm`] returns
//! [`AiffError::UnsupportedPcmCompression`] for those so callers can
//! dispatch cleanly.
//!
//! The optional text chunks (`NAME`, `AUTH`, `(c) `, `ANNO`, `COMT`),
//! marker / instrument chunks (`MARK`, `INST`), and the MIDI / AESD /
//! APPL chunks are recognised by the chunk walker and skipped silently
//! by the FORM-level parser; later rounds will surface them as
//! structured fields.
//!
//! ## Free-standing vs. framework wiring
//!
//! With the default `registry` feature on, the crate also registers a
//! [`Demuxer`](oxideav_core::Demuxer) factory under the codec id
//! `"aiff"` in `oxideav-core`'s `ContainerRegistry` via the
//! [`oxideav_core::register!`] macro. Build with
//! `default-features = false` to use the chunk walker, extended-float
//! decoder, and PCM readers without any framework dependency.

#![forbid(unsafe_code)]
#![deny(missing_docs)]
#![deny(rust_2018_idioms)]

pub mod chunk;
pub mod common;
pub mod error;
pub mod extended;
pub mod form;
pub mod pcm;

#[cfg(feature = "registry")]
pub mod demuxer;

pub use crate::chunk::{Chunk, ChunkIter};
pub use crate::common::{
    parse_common, CommonChunk, COMPRESSION_FL32, COMPRESSION_FL32_UC, COMPRESSION_FL64,
    COMPRESSION_FL64_UC, COMPRESSION_NONE, COMPRESSION_RAW, COMPRESSION_SOWT, COMPRESSION_TWOS,
};
pub use crate::error::{AiffError, Result};
pub use crate::extended::{decode_extended, decode_sample_rate};
pub use crate::form::{parse, Form, SoundData};
pub use crate::pcm::{decode_pcm, is_pcm_compression, PcmSamples};

#[cfg(feature = "registry")]
pub use crate::demuxer::{make_demuxer, register, AiffDemuxer, FORMAT_NAME};

/// Codec id string under which the demuxer factory installs itself
/// in `oxideav-core`'s `ContainerRegistry` (`"aiff"`).
pub const CODEC_ID_STR: &str = "aiff";

#[cfg(feature = "registry")]
oxideav_core::register!("aiff", register);

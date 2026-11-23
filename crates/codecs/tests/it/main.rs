//! The integration tests, one module per file, linked into one test binary.
//! `tests/jxl_tail_heap.rs` stays a binary of its own: it installs dhat as the global allocator.

mod bench;
mod corpus;
mod float_tiff;
mod fuzz;
mod header;
#[cfg(feature = "heif")]
mod heif;
#[cfg(feature = "jxl")]
mod jxl_streaming;
mod roundtrip;

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added — round 185 Phase 1 bootstrap

First public landing of `oxideav-aiff`. Implements the AIFF / AIFF-C
container parser, the EA IFF 85 chunk walker, the 80-bit IEEE 754
extended-precision sample-rate decoder, and the uncompressed PCM
flavour readers, all built clean-room against the staged Apple specs
(`docs/audio/aiff/aiff-1.3.pdf`, `aiff-c.pdf`, `aiff-c.txt`) and the
clean-room layout summary (`docs/audio/aiff/aiff-aifc-format.md`).

- **`chunk::ChunkIter`** — EA IFF 85 ckID + ckSize + odd-size pad-byte
  iterator. Tolerates a missing trailing pad at EOF; flags an
  oversized chunk that would overrun its parent container.
- **`extended::decode_extended` / `decode_sample_rate`** — 10-byte
  80-bit IEEE 754 extended-precision float → `f64`. Sample rates
  round-trip exactly for every common audio rate (8 000..192 000 Hz).
  NaN / ±infinity / zero / negative values are rejected by the
  `decode_sample_rate` guard.
- **`common::parse_common`** — parses the COMM chunk for both
  formType `b"AIFF"` (18-byte body) and `b"AIFC"` (≥22-byte body +
  Pascal-string compressionName). Rejects sampleSize outside
  `1..=32`, zero/negative numChannels, and short bodies. Exposes the
  on-wire `compressionType` FourCC.
- **`form::parse`** — top-level FORM walker. Selects between AIFF
  v1.3 and AIFF-C by formType; collects COMM, SSND (with offset
  honouring), and FVER from the inner chunk stream regardless of
  order (per spec §4); allows missing SSND when `numSampleFrames ==
  0`; silently skips unknown / not-yet-modelled chunks.
- **`pcm::decode_pcm` + `is_pcm_compression`** — sample readers for
  the uncompressed AIFF-C `compressionType` FourCCs `NONE`, `twos`,
  `sowt`, `raw ` (8-bit unsigned, offset-binary), `fl32` / `FL32`,
  and `fl64` / `FL64`. Integer flavours promote to left-justified
  `i32`; float flavours stay native. Deinterleaves frame-interleaved
  payload into per-channel planes.
- **`demuxer::AiffDemuxer`** (default `registry` feature) —
  `oxideav_core::Demuxer` implementation. Registers under the format
  name `"aiff"`, the `.aif` / `.aiff` / `.aifc` filename extensions,
  and a probe that matches `FORM????AIFF` / `FORM????AIFC` at offset
  0. Emits one packet carrying the SSND payload; sets the stream's
  `CodecParameters.codec_id` from the AIFF-C `compressionType`
  (`sowt`→`pcm_s16le`, `ima4`→`adpcm_ima_qt`, `ulaw`→`ulaw`, …) and
  preserves the FourCC on `CodecParameters.tag` for muxer round-trip
  in a later round.
- **Direct-API `make_demuxer(bytes)`** parallel to the
  `oxideav_core::register!` entry, per the dual-API convention.

### Not yet implemented

- AIFF / AIFF-C muxer.
- Text-metadata chunks (`NAME`, `AUTH`, `(c) `, `ANNO`, `COMT`) are
  recognised by the chunk walker but not surfaced as structured
  fields.
- Marker / Instrument chunks (`MARK`, `INST`) are recognised but
  not surfaced.
- `MIDI` / `AESD` / `APPL` chunks are recognised but not surfaced.
- The compressed AIFF-C codecs (`ima4`, `ulaw`, `alaw`, `MAC3`,
  `MAC6`, `GSM `, G.72x, QDMC / QDM2 / Qclp) decode in sibling codec
  crates; `oxideav-aiff` only exposes the wire FourCC.

### Provenance

All design decisions trace to `docs/audio/aiff/aiff-aifc-format.md`
and the staged Apple specifications it summarises. No external
implementation source was consulted, quoted, paraphrased, or used as
a cross-check oracle.

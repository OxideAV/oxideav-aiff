# oxideav-aiff

Pure-Rust, clean-room AIFF / AIFF-C (AIFC) container parser for the
OxideAV framework.

AIFF is an application of the **EA IFF 85** chunked-file standard
(Electronic Arts, 1985): the whole file is one big-endian `FORM`
chunk carrying a `COMM` (format) chunk and an `SSND` (sample data)
chunk, plus optional marker / instrument / metadata chunks. It is
the big-endian Macintosh counterpart to Microsoft's little-endian
RIFF/WAVE, formalised in two Apple specifications:

- **AIFF v1.3** (Apple, January 4 1989) — the original uncompressed
  format. `FORM/AIFF`, `COMM`, `SSND`, 80-bit IEEE-extended sample
  rate, big-endian two's-complement PCM.
- **AIFF-C / AIFC** (Apple, draft 8/26/91) — the compressed-capable
  extension. Adds an `FVER` Format Version chunk and extends `COMM`
  with a `compressionType` FourCC + Pascal-string compression name,
  carrying both PCM flavours (`NONE`, `twos`, `sowt`, `raw `,
  `fl32`, `fl64`) and compressed codecs (`ima4`, `ulaw`, `alaw`,
  `MAC3`, `MAC6`, `GSM `, G.72x, QDMC/QDM2, Qclp).

The crate is built against the clean-room layout summary
`docs/audio/aiff/aiff-aifc-format.md` derived from the staged Apple
specs (`aiff-1.3.pdf`, `aiff-c.pdf`, `aiff-c.txt`); no codec or
demuxer source from any other project was consulted.

## What round 185 lands (Phase 1 bootstrap)

- **EA IFF 85 chunk walker** — `chunk::ChunkIter` with ckID + ckSize
  + odd-size pad-byte handling, tolerant of a missing trailing pad
  at EOF (a common encoder quirk).
- **80-bit IEEE 754 extended-precision sample-rate decode** —
  `extended::decode_extended` and `decode_sample_rate`. Round-trips
  every common rate (8 000 / 11 025 / 16 000 / 22 050 / 24 000 /
  32 000 / 44 100 / 48 000 / 88 200 / 96 000 / 176 400 / 192 000 Hz).
  Rejects NaN, ±infinity, zero, and negative values when the input
  is fed to the `decode_sample_rate` guard.
- **COMM chunk parser** (`common::parse_common`) for both the AIFF
  v1.3 18-byte form and the AIFF-C ≥22-byte form (with
  compressionType + Pascal-string compressionName).
- **FORM walker** (`form::parse`) — selects between FORM/AIFF and
  FORM/AIFC by formType, scans inner chunks (order-independent per
  spec §4), collects COMM, SSND, and FVER; passes unknown chunks
  through unchanged.
- **PCM compression-flavour readers** (`pcm::decode_pcm`) for the
  uncompressed AIFF-C FourCCs:
  - `NONE` / `twos` → big-endian two's-complement, any `sampleSize`
    in 1..=32; promoted to left-justified `i32`.
  - `sowt` → little-endian two's-complement (byte-swapped); same
    promotion.
  - `raw ` → 8-bit unsigned (offset-binary); `sampleSize` must be 8.
  - `fl32` / `FL32` → 32-bit IEEE float, big-endian.
  - `fl64` / `FL64` → 64-bit IEEE float, big-endian.
- **`oxideav-core` Demuxer wiring** (`demuxer::AiffDemuxer`) — under
  the default `registry` feature, installs a `ContainerRegistry`
  factory under format name `"aiff"`, a probe function recognising
  `FORM????AIFF` and `FORM????AIFC` at offset 0, and the `.aif` /
  `.aiff` / `.aifc` filename extensions. Emits one packet carrying
  the SSND payload; sets the stream's `CodecParameters.codec_id`
  to the framework codec id mapped from the AIFF-C
  `compressionType` (e.g. `sowt` → `pcm_s16le`,
  `ima4` → `adpcm_ima_qt`, `ulaw` → `ulaw`) and preserves the
  FourCC on `CodecParameters.tag` for muxer round-trip.

## Status snapshot

| Surface                                        | Decoder        | Encoder |
|------------------------------------------------|----------------|---------|
| FORM/AIFF chunk walking                        | ✅ round 185   | n/a     |
| FORM/AIFC chunk walking                        | ✅ round 185   | n/a     |
| 80-bit IEEE extended sample-rate decode        | ✅ round 185   | n/a     |
| PCM flavours `NONE` / `twos` / `sowt` / `raw ` | ✅ round 185   | ❌ next  |
| Float flavours `fl32` / `fl64`                 | ✅ round 185   | ❌ next  |
| `ima4` ADPCM compressionType                   | (routes to oxideav-adpcm) | n/a |
| `ulaw` / `alaw` compressionTypes               | (routes to oxideav-g711)  | n/a |
| `MAC3` / `MAC6` / `GSM ` / G.72x / QDMC / QDM2 | unrouted (no sibling)     | n/a |
| `NAME`/`AUTH`/`(c) `/`ANNO`/`COMT` text chunks | recognised, not surfaced  | n/a |
| `MARK`/`INST` marker + instrument chunks       | recognised, not surfaced  | n/a |
| `MIDI`/`AESD`/`APPL` raw-payload chunks        | recognised, not surfaced  | n/a |
| AIFF / AIFF-C muxer                            | ❌ next round  | ❌      |

## Free-standing build

`default-features = false` drops the `oxideav-core` dependency and
keeps the chunk walker, COMM/SSND/FVER parser, 80-bit extended
decoder, and PCM sample readers. A free-standing consumer that just
needs to read an `.aiff` / `.aifc` file's audio bytes can stay on
that minimal surface:

```toml
[dependencies]
oxideav-aiff = { version = "0.0", default-features = false }
```

## Spec provenance

Built against the clean-room layout summary
[`docs/audio/aiff/aiff-aifc-format.md`](../../docs/audio/aiff/aiff-aifc-format.md),
which is itself derived from the staged Apple specifications:

- [`docs/audio/aiff/aiff-1.3.pdf`](../../docs/audio/aiff/aiff-1.3.pdf) — *Audio
  Interchange File Format AIFF*, version 1.3 (Apple Computer,
  1989-01-04).
- [`docs/audio/aiff/aiff-c.pdf`](../../docs/audio/aiff/aiff-c.pdf) — *Audio
  Interchange File Format AIFF-C* draft (Apple Computer,
  8/26/91 = "same as 9/30/90").
- [`docs/audio/aiff/aiff-c.txt`](../../docs/audio/aiff/aiff-c.txt) — plain-text
  transcription of the AIFF-C draft.

No external implementation source was consulted, quoted, paraphrased,
or used as a cross-check oracle for any line in this crate.
Black-box invocations of validator binaries as opaque file-to-file
processes remain permissible per workspace policy for future rounds.

## License

MIT. See [`LICENSE`](LICENSE).

# oxideav-aiff has been retired

As of OxideAV 2026-05-30, AIFF / AIFF-C container support has moved
into [`oxideav-iff`](https://github.com/OxideAV/oxideav-iff) under
the `oxideav_iff::aiff` module.

Migration:

```toml
# Before
oxideav-aiff = "0.0.1"

# After (next oxideav-iff release after the merge, v0.0.9 or later)
oxideav-iff = "0.0"
```

Rust paths: `oxideav_aiff::*` → `oxideav_iff::aiff::*`.

The full surface moved over: `Chunk` / `ChunkIter` slice-based walker,
80-bit IEEE-extended sample-rate decode, `CommonChunk` /
`parse_common`, `Form` / `parse` / `SoundData`, PCM compression-
flavour readers (`decode_pcm`, `is_pcm_compression`, `PcmSamples`),
and the `AiffDemuxer` factory registered under codec id `"aiff"` for
`.aif` / `.aiff` / `.aifc` files.

The merge commit on `oxideav-iff` is
[`2ba62ee`](https://github.com/OxideAV/oxideav-iff/commit/2ba62ee).

This repository is archived. `oxideav-aiff` `0.0.1` has been yanked
from crates.io.

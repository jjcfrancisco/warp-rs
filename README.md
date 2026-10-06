# warp-rs

Raster warping and resampling in Rust. Arrays in, arrays out.

**Status: early.** The affine transform exists. Nothing warps yet.

## What it will do

Take a raster on one grid and produce it on another. No file I/O, no bundled CRS
database, no network.

## Development

```
just test    # run the tests
just lint    # format, check, clippy
just --list  # everything else
```

## How this is written

The code is human-written, in conversation with Claude. Claude is the discussion
partner for design, trade-offs and how existing tools handle the corner cases. A person
writes the implementation.

Two exceptions, stated plainly:

- **Unit tests** are written by Claude.
- **The affine transform** (`src/affine.rs`) uses rasterio's
  [`affine`](https://github.com/rasterio/affine/blob/main/src/affine/__init__.py)
  library as its reference for the maths.

## Contributing

Contributions are welcome. Open an issue to discuss anything larger than a
small fix, so the design conversation happens before the code does.

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)
  at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the
Apache-2.0 license, shall be dual-licensed as above, without any
additional terms or conditions.

# rlbot-bm

This crate containts the easy to use rust wrappers for [RLBot-BM](https://github.com/L0laapk3/RLBot-BM).
As is custom, there is a `*-sys` crate that links the external lib [rlbot-bm-sys](https://github.com/LHolten/rlbot-bm-sys).

The wrappers make use of a lot of new-types to hide implementation details. This means that when RLBot-BM is updated it is possible to hide these changes and keep the same API.

## Rust features

- Wrappers are "zero copy", they are basically just methods to access the bindgen structs.
- New-types for different kinds of indices.
- `Quat` and `Vec3` types are compatible with `quaternion` and `vecmath` crates.
- Combined return types like `jumped_at() -> Option<Instant>` (it combines `jumped` with a timestamp). 

## Example

You can see an example ATBA implementation in [atba.rs](rlbot-bm/examples/atba.rs).
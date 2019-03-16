## copyless
[![Build Status](https://travis-ci.org/kvark/copyless.svg)](https://travis-ci.org/kvark/copyless)
[![Crates.io](https://img.shields.io/crates/v/copyless.svg)](https://crates.io/crates/copyless)

Rust abstractions can be zero cost in theory, but offten reveal quite a few unnecessary `memcpy` calls in practice. This library provides a number of trait extensions for standard containers that expose API that is more friendly to LLVM optimization passes and doesn't end up with as many copies.

It aims to accelerate [WebRender](https://github.com/servo/webrender) and [gfx-rs](https://github.com/gfx-rs/gfx).

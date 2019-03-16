## copyless
[![Build Status](https://travis-ci.org/kvark/copyless.svg)](https://travis-ci.org/kvark/copyless)
[![Crates.io](https://img.shields.io/crates/v/copyless.svg)](https://crates.io/crates/copyless)

Rust abstractions can be zero cost in theory, but offten reveal quite a few unnecessary `memcpy` calls in practice. This library provides a number of trait extensions for standard containers that expose API that is more friendly to LLVM optimization passes and doesn't end up with as many copies.

It aims to accelerate [WebRender](https://github.com/servo/webrender) and [gfx-rs](https://github.com/gfx-rs/gfx).

## Background

The `memcpy` instructions showed in profiles of WebRender running in Gecko. @jrmuizel built a tool called [memcpy-find](https://github.com/jrmuizel/memcpy-find) that analyzes LLVM IR and spews out the call stacks that end up producing `memcpy` instructions. We figured out a way to convince the compiler to eliminate the copies. This library attemts to make these ways available to Rust ecosystem, at least until the compiler gets smart enough ;)

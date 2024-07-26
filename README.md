<h1 align="center">
    <img src="./.github/cabrinha.png" alt="Cabrinha" width="200px" align="center">
</h1>

<h4 align="center">
    A suite of `std` components for the OP Stack in rust.
</h4>

<p align="center">
  <a href="https://github.com/refcell/cabrinha/actions/workflows/ci.yml"><img src="https://github.com/refcell/cabrinha/actions/workflows/ci.yml/badge.svg?label=ci" alt="CI"></a>
  <a href="https://github.com/refcell/cabrinha/actions/workflows/book.yml"><img src="https://github.com/refcell/cabrinha/actions/workflows/book.yml/badge.svg?label=Book" alt="Book"></a>
  <img src="https://img.shields.io/badge/License-MIT-green.svg?label=license&labelColor=2a2f35" alt="License">
  <a href="https://refcell.github.io/cabrinha"><img src="https://img.shields.io/badge/Contributor%20Book-854a15?logo=mdBook&labelColor=2a2f35" alt="Book"></a>
</p>

<p align="center">
  <a href="#whats-cabrinha">What's Cabrinha?</a> •
  <a href="#status">Status</a> •
  <a href="#getting-started">Getting Started</a> •
  <a href="https://refcell.github.io/cabrinha/CONTRIBUTING.html">Contributing</a> •
  <a href="#credits">Credits</a>
</p>

## What's Cabrinha?

_`cabrinha`: little goat_

Cabrinha is a suite of `std` components for the [OP Stack][op-stack].

Where [kona][kona] contains fault-proof specific `no_std` components, non-fault proof
components of the [OP Stack][op-stack] do not need to be `no_std`. Cabrinha is where
these `std` components live.

## Status

`cabrinha` is currently in active development, and is not yet ready for use in production.

## Getting Started

OP Stack components are individually defined in [crates](./crates/).

To learn more, see the [book][book].

## Book

The [book][book] contains a more in-depth overview of the project, contributor guidelines, tutorials for working with Cabrinha crates and binaries.

## Credits

`cabrinha` is inspired by the work of several teams, namely [OP Labs][op-labs] and other contributors' work on [`kona`][kona].

[j]: https://github.com/casey/just
[goat]: ./bin/goat/
[kona]: https://github.com/ethereum-optimism/kona
[op-stack]: https://github.com/ethereum-optimism/optimism
[op-batcher]: https://github.com/ethereum-optimism/optimism/tree/develop/op-batcher
[book]: https://refcell.github.io/cabrinha
[op-labs]: https://github.com/ethereum-optimism

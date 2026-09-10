# purescript-assert

[![Latest release](http://img.shields.io/github/release/purescript/purescript-assert.svg)](https://github.com/purescript/purescript-assert/releases)
[![Build status](https://github.com/purescript/purescript-assert/workflows/CI/badge.svg?branch=master)](https://github.com/purescript/purescript-assert/actions?query=workflow%3ACI+branch%3Amaster)
[![Pursuit](https://pursuit.purescript.org/packages/purescript-assert/badge)](https://pursuit.purescript.org/packages/purescript-assert)

Basic assertions library for low level testing. This is primarily for testing the core libraries that cannot use [`purescript-quickcheck`](https://github.com/purescript/purescript-quickcheck) without resulting in circular dependencies.

## Installation

```
spago install assert
```

## Documentation

Module documentation is [published on Pursuit](http://pursuit.purescript.org/packages/purescript-assert).

## Rust tests

With the sibling `purust`, `purust-prelude`, `purust-effect` and `purust-console` checkouts available, run:

```sh
bin/test -c
```

The runner rebuilds Purust with `-c`, generates fresh TAST and Rust through Spago, compiles the Rust executable and checks 11 scenarios directly in Bash. Each test process has a ten-second timeout. Ten scenarios must terminate with a nonzero exit status and the expected assertion message. The successful scenario covers the public assertion helpers, deferred execution and replay, including one callback invocation per execution of `checkThrows`.

The runner selects the sibling TAST-enabled PureScript build when available. Set `PURS=/absolute/path/to/purs` to choose another build. `bin/test` runs the same checks without rebuilding Purust; cleanup is limited to this package.

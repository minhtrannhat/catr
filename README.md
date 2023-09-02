# Rust rewrite of GNU cat

``` text
catr 0.1.0
    Minh Tran <minh@minhtrannhat.com>
Rust rewrite of cat

Usage: catr [OPTIONS] [FILE]...
Arguments:
  [FILE]...  Input file(s) [default: -] [default: -]

Options:
  -n             Number lines
  -b             Number nonblank lines
  -h, --help     Print help
  -V, --version  Print version
```

## Build
`cargo build`

## Test
`./mk-outs.sh && cargo test`

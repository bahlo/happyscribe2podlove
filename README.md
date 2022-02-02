# happyscribe2podlove

Super smol cli tool that converts JSON from Happy Scribe to a sane VTT format that works with the [Podlove Publisher](https://podlove.org).

## Get started

1. Export your transcription on Happy Scribe as JSON (make sure to use full names)
2. Pipe the JSON into this CLI and it will print valid VTT

## Example
```sh
git clone https://github.com/bahlo/happyscribe2podlove
cd happyscribe2podlove
cargo run < happyscribe.json > podlove.vtt
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

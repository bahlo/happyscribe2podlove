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

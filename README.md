# Shared libs, binaries and templates for Advent of Code!

## Lib

`runner::solve` is used to build a timing, reporting `main` function for a 
day's two parts.


## Bin

`aocfin` allows for quick download and write of the day's input to disk.

## Config

`cargo-generate` contains a config file 
(setup with `ln -s <REPO_PATH>/cargo-generate/cargo-generate.toml` from
`~/.cargo`) and then use `cargo generate aocday --name dayXX` to scaffold.
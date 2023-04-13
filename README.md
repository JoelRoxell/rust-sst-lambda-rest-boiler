# Rust Lambda SST Boiler Project 

## The Project Structure

1. Lambdas are placed in `~/packages/functions`
2. Shared & internal implementations should live in  `~/packages/core`

The project will produce a binary per lambda (defined in `~/packages/functions/Cargo.toml`)
and a lib file using `~/packages/core/Cargo.toml`

## Build

> Make sure you have cargo & yarn installed

### Remove build Artifacts
`cargo clean` 

### Build via cargo
`cargo build`

### Build via SST
> If you have troubles here try do a `cargo clean` before build

`yarn sst build`

## Deploy

> Make sure to have proper AWS envs set.

`yarn sst deploy`


# username-zome

<!-- [![hc-institute-japan](https://circleci.com/gh/hc-institute-japan/p2pmessage-zome.svg?style=svg)](https://circleci.com/gh/hc-institute-japan/p2pmessage-zome) -->

Small zome for allowing synchronous (soon asynchronous) messaging between two agents, in Holochain RSM.


This module is designed to be included in other DNAs, assuming as little as possible from those. It is packaged as a holochain zome and no built-in UI is provided for it.
  
### Including the zome in your DNA

1. Create a new folder in the `zomes` of the consuming DNA, with the name you want to give to this zome in your DNA.
2. Add a new `Cargo.toml` in that folder. In its content, paste the `Cargo.toml` content from any zome.
3. Change the `name` properties of the `Cargo.toml` file to the name you want to give to this zome in your DNA.
4. Add this zome as a dependency in the `Cargo.toml` file:
```toml
[dependencies]
username = {git = "https://github.com/holochain-open-dev/username-zome", package = "username"}
```
5. Create a `src` folder besides the `Cargo.toml` with this content:
```rust
extern crate username;
```
6. Add the zome into your `*.dna.workdir/dna.json` file.
7. Compile the DNA with the usual `CARGO_TARGET=target cargo build --release --target wasm32-unknown-unknown`.

## Developer setup

This respository is structured in the following way:

- `zome/`: example DNA with the `username` code.
- Top level `Cargo.toml` is a virtual package necessary for other DNAs to include this zome by pointing to this git repository.

Read the [Zome developer setup](/zome/README.md).

## Contributions
We would like to thank [@guillemcordoba](https://github.com/guillemcordoba) and [holochain-open-dev](https://github.com/holochain-open-dev) for providing a reusable module template to easily create zomes that can be reusable in other Holochain pojects. If you are interested in using the same template, check it out [here](https://github.com/holochain-open-dev/reusable-module-template)

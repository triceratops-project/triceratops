# Prerequisites
Triceratops is written in a mix of languages, primarily Rust and TypeScript.

As such you are going to need the following installed:
- [Rust](https://www.rust-lang.org/)
- [Node](https://nodejs.org/) (LTS recommended)


We also use a build system called [Just](https://just.systems/), kinda like make or cmake just a tonne simpler.

---

# Building
Navigate to the root of the project, all commands should be run from the root of the project. 

## AIO

You can run `just build` to build the backend and the panel at the same time, in release mode; ready for distribution.

## Building the panel

Run `just panel` to build the panel, this will output the built panel to `.apps/panel/build/`. (you might need to run it twice to make Svelte & Vite happy)

## Building the backend

Run `just webserver` to build the Rust backend, this might take a while to build depending on your hardware. The binary will be output to `./target/debug/triceratops-server`.


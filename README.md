# sing

CLI app to generate music and sing your text/file/code written in Rust.

## Getting Started


### Prerequisites

Make sure your have `cargo` available in your machine, see [Rust official website](https://www.rust-lang.org/tools/install) for the installation guide if you don't have `cargo` yet.


### Installing

```
cargo install sing
```

### Usage

Generate music and sing with text:

```
sing -t "Hello World"
```

Generate music and sing based on a file:

```
sing -f <path-to-file>
```

Getting help:

```
sing -h
```

## Built With

* [clap](https://crates.io/crates/clap) - Command Line Argument Parser for Rust
* [rodio](https://crates.io/crates/rodio) - Audio playback library
* [spinner](https://crates.io/crates/spinner) - A simple library to add more interactivity to your terminal applications.

## TODO

I'm still pretty new with Rust, here are a list of things I'm trying to work on:

- [ ] User can save the generated sound as wav or mp3
- [ ] More available sounds rather than simple sine wave
- [ ] Better logic to generate music, also probably use some pre defined scales.
- [ ] Multi track would be fun


## Contributing

Free feel to open PR for any change/feature you would like to have, the project is still in a very early stage.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/haochuan/sing/tags). 


## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details


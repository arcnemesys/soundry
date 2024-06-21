# Soundry

Soundry is an experimental parser for the SFZ file format, written in Rust that intends to be capable of handling both SFZ versions. As well, it intends to implement the full set of opcodes for various headers, and ideally, parse SFZ files programmatically depending on the tools that would use the parser. For example, Meadlowlark is a Rust based DAW, for which someone could use Soundry to develop a synthesizer plugin that provides instruments, synths, and the like.

## Features

- Efficient Parsing: Utilizes the nom crate for fast and accurate parsing of SFZ files.
- Value Refinement: Uses the refinement crate to guarantee that all opcode values are within the ranges defined in the specification(s).
- Flexibility: Allows for handling both SFZ versions and whatever set of headers/opcodes are supported by a given player.

## Getting Started

### Installing

Bringing Soundry into your project is as simple as adding the following to your `Cargo.toml` file:

```toml
[dependencies]
sfz_parser = "0.1.0"
```

Alternatively, if you'd like to build it from source, run the following in the directory of your choosing:

```bash
git clone https://github.com/arcnemesys/soundry.git
cd soundry && cargo

```


## Usage

A simple placeholder example of using Soundry is given below, and will be replaced by a more refined example in the near future.

```rust
use soundry::control::parse_control_header;

fn main() {

  let control_header = r#"<control>
    default_path=Samples\
    #define $EXT wav
    #include \"data\control.sfz\"
    #include \"data\multiout.sfz\"
    #include \"data\global.sfz\"
    #include \"data\kick.sfz\"
    #include \"data\snare.sfz\"
    #include \"data\tom1.sfz\"
    #include \"data\tom2.sfz\"
    #include \"data\hihat.sfz\"
    #include \"data\ride.sfz\"
    #include \"data\crash.sfz\"#;

    match parse_control_header(control_header) {
        Ok((_, control)) => {
            let expected_control_header = Control { 
              header_type: Control, 
              define_directives: {"EXT": "wav"},
              include_directives: [
                "data\\control.sfz",
                "data\\multiout.sfz",
                "data\\global.sfz",
                "data\\kick.sfz",
                "data\\snare.sfz",
                "data\\tom1.sfz",
                "data\\tom2.sfz",
                "data\\hihat.sfz",
                "data\\ride.sfz",
                "data\\crash.sfz"
              ],
              default_path: "Samples\\", 
              note_offset: 0, 
              octave_offset: 0, 
              label_ccn: [], 
              set_ccn: [] 
              };
            assert_eq!(control, expected_control_header);
            println!("Successfully parsed control header: {:?}", control);
        },
        Err(e) => eprintln!("Failed to parse SFZ control header: {}", e),
    }
}


```

## Dependencies

  - [Nom](https://github.com/rust-bakery/nom).
  - [Refinement](https://docs.rs/refinement/latest/refinement/).


## Acknowledgments

  - Nom author(s)
  - Refinement authors(s)
  - Meadowlark author(s)
  - Hound, Rodio, Dasp, Fundsp


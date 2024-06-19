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
use soundry::parse_sfz;

fn main() {

    let region_header = "<region>\nkey=62\nsample=snare.wav";

    match parse_region_header(region_header) {
        Ok((_, region)) => {
            let expected_region = Region {
                key: 62,
                sample: String::from("snare.wav"),
            };
            assert_eq!(region, expected_region);
            println!("Successfully parsed region: {:?}", region);
        },
        Err(e) => eprintln!("Failed to parse SFZ region header: {}", e),
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


// Thanks to Anders Danhielson
// Reference Material
// http://drealm.info/sfz/plj-sfz.xhtml
// https://sfzformat.com/headers/
// Opcode List: https://www.linuxsampler.org/sfz/
// https://sfzformat.com/legacy/
// SFZ Tutorial/Intro: https://sfzformat.com/tutorials/basics/
// MIDI CC Message List: https://atherproducer.com/online-tools-for-musicians/midi-cc-list/
// https://www.sustainable-music.org/demystifying-sfz-a-guide-to-understanding-the-sfz-format-for-sound-libraries/
// https://github.com/sfz/tests/tree/master
// https://sfzlab.github.io/sfz-website/
// https://edrums.github.io/en/linuxsampler/sfz/#Effects
// https://raw.githubusercontent.com/sfzinstruments/mappings/master/Pettinhouse/Yamaha%209000/Yamaha%209000.sfz
// https://github.com/sfzinstruments/mappings/blob/master/PastToFuture%20Disco%20Drums/Disco%20Drums%20-%20Multiout.sfz

// TODO: Dedupe EG/LFO variants to be defined once and reused.
// TODO [in progress]: Explore refinement types.
// TODO [in progress]: Implement parsing with nom.
// TODO: Add Cakewalk specific codes.
// TODO: Add v2 opcodes for
// Sample Playback
// Voice LifeCycle
// Midi Conditions
// Internal Conditions
// Triggers
// Amplifier
// EQ
// Filter
// Pitch
// LFO
// Curves
// Effects
// Loading
// Wavetable Oscillator


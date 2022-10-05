use std::env;

mod chord {
    #[derive(Copy, Clone, Debug)]
    pub enum Chord {
        C,
        Db,
        D,
        Eb,
        E,
        F,
        Gb,
        G,
        Ab,
        A,
        Bb,
        B,
    }

    impl Chord {
        pub fn from_str(chord: &str) -> Self {
            match chord {
                "C" => Self::C,
                "C#" | "Db" => Self::Db,
                "D" => Self::D,
                "D#" | "Eb" => Self::Eb,
                "E" => Self::E,
                "F" => Self::F,
                "F#" | "Gb" => Self::Gb,
                "G" => Self::G,
                "G#" | "Ab" => Self::Ab,
                "A" => Self::A,
                "A#" | "Bb" => Self::Bb,
                "B" => Self::B,
                s => panic!("Unknown chord: {s}"),
            }
        }

        pub fn to_str(self) -> &'static str {
            match self {
                Self::C => "C",
                Self::Db => "Db",
                Self::D => "D",
                Self::Eb => "Eb",
                Self::E => "E",
                Self::F => "F",
                Self::Gb => "Gb",
                Self::G => "G",
                Self::Ab => "Ab",
                Self::A => "A",
                Self::Bb => "Bb",
                Self::B => "B",
            }
        }

        fn to_u8(self) -> u8 {
            match self {
                Self::C => 0,
                Self::Db => 1,
                Self::D => 2,
                Self::Eb => 3,
                Self::E => 4,
                Self::F => 5,
                Self::Gb => 6,
                Self::G => 7,
                Self::Ab => 8,
                Self::A => 9,
                Self::Bb => 10,
                Self::B => 11,
            }
        }

        fn from_u8(value: u8) -> Self {
            match value {
                0 => Self::C,
                1 => Self::Db,
                2 => Self::D,
                3 => Self::Eb,
                4 => Self::E,
                5 => Self::F,
                6 => Self::Gb,
                7 => Self::G,
                8 => Self::Ab,
                9 => Self::A,
                10 => Self::Bb,
                11 => Self::B,
                v => panic!("Unknown chord number: {v}"),
            }
        }

        pub fn advance_frets(self, frets: u8) -> Self {
            Self::from_u8((self.to_u8() + frets) % 12)
        }

        pub fn is_barre_chord(&self) -> bool {
            match self {
                Self::C | Self::D | Self::Eb | Self::E | Self::G | Self::A => false,
                _ => true,
            }
        }
    }

    impl std::fmt::Display for Chord {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.to_str())
        }
    }
}

use chord::Chord;

fn progression_has_barre_chord(p: &Vec<Chord>) -> bool {
    p.iter().any(|c| c.is_barre_chord())
}

fn progression_has_no_barre_chord(p: &Vec<Chord>) -> bool {
    !progression_has_barre_chord(p)
}

fn main() {
    let mut args: Vec<_> = env::args().skip(1).map(|a| Chord::from_str(&a)).collect();

    if progression_has_no_barre_chord(&args) {
        println!("Given chord progression already has no barre chords!");
        return;
    }

    for i in 1..12 {
        args = args.iter().map(|c| c.advance_frets(1)).collect();
        if progression_has_no_barre_chord(&args) {
            println!("Chord progression found! It needs a capo on fret {i}");
            let joined = args
                .iter()
                .map(|c| c.to_str())
                .collect::<Vec<_>>()
                .join(" ");
            println!("{}", joined);
            return;
        }
    }

    println!("Couldn't find a progression without barre chords")
}

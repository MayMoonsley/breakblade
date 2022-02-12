use clap::Parser;
use std::fs::File;
use std::io;
use std::error::Error;
use std::path::Path;
use wav::{BitDepth, Header};

enum SplitMode {
    // split into segments equal to one bar of this tempo
    Tempo(usize),
    // split into N equally sized segments
    Beats(usize)
}

enum SplitErr {
    Both,
    Neither
}

impl SplitErr {
    pub fn err_string(&self) -> &'static str {
        match self {
            SplitErr::Both => "Cannot specify both tempo and beats.",
            SplitErr::Neither => "Must specify either tempo or beats."
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Command {
    /// Path of file for the loop to slice up
    #[clap(short, long)]
    input: String,
    /// Tempo of loop, in BPM
    #[clap(short, long)]
    tempo: Option<usize>,
    /// Number of segments of equal size to split the loop into
    #[clap(short, long)]
    beats: Option<usize>
}

impl Command {
    pub fn split_mode(&self) -> Result<SplitMode, SplitErr> {
        if let (Some(_), Some(_)) = (self.beats, self.tempo) {
            Err(SplitErr::Both)
        } else if let Some(beats) = self.beats {
            Ok(SplitMode::Beats(beats))
        } else if let Some(tempo) = self.tempo {
            Ok(SplitMode::Tempo(tempo))
        } else {
            Err(SplitErr::Neither)
        }
    }
}

// todo: this should return a Result
fn read_input(input_path: &Path) -> Option<(Header, BitDepth)> {
    let mut file = File::open(input_path).ok()?;
    let parsed = wav::read(&mut file).ok()?;
    Some(parsed)
}

fn split<T: Copy>(split_mode: SplitMode, header: Header, arr: &[T]) -> Vec<BitDepth> where Vec<T>: Into<BitDepth> {
    match split_mode {
        SplitMode::Tempo(bpm) => {
            let segment_len = header.sampling_rate as usize * bpm / 60;
            arr.chunks(segment_len)
            .map(|a| a.to_owned().into())
            .collect()
        }
        SplitMode::Beats(beats) => {
            let beat_len = arr.len() / beats;
            (0..beats)
                .map(|index| {
                    let sample_index = index * beat_len;
                    if index == beats - 1 {
                        arr[sample_index..].to_owned().into()
                    } else {
                        arr[sample_index..(sample_index + beat_len)].to_owned().into()
                    }
                })
                .collect()

        }
    }
}

fn split_input(split_mode: SplitMode, header: Header, bit_depth: BitDepth) -> (Header, Vec<BitDepth>) {
    match bit_depth {
        BitDepth::Eight(arr) => (header, split(split_mode, header, &arr)),
        BitDepth::Sixteen(arr) => (header, split(split_mode, header, &arr)),
        BitDepth::TwentyFour(arr) => (header, split(split_mode, header, &arr)),
        BitDepth::ThirtyTwoFloat(arr) => (header, split(split_mode, header, &arr)),
        BitDepth::Empty => (header, vec![BitDepth::Empty])
    }
}

fn write_buffers(path: &Path, header: Header, buffers: Vec<BitDepth>) -> io::Result<()> {
    let slug = path.file_stem()
        .and_then(|p| p.to_str())
        .ok_or(io::Error::from(io::ErrorKind::InvalidInput))?;
    for (i, buffer) in buffers.iter().enumerate() {
        let mut out_file = File::create(path.with_file_name(format!("{}_{}.wav", slug, i)))?;
        wav::write(header, buffer, &mut out_file)?;
    }
    Ok(())
}

fn main() {
    let command = Command::parse();
    let input_path = Path::new(&command.input);
    match command.split_mode() {
        Ok(split_mode) => {
            match read_input(input_path) {
                Some((header, bit_depth)) => {
                    let (header, buffers) = split_input(split_mode, header, bit_depth);
                    match write_buffers(input_path, header, buffers) {
                        Ok(()) => println!("Written successfully."),
                        Err(e) => eprintln!("ERR: {}", e.description())
                    }
                }
                None => eprintln!("Error reading or parsing file.")
            }
        },
        Err(err) => {
            eprintln!("ERR: {}", err.err_string());
        }
    }
}

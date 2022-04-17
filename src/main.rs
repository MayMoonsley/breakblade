use clap::{Args, Parser, Subcommand};
use std::fs::File;
use std::io;
use std::path::Path;
use wav::{BitDepth, Header};

mod threshold;
use threshold::Threshold;
mod slice_util;
use slice_util::{SkipWhile, SkipFromRightWhile};

#[derive(Subcommand, Copy, Clone)]
enum SplitMode {
    // split into segments equal to one [Y]th note at X BPM
    Tempo(Tempo),
    // split into N equally sized segments
    Beats(Beats)
}

/// Split the loop based on its tempo
#[derive(Args, Copy, Clone)]
struct Tempo {
    /// Tempo of loop, in BPM
    #[clap(short, long)]
    tempo: usize,
    /// Note value (4 = quarter note, 8 = eighth note, etc)
    #[clap(short, long, default_value_t = 4)]
    note_value: usize,
    /// Remove silence from the beginning of the input
    #[clap(long)]
    trim_leading_silence: bool,
    /// Remove silence from the end of the input (this can remove transients / decays, so be careful)
    #[clap(long)]
    trim_trailing_silence: bool,
    /// Threshold for what is considered to be silence, in dBFS
    #[clap(long, default_value_t = -40.0)]
    silence_threshold: f64
}

/// Split the loop into beats of equal size
#[derive(Args, Copy, Clone)]
struct Beats {
    /// Number of segments of equal size to split the loop into
    #[clap(short, long)]
    beats: usize
}

#[derive(Parser)]
#[clap(author = "May Lawver", version, about = "A tool for musically useful sample splitting.", long_about = None)]
struct Command {
    /// Path of file for the loop to slice up
    #[clap(short, long)]
    input: String,
    /// Path to output files to; given file.wav, it will output file_00.wav, file_01.wav, etc (defaults to input path)
    #[clap(short, long)]
    output: Option<String>,
    #[clap(subcommand)]
    mode: SplitMode,
}

impl Command {
    pub fn output_path(&self) -> &Path {
        self.output.as_ref()
            .map(|s| Path::new(s))
            .unwrap_or_else(|| Path::new(&self.input))
    }
}

// todo: this should return a Result
fn read_input(input_path: &Path) -> Option<(Header, BitDepth)> {
    let mut file = File::open(input_path).ok()?;
    let parsed = wav::read(&mut file).ok()?;
    Some(parsed)
}

fn split<T: Copy + Threshold>(split_mode: SplitMode, header: Header, arr: &[T]) -> Vec<BitDepth> where Vec<T>: Into<BitDepth> {
    match split_mode {
        SplitMode::Tempo(Tempo { tempo, note_value, trim_leading_silence, trim_trailing_silence, silence_threshold }) => {
            println!("{} {}", trim_leading_silence, trim_trailing_silence);
            let segment_len = (header.sampling_rate as usize * tempo * 4) / (60 * note_value);
            arr.skip_while(|&x| trim_leading_silence && x.to_dbfs() <= silence_threshold) // remove leading silence
                .skip_from_right_while(|&x| trim_trailing_silence && x.to_dbfs() <= silence_threshold) // remove trailing silence
                .chunks(segment_len) // split based on tempo
                .map(|a| a.to_owned().into()) // convert each buffer to a vec
                .collect() // convert to a vec
        }
        SplitMode::Beats(Beats { beats }) => {
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
        .ok_or_else(|| io::Error::from(io::ErrorKind::InvalidInput))?;
    for (i, buffer) in buffers.iter().enumerate() {
        let mut out_file = File::create(path.with_file_name(format!("{}_{:0>2}.wav", slug, i)))?;
        wav::write(header, buffer, &mut out_file)?;
    }
    Ok(())
}

fn main() {
    let command = Command::parse();
    let input_path = Path::new(&command.input);
    match read_input(input_path) {
        Some((header, bit_depth)) => {
            let (header, buffers) = split_input(command.mode, header, bit_depth);
            match write_buffers(command.output_path(), header, buffers) {
                Ok(()) => println!("Written successfully."),
                Err(e) => eprintln!("ERR: {}", e)
            }
        }
        None => eprintln!("Error reading or parsing file.")
    }
}

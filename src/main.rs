use clap::Parser;

enum SplitMode {
    Tempo(usize),
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

fn main() {
    let command = Command::parse();
    match command.split_mode() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("ERR: {}", err.err_string());
        }
    }
}

use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::ops::AddAssign;

#[derive(Debug)]
struct Counts {
    lines: usize,
    words: usize,
    chars: usize,
}

impl Counts {
    fn new() -> Self {
        Self {
            lines: 0,
            words: 0,
            chars: 0,
        }
    }

    // characters per word
    fn cpw(&self) -> f32 {
        self.chars as f32 / self.words as f32
    }
}

impl AddAssign for Counts {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            lines: self.lines + other.lines,
            words: self.words + other.words,
            chars: self.chars + other.chars,
        };
    }
}

fn get_counts(filename: &str) -> io::Result<Counts> {
    let mut counts = Counts::new();

    let f = File::open(filename)?;
    let f = BufReader::new(f);

    for line in f.lines() {
        let line = line.unwrap();
        counts.lines += 1;
        counts.chars += line.len();
        counts.words += line.split_ascii_whitespace().count();
    }

    Ok(counts)
}

const MIN_WORDS: usize = 200;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut totals = Counts::new();

    let mut processed: usize = 0;
    let mut skipped: usize = 0;

    for filename in args[1..].iter() {
        //println!("Processing {}", filename);
        let counts = get_counts(&filename)?;
        if counts.words < MIN_WORDS {
            println!(
                "- Skipping {}; not enough words: {} < {}",
                filename, counts.words, MIN_WORDS
            );
            skipped += 1;
            continue;
        }

        totals += counts;
        processed += 1;
    }

    println!("Processed {} files and skipped {}.", processed, skipped);
    println!("{:?}", totals);

    if processed > 0 {
        println!(
            "Averages {{ lines: {}, words: {}, chars: {} }}",
            (totals.lines as f32 / processed as f32).round(),
            (totals.words as f32 / processed as f32).round(),
            (totals.chars as f32 / processed as f32).round(),
        );
        println!("Characters per word: {:.2}", totals.cpw());
        println!("Max words per tweet: {}", (280_f32 / totals.cpw()).floor());
    }

    Ok(())
}

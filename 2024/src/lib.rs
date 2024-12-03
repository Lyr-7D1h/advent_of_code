use std::{
    collections::HashMap,
    fmt::Display,
    fs::read_to_string,
    time::{Duration, Instant},
};

use clap::Parser;

use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author = "Lyr", version = "0.1", about = "Advent of Code")]
pub struct Args {
    #[arg()]
    pub part: String,

    #[arg()]
    pub input: PathBuf,

    #[arg(short, long)]
    pub performance: bool,

    #[arg(long = "pa", default_value = "100")]
    pub performance_accuracy: u32,
}

pub struct Aoc {
    args: Args,
    parts: HashMap<String, Box<dyn Fn(String) -> (Box<dyn Display>, Duration)>>,
}

impl Aoc {
    pub fn new() -> Aoc {
        let args = Args::parse();

        Aoc {
            args,
            parts: HashMap::new(),
        }
    }

    pub fn part<T: Display + 'static>(&mut self, key: &str, part: fn(String) -> T) {
        self.parts.insert(
            key.to_string(),
            Box::new(move |input| {
                let start = Instant::now();
                let result = part(input);
                let elapsed = start.elapsed();
                (Box::new(result), elapsed)
            }),
        );
    }

    pub fn run(&self) {
        let part = self
            .parts
            .get(&self.args.part)
            .expect(&format!("Part {} not found", self.args.part));

        let input = read_to_string(&self.args.input).expect("input file not found");

        // if performance enabled run multiple times and measure average
        if self.args.performance {
            let mut total = Duration::ZERO;
            let mut result = None;

            for _ in 0..self.args.performance_accuracy {
                let (r, d) = part(input.clone());
                if let None = result {
                    result = Some(r);
                }
                total += d;
            }

            println!(
                "\n\n{} finished\nResult: {}\nAverage Duration: {:?}",
                self.args.part,
                result.unwrap(),
                total / self.args.performance_accuracy
            );

            return;
        }

        let (result, duration) = part(input.clone());

        println!(
            "{} finished\nResult: {}\nDuration: {:?}",
            self.args.part, result, duration
        )
    }
}

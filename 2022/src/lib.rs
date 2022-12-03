mod args;
use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::BufReader,
    time::{Duration, Instant},
};

use args::Args;
use clap::Parser;

pub type Input = BufReader<File>;

const PERFORMANCE_ACCURACY: u32 = 500;

pub struct Aoc {
    args: Args,
    parts: HashMap<String, Box<dyn Fn(Input) -> (Box<dyn Display>, Duration)>>,
}

impl Aoc {
    pub fn new() -> Aoc {
        let args = Args::parse();

        Aoc {
            args,
            parts: HashMap::new(),
        }
    }

    pub fn part<T: Display + 'static + Eq>(&mut self, key: &str, part: fn(Input) -> T) {
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

        // if performance enabled run multiple times and measure average
        // TODO make sure rust compiler doesn't do any optimizations
        if self.args.performance {
            let mut total = Duration::ZERO;
            let mut result = None;

            for _ in 0..PERFORMANCE_ACCURACY {
                let (r, d) = part(BufReader::new(
                    File::open(&self.args.input).expect("input file not found"),
                ));
                if let None = result {
                    result = Some(r);
                }
                total += d;

                // TODO ensure that results equal
                // if r != result {
                //     panic!("part returned different result!")
                // }
            }

            println!(
                "{} finished\nResult: {}\nAverage Duration: {:?}",
                self.args.part,
                result.unwrap(),
                total / PERFORMANCE_ACCURACY
            );

            return;
        }

        let file = File::open(&self.args.input).expect("input file not found");
        let input = BufReader::new(file);

        let (result, duration) = part(input);

        println!(
            "{} finished\nResult: {}\nDuration: {:?}",
            self.args.part, result, duration
        )
    }
}

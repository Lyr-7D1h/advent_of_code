use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::BufReader,
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

pub type Input = BufReader<File>;

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

            for _ in 0..self.args.performance_accuracy {
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
                "\n\n{} finished\nResult: {}\nAverage Duration: {:?}",
                self.args.part,
                result.unwrap(),
                total / self.args.performance_accuracy
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

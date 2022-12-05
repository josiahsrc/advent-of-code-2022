use clap::Parser;
use std::{fs::File, io::Read};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  file: String,
}

// Create a struct with an Index and a count. This struct is comparable by count.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RollingCount {
  index: usize,
  count: i32,
}

fn main() {
  let args = Args::parse();
  let mut file = File::open(args.file).expect("file not found");

  let mut contents = String::new();
  file
    .read_to_string(&mut contents)
    .expect("something went wrong reading the file");
  println!("With text:\n{}", contents);

  let lines = contents.lines();

  let mut index = 0;
  let mut rolling = 0;
  let mut all_rolling: Vec<RollingCount> = Vec::new();

  for line in lines {
    if line.is_empty() {
      all_rolling.push(RollingCount {
        index: index,
        count: rolling,
      });
      index += 1;
      rolling = 0;
    } else {
      let value = line.parse::<i32>().unwrap();
      rolling += value;
    }
  }

  all_rolling.push(RollingCount {
    index: index,
    count: rolling,
  });

  println!("total elf count: {}", all_rolling.len());

  // sort the all_rolling vector by count
  all_rolling.sort_by(|a, b| b.count.cmp(&a.count));

  println!("Elf 0: {}", all_rolling[0].count);
  println!("Elf 1: {}", all_rolling[1].count);
  println!("Elf 2: {}", all_rolling[2].count);

  let sum_of_top_3 = all_rolling[0].count + all_rolling[1].count + all_rolling[2].count;
  println!("Sum of top 3: {}", sum_of_top_3);
}

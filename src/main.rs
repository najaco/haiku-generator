use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use haiku_generator::HaikuGenerator;

static UNIX_DICTIONARY: &'static str = "/usr/share/dict/words";

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(lines) = read_lines(UNIX_DICTIONARY) {
        let haiku_generator =
            HaikuGenerator::new(lines.map(|s| s.unwrap().to_lowercase()).collect());
        let haiku = haiku_generator.generate_haiku();
        println!("{}", haiku.join("\n"))
    }
}

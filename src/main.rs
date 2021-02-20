use std::fs;
use std::env;

use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rng = rand::thread_rng();

    //println!("{:?}", args);

    let wordlist = if args.len() >= 2 {
        args[1].clone()
    } else {
        "/usr/local/etc/words.txt".to_string()
    };

    //println!("Wordlist: {}", wordlist);

    let words = fs::read_to_string(wordlist).unwrap();
    let words: Vec<&str> = words
        .split("\n")
        .collect();
    //println!("{:?}", words);

    let mut prophecy: Vec<String> = Vec::new();

    for i in 0..rng.gen_range(4..7) {
        let mut line: Vec<String> = Vec::new();
        for j in 0..rng.gen_range(6..9) {
            line.push(words[rng.gen_range(0..words.len())].to_string())
        }
        prophecy.push(line.join(" "));
    }

    println!("{}", prophecy.join("\n"));
}

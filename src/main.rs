#![feature(random)]

use std::env;
use std::random::random;

fn print_help(program: &str) {
    println!("Generate random, pronounceable names.");
    println!();
    println!("Usage:");
    println!("  {program} [COUNT|MIN-MAX] [LETTERS|MIN-MAX] [DIGITS|MIN-MAX]");
    println!();
    println!("Arguments:");
    println!("  COUNT    Number of names to generate (default: 1)");
    println!("  LETTERS  Letter count per name (default: 1)");
    println!("  DIGITS   Digit count per name (default: 1)");
    println!();
    println!("Each argument accepts either a single number (e.g. 4)");
    println!("or a range MIN-MAX (e.g. 3-8), chosen randomly each run.");
    println!();
    println!("Options:");
    println!("  -h, --help  Show this help message");
}

fn random_range(min: usize, max: usize) -> usize {
    let (min, max) = (min.min(max), max.max(min));
    let x = random::<usize>(..);
    if max - min == usize::MAX {
        x
    } else {
        min + (x % (max - min + 1))
    }
}

fn parse_arg(arg: Option<&String>) -> usize {
    const DEFAULT: usize = 1;
    if let Some(s) = arg {
        // Support range syntax like "3-8" in addition to a single integer.
        if let Some((a, b)) = s.split_once('-') {
            let min = a.parse::<usize>().unwrap_or(DEFAULT);
            let max = b.parse::<usize>().unwrap_or(min);
            random_range(min, max)
        } else {
            s.parse::<usize>().unwrap_or(DEFAULT)
        }
    } else {
        DEFAULT
    }
}

fn generate_name(letter_count: usize, digit_count: usize) -> String {
    const CONSONANTS: &[u8] = b"bcdfghjklmnpqrstvwxyz";
    const VOWELS: &[u8] = b"aeiou";

    let mut result = String::with_capacity(letter_count + digit_count);

    // Alternate consonant/vowel for a more pronounceable shape.
    for i in 0..letter_count {
        let ch = if i % 2 == 0 {
            CONSONANTS[random::<usize>(..) % CONSONANTS.len()] as char
        } else {
            VOWELS[random::<usize>(..) % VOWELS.len()] as char
        };

        if i == 0 {
            result.push(ch.to_ascii_uppercase());
        } else {
            result.push(ch);
        }
    }

    for _ in 0..digit_count {
        let digit = (b'0' + random::<u8>(..) % 10) as char;
        result.push(digit);
    }

    result
}

fn main() {
    let mut arg_iter = env::args();
    let program = arg_iter.next().unwrap_or_else(|| String::from("nunu-name"));
    let args: Vec<String> = arg_iter.collect();

    if args.iter().any(|arg| arg == "-h" || arg == "--help") {
        print_help(&program);
        return;
    }

    let count = parse_arg(args.first());
    let letters = parse_arg(args.get(1));
    let digits = parse_arg(args.get(2));

    for _ in 0..count {
        println!("{}", generate_name(letters, digits,));
    }
}

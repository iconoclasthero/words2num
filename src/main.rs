macro_rules! hashmap {
    ($( $key: expr => $val: expr ),* $(,)?) => {{
         let mut m = std::collections::HashMap::new();
         $( m.insert($key, $val); )*
         m
    }}
}

use std::env;
use std::io::{self, BufRead};

fn main() {
    let mut additive = false;
    let mut help = false;
    let mut pad = None;
    let mut filtered = vec![];

    for arg in env::args().skip(1) {
        if arg == "--add" || arg == "--math" {
            additive = true;
        } else if arg == "-h" || arg == "--help" {
            help = true;
        } else if let Some(rest) = arg.strip_prefix('-') {
            if let Ok(n) = rest.parse::<usize>() {
                pad = Some(n);
            } else {
                filtered.push(arg);
            }
        } else {
            filtered.push(arg);
        }
    }

    if help {
        print_help();
        return;
    }

    if filtered.is_empty() {
        for line in io::stdin().lock().lines() {
            let line = line.unwrap_or_default();
            show_result(&line, additive, pad);
        }
    } else {
        let input = filtered.join(" ");
        show_result(&input, additive, pad);
    }
}

fn show_result(input: &str, additive: bool, pad: Option<usize>) {
    let out = if additive {
        words_to_number_additive(input)
    } else {
        words_to_number_concat(input)
    };

    match out {
        Some(n) => {
            if let Some(width) = pad {
                println!("{:0width$}", n, width = width);
            } else {
                println!("{}", n);
            }
        }
        None => {
            eprintln!("Could not parse: '{}'", input);
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!(
        "Usage:
  words2num [OPTIONS] \"twenty oh five\"
  echo \"four sixty seven\" | words2num -3
  words2num --add \"four hundred sixty two\"

Options:
  -[N]           Zero-pad to N digits (e.g. -2 for 01)
  --add, --math  Use additive parsing (e.g. 400 + 20 + 2)
  -h, --help     Show this help message"
    );
}

fn words_to_number_concat(s: &str) -> Option<i64> {
    let units = hashmap![
        "zero" => 0, "oh" => 0, "one" => 1, "two" => 2, "three" => 3, "four" => 4,
        "five" => 5, "six" => 6, "seven" => 7, "eight" => 8, "nine" => 9
    ];
    let teens = hashmap![
        "ten" => 10, "eleven" => 11, "twelve" => 12, "thirteen" => 13,
        "fourteen" => 14, "fifteen" => 15, "sixteen" => 16,
        "seventeen" => 17, "eighteen" => 18, "nineteen" => 19
    ];
    let tens = hashmap![
        "twenty" => 20, "thirty" => 30, "forty" => 40, "fifty" => 50,
        "sixty" => 60, "seventy" => 70, "eighty" => 80, "ninety" => 90
    ];
    let mult = hashmap![
        "hundred" => 100, "thousand" => 1000
    ];

    let cleaned = s.to_lowercase().replace('-', " ");
    let words: Vec<_> = cleaned
        .split_whitespace()
        .filter(|w| *w != "and")
        .collect();

    let mut result = 0;
    let mut current = 0;
    let mut i = 0;

    while i < words.len() {
        let w = words[i];

        if let Some(&v) = units.get(w) {
            current = current * 10 + v;
        } else if let Some(&v) = teens.get(w) {
            current = current * 100 + v;
        } else if let Some(&v) = tens.get(w) {
            if i + 1 < words.len() {
                if let Some(&u) = units.get(words[i + 1]) {
                    current = current * 100 + v + u;
                    i += 1;
                } else {
                    current = current * 100 + v;
                }
            } else {
                current = current * 100 + v;
            }
        } else if let Some(&m) = mult.get(w) {
            if current == 0 {
                current = 1;
            }
            current *= m;
            result += current;
            current = 0;
        } else {
            return None;
        }

        i += 1;
    }

    Some(result + current)
}

fn words_to_number_additive(s: &str) -> Option<i64> {
    let units = hashmap![
        "zero" => 0, "oh" => 0, "one" => 1, "two" => 2, "three" => 3, "four" => 4,
        "five" => 5, "six" => 6, "seven" => 7, "eight" => 8, "nine" => 9,
        "ten" => 10, "eleven" => 11, "twelve" => 12, "thirteen" => 13,
        "fourteen" => 14, "fifteen" => 15, "sixteen" => 16,
        "seventeen" => 17, "eighteen" => 18, "nineteen" => 19,
        "twenty" => 20, "thirty" => 30, "forty" => 40, "fifty" => 50,
        "sixty" => 60, "seventy" => 70, "eighty" => 80, "ninety" => 90
    ];
    let scales = hashmap![
        "hundred" => 100,
        "thousand" => 1_000,
        "million" => 1_000_000,
        "billion" => 1_000_000_000
    ];

    let mut total = 0;
    let mut current = 0;

    for word in s
        .to_lowercase()
        .replace('-', " ")
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphabetic()))
    {
        if let Some(&v) = units.get(word) {
            current += v;
        } else if let Some(&v) = scales.get(word) {
            if v == 100 {
                current *= v;
            } else {
                total += current * v;
                current = 0;
            }
        } else if word == "and" {
            continue;
        } else {
            return None;
        }
    }

    Some(total + current)
}

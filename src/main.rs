use std::{
    env,
    fs::{read_to_string, File},
    process::exit,
};

use cedict::parse_reader;

use crate::pinyin::numeral_to_unicode;

mod pinyin;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>(); // skips the first arg (program name)
    const NUM_ARGS: u32 = 1;
    if args.len() != NUM_ARGS.try_into().unwrap() {
        // just handle filename
        eprintln!("Expected {NUM_ARGS} arguments but got {}", args.len());
        exit(1);
    }
    let dictionary_list = match File::open("cedict_ts.u8") {
        Ok(file) => parse_reader(file).collect::<Vec<_>>(),
        Err(e) => {
            eprintln!("Could not read CEDICT file: {:?}", e);
            exit(1);
        }
    };

    let words = match read_to_string(args[0].clone()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not read input file: {:?}", e);
            exit(1);
        }
    };

    // format should be one word on one line
    // println!("~"); // separator for anki? - since this isn't used in any of the definitions
    for word in words.lines() {
        let def = match dictionary_list.iter().find(|e| e.simplified() == word) {
            Some(d) => {
                format!(
                    "{}|\"{}\n{}\"",
                    word,
                    d.pinyin()
                        .split_ascii_whitespace()
                        .map(numeral_to_unicode)
                        .collect::<Vec<String>>()
                        .join(" "),
                    d.definitions().collect::<Vec<_>>().join(" / ")
                )
            }
            None => format!("# Could not find definition for {}", word),
        };
        println!("{def}");
    }

    exit(0);
}

use std::{
    collections::HashMap,
    env,
    fs::{read_to_string, File},
    process::exit,
};

use radix_trie::{self, Trie};

use cedict::{parse_reader, DictEntry};

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
    // build up trie's for simplified search
    let mut simplified_trie: Trie<&str, Vec<u32>> = Trie::new();
    for (i, d_entry) in dictionary_list.iter().enumerate() {
        let i: u32 = i.try_into().unwrap(); // shouldn't fail unless running on lower than 32-bit system
        simplified_trie.map_with_default(d_entry.simplified(), |l| l.push(i), vec![i]);
    }

    let words = match read_to_string(args[0].clone()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not read input file: {:?}", e);
            exit(1);
        }
    };

    let mut not_found = Vec::new();

    // format should be one word on one line
    // println!("~"); // separator for anki? - since this isn't used in any of the definitions
    for word in words.lines() {
        let def = match simplified_trie.get(word) {
            Some(v) => {
                let entries: Vec<&DictEntry<String>> =
                    v.iter().map(|i| &dictionary_list[*i as usize]).collect();
                // TODO
                // what if multiple words w/ same simplified characters?
                // - what if they have the same pinyin?
                // - what if they have different pinyin?
                let defs_block = if entries.len() > 1 {
                    // use a hashmap of unique pinyins to a list of multiple possible definitions
                    // so entries with the same pinyin will have multiple elements in their lists
                    let mut defs: HashMap<&str, Vec<String>> = HashMap::new();
                    for dict_entry in &entries {
                        let e = defs.entry(dict_entry.pinyin()).or_default();
                        e.push(dict_entry.definitions().collect::<Vec<_>>().join(" / "));
                    }
                    // now we have a Map {pinyin -> [def, ...]}, combine into list of ["pinyin\n def\n def...", ...]
                    defs.iter()
                        .map(|(pinyin, def_list)| {
                            format!(
                                "{}\n{}",
                                pinyin
                                    .split_ascii_whitespace()
                                    .map(numeral_to_unicode)
                                    .collect::<Vec<String>>()
                                    .join(""),
                                def_list.join("\n")
                            )
                        })
                        .collect::<Vec<String>>()
                        .join("\n\n")
                } else {
                    // since the other branch involves a lot of extra allocations and processing
                    // that is unnecessary if there is only one entry, do the old case in this branch
                    format!(
                        "{}\n{}",
                        entries[0] // hardcode to just take the first entry for now
                            .pinyin()
                            .split_ascii_whitespace()
                            .map(numeral_to_unicode)
                            .collect::<Vec<String>>()
                            .join(""),
                        entries[0].definitions().collect::<Vec<_>>().join(" / ")
                    )
                };
                format!("{}|\"{}\"", word, defs_block)
            }
            None => format!("# Could not find definition for {}", word),
        };
        if !def.starts_with('#') {
            println!("{def}");
        } else {
            not_found.push(def);
        }
    }

    println!(
        "\n### Lookup complete: {} words were not found",
        not_found.len()
    );
    for nf in not_found {
        println!("{nf}");
    }

    exit(0);
}

use std::collections::HashSet;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn read_to_set(fname: &str) -> HashSet<String> {
    // read a file line by line into a set
    let mut set = HashSet::new();
    let fpath = Path::new(fname);
    let f = File::open(&fpath)
                .expect("Couldn't open file");

    for line in BufReader::new(f).lines() {
        set.insert(line.unwrap().to_uppercase());
    }

    return set
}


pub fn is_word(w: &str, dict: &HashSet<String>) -> bool {
    // is this a word
    dict.contains(&w.to_uppercase())
}


pub fn valid_crossword(cword: &str, letter: &str, dict: &HashSet<String>) -> bool {
    //Is placing letter valid with respect to a crossword?
    return cword.len() == 1 || is_word(&cword.replace(".", letter), dict);

}

pub fn canonical(l: &char) -> char {
    if l.is_alphabetic() {
        return *l;
    } else {
        return '.';}
}

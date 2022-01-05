use std::io;
use std::collections::HashSet;

use crate::board::{Board};
use crate::plays::{Play};
use crate::words::{is_word};
use crate::racks::{letters, remove};
use crate::score::{enumerate_play, cross_plays};


fn parse_direction(mut d: String) -> usize {
    d.pop(); // remove \n
    match d.as_str() {
        "A" => 1,
        "D" => 17,
        _ => 0
    }
}

fn parse_position(mut pos: String) -> usize {
    pos.pop(); // get rid of the \n 
    let cols = &"ABCDEFGHIJKLMNO";

    let spl: Vec<&str> = pos.split(",").collect();
    let col = cols.chars().position(|c| c.to_string() == spl[0]).unwrap();
    let row = spl[1].parse::<usize>().unwrap();
    return row * 18 + col + 1

}

fn is_valid(b: &Board, start: &usize, dir: &usize, 
            word: &str, rack: &str, dict: &HashSet<String>) -> bool {

    let p = Play {start: *start, dir: *dir, letters: word.to_string(), rack: rack.to_string()};

    if start > &270 {
        // is the index valid?
        return false
    }
    if ![1, 18].contains(dir) {
        // is the dir a dir?
        return false
    }
    if b.check_bounds(&p) {
        // is the play on the board?
        return false
    }
    if !is_word(word, dict) {
        // is it a word?
        return false
    }
    // Do the letters match the board?
    // also collect the letters to check the rack
    let mut letters_played = "".to_string();
    for (i, l) in enumerate_play(&p) {
        if !".:;*-=".contains(b.board[i]) && !(l.to_ascii_uppercase() == b.board[i]) {
            return false
        }
        if l.to_ascii_uppercase() != b.board[i] && b.board[i].is_alphabetic() {
            letters_played.push(l.to_ascii_uppercase());
        }
    }
    let mut new_rack = letters(rack);
    for w in letters_played.chars() {
        // is it in the rack?
        if !new_rack.contains(&w.to_string()) {
            return false
        }
        new_rack = remove(&w.to_string(), &new_rack);
    }
    // Do the letters match the board?
    for (i, l) in enumerate_play(&p) {
        if !".:;*-=".contains(b.board[i]) && !(l.to_ascii_uppercase() == b.board[i]) {
            return false
        }
    }
    let cwords = cross_plays(b, &p);
    for cw in cwords {
        // are all the crosswords words?
        if !is_word(&cw.letters, dict) || cw.letters.len() > 1 {
            return false
        }
    }

    return true
}

pub fn input_play(rack: &str, b: &Board, dict: &HashSet<String>) -> Play {
    // get a users play, and return it. This doesn't
    // update the rack we will do that in main!


    println!("Where would you like to play? (col, row)");
    let mut position = String::new();
    io::stdin()
        .read_line(&mut position)
        .expect("Failed to read input");
    
    let start = parse_position(position);

    println!("What direction? (A/D)");
    let mut direction = String::new();
    io::stdin()
        .read_line(&mut direction)
        .expect("Failed to read input");
    
    let dir = parse_direction(direction);

    println!("Enter your word:");
    let mut word = String::new();
    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read input");

    word.pop(); // remove \n
    let valid = is_valid(b, &start, &dir, &word, rack, dict);
    if valid {
        return Play {start: start, dir: dir, letters: word, rack: rack.to_string()};
    } else {
        println!("Invalid play! Try again.");
        return input_play(rack, b, dict);
    }
}


use std::collections::HashMap;

use crate::board::{Board, other};
use crate::plays::{Play};



static POINTS: &[(char, i32)] = &[
    ('A', 1), ('B', 3), ('C', 3), ('D', 2), ('E', 1), ('F', 4), ('G', 2), 
    ('H', 4), ('I', 1), ('J', 8), ('K', 5), ('L', 1), ('M', 3), ('N', 1), 
    ('O', 1), ('P', 3), ('Q', 10), ('R', 1), ('S', 1), ('T', 1), ('U', 1), 
    ('V', 4), ('W', 4), ('X', 8), ('Y', 4), ('Z', 10)];


pub fn tile_points(tile: &char) -> i32 {
    let pts = POINTS.binary_search_by(|(k, _)| k.cmp(&tile)).map(|x| POINTS[x].1);
    match pts {
        Ok(p) => p,
        Err(e) => 0
    }
}


pub fn enumerate_play(p: &Play) -> Vec<(usize, char)> {
    let mut out = vec![];
    for (i, l) in p.letters.chars().enumerate() {
        out.push((p.start + i * p.dir, l));
    }
    return out;
}

fn letters_played(b: &Board, p: &Play) -> i32 {
    let mut c = 0;
    for (_i, l) in enumerate_play(p) {
        if ".:;*-=".contains(l) {
            c += 1;
        }
    }
    return c
}

fn bingo(b: &Board, p: &Play) -> i32 {
    
    if p.rack.len() == 0 && letters_played(b, p) == 7 {
        50
    } else {
        0
    }
}

fn word_multiplier(sq: &char) -> i32 {
    if sq == &'=' {
        3
    } else if sq == &'-' {
        2
    } else {
        1
    }
}

fn letter_multiplier(sq: &char) -> i32 {
    if sq == &';' {
        3
    } else if sq == &':' {
        2
    } else {
        1
    }
}

fn word_score(b: &Board, p: &Play) -> i32 {
    let mut total = 0;
    let mut word  = 1;

    for (i, l) in enumerate_play(p) {
        let sq = b.board[i];
        word *= word_multiplier(&sq);
        total += tile_points(&l) * letter_multiplier(&sq);
    }
    return total
}

pub fn cross_plays(b: &Board, p: &Play) -> Vec<Play> {

    let cross = other(&(p.dir as isize));
    
    let mut out: Vec<Play> = vec![];
    for (i, l) in enumerate_play(p) { 
        if ".:;*-=".contains(b.board[i]) && (
            b.board[i - cross as usize].is_alphabetic() || b.board[i + cross as usize].is_alphabetic()) {

            let start = b.scan_letters_from(&(i as isize), &(-cross));
            let end   = b.scan_letters_from(&(i as isize), &cross);

            let before = b.get_slice(&start, &end, &cross);
            let after  = b.get_slice(&(start + cross), &(end + cross), &cross);
            let word = before + &l.to_string() + &after;
            out.push(Play {start: start as usize, dir: cross as usize, letters: word, rack: p.rack.clone() });
        }

    }
    return out
}

pub fn score(b: &Board, p: &Play) -> i32 {
    
    let w  = word_score(b, p);
    let bo = bingo(b, p);
    let c: i32  = cross_plays(b, p).iter().map(|x| word_score(b, x)).sum();

    return w + bo + c
}



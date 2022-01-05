use std::collections::HashSet;

use crate::board::{Board};
use crate::racks;
use crate::words::{valid_crossword, is_word};

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Play {
    pub start:   usize, //index of start point
    pub dir:     usize, // direction, 1 or 17
    pub letters: String,
    pub rack:    String,
}


 pub fn all_plays(b: &Board, rack: &str, prefixes: &HashSet<String>, dict: &HashSet<String>) -> Vec<Play> {

    let anchors = b.all_anchors();
    let rack_prefs = racks::rack_prefixes(&rack, &prefixes);
    let mut plays = vec![Play {start: 0, dir: 1, letters: "".to_string(), rack: "".to_string()}];

    for anchor in anchors {
        for dir in [1, 18].iter() {
            for play in prefix_plays(&rack_prefs, b, &anchor, &dir, rack) {
                let _x = extend_play(b, &play, prefixes, dict, &mut plays);
            }
        }
    }
    return plays;
}


fn prefix_plays(prefs: &HashSet<String>, b: &Board, anchor: &isize, dir: &isize, rack: &str) -> Vec<Play> {
    if b.board[(anchor - dir) as usize].is_alphabetic() {
        let start = b.scan_letters_from(&anchor, &-dir);
        let letts = b.get_slice(&start, &anchor, &dir);
        
        return vec![Play {start: start as usize, dir: *dir as usize, letters: letts, rack: rack.to_string()}];

    } else {
        let maxlen = ((anchor - b.scan_to_anchor(anchor, &(-dir))) / dir) as isize;
        let mut plays = vec![];
        
        for pref in prefs {
            if pref.len() <= maxlen as usize {
                let start = anchor - (pref.len() as isize) * dir;
                let rack = racks::remove(pref, rack);
                
                plays.push(Play{start: start as usize, dir: *dir as usize, letters: pref.to_string(), rack: rack});
            }
        }
        return plays;
    }
}


fn extend_play(b: &Board, p: &Play, prefs: &HashSet<String>, dict: &HashSet<String>,
               mut plays: &mut Vec<Play>) -> () {
    
    let s = p.start + p.dir * p.letters.len() as usize;
    if b.board[s] == '#' {
        return (); 
    }
    let cword = b.crossword(&(s as isize), &(p.dir as isize));
    let poss_letts = if b.board[s].is_alphabetic() {b.board[s].to_string().to_uppercase()} else {racks::letters(&p.rack)};

    for l in poss_letts.chars() {
        let pref2 = p.letters.clone() + &l.to_string();
        if prefs.contains(&pref2.to_uppercase()) && valid_crossword(&cword, &l.to_string(), dict){
            
            let rack2 = if b.board[s].is_alphabetic() {p.rack.clone()} else {racks::remove(&l.to_string(), &p.rack)};
            let play2 = Play{start: p.start, dir: p.dir, letters: pref2.clone(), rack: rack2};
            
            if is_word(&pref2, dict) && !b.board[s + p.dir].is_alphabetic() {
                plays.push(play2.clone());
            }
            let _x = extend_play(b, &play2, prefs, dict, plays);
        }
    }
}

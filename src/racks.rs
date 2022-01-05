use std::collections::HashSet;
use itertools::Itertools;
use std::cmp;

use rand::seq::SliceRandom;
use rand::thread_rng;


static BLANK: &str = &"_";
static ALPHABET: &str = &"ABCDEFGHIJKLMNOPQRSTUVWXYZ";


pub fn replenish(rack: &mut String, bag: &mut Vec<char>) {

    let mut rng = thread_rng();
    bag.shuffle(&mut rng);

    let n = bag.len().saturating_sub(7 - rack.len());
    let letts: String = bag.split_off(n).into_iter().collect();

    rack.push_str(&letts);
}

pub fn remove_mutable(letts: &str, rack: &mut String) {

    let tiles = letts.replace(char::is_lowercase, "_");

    for t in tiles.chars() {
        *rack = rack.replacen(&t.to_string(), "", 1);
    }
}

pub fn remove(letts: &str, rack: &str) -> String {

    let tiles = letts.replace(char::is_lowercase, "_");
    let mut new_rack = rack.to_string();

    for t in tiles.chars() {
        new_rack = new_rack.replacen(&t.to_string(), "", 1);
    }
    return new_rack
}


pub fn letters(rack: &str) -> String {

    let mut letts: String = "".to_string();

    if rack.contains('_') {
        letts.push_str(&rack.replace(BLANK, ""));
        letts.push_str(&ALPHABET.to_lowercase());

    } else {
        letts.push_str(&rack.chars().unique().collect::<String>());
    }

    return letts;
}

fn extend_prefixes(prefix: &str, prefixes: &HashSet<String>, 
                   rack: &str, mut results: &mut HashSet<String>) {

    if prefixes.contains(&prefix.to_uppercase()) {
        results.insert(prefix.to_string().clone());

        for l in letters(&rack).chars() {
            let new_pref = prefix.to_string().clone() + &l.to_string();

            extend_prefixes(&new_pref, &prefixes,
                            &remove(&l.to_string(), &rack), results);
        }
    }
}


pub fn rack_prefixes(rack: &str, prefixes: &HashSet<String>) -> HashSet<String> {
    
    let mut res = HashSet::new();

    extend_prefixes(&"", prefixes, &rack, &mut res);

    return res;

}

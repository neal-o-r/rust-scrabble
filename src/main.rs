use std::collections::HashSet;
use colored::Colorize;

mod board;
mod plays;
mod words;
mod racks;
mod score;
mod input;

fn best_play(b: &board::Board, rack: &str, 
             prefixes: &HashSet<String>, dict: &HashSet<String>) -> plays::Play {

    // select the best possible play
    let ps = plays::all_plays(b, rack, prefixes, dict);
    let best = ps.iter().max_by_key(|x| score::score(b, x));
    return best.unwrap().clone()
}

fn print_state(r1: &str, s1: &i32, s2: &i32) {
    let s = format!("
             Your Rack: {}          Your Score: {} -- Robo Score: {}", r1, s1, s2);
    println!("{}", s.white())
}

fn main() {

    let mut bag: Vec<char> = 
        "AAAAAAAAABBCCDDDDEEEEEEEEEEEEFFGGGHHIIIIIIIIIJKLLLLMMNNNNNNOOOOOOOOPPQRRRRRRSSSSTTTTTTUUUUVVWWXYYZ__".chars().collect();

    let prefixes = words::read_to_set("../data/prefixes.txt");
    let dict = words::read_to_set("../data/dict.txt");

    
    let mut b = board::Board::default();
    let mut rack1 = "".to_string();
    let mut rack2 = "".to_string();

    racks::replenish(&mut rack1, &mut bag);
    racks::replenish(&mut rack2, &mut bag);
    
    let mut score1 = 0;
    let mut score2 = 0;

    let empty_play = plays::Play {start: 0, dir: 1, letters: "".to_string(), rack: "".to_string()};
    
    let mut turn = 0;
    let mut did_play = true; //did either player take a turn?
    while did_play {
        
        if turn == 0 {
            b.display();
            print_state(&rack1, &score1, &score2);
            
            let p = input::input_play(&rack1, &b, &dict);
            b.make_play(&p);
            
            score1 += score::score(&b, &p);
            racks::remove_mutable(&p.letters, &mut rack1);
            racks::replenish(&mut rack1, &mut bag);
            println!("{}", rack1);
            if p == empty_play {
                did_play = false;
            }

        } else {

            b.display();
            print_state(&rack1, &score1, &score2);
            let p = best_play(&b, &rack2, &prefixes, &dict); 
            b.make_play(&p);
            
            score2 += score::score(&b, &p);
            racks::remove_mutable(&p.letters, &mut rack2);
            racks::replenish(&mut rack2, &mut bag);
        
            if p == empty_play {
                did_play = false;
            }
        }
        
        turn = (turn + 1) % 2 //switch turn
        
    }  
}

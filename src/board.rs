use prettytable::{Table, Row, Cell, format};

use crate::plays::{Play};
use crate::words::{canonical};


static EMPTYBOARD: &str = "#################\n#...=..;.;..=...#\n#..:..-...-..:..#\n#.:..:.....:..:.#\n#=..;...-...;..=#\n#..:...:.:...:..#\n#.-...;...;...-.#\n#;...:.....:...;#\n#...-...*...-...#\n#;...:.....:...;#\n#.-...;...;...-.#\n#..:...:.:...:..#\n#=..;...-...;..=#\n#.:..:.....:..:.#\n#..:..-...-..:..#\n#...=..;.;..=...#\n#################"; 



pub fn other(dir: &isize) -> isize {
    // swap directions, little helper func
    if *dir == 1 as isize {
        return 18 as isize;
    } else {
        return 1 as isize;
    }
}


#[derive(Clone)]
pub struct Board {
    pub board: Vec<char>,
    across: isize,
    down: isize,
    directions: [isize; 4], 

}

impl Default for Board {
    fn default() -> Board {
        Board {board: EMPTYBOARD.chars().collect(),
               across: 1,
               down: 18,
               directions: [1, 18, -1, -18],
        }
    }
}


impl Board {

    pub fn check_bounds(&self, play: &Play) -> bool {

        let end = play.start + play.letters.len() * play.dir;
        let inds = (play.start..end).step_by(play.dir);
        let mut vals = vec![];
        
        for i in inds {
            vals.push(self.board[i]);
        }

        return vals.contains(&'#');
    }

    pub fn is_anchor(&self, index: &isize) -> bool {

        let sq = self.board[*index as usize];
        let star = sq == '*';

        let mut neighbours = vec![];
        for d in &self.directions {
            neighbours.push(self.board[(index + d) as usize]);
        }

        return star || (".:;*-=".contains(sq) && neighbours.iter().any(|x| x.is_alphabetic()))
        
    }

    pub fn all_anchors(&self) -> Vec<isize> {

        let anchors = (18 .. 270).filter(|x| self.is_anchor(&(*x as isize))).collect();
        return anchors
    }

    pub fn scan_letters_from(&self, index: &isize, dir: &isize) -> isize {

        let mut s = *index;

        while self.board[(s + dir) as usize].is_alphabetic() {
            s += dir;
        }
        
        return s;
    }

    pub fn scan_to_anchor(&self, index: &isize, dir: &isize) -> isize {
        
        let mut s = *index;

        while self.board[(s + dir) as usize] != '#' && 
             !self.is_anchor(&(s + dir)) {
            s += dir
        }
        return s;
    }

    pub fn get_slice(&self, start: &isize, end: &isize, dir: &isize) -> String {
        
        let inds = (*start.. *end).step_by(*dir as usize);
        let mut slice = "".to_string();
        
        for i in inds {
            slice.push(self.board[i as usize]);
        } 

        return slice
    }

    pub fn crossword(&self, index: &isize, dir: &isize) -> String {

        let d = other(&dir);
        let start = self.scan_letters_from(&index, &(-d));
        let end = self.scan_letters_from(&index, &d) + d;

        let mut cword = "".to_string();

        for i in (start..end).step_by(d as usize) {
            cword.push(canonical(&self.board[i as usize]));
        }

        return cword;
    }


    pub fn make_play(&mut self, play: &Play) {
        
        let end = play.start + play.letters.len() * play.dir;
        let inds = (play.start..end).step_by(play.dir);

        for (i, c) in inds.zip(play.letters.chars()) {
            self.board[i] = c;
        }

    }

    fn char_to_cell(c: &str) -> Cell {
        match c {
            "." => Cell::new(" "),
            ":" => Cell::new("DL").style_spec("Fbc"),
            ";" => Cell::new("TL").style_spec("Fmc"),
            "-" => Cell::new("DW").style_spec("Fcc"),
            "=" => Cell::new("TW").style_spec("Frc"),
            "*" => Cell::new("★").style_spec("Fyc"),
            _ => Cell::new(c).style_spec("Fwc")
        }
    }

    pub fn display(&self) {
        let mut table = Table::new();

        let format = format::FormatBuilder::new()
            .column_separator('║')
            .borders('║')
            .separators(&[format::LinePosition::Top],
                    format::LineSeparator::new('═', '╦', '╔', '╗'))
            .separators(&[format::LinePosition::Intern],
                    format::LineSeparator::new('═', '╬', '╠', '╣'))
            .separators(&[format::LinePosition::Bottom],
                    format::LineSeparator::new('═', '╩', '╚', '╝'))
            .padding(1, 1)
            .build();
        table.set_format(format);

        let board_str = self.board.iter().cloned().collect::<String>();
        let lines = board_str.split('\n');

        for (i, l) in lines.enumerate() {
            if i == 0 || i == 16 {
                let row = " ABCDEFGHIJKLMNO".chars().map(
                                |x| Cell::new(&x.to_string()).style_spec("Fw")).collect();
                table.add_row(row);

            } else {
                let mut row: Row = Row::new(vec![Cell::new(&i.to_string()).style_spec("Fw")]);
                for c in l.chars() {
                    if !(c == '#') {
                        let cell = Board::char_to_cell(&c.to_string());
                        row.add_cell(cell);
                    }
                }
                row.add_cell(Cell::new(&i.to_string()).style_spec("Fw"));
                table.add_row(row);
            }

        }
        print!("{}[2J", 27 as char);
        table.printstd();
    }
}

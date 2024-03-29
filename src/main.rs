use std::env;
use core::fmt;
use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};


#[derive(Clone, Copy)] 
enum Cell {
    Empty,
    Hole,
    Const(u8),
    Candidate(u8),
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Empty => write!(f, " __ " ),
            Cell::Hole => write!(f, " xx " ),
            Cell::Const(v) => write!(f," {:02} ",v),
            Cell::Candidate(v) => write!(f," {:02} ",v),
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct Line{
    line: Vec<Cell>
}

#[derive(Clone, Copy)] 
struct Point {
    line: usize,
    col: usize
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.line == other.line) && (self.col == other.col)
    }
}

struct Board {
    board: Vec<Line>,
    consts: Vec<u8>,
    start: Point,
    end: Point,
    current_const_index: usize,
}

impl Line{
    fn new(text_line:&String) -> Line {
        let mut l = Line {
            line: Vec::new()
        };
        let text_line = text_line.trim();
        let v: Vec<&str> = text_line.split(' ').collect();

        for c in v {
            match c {
                "_" => l.line.push( Cell::Empty ),
                "e" => l.line.push( Cell::Empty ),
                "x" => l.line.push( Cell::Hole ),
                _ =>  l.line.push( Cell::Const( c.parse::<u8>().unwrap() ) ),
            }
        }
        return l;
    }

    fn print( &self, indent: i32)
    {
        for _i in 1..indent {
            print!(" ");
        }
        for c in &self.line {
            print!("{}",c);
        }
        println!("");
    }
}

impl Board {
    fn new(file_name:&String) -> Board {
        let mut b = Board {
            board: Vec::new(),
            consts: Vec::new(),
            start: Point{ line:0, col:0},
            end: Point{ line:0, col:0 },
            current_const_index: 1
        };

        if let Ok(text_lines) = read_lines(file_name) {
            for text_line in text_lines {
                if let Ok(t_line) = text_line {
                    b.board.push( Line::new(&t_line) );
                }
            }
        }
      
        b.find_constants();
        
        return b;
    }

    fn print(&self) {
        let indent = 0;
        for line in &self.board {
            line.print(indent);
        }
    }

    fn find_constants(&mut self) {
        let mut max_value = 0;
        for (l_index, line) in self.board.iter().enumerate() {
            for (c_index, cell) in line.line.iter().enumerate() {
                if let Cell::Const(v) = cell {
                    self.consts.push(*v);
                    if *v == 1 {
                        self.start = Point{line:l_index, col: c_index};
                    }
                    if *v > max_value {
                        self.end = Point{line:l_index, col: c_index};
                        max_value = *v;
                    }
                }
            }
        }
        self.consts.sort();
    }

    fn find_neighbours_in_adjcent_line(&self, cur_size: usize, adj_size: usize, cur_col: usize) -> Vec<usize> {
        //TODO: will not work for lines with same size since even if we know which line is higher, it doesn't tell us which is indented to left regard the other
        let offset = (cur_size as isize) - (adj_size as isize);
        let new_col = (cur_col as isize) - offset;
        let mut r = Vec::new();

        if cur_col < adj_size {
            r.push(cur_col);
        }

        if new_col >= 0 
        {
            let new_col = new_col as usize;
            if ( new_col < adj_size) {
                r.push( new_col );
            }

        }
        
        return r;
    }

    fn find_neighbours(&self, current_cell: Point) -> Vec<Point> {
        let mut r = Vec::new();
        let cur_line = current_cell.line;
        let cur_col = current_cell.col;
        let cur_line_size = self.board[cur_line].line.len();
        
        
        if cur_line > 0 {
            let prev_line_size = self.board[cur_line-1].line.len();
            let prev_cols = self.find_neighbours_in_adjcent_line( cur_line_size, prev_line_size, cur_col);
            for c in prev_cols{
                r.push( Point{line:cur_line-1, col:c} );
            }
        }

        if cur_line + 1 < self.board.len() {
            let next_line_size = self.board[cur_line+1].line.len();
            let next_cols = self.find_neighbours_in_adjcent_line( cur_line_size, next_line_size, cur_col);
            for c in next_cols{
                r.push( Point{line:cur_line+1, col:c} );
            }
        }


        if cur_col > 0{
            r.push(Point{ line: cur_line, col: cur_col - 1});
        }

        if cur_col + 1 < self.board[cur_line].line.len(){
            r.push(Point{ line: cur_line, col: cur_col + 1});
        }
        
        return r;
    }

    fn solve(&mut self, current_cell: Point) {
        
        if current_cell == self.end {
            self.print();
            return;
        }

        let neighbours: Vec<Point> = self.find_neighbours( current_cell );
        
        let mut cur_value = 0;
        
        match self.board[current_cell.line].line[current_cell.col] {
            Cell::Const(v) => cur_value = v,
            Cell::Candidate(v) => cur_value = v,
            _ => return
        }
        
        
        for n in neighbours {
            
            match self.board[n.line].line[n.col]{
                Cell::Empty => {
                    if cur_value + 1 < self.consts[self.current_const_index]{
                        self.board[n.line].line[n.col] = Cell::Candidate(cur_value+1);
                        self.solve( n );
                        self.board[n.line].line[n.col] = Cell::Empty;
                    }                    
                },
                Cell::Hole => (),
                Cell::Const(v) => {
                    if (cur_value + 1 == v) && (v == self.consts[self.current_const_index]){                        
                        self.current_const_index += 1;
                        self.solve( n );
                        self.current_const_index -= 1;
                    }                    
                },
                Cell::Candidate(_v) => ()
            }            
        }
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <input_file>", args[0]);
        return;
    }

    let file_name = &args[1];

    let mut b = Board::new(file_name);
    b.solve(b.start);
}
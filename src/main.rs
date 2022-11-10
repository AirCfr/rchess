use termion::{*, color::{Color}};

//"\x1b[2J"

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum Piece {
    Empty,
    Wpawn,
    Wknigth,
    Wbishop,
    Wrook,
    Wqueen,
    Wking,
    Bpawn,
    Bknigth,
    Bbishop,
    Brook,
    Bqueen,
    Bking
}
impl Piece {
    fn to_string(self) -> String{
        match self {
            Piece::Empty => {" ".to_string()},
            Piece::Bpawn | Piece::Wpawn => {"p".to_string()},
            Piece::Bknigth | Piece::Wknigth => {"k".to_string()},
            Piece::Bbishop | Piece::Wbishop => {"b".to_string()},
            Piece::Brook | Piece::Wrook => {"R".to_string()},
            Piece::Bqueen | Piece::Wqueen => {"Q".to_string()},
            Piece::Bking | Piece::Wking => {"K".to_string()}
        }
    }
    fn print(p:Piece, cd: (&dyn Color, &dyn Color), final_pos:(u16,u16)){
        
        print!("{}",termion::color::Bg(cd.0));
        print!("{}",termion::color::Fg(cd.1));
        print!(" ");
        print!("{}", p.to_string());
        print!(" ");


        //print!("{}", termion::cursor::Goto(final_pos.0, final_pos.1));
        return ();
    }
}
struct Grid {
    size: usize,
    data: Vec<Vec<Piece>>
}

impl Grid {
    fn new_empty() -> Grid {
        Grid {
            size: 8,
            data: vec![vec![Piece::Wking; 8]; 8]
        }
    }
    fn print(self) {
        
        print!("{}", termion::cursor::Goto(1,1));
        let mut overlay_num = 8;

        let mut cpt: u16 = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                Piece::print(self.data[i][j], match cpt%2 {
                    0 => (&color::LightMagenta,&color::Cyan),
                    _ => (&color::White,&color::Black), 
                }, ((1+(cpt*3))%27, (i + 1).try_into().unwrap()));
                cpt += 1;
            }
            cpt += 1;
            print!("{}",termion::color::Bg(termion::color::Black));
            print!("{}",termion::color::Fg(termion::color::White));
            println!(" - {}", overlay_num);
            overlay_num -= 1;
        }
        println!(" |  |  |  |  |  |  |  | ");
        println!(" A  B  C  D  E  F  G  H ");
    }
}
fn main() {
    println!("Hello, world!");
    let gride:Grid = Grid::new_empty();
    print!("{}",termion::clear::All);    
    gride.print();
}


//===   tests   ===//

// 1 Build an empty grid and print it
// 2 Notation translator (a8 -> 1.1 && 1.1 -> a8)
// 3 Pieces moves
// 4 rounds
use std::{
    io::{stdin, stdout, BufRead, BufReader, Write, BufWriter},
    net::{SocketAddr, TcpListener, TcpStream}, fmt::format,
};
use termion::{color::Color, *};

const SIZE_SQUARE_PRINT: u16 = 5;

#[derive(Clone, Copy)]
pub struct Piece {
    ptype: PieceType,
    color: PieceColor,
    coords: (usize, usize),
}

impl Piece {
    fn print(self, cd: (&dyn Color, &dyn Color), begin_pos: (u16, u16)) {
        print!("{}", termion::cursor::Goto(begin_pos.0, begin_pos.1));

        print!("{}", termion::color::Bg(cd.0));
        print!("{}", termion::color::Fg(cd.1));
        print!(
            "     {}{}",
            termion::cursor::Left(SIZE_SQUARE_PRINT),
            termion::cursor::Down(1)
        );
        print!(
            "  {}  {}{}",
            self.ptype.to_string(),
            termion::cursor::Left(SIZE_SQUARE_PRINT),
            termion::cursor::Down(1)
        );
        print!("     ");

        return ();
    }
    fn print2(self, cd: (&dyn Color, &dyn Color), begin_pos: (u16, u16), mut connection: &TcpStream) {
        connection.write_all(format!("{}", termion::cursor::Goto(begin_pos.0, begin_pos.1)).as_bytes()).unwrap();
        connection.write_all(format!("{}", termion::color::Bg(cd.0)).as_bytes()).unwrap();
        connection.write_all(format!("{}", termion::color::Fg(cd.1)).as_bytes()).unwrap();
        connection.write_all(
            format!("     {}{}",
            termion::cursor::Left(SIZE_SQUARE_PRINT),
            termion::cursor::Down(1)
        ).as_bytes()).unwrap();
        connection.write_all(
            format!("  {}  {}{}",
            self.ptype.to_string(),
            termion::cursor::Left(SIZE_SQUARE_PRINT),
            termion::cursor::Down(1)
        ).as_bytes()).unwrap();
        connection.write_all("     ".to_string().as_bytes()).unwrap();

        return ();
    }

    fn can_move_to(self) -> bool {
        true //todo
    }
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
enum PieceType {
    Empty,
    Pawn,
    Knigth,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    fn to_string(self) -> String {
        match self {
            PieceType::Empty => " ".to_string(),
            PieceType::Pawn => "p".to_string(),
            PieceType::Knigth => "k".to_string(),
            PieceType::Bishop => "b".to_string(),
            PieceType::Rook => "R".to_string(),
            PieceType::Queen => "Q".to_string(),
            PieceType::King => "K".to_string(),
        }
    }

    fn to_piece(self, color: PieceColor, coords: (usize, usize)) -> Piece {
        Piece {
            ptype: self,
            color,
            coords,
        }
    }
}

#[derive(Clone, Copy)]
enum PieceColor {
    Empty,
    White,
    Black,
    _Blue,
    _Red,
    _Green,
}
//#[derive(Clone, Copy)]
struct Grid {
    size: usize,
    data: Vec<Vec<Piece>>,
    is_mat: bool,
    is_pat: bool,
    mat_color: PieceColor,
}

impl Grid {
    fn _new_empty() -> Grid {
        Grid {
            size: 8,
            data: vec![vec![PieceType::King.to_piece(PieceColor::White, (0, 0)); 8]; 8],
            is_mat: false,
            is_pat: false,
            mat_color: PieceColor::Empty,
        }
    }

    fn new_normal() -> Grid {
        Grid {
            size: 8,
            data: vec![
                vec![
                    PieceType::Rook.to_piece(PieceColor::Black, (0, 0)),
                    PieceType::Knigth.to_piece(PieceColor::Black, (1, 0)),
                    PieceType::Bishop.to_piece(PieceColor::Black, (2, 0)),
                    PieceType::King.to_piece(PieceColor::Black, (3, 0)),
                    PieceType::Queen.to_piece(PieceColor::Black, (4, 0)),
                    PieceType::Bishop.to_piece(PieceColor::Black, (5, 0)),
                    PieceType::Knigth.to_piece(PieceColor::Black, (6, 0)),
                    PieceType::Rook.to_piece(PieceColor::Black, (7, 0)),
                ],
                vec![
                    PieceType::Pawn.to_piece(PieceColor::Black, (0, 1)),
                    PieceType::Pawn.to_piece(PieceColor::Black, (1, 1)),
                    PieceType::Pawn.to_piece(PieceColor::Black, (2, 1)),
                    PieceType::Pawn.to_piece(PieceColor::Black, (3, 1)),
                    PieceType::Pawn.to_piece(PieceColor::Black, (4, 1)),
                    PieceType::Pawn.to_piece(PieceColor::Black, (5, 1)),
                    PieceType::Pawn.to_piece(PieceColor::Black, (6, 1)),
                    PieceType::Pawn.to_piece(PieceColor::Black, (7, 1)),
                ],
                vec![PieceType::Empty.to_piece(PieceColor::Empty, (9, 9)); 8],
                vec![PieceType::Empty.to_piece(PieceColor::Empty, (9, 9)); 8],
                vec![PieceType::Empty.to_piece(PieceColor::Empty, (9, 9)); 8],
                vec![PieceType::Empty.to_piece(PieceColor::Empty, (9, 9)); 8],
                vec![
                    PieceType::Pawn.to_piece(PieceColor::White, (0, 6)),
                    PieceType::Pawn.to_piece(PieceColor::White, (1, 6)),
                    PieceType::Pawn.to_piece(PieceColor::White, (2, 6)),
                    PieceType::Pawn.to_piece(PieceColor::White, (3, 6)),
                    PieceType::Pawn.to_piece(PieceColor::White, (4, 6)),
                    PieceType::Pawn.to_piece(PieceColor::White, (5, 6)),
                    PieceType::Pawn.to_piece(PieceColor::White, (6, 6)),
                    PieceType::Pawn.to_piece(PieceColor::White, (7, 6)),
                ],
                vec![
                    PieceType::Rook.to_piece(PieceColor::White, (0, 7)),
                    PieceType::Knigth.to_piece(PieceColor::White, (1, 7)),
                    PieceType::Bishop.to_piece(PieceColor::White, (2, 7)),
                    PieceType::King.to_piece(PieceColor::White, (3, 7)),
                    PieceType::Queen.to_piece(PieceColor::White, (4, 7)),
                    PieceType::Bishop.to_piece(PieceColor::White, (5, 7)),
                    PieceType::Knigth.to_piece(PieceColor::White, (6, 7)),
                    PieceType::Rook.to_piece(PieceColor::White, (7, 7)),
                ],
            ],
            is_mat: false,
            is_pat: false,
            mat_color: PieceColor::Empty,
        }
    }

    fn get_next_coord(i: u16, j: u16, s: u16) -> (u16, u16) {
        let x: u16 = 1 + (j) * s;
        let y: u16 = 1 + (i) * (s - 2);
        return (x, y);
    }

    fn print(&self) {
        print!("{}", termion::cursor::Goto(1, 1));
        let mut overlay_num = 8;

        let mut cpt: u16 = 0;

        for i in 0..self.size {
            for j in 0..self.size {
                Piece::print(
                    self.data[i][j],
                    (
                        match cpt % 2 {
                            0 => &color::LightBlue, //todo getter foreground /
                            _ => &color::Magenta,
                        },
                        match self.data[i][j].color {
                            PieceColor::Black => &color::Black,
                            PieceColor::White => &color::White,
                            _ => &color::Black,
                        },
                    ),
                    Grid::get_next_coord(
                        i.try_into().unwrap(),
                        j.try_into().unwrap(),
                        SIZE_SQUARE_PRINT,
                    ),
                );
                //print!("{:#?}",Grid::get_next_coord(i.try_into().unwrap(), j.try_into().unwrap(), SIZE_SQUARE_PRINT));
                stdout().flush().unwrap();
                cpt += 1;
            }
            cpt += 1;
            print!(
                "{}{}{}-{}",
                termion::cursor::Up(1),
                termion::color::Fg(termion::color::White),
                termion::color::Bg(termion::color::Black),
                overlay_num
            );
            //print!("{}",termion::color::Fg(termion::color::White));
            //print!("-{}", overlay_num);
            overlay_num -= 1;
        }
        print!("{}", termion::cursor::Goto(1, 1 + (8 * 3)));
        println!("  |    |    |    |    |    |    |    |  ");
        println!("  A    B    C    D    E    F    G    H  ");
    }

    fn print2(&self, mut connection: &TcpStream) {
        connection.write_all(termion::cursor::Goto(1, 1).to_string().as_bytes()).unwrap();
        let mut overlay_num = 8;

        let mut cpt: u16 = 0;

        for i in 0..self.size {
            for j in 0..self.size {
                //let mut connec:TcpStream = connection.try_clone().unwrap(); 
                Piece::print2(
                    self.data[i][j],
                    (
                        match cpt % 2 {
                            0 => &color::LightBlue, //todo getter foreground /
                            _ => &color::Magenta,
                        },
                        match self.data[i][j].color {
                            PieceColor::Black => &color::Black,
                            PieceColor::White => &color::White,
                            _ => &color::Black,
                        },
                    ),
                    Grid::get_next_coord(
                        i.try_into().unwrap(),
                        j.try_into().unwrap(),
                        SIZE_SQUARE_PRINT,
                    ), connection);
                //print!("{:#?}",Grid::get_next_coord(i.try_into().unwrap(), j.try_into().unwrap(), SIZE_SQUARE_PRINT));
                stdout().flush().unwrap();
                cpt += 1;
            }
            cpt += 1;
            connection.write_all(
                format!("{}{}{}-{}",
                termion::cursor::Up(1),
                termion::color::Fg(termion::color::White),
                termion::color::Bg(termion::color::Black),
                overlay_num).as_bytes()
            ).unwrap();
            //print!("{}",termion::color::Fg(termion::color::White));
            //print!("-{}", overlay_num);
            overlay_num -= 1;
        }
        connection.write_all(termion::cursor::Goto(1, 1 + (8 * 3)).to_string().as_bytes()).unwrap();
        connection.write_all("  |    |    |    |    |    |    |    |  \n".to_string().as_bytes()).unwrap();
        connection.write_all("  A    B    C    D    E    F    G    H  \n".to_string().as_bytes()).unwrap();
    }

    fn get_piece(&self, pos: (usize, usize)) -> Result<Piece, &'static str> {
        if pos.0 < self.size && pos.1 < self.size {
            Ok(self.data[pos.0][pos.1])
        } else {
            Err("Position out of bound")
        }
    }

    fn alpha_to_index(mut s: String) -> Result<(usize, usize), ()> {
        if (s.len() == 3) {
            s.pop().unwrap();
            Ok((
                match s.pop().unwrap() {
                    '1' => 7 as usize,
                    '2' => 6 as usize,
                    '3' => 5 as usize,
                    '4' => 4 as usize,
                    '5' => 3 as usize,
                    '6' => 2 as usize,
                    '7' => 1 as usize,
                    '8' => 0 as usize,
                    _ => return Err(()),
                },
                match s.pop().unwrap() {
                    'a' | 'A' => 0 as usize,
                    'b' | 'B' => 1 as usize,
                    'c' | 'C' => 2 as usize,
                    'd' | 'D' => 3 as usize,
                    'e' | 'E' => 4 as usize,
                    'f' | 'F' => 5 as usize,
                    'g' | 'G' => 6 as usize,
                    'h' | 'H' => 7 as usize,
                    _ => return Err(()),
                },
            ))
        } else {
            Err(())
        }
    }

    //fn is_king_mat(&self) -> bool {
    //    false //todo
    //}

    fn empty_case(&mut self, pos: (usize, usize)) {
        self.data[pos.1][pos.0] = PieceType::Empty.to_piece(PieceColor::Empty, (9, 9))
    }

    fn move_piece_to(&mut self, p: Piece, pos: (usize, usize)) -> Result<Piece, &'static str> {
        let result = self.get_piece(pos);
        match result {
            Ok(_) => match result.clone().unwrap().ptype {
                _ => {
                    if (result.unwrap().can_move_to()) {
                        self.empty_case(p.coords);
                        self.data[pos.0][pos.1] = p;
                        result.unwrap().coords = pos;
                        return result;
                    } else {
                        Err("This unit cant go there")
                    }
                }
            },
            Err(_) => return result,
        }
    }
}

fn game(mut connection: TcpStream, _address: SocketAddr) {
    
    let mut gride: Grid = Grid::new_normal();
    let mut user_input = "Nothing, Your turn !\n".to_string();

    loop {

        
        //print refreshed grid TODO replace it by print both
        print!("{}", termion::clear::All);
        connection.write_all(termion::clear::All.to_string().as_bytes()).unwrap();
        connection.write_all(termion::cursor::Goto(1,1).to_string().as_bytes()).unwrap();
        //connection.write_all("GridPlaceholderAfterServerTurn\n".as_bytes()).unwrap();
        gride.print2(&connection);
        gride.print();
        print!("{}", termion::cursor::Down(1));
        connection.write_all(format!("Server played :{user_input}").as_bytes()).unwrap();

        let mut reader = BufReader::new(&connection);
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        //println!("client to self > {line}");
        println!("Client played : {line}");

        //print refreshed grid TODO replace it by print both
        connection.write_all(termion::clear::All.to_string().as_bytes()).unwrap();
        connection.write_all(termion::cursor::Goto(1,1).to_string().as_bytes()).unwrap();
        //connection.write_all("GridPlaceholderAfterClientTurn\n".as_bytes()).unwrap();
        gride.print2(&connection);
        gride.print();
        connection.write_all(termion::cursor::Down(1).to_string().as_bytes()).unwrap();


        let stdin = std::io::stdin();
        user_input = "".to_string();
        stdin.read_line(&mut user_input).unwrap();
        //println!("self to client > {user_input}");
    }
}

//TODO print_both(s:string) + refactor de grid.print()
// NB en fait: 
// let mut sender = BufWriter

fn main() {
    let server = TcpListener::bind("127.0.0.1:8080").unwrap();
    let (mut connection, address) = server.accept().unwrap();

    connection
    .write_all(termion::clear::All.to_string().as_bytes())
    .unwrap();
    connection
    .write_all(termion::cursor::Goto(1,1).to_string().as_bytes())
    .unwrap();

    print!("{}", termion::clear::All);
    //gride.print();

    game(connection, address);

    // print!("{}", termion::clear::All);
    // let mut gride: Grid = Grid::new_normal();
    // gride.print();
    // gride
    // .move_piece_to(gride.get_piece((0, 1)).unwrap(), (0, 3))
    // .unwrap();
    // gride.print();
    //
    // let mut user_input = String::new();
    // let stdin = std::io::stdin();
    // stdin.read_line(&mut user_input);
    // dbg!(&user_input);
    //
    // let piecs = gride.get_piece(Grid::alpha_to_index(user_input).unwrap());
    // println!("Hello, world!");
}

//===   tests   ===//

// 1 Build an empty grid and print it
//  let gride:Grid = Grid::new_empty();
//  gride.print();
// 2 Notation translator (a8 -> 1.1 && 1.1 -> a8)
// 3 PieceTypes moves
//      PieceType trait / struct
// 4 rounds
//      cursor - keyboard
// 5 telnet go brr
// 6 verifs and other boring stuff
//

// === scrap === //
//
// ((1+(cpt*3))%27, (i + 1).try_into().unwrap())


// connection
// .write_all(format!("bonjour, {address:?}\n").as_bytes())
// .unwrap();

//"\x1b[2J"

// Wpawn,
// Wknigth,
// Wbishop,
// Wrook,
// Wqueen,
// Wking,
// Bpawn,
// Bknigth,
// Bbishop,
// Brook,
// Bqueen,
// Bking,
// fn to_string(self) -> String {
//     match self {
//         PieceType::Empty => " ".to_string(),
//         PieceType::Bpawn | PieceType::Wpawn => "p".to_string(),
//         PieceType::Bknigth | PieceType::Wknigth => "k".to_string(),
//         PieceType::Bbishop | PieceType::Wbishop => "b".to_string(),
//         PieceType::Brook | PieceType::Wrook => "R".to_string(),
//         PieceType::Bqueen | PieceType::Wqueen => "Q".to_string(),
//         PieceType::Bking | PieceType::Wking => "K".to_string(),
//     }
// }
// fn to_color(self) -> String {
//     match self {
//         PieceType::Empty => " ".to_string(),
//         PieceType::Bpawn | PieceType::Wpawn => "p".to_string(),
//         PieceType::Bknigth | PieceType::Wknigth => "k".to_string(),
//         PieceType::Bbishop | PieceType::Wbishop => "b".to_string(),
//         PieceType::Brook | PieceType::Wrook => "R".to_string(),
//         PieceType::Bqueen | PieceType::Wqueen => "Q".to_string(),
//         PieceType::Bking | PieceType::Wking => "K".to_string(),
//     }
// }
// match cpt % 2 {
//     0 => (&color::LightBlue, &color::Black), // getter foreground /
//     _ => (&color::Magenta, &color::White),
// },

// let server = TcpListener::bind("127.0.0.1:8080").unwrap();
// loop {
// let (mut connection, address) = server.accept().unwrap();
//
// connection
// .write_all(format!("bonjour, {address:?}\n").as_bytes())
// .unwrap();
//
// let mut reader = BufReader::new(&connection);
//
// loop {
// let mut line = String::new();
// reader.read_line(&mut line).unwrap();
// println!("{line}");
// }
// }

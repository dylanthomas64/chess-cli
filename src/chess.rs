use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Write;
use std::iter::Map;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum TYPE {
    //true if untouched and therefore can move 2 squares
    PAWN(bool),
    BISHOP,
    KNIGHT,
    ROOK,
    QUEEN,
    KING,
}

//piece defined by colour and type
#[derive(Debug, Clone)]
enum PIECE {
    BLACK(TYPE),
    WHITE(TYPE),
}

#[derive(Debug)]
struct ParsePieceError;

impl FromStr for PIECE {
    type Err = ParsePieceError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(PIECE::WHITE(TYPE::BISHOP)),
            "N" => Ok(PIECE::WHITE(TYPE::KNIGHT)),
            "R" => Ok(PIECE::WHITE(TYPE::ROOK)),
            "Q" => Ok(PIECE::WHITE(TYPE::QUEEN)),
            "K" => Ok(PIECE::WHITE(TYPE::KING)),
            _ => Err(ParsePieceError),
        }
    }
}

impl fmt::Display for PIECE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match &self {
            PIECE::WHITE(t) => match t {
                TYPE::PAWN(_) => "♟",
                TYPE::BISHOP => "♝",
                TYPE::KNIGHT => "♞",
                TYPE::ROOK => "♜",
                TYPE::QUEEN => "♛",
                TYPE::KING => "♚",
            },
            PIECE::BLACK(t) => match t {
                TYPE::PAWN(_) => "♙",
                TYPE::BISHOP => "♗",
                TYPE::KNIGHT => "♘",
                TYPE::ROOK => "♖",
                TYPE::QUEEN => "♕",
                TYPE::KING => "♔",
            },
        };
        write!(f, "{}", s)
    }
}

//2d vector representation of the board

pub struct Board {
    pieces: Vec<Vec<Option<PIECE>>>,
}

/*
pub struct Board2 {
    map: HashMap<Option<PIECE>>
} */

impl Board {
    pub fn new() -> Board {
        let mut pieces: Vec<Vec<Option<PIECE>>> = Vec::new();
        // generate inital board state
        // start pushing from the bottom (white first so indexing will translate easily to board coordinates)

        let mut row1: Vec<Option<PIECE>> = Vec::new();
        row1.push(Some(PIECE::WHITE(TYPE::ROOK)));
        row1.push(Some(PIECE::WHITE(TYPE::KNIGHT)));
        row1.push(Some(PIECE::WHITE(TYPE::BISHOP)));
        row1.push(Some(PIECE::WHITE(TYPE::QUEEN)));
        row1.push(Some(PIECE::WHITE(TYPE::KING)));
        row1.push(Some(PIECE::WHITE(TYPE::BISHOP)));
        row1.push(Some(PIECE::WHITE(TYPE::KNIGHT)));
        row1.push(Some(PIECE::WHITE(TYPE::ROOK)));
        pieces.push(row1);
        let mut row2: Vec<Option<PIECE>> = Vec::new();
        for _ in 0..8 {
            row2.push(Some(PIECE::WHITE(TYPE::PAWN(true))));
        }
        pieces.push(row2);

        for _ in 0..4 {
            let mut empty_row: Vec<Option<PIECE>> = Vec::new();
            for _ in 0..8 {
                empty_row.push(None);
            }
            pieces.push(empty_row);
        }
        let mut row7: Vec<Option<PIECE>> = Vec::new();
        for _ in 0..8 {
            row7.push(Some(PIECE::BLACK(TYPE::PAWN(true))));
        }
        pieces.push(row7);
        let mut row8: Vec<Option<PIECE>> = Vec::new();
        row8.push(Some(PIECE::BLACK(TYPE::ROOK)));
        row8.push(Some(PIECE::BLACK(TYPE::KNIGHT)));
        row8.push(Some(PIECE::BLACK(TYPE::BISHOP)));
        row8.push(Some(PIECE::BLACK(TYPE::QUEEN)));
        row8.push(Some(PIECE::BLACK(TYPE::KING)));
        row8.push(Some(PIECE::BLACK(TYPE::BISHOP)));
        row8.push(Some(PIECE::BLACK(TYPE::KNIGHT)));
        row8.push(Some(PIECE::BLACK(TYPE::ROOK)));
        pieces.push(row8);
        Board { pieces }
    }
}

impl Board {
    fn print(&self) {
        //unroll 2d vector into a 1d vector of strs
        let v: Vec<String> = self
            .pieces
            .iter()
            .flatten()
            .map(|opt| match opt {
                Some(piece) => format!("{}", piece),
                None => " ".to_string(),
            })
            .collect();

        let row1 = format!(
            "1 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #",
            v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]
        );
        let row2 = format!(
            "2 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #",
            v[8], v[9], v[10], v[11], v[12], v[13], v[14], v[15]
        );
        let row3 = format!(
            "3 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #",
            v[16], v[17], v[18], v[19], v[20], v[21], v[22], v[23]
        );
        let row4 = format!(
            "4 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #",
            v[24], v[25], v[26], v[27], v[28], v[29], v[30], v[31]
        );
        let row5 = format!(
            "5 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #",
            v[32], v[33], v[34], v[35], v[36], v[37], v[38], v[39]
        );
        let row6 = format!(
            "6 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #",
            v[40], v[41], v[42], v[43], v[44], v[45], v[46], v[47]
        );
        let row7 = format!(
            "7 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #",
            v[48], v[49], v[50], v[51], v[52], v[53], v[54], v[55]
        );
        let row8 = format!(
            "8 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #",
            v[56], v[57], v[58], v[59], v[60], v[61], v[62], v[63]
        );

        println!("     A     B     C     D     E     F     G     H   ");
        println!("  #################################################");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("{row8}");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("  #################################################");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("{row7}");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("  #################################################");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("{row6}");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("  #################################################");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("{row5}");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("  #################################################");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("{row4}");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("  #################################################");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("{row3}");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("  #################################################");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("{row2}");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("  #################################################");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("{row1}");
        println!("  #     #     #     #     #     #     #     #     #");
        println!("  #################################################");
    }

    pub fn play(&mut self) {
        let mut usr_input = String::new();
        //let args = Args::parse();

        Self::print(&self);

        while usr_input != "q" {
            usr_input.clear();
            println!("Your move: ");

            io::stdout().flush().expect("Cannot flush stdout");
            io::stdin()
                .read_line(&mut usr_input)
                .expect("failed to read user input");
            usr_input = usr_input
                .strip_suffix("\r\n")
                .or(usr_input.strip_suffix("\n"))
                .unwrap_or(&usr_input)
                .to_string();

            match validate_input(&usr_input) {
                Ok(usr_input) => {
                    let move_type: MoveType = parse_input(&usr_input).unwrap();
                    self.update_board(move_type).unwrap();
                    println!("success!");
                    self.print();
                }
                Err(err) => println!("error! {}", err),
            }

            //assert_ne!(usr_input, "q");
        }
        println!("Exiting program...");
    }

    // update the board with applied move
    pub fn update_board(&mut self, move_type: MoveType) -> Result<(), ()> {
        // special case: if rank 4 then do this. else do what's below for e5.
        // e4 empty?
        // e3 not empty? contains white pawn? => e3 -> e4, contains Some(PIECE) => invalid
        // e2 contains white pawn(never moved = true)? => e3 -> e4
        match move_type {
            MoveType::PawnPush((rank, file)) => self.pawn_push(rank, file),
            MoveType::Capture(move_struct) => self.capture(move_struct),
            _ => Err(()),
        }
    }

    pub fn pawn_push(&mut self, rank: usize, file: usize) -> Result<(), ()> {
        // if piece is occupied
        if let Some(piece) = &self.pieces[rank][file] {
            println!("space occupied...");
            Err(())
        } else {
            // if there is a piece 1 rank below
            if let Some(piece) = &self.pieces[rank - 1][file] {
                match piece {
                    //if it's a pawn
                    PIECE::WHITE(TYPE::PAWN(_)) => {
                        self.pieces[rank - 1][file] = None;
                        self.pieces[rank][file] = Some(PIECE::WHITE(TYPE::PAWN(false)));
                        return Ok(());
                    }
                    // if it's anything else
                    _ => {
                        println!("occupied below");
                        return Err(());
                    }
                }
            }
            // if there's a piece 2 ranks below
            if let Some(piece) = &self.pieces[rank - 2][file] {
                match piece {
                    PIECE::WHITE(TYPE::PAWN(can_double_move)) => {
                        if *can_double_move {
                            self.pieces[rank - 2][file] = None;
                            self.pieces[rank][file] = Some(PIECE::WHITE(TYPE::PAWN(false)));
                            return Ok(());
                        } else {
                            return Err(());
                        }
                    }
                    // is any other piece
                    _ => todo!(),
                }
            }
            Err(())
        }
    }

    pub fn capture(&mut self, move_struct: Move) -> Result<(), ()> {
        match move_struct.piece {
            PIECE::WHITE(TYPE::PAWN(_)) => {
                let (rank, file) = move_struct.coordinate;
                // check black piece exists at coord
                // if pawn, check white pawn exists at ( (rank - 1), (file +- 1)
                if let Some(PIECE::BLACK(_)) = self.pieces[rank][file] {
                    // is there a white pawn on {qualifier} file and rank -1
                    let attacker_file = file_to_index(&move_struct.file_qualifier.unwrap());
                    if let Some(PIECE::WHITE(TYPE::PAWN(_))) = &self.pieces[rank - 1][attacker_file]
                    {
                        self.pieces[rank][file] = (self.pieces[rank - 1][attacker_file]).clone();
                        self.pieces[rank - 1][attacker_file] = None;
                        Ok(())
                    } else {
                        Err(())
                    }
                } else {
                    Err(())
                }
            }
            _ => todo!(),
        }
    }
}

pub fn validate_input(usr_input: &str) -> Result<&str, &str> {
    //regex credit https://8bitclassroom.com/2020/08/16/chess-in-regex/
    let input = usr_input.to_owned() + " ";
    let re = Regex::new(
        r"[KQRBN]?[a-h]?[1-8]?x?[a-h][1-8](\=[QRBN])?[+#]? | [Oo0]-[Oo0]-[Oo0]|[Oo0]-[Oo0]",
    )
    .unwrap();
    assert!(re.is_match("d4 "));
    let Some(caps) = re.captures(&input) else {
        return Err("not a valid input");
    };
    println!("captured: {:?}", caps);
    Ok(usr_input)
}

pub fn parse_input(usr_input: &str) -> Result<MoveType, ParseMoveError> {
    /*4.Ba4 Nf6 5.O-O Be7 6.Re1 b5 7.Bb3 d6 8.c3 O-O 9.h3 Nb8 10.d4 Nbd7
    11.c4 c6 12.cxb5 axb5 13.Nc3 Bb7 14.Bg5 b4 15.Nb1 h6 16.Bh4 c5 17.dxe5
    Nxe4 18.Bxe7 Qxe7 19.exd6 Qf6 20.Nbd2 Nxd6 21.Nc4 Nxc4 22.Bxc4 Nb6
    23.Ne5 Rae8 24.Bxf7+ Rxf7 25.Nxf7 Rxe1+ 26.Qxe1 Kxf7 27.Qe3 Qg5 28.Qxg5
    hxg5 29.b3 Ke6 30.a3 Kd6 31.axb4 cxb4 32.Ra5 Nd5 33.f3 Bc8 34.Kf2 Bf5
    35.Ra7 g6 36.Ra6+ Kc5 37.Ke1 Nf4 38.g3 Nxh3 39.Kd2 Kb5 40.Rd6 Kc5 41.Ra6*/
    // trim spaces etc
    // castle?
    // pawn move? 2 chars
    // other piece move? 3 or 4 chars
    // capture?

    // pawn push
    if usr_input.len() == 2 {
        let mut it = usr_input.chars();
        Ok(MoveType::PawnPush(coordinate_to_index(
            &it.next().unwrap().to_string(),
            &it.next().unwrap().to_string(),
        )))
    }
    // capture
    else if usr_input.contains("x") {
        let mut it = usr_input.split("x");
        let piece_str: &str = it.next().unwrap();
        let mut coord_it = it.next().unwrap().chars();

        // has only piece or just a qualifier(pawn) eg. Nxd4 or exd4
        if piece_str.len() == 1 {
            if let Ok(piece) = piece_str.parse::<PIECE>() {
                Ok(MoveType::Capture(Move {
                    coordinate: coordinate_to_index(
                        &coord_it.next().unwrap().to_string(),
                        &coord_it.next().unwrap().to_string(),
                    ),
                    piece: piece,
                    file_qualifier: None,
                }))
            } else {
                // pawn capture
                Ok(MoveType::Capture(Move {
                    coordinate: coordinate_to_index(
                        &coord_it.next().unwrap().to_string(),
                        &coord_it.next().unwrap().to_string(),
                    ),
                    piece: PIECE::WHITE(TYPE::PAWN(false)),
                    file_qualifier: Some(piece_str.to_string()),
                }))
            }
        }
        // has a piece and qualifier
        else if piece_str.len() == 2 {
            let mut piece_it = piece_str.chars();
            Ok(MoveType::Capture(Move {
                coordinate: coordinate_to_index(
                    &coord_it.next().unwrap().to_string(),
                    &coord_it.next().unwrap().to_string(),
                ),
                piece: piece_it.nth(1).unwrap().to_string().parse().unwrap(),
                file_qualifier: Some(piece_it.nth(0).unwrap().to_string()),
            }))
        } else {
            Err(ParseMoveError)
        }
    }
    // move type not implemented yet
    else {
        todo!();
        //Err(ParseMoveError)
    }
}

enum MoveType {
    PawnPush((usize, usize)),
    Normal(Move),
    Capture(Move),
    ShortCastle,
    LongCastle,
}

#[derive(Debug)]
struct Move {
    // (file, rank) eg. (e, 4)
    coordinate: (usize, usize),
    //does it specify a type? if none it's a pawn
    piece: PIECE,
    // exd5 has file qualifier of "e"
    file_qualifier: Option<String>,
}
#[derive(Debug)]
struct ParseMoveError;

// chess coordinate to 2d vector index. NOTE: origin is bottom-left like a chess bboard
fn coordinate_to_index(file: &str, rank: &str) -> (usize, usize) {
    // a, b, c -> 0, 1, 2
    // 1, 2, 3 -> 0, 1, 2
    let y = file_to_index(file);

    let x: usize = rank.parse::<usize>().unwrap() - 1;

    (x, y)
}

fn file_to_index(file: &str) -> usize {
    let y: usize = match file {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        "f" => 5,
        "g" => 6,
        "h" => 7,
        _ => panic!(),
    };
    y
}

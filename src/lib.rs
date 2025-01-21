use regex::Regex;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Copy)]
enum Type {
    // has moved: bool
    Pawn(bool),
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

impl FromStr for Type {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "B" => Ok(Type::Bishop),
            "N" => Ok(Type::Knight),
            "R" => Ok(Type::Rook),
            "Q" => Ok(Type::Queen),
            "K" => Ok(Type::King),
            _ => Err(Error::ParsePiece),
        }
    }
}

//piece defined by colour and type
#[derive(Debug, Clone, PartialEq)]
enum Piece {
    Black(Type),
    White(Type),
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match &self {
            Piece::White(t) => match t {
                Type::Pawn(_) => "♟",
                Type::Bishop => "♝",
                Type::Knight => "♞",
                Type::Rook => "♜",
                Type::Queen => "♛",
                Type::King => "♚",
            },
            Piece::Black(t) => match t {
                Type::Pawn(_) => "♙",
                Type::Bishop => "♗",
                Type::Knight => "♘",
                Type::Rook => "♖",
                Type::Queen => "♕",
                Type::King => "♔",
            },
        };
        write!(f, "{}", s)
    }
}

pub enum Player {
    White,
    Black,
}

#[derive(Debug)]
pub enum Error {
    ParsePiece,
    InvalidInput,
    Movement(String),
    Capture(String),
}

#[allow(dead_code)]
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

    piece_type: Type,
    // exd5 has file qualifier of "e"
    file_qualifier: Option<String>,
}

//2d vector representation of the board

pub struct Board {
    pieces: Vec<Vec<Option<Piece>>>,
}

/*
pub struct Board2 {
    map: HashMap<Option<Piece>>
} */

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    pub fn new() -> Board {
        let mut pieces: Vec<Vec<Option<Piece>>> = Vec::new();
        // generate inital board state
        // start pushing from the bottom (white first so indexing will translate easily to board coordinates)
        let row1: Vec<Option<Piece>> = vec![
            Some(Piece::White(Type::Rook)),
            Some(Piece::White(Type::Knight)),
            Some(Piece::White(Type::Bishop)),
            Some(Piece::White(Type::Queen)),
            Some(Piece::White(Type::King)),
            Some(Piece::White(Type::Bishop)),
            Some(Piece::White(Type::Knight)),
            Some(Piece::White(Type::Rook)),
        ];
        pieces.push(row1);
        let mut row2: Vec<Option<Piece>> = Vec::new();
        for _ in 0..8 {
            row2.push(Some(Piece::White(Type::Pawn(true))));
        }
        pieces.push(row2);

        for _ in 0..4 {
            let mut empty_row: Vec<Option<Piece>> = Vec::new();
            for _ in 0..8 {
                empty_row.push(None);
            }
            pieces.push(empty_row);
        }
        let mut row7: Vec<Option<Piece>> = Vec::new();
        for _ in 0..8 {
            row7.push(Some(Piece::Black(Type::Pawn(true))));
        }
        pieces.push(row7);
        let row8: Vec<Option<Piece>> = vec![
            Some(Piece::Black(Type::Rook)),
            Some(Piece::Black(Type::Knight)),
            Some(Piece::Black(Type::Bishop)),
            Some(Piece::Black(Type::Queen)),
            Some(Piece::Black(Type::King)),
            Some(Piece::Black(Type::Bishop)),
            Some(Piece::Black(Type::Knight)),
            Some(Piece::Black(Type::Rook)),
        ];
        pieces.push(row8);
        Board { pieces }
    }
    // position startpos == rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
    pub fn new_from_fen() -> Board {
        todo!()
    }
}

impl Board {
    #[allow(unused)]
    pub fn export_fen(&self) {
        todo!()
    }
    #[allow(unused)]
    pub fn export_pgn(&self) {
        todo!()
    }

    pub fn print(&self) {
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

    pub fn play_move(&mut self, player: &Player, usr_input: &str) -> Result<(), Error> {
        match validate_input(usr_input) {
            Ok(usr_input) => {
                let move_type: MoveType = parse_input(usr_input)?;
                self.update_board(move_type, player)?;
                self.print();
                Ok(())
            }
            Err(err) => Err(err),
        }

        //assert_ne!(usr_input, "q");
    }

    // update the board by running relevant method
    fn update_board(&mut self, move_type: MoveType, player: &Player) -> Result<(), Error> {
        match move_type {
            MoveType::PawnPush((rank, file)) => self.pawn_push(rank, file, player),
            MoveType::Capture(move_struct) => self.capture(&move_struct, player),
            MoveType::Normal(move_struct) => self.normal_move(&move_struct, player),
            MoveType::LongCastle => self.long_castle(player),
            MoveType::ShortCastle => self.short_castle(player),
        }
    }

    fn pawn_push(&mut self, rank: usize, file: usize, player: &Player) -> Result<(), Error> {
        // if piece is occupied
        if self.pieces[rank][file].is_some() {
            Err(Error::Movement("space occupied...".to_string()))
        } else {
            match player {
                // White TO MOVE
                Player::White => {
                    // if there is a piece 1 rank below
                    if let Some(piece) = &self.pieces[rank - 1][file] {
                        match piece {
                            //if it's a pawn
                            Piece::White(Type::Pawn(_)) => {
                                self.pieces[rank - 1][file] = None;
                                self.pieces[rank][file] = Some(Piece::White(Type::Pawn(false)));
                                return Ok(());
                            }
                            // if it's anything else
                            _ => {
                                println!("occupied below");
                                return Err(Error::Movement("blocked path".to_string()));
                            }
                        }
                    }
                    // square below is clear, now check if there's a piece 2 ranks below
                    else if let Some(piece) = &self.pieces[rank - 2][file] {
                        match piece {
                            Piece::White(Type::Pawn(can_double_move)) => {
                                if *can_double_move {
                                    self.pieces[rank - 2][file] = None;
                                    self.pieces[rank][file] = Some(Piece::White(Type::Pawn(false)));
                                    return Ok(());
                                } else {
                                    return Err(Error::Movement(
                                        "Pawn cannot double move".to_string(),
                                    ));
                                }
                            }
                            // is any other piece
                            _ => return Err(Error::Movement("invalid pawn move".to_string())),
                        }
                    }
                }
                // Black TO MOVE
                Player::Black => {
                    // if there is a piece 1 rank ABOVE
                    if let Some(piece) = &self.pieces[rank + 1][file] {
                        match piece {
                            //if it's a pawn
                            Piece::Black(Type::Pawn(_)) => {
                                self.pieces[rank + 1][file] = None;
                                self.pieces[rank][file] = Some(Piece::Black(Type::Pawn(false)));
                                return Ok(());
                            }
                            // if it's anything else
                            _ => {
                                println!("occupied below");
                                return Err(Error::Movement("blocked path".to_string()));
                            }
                        }
                    }
                    // square below is clear, now check if there's a piece 2 ranks ABOVE
                    else if let Some(piece) = &self.pieces[rank + 2][file] {
                        match piece {
                            Piece::Black(Type::Pawn(can_double_move)) => {
                                if *can_double_move {
                                    self.pieces[rank + 2][file] = None;
                                    self.pieces[rank][file] = Some(Piece::Black(Type::Pawn(false)));
                                    return Ok(());
                                } else {
                                    return Err(Error::Movement(
                                        "Pawn cannot double move".to_string(),
                                    ));
                                }
                            }
                            // is any other piece
                            _ => return Err(Error::Movement("invalid pawn move".to_string())),
                        }
                    }
                }
            }

            Err(Error::Movement("generic pawn push error".to_string()))
        }
    }

    fn normal_move(&mut self, move_struct: &Move, player: &Player) -> Result<(), Error> {
        /*
        bishop, check diagonals
        rook, check right angles
        knight, no need to checks Ls as can jump
        king,
        queen == rook + bishop
        */
        let (rank, file) = move_struct.coordinate;
        if self.pieces[rank][file].is_some() {
            return Err(Error::Movement(
                "space is occupied! Maybe try capture notation instead".to_string(),
            ));
        }

        match move_struct.piece_type {
            Type::Bishop => {
                let (from_rank, from_file) = self.check_diagonals(move_struct, player)?;
                self.pieces[rank][file] = self.pieces[from_rank][from_file].clone();
                self.pieces[from_rank][from_file] = None;
                Ok(())
            }
            Type::Knight => {
                let (from_rank, from_file) = self.check_knight_lines(move_struct, player)?;
                self.pieces[rank][file] = self.pieces[from_rank][from_file].clone();
                self.pieces[from_rank][from_file] = None;
                Ok(())
            }
            Type::Rook => {
                let (from_rank, from_file) = self.check_straight_lines(move_struct, player)?;
                self.pieces[rank][file] = self.pieces[from_rank][from_file].clone();
                self.pieces[from_rank][from_file] = None;
                Ok(())
            }
            Type::Queen => {
                // diagonals and straight lines
                if let Ok((diagonal_rank, diagonal_file)) =
                    self.check_diagonals(move_struct, player)
                {
                    //found a match on diagonal
                    self.pieces[rank][file] = self.pieces[diagonal_rank][diagonal_file].clone();
                    self.pieces[diagonal_rank][diagonal_file] = None;
                } else {
                    let (straight_rank, straight_file) =
                        self.check_straight_lines(move_struct, player)?;
                    self.pieces[rank][file] = self.pieces[straight_rank][straight_file].clone();
                    self.pieces[straight_rank][straight_file] = None;
                }
                Ok(())
            }
            Type::King => {
                let (from_rank, from_file) = self.check_surrounding_squares(move_struct, player)?;
                self.pieces[rank][file] = self.pieces[from_rank][from_file].clone();
                self.pieces[from_rank][from_file] = None;
                Ok(())
            }
            Type::Pawn(_) => {
                panic!();
            }
        }
    }

    fn capture(&mut self, move_struct: &Move, player: &Player) -> Result<(), Error> {
        let (rank, file) = move_struct.coordinate;
        let piece = self.pieces[rank][file].clone();

        if piece.is_none() {
            return Err(Error::Capture("cannot capture empty square".to_string()));
        } else {
            match player {
                Player::White => {
                    if let Some(Piece::White(_)) = piece {
                        return Err(Error::Capture("cannot capture your own piece".to_string()));
                    }
                }
                Player::Black => {
                    if let Some(Piece::Black(_)) = piece {
                        return Err(Error::Capture("cannot capture your own piece".to_string()));
                    }
                }
            }
        }

        match move_struct.piece_type {
            Type::Pawn(_) => {
                // check black piece exists at coord
                // if pawn, check pawn exists at ( (rank - 1), (file +- 1)

                // is there a white pawn on {qualifier} file and rank -1
                let attacker_file = file_to_index(move_struct.file_qualifier.as_ref().unwrap());
                println!("attacker file = {}", attacker_file);

                // different colours move pawns in opposite directions
                // calculate the rank offset (-1, 1) allowed for pawn capture. if already rank 0 then cannot check below that for valid attacks therefor no valid capture
                let from_rank: usize = match player {
                    Player::White => rank - 1,
                    Player::Black => rank + 1,
                };
                match self.pieces[from_rank][attacker_file] {
                    Some(Piece::White(Type::Pawn(_))) | Some(Piece::Black(Type::Pawn(_))) => {
                        self.pieces[rank][file] = (self.pieces[from_rank][attacker_file]).clone();
                        self.pieces[from_rank][attacker_file] = None;
                        Ok(())
                    }
                    None => Err(Error::Capture(
                        "no pawn capable of capture found".to_string(),
                    )),
                    _ => Err(Error::Capture("pawn capture error".to_string())),
                }
            }
            Type::Bishop => {
                let (rank_0, file_0) = self.check_diagonals(move_struct, player)?;
                self.pieces[rank][file] = self.pieces[rank_0][file_0].clone();
                self.pieces[rank_0][file_0] = None;

                Ok(())
            }
            Type::Rook => {
                let (rank_0, file_0) = self.check_straight_lines(move_struct, player)?;
                self.pieces[rank][file] = self.pieces[rank_0][file_0].clone();
                self.pieces[rank_0][file_0] = None;

                Ok(())
            }
            Type::Knight => {
                let (rank_0, file_0) = self.check_knight_lines(move_struct, player)?;
                self.pieces[rank][file] = self.pieces[rank_0][file_0].clone();
                self.pieces[rank_0][file_0] = None;

                Ok(())
            }
            Type::Queen => {
                if let Ok((rank_0, file_0)) = self.check_straight_lines(move_struct, player) {
                    self.pieces[rank][file] = self.pieces[rank_0][file_0].clone();
                    self.pieces[rank_0][file_0] = None;
                } else {
                    let (rank_0, file_0) = self.check_diagonals(move_struct, player)?;
                    self.pieces[rank][file] = self.pieces[rank_0][file_0].clone();
                    self.pieces[rank_0][file_0] = None;
                }
                Ok(())
            }
            Type::King => {
                let (from_rank, from_file) = self.check_surrounding_squares(move_struct, player)?;
                self.pieces[rank][file] = self.pieces[from_rank][from_file].clone();
                self.pieces[from_rank][from_rank] = None;

                Ok(())
            }
            _ => todo!(),
        }
    }

    fn short_castle(&mut self, player: &Player) -> Result<(), Error> {
        todo!()
    }

    fn long_castle(&mut self, player: &Player) -> Result<(), Error> {
        todo!()
    }

    fn pawn_promotion(&mut self, player: &Player) -> Result<(), Error> {
        todo!()
    }

    // checks for clear diagonal path between locaton and destination
    // returns the location of the piece to move on success
    fn check_diagonals(
        &self,
        move_struct: &Move,
        player: &Player,
    ) -> Result<(usize, usize), Error> {
        // search in all diagonal free spaces until the right piece is found
        let (rank, file) = move_struct.coordinate;
        let target_piece = match player {
            Player::White => Piece::White(move_struct.piece_type),
            Player::Black => Piece::Black(move_struct.piece_type),
        };

        if move_struct.file_qualifier.is_some() {
            panic!()
        };

        let range = 0..=7;

        let offset_array: [i8; 2] = [1, -1];
        for rank_offset in offset_array {
            for file_offset in offset_array {
                let mut search_rank: i8 = rank as i8 + rank_offset;
                let mut search_file: i8 = file as i8 + file_offset;
                while range.contains(&search_rank) && range.contains(&search_file) {
                    let piece = &self.pieces[search_rank as usize][search_file as usize];
                    if piece.is_none() {
                        search_rank += rank_offset;
                        search_file += file_offset;
                        continue;
                    } else if piece.as_ref().unwrap() == &target_piece {
                        return Ok((search_rank as usize, search_file as usize));
                    } else {
                        break;
                    }
                }
            }
        }
        Err(Error::Movement("error checking diagonals".to_string()))
    }

    fn check_straight_lines(
        &self,
        move_struct: &Move,
        player: &Player,
    ) -> Result<(usize, usize), Error> {
        // currently copy paste of diagnols with different offset array used
        let (rank, file) = move_struct.coordinate;
        let target_piece = match player {
            Player::White => Piece::White(move_struct.piece_type),
            Player::Black => Piece::Black(move_struct.piece_type),
        };

        if move_struct.file_qualifier.is_some() {
            todo!()
        };

        let range = 0..=7;

        let offset_array: [(i8, i8); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

        // 01, 10, -10, 0-1
        for (rank_offset, file_offset) in offset_array {
            let mut search_rank: i8 = rank as i8 + rank_offset;
            let mut search_file: i8 = file as i8 + file_offset;
            while range.contains(&search_rank) && range.contains(&search_file) {
                let piece = &self.pieces[search_rank as usize][search_file as usize];
                if piece.is_none() {
                    search_rank += rank_offset;
                    search_file += file_offset;
                    continue;
                } else if piece.as_ref().unwrap() == &target_piece {
                    return Ok((search_rank as usize, search_file as usize));
                } else {
                    break;
                }
            }
        }
        Err(Error::Movement("error checking straight lines".to_string()))
    }

    fn check_knight_lines(
        &self,
        move_struct: &Move,
        player: &Player,
    ) -> Result<(usize, usize), Error> {
        let (rank, file) = move_struct.coordinate;
        let target_piece = match player {
            Player::White => Piece::White(move_struct.piece_type),
            Player::Black => Piece::Black(move_struct.piece_type),
        };

        if move_struct.file_qualifier.is_some() {
            panic!()
        };

        let range = 0..=7;

        let offset_array: [(i8, i8); 8] = [
            (1, 2),
            (1, -2),
            (-1, 2),
            (-1, -2),
            (-2, 1),
            (-2, -1),
            (2, 1),
            (2, -1),
        ];

        // 21, 12, -12, 1-2, -21, -2-1, -1-2, -12,

        for (rank_offset, file_offset) in offset_array {
            let search_rank: i8 = rank as i8 + rank_offset;
            let search_file: i8 = file as i8 + file_offset;

            if range.contains(&search_rank) && range.contains(&search_file) {
                let piece = &self.pieces[search_rank as usize][search_file as usize];
                if piece.is_none() {
                    continue;
                } else if piece.as_ref().unwrap() == &target_piece {
                    return Ok((search_rank as usize, search_file as usize));
                }
            }
        }
        Err(Error::Movement("error checking knight line".to_string()))
    }
    fn check_surrounding_squares(
        &mut self,
        move_struct: &Move,
        player: &Player,
    ) -> Result<(usize, usize), Error> {
        let (rank, file) = move_struct.coordinate;
        let target_piece = match player {
            Player::White => Piece::White(move_struct.piece_type),
            Player::Black => Piece::Black(move_struct.piece_type),
        };

        if move_struct.file_qualifier.is_some() {
            panic!()
        };
        let range = 0..=7;
        let offsets = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
        ];

        for (file_offset, rank_offset) in offsets {
            let search_rank: i8 = rank as i8 + rank_offset;
            let search_file: i8 = file as i8 + file_offset;

            if range.contains(&search_rank) && range.contains(&search_file) {
                let piece = &self.pieces[search_rank as usize][search_file as usize];
                if piece.is_none() {
                    continue;
                } else if piece.as_ref().unwrap() == &target_piece {
                    return Ok((search_rank as usize, search_file as usize));
                }
            }
        }

        Err(Error::Movement(
            "error checking king surrounding squares".to_string(),
        ))
    }
}

// checks that input is valid using regex (may not be a valid move in game but is valid notiation)
fn validate_input(usr_input: &str) -> Result<&str, Error> {
    // regex help credit https://8bitclassroom.com/2020/08/16/chess-in-regex/
    let input = usr_input.to_owned() + " ";
    let re = Regex::new(
        r"[KQRBN]?[a-h]?[1-8]?x?[a-h][1-8](\=[QRBN])?[+#]? | [Oo0]-[Oo0]-[Oo0]|[Oo0]-[Oo0]",
    )
    .unwrap();
    assert!(re.is_match("d4 "));
    let Some(caps) = re.captures(&input) else {
        return Err(Error::InvalidInput);
    };
    println!("captured: {:?}", caps);
    Ok(usr_input)
}

// converts validated input to a MoveType
fn parse_input(usr_input: &str) -> Result<MoveType, Error> {
    /*4.Ba4 Nf6 5.O-O Be7 6.Re1 b5 7.Bb3 d6 8.c3 O-O 9.h3 Nb8 10.d4 Nbd7
    11.c4 c6 12.cxb5 axb5 13.Nc3 Bb7 14.Bg5 b4 15.Nb1 h6 16.Bh4 c5 17.dxe5
    Nxe4 18.Bxe7 Qxe7 19.exd6 Qf6 20.Nbd2 Nxd6 21.Nc4 Nxc4 22.Bxc4 Nb6
    23.Ne5 Rae8 24.Bxf7+ Rxf7 25.Nxf7 Rxe1+ 26.Qxe1 Kxf7 27.Qe3 Qg5 28.Qxg5
    hxg5 29.b3 Ke6 30.a3 Kd6 31.axb4 cxb4 32.Ra5 Nd5 33.f3 Bc8 34.Kf2 Bf5
    35.Ra7 g6 36.Ra6+ Kc5 37.Ke1 Nf4 38.g3 Nxh3 39.Kd2 Kb5 40.Rd6 Kc5 41.Ra6*/

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
            if let Ok(piece_type) = piece_str.parse::<Type>() {
                Ok(MoveType::Capture(Move {
                    coordinate: coordinate_to_index(
                        &coord_it.next().unwrap().to_string(),
                        &coord_it.next().unwrap().to_string(),
                    ),
                    piece_type,
                    file_qualifier: None,
                }))
                //if it cannot be parsed into a type it is likely a pawn capture
            } else {
                // pawn capture
                Ok(MoveType::Capture(Move {
                    coordinate: coordinate_to_index(
                        &coord_it.next().unwrap().to_string(),
                        &coord_it.next().unwrap().to_string(),
                    ),
                    piece_type: Type::Pawn(false),
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
                piece_type: piece_it.nth(1).unwrap().to_string().parse().unwrap(),
                file_qualifier: Some(piece_it.next().unwrap().to_string()),
            }))
        } else {
            Err(Error::ParsePiece)
        }
    }
    // non pawn movement
    else if usr_input.len() == 3 {
        let mut it = usr_input.chars();
        let piece_type: Type = it.next().unwrap().to_string().parse()?;

        Ok(MoveType::Normal(Move {
            coordinate: coordinate_to_index(
                &it.next().unwrap().to_string(),
                &it.next().unwrap().to_string(),
            ),
            piece_type,
            file_qualifier: None,
        }))
    } else {
        todo!();
    }
}

// helper functions

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
        _ => todo!(),
    };
    y
}

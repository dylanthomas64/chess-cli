use std::fmt;
use regex::Regex;
use std::io;
use std::io::Write;

enum TYPE {
    PAWN,
    BISHOP,
    KNIGHT,
    ROOK,
    QUEEN,
    KING,
}

//piece defined by colour and type
enum PIECE {
    BLACK(TYPE),
    WHITE(TYPE),
}

impl fmt::Display for PIECE {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let s = match &self {
            PIECE::WHITE(t) => match t {
                TYPE::PAWN => "♟",
                TYPE::BISHOP => "♝",
                TYPE::KNIGHT => "♞",
                TYPE::ROOK => "♜",
                TYPE::QUEEN => "♛",
                TYPE::KING => "♚",
            },
            PIECE::BLACK(t) => match t {
                TYPE::PAWN => "♙",
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
            row2.push(Some(PIECE::WHITE(TYPE::PAWN)));
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
            row7.push(Some(PIECE::BLACK(TYPE::PAWN)));
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
        Board {pieces}
    }
}




impl Board {
    fn print(board: &Self) {
        //unroll 2d vector into a 1d vector of strs
        let v: Vec<String> = board.pieces.iter().flatten()
        .map(|opt| match opt {
            Some(piece) => format!("{}", piece),
            None => " ".to_string(),
        }).collect();
    
        let row1 = format!("1 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7]);
        let row2 = format!("2 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[8], v[9], v[10], v[11], v[12], v[13], v[14], v[15]);
        let row3 = format!("3 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[16], v[17], v[18], v[19], v[20], v[21], v[22], v[23]);
        let row4 = format!("4 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[24], v[25], v[26], v[27], v[28], v[29], v[30], v[31]);
        let row5 = format!("5 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[32], v[33], v[34], v[35], v[36], v[37], v[38], v[39]);
        let row6 = format!("6 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[40], v[41], v[42], v[43], v[44], v[45], v[46], v[47]);
        let row7 = format!("7 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[48], v[49], v[50], v[51], v[52], v[53], v[54], v[55]);
        let row8 = format!("8 #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #  {}  #", v[56], v[57], v[58], v[59], v[60], v[61], v[62], v[63]);
    
    
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

    pub fn play(board: &Self) {

        let mut usr_input = String::new();
        //let args = Args::parse();
        
        Self::print(&board);

        while usr_input != "q" {
            usr_input.clear();
            println!("Your move: ");
    
            io::stdout().flush().expect("Cannot flush stdout");
            io::stdin().read_line(&mut usr_input).expect("failed to read user input");
            usr_input = usr_input.strip_suffix("\r\n").or(usr_input.strip_suffix("\n")).unwrap_or(&usr_input).to_string();
            
            match validate_input(&usr_input) {
                Ok(usr_input) => println!("you moved: {}", usr_input),
                Err(err) => println!("error! {}", err),
            }
            
            
            
            //assert_ne!(usr_input, "q");
            
        }
        println!("Exiting program...");
    }
}


fn validate_input(usr_input: &str) -> Result<&str, &str>{
    //regex credit https://8bitclassroom.com/2020/08/16/chess-in-regex/
    let input = usr_input.to_owned() + " ";
    let re = Regex::new(
    r"[KQRBN]?[a-h]?[1-8]?x?[a-h][1-8](\=[QRBN])?[+#]? | [Oo0]-[Oo0]-[Oo0]|[Oo0]-[Oo0]").unwrap();
    assert!(re.is_match("d4 "));
    let Some(caps) = re.captures(&input)
    else {
        return Err("not a valid input")
    };
    println!("captured: {:?}", caps);
    Ok(usr_input)
}

fn parse_input(usr_input: &str) -> Result<&str, &str> {
    /*4.Ba4 Nf6 5.O-O Be7 6.Re1 b5 7.Bb3 d6 8.c3 O-O 9.h3 Nb8 10.d4 Nbd7
11.c4 c6 12.cxb5 axb5 13.Nc3 Bb7 14.Bg5 b4 15.Nb1 h6 16.Bh4 c5 17.dxe5
Nxe4 18.Bxe7 Qxe7 19.exd6 Qf6 20.Nbd2 Nxd6 21.Nc4 Nxc4 22.Bxc4 Nb6
23.Ne5 Rae8 24.Bxf7+ Rxf7 25.Nxf7 Rxe1+ 26.Qxe1 Kxf7 27.Qe3 Qg5 28.Qxg5
hxg5 29.b3 Ke6 30.a3 Kd6 31.axb4 cxb4 32.Ra5 Nd5 33.f3 Bc8 34.Kf2 Bf5
35.Ra7 g6 36.Ra6+ Kc5 37.Ke1 Nf4 38.g3 Nxh3 39.Kd2 Kb5 40.Rd6 Kc5 41.Ra6*/
}
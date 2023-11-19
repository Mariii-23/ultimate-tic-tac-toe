use std::error::Error;
use std::io;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Empty,
    Draw,
    Player1,
    Player2
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Board{
    piece: [State; 9],
    state: State
}

impl Board {
    pub fn  build() -> Board {
        Board{
            piece: [State::Empty; 9],
            state: State::Empty
        }
    }

    fn can_play_board(&self) -> bool {
        self.state == State::Empty
    }

    fn play_move(&mut self, player1: bool, position: usize) -> Result<(), Box<dyn Error>> {
        if valide_position(position) {
            if self.can_play_board() {
                if self.piece[position] == State::Empty {
                    match player1 {
                        true => self.piece[position] = State::Player1,
                        false => self.piece[position] = State::Player2,
                    };
                    return Ok(());
                }
                return Err("Space is not null".into());
            }
            return Err("State is not null".into());
        }

        Err("Position Invalid".into())
    }

    fn update_state(&mut self) -> bool {
        // Check rows and columns
        for i in 0..3 {
            // Check rows
            if self.piece[i + 3] != State::Empty && self.piece[i] == self.piece[i + 3] && self.piece[i + 3] == self.piece[i + 6] { 
                self.state = self.piece[i];
                return true;
            }

            // Check columns
            if self.piece[i * 3 + 1] != State::Empty &&  self.piece[i * 3] == self.piece[i * 3 + 1] && self.piece[i * 3 + 1] == self.piece[i * 3 + 2] {
                self.state = self.piece[i * 3];
                return true;
            }
        }

        // Check diagonals
        if  self.piece[4] != State::Empty && (
            self.piece[0] == self.piece[4] && self.piece[4] == self.piece[8])
            || (self.piece[2] == self.piece[4] && self.piece[4] == self.piece[6])
        {
            self.state = self.piece[4];
            return true;
        }

        // Check draw
        if self.piece.iter().all(|&cell| cell != State::Empty) {
            self.state = State::Draw;
            return true;
        }

        false
    }
}

struct JogoGalo{
    board: [Board; 9],
    state: State,
    last_position: Option<usize>,
    // if true is player 1, else player 2
    player_turn: bool
}
impl JogoGalo {
   pub fn build() -> JogoGalo{
    JogoGalo { board: [Board::build(); 9], state: State::Empty, player_turn: true, last_position: None}
   }

   fn play_move(&mut self, tab: usize, position: usize) -> Result<(), Box<dyn Error>> {
    let player1 = self.player_turn;
    if valide_position(tab) {
        if self.board[tab].can_play_board() &&
            self.last_position.is_some() && self.last_position.unwrap() != tab
             {
                return Err("Wrong tab".into());
        }

        match self.board[tab].play_move(player1, position) {
           Ok(_) => {
                self.player_turn = !self.player_turn;

                // Update the board status
                if self.board[tab].update_state() {
                    // Update the global board status
                     self.update_state();
                } 

                // Dar update last position
                if self.board[position].state == State::Player1 || self.board[position].state == State::Player2 {
                    self.last_position = None;
                } else {
                    self.last_position = Some(position);
                }

              return Ok(()); 
           },
           Err(e) => {
               return Err(e);
           }
        }
    }
    Err("Board Invalid".into())
   }


    fn update_state(&mut self) -> bool {
        // Check rows and columns
        for i in 0..3 {
            // Check rows 
            if self.board[i + 3].state != State::Empty && self.board[i].state == self.board[i + 3].state && self.board[i + 3].state == self.board[i + 6].state { 
                self.state = self.board[i].state;
                return true;
            }

            // Check columns
            if self.board[i * 3 + 1].state != State::Empty && self.board[i * 3].state == self.board[i * 3 + 1].state && self.board[i * 3 + 1].state == self.board[i * 3 + 2].state {
                self.state = self.board[i * 3].state;
                return true;
            }
        }

        // Check diagonals
        if  self.board[4].state != State::Empty && (
            self.board[0].state == self.board[4].state && self.board[4].state == self.board[8].state)
            || (self.board[2].state == self.board[4].state && self.board[4].state == self.board[6].state)
        {
            self.state = self.board[4].state;
            return true;
        }

        // Check draw
        if self.board.iter().all(|&cell| cell.state != State::Empty) {
            self.state = State::Draw;
            return true;
        }

        false
    }

    fn run(&mut self) {
        while self.state == State::Empty {
           println!();
           self.print_board(); 

           let mut input = String::new();
           print!("Move = ");
           use std::io::Write;
           io::stdout().flush().unwrap();
           io::stdin()
               .read_line(&mut input)
               .expect("Error reading the input");

           match separate_numbers(&input) {
               Some((tab, position)) => {
                    //TODO: Explain errors better
                    if let Err(e) = self.play_move(tab - 1, position - 1) {
                        println!("{}",e);
                    }
               }
               None => println!("Wrong Input."),
           }
           println!();
        }
        //TODO:
        //println!("Winner :: {}", self.state);
    }

    fn print_global_board(&self) {
        for i in 0..3 {
            print!("                     ");
            for j in 0..3 {
                match self.board[i * 3 + j].state {
                    State::Player1 => print!("X "),
                    State::Player2 => print!("O "),
                    State::Draw => print!("- "),
                    _ => print!("  "),
                }

                if j != 2 {
                    print!("| ");
                }
            }
            println!();
            if i != 2 {
                print!("                     ");
                println!("---------");
            }
        }
    }

    fn print_all_boards(&self) {
        for i in 0..3 {
            for a in 0..3 {

                print!("   ");
                for j in 0..3 {
                    for b in 0..3 {
                        match self.board[i * 3 + j].piece[a * 3 + b ] {
                            State::Player1 => print!("X "),
                            State::Player2 => print!("O "),
                            _ => {
                                if self.board[i * 3 + j].state != State::Empty {
                                    print!("  ")
                                } else {
                                    print!("* ")
                                }
                            },
                        }

                        if b != 2 {
                            print!("| ");
                        } else {
                            if j!= 2{
                                print!("   ||   ");
                            }
                        }

                    }
                }

                println!();
                if a != 2 {
                    println!("   ---------   ||   ---------    ||   ---------");
                }
            }
            println!("               ||                ||");
            if i != 2 {
                println!("----------------------------------------------------");
                println!("----------------------------------------------------");
                println!("               ||                ||");
            }
        }
    }

    fn print_board(&self) {
        let last_play = if self.last_position.is_none() {
            "-".to_string()
        } else {
            (self.last_position.unwrap() + 1).to_string()
        };

        let next_player = if self.player_turn {"Player 1"} else {"Player 2"};

        self.print_global_board();
        println!();
        println!("Next Player: {}", next_player);
        println!("Last Play: {}", last_play);
        println!();
        self.print_all_boards();
    }
}

fn valide_position(position: usize) -> bool {
    //position >= 0 && 
    position <= 8
}

fn separate_numbers(input: &str) -> Option<(usize, usize)> {
    let partes: Vec<&str> = input.trim().split("-").collect();
    if partes.len() == 2 {
        if let Ok(num1) = partes[0].parse::<usize>() {
            if let Ok(num2) = partes[1].parse::<usize>() {
                return Some((num1, num2));
            }
        }
    }
    None
}

fn main() {
    let mut jogo_galo = JogoGalo::build();
    jogo_galo.run();
    jogo_galo.print_global_board();
}

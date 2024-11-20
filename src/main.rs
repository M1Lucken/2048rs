use rand::prelude::*;
use cursive::{
    Cursive,
    views::{Dialog, TextView},
    view::Nameable,
    event::Key,
};

#[derive(Clone, Copy)]
enum GameDirection {
    Up,
    Down,
    Left,
    Right,
}

struct Game {
    board: [[u32; 4]; 4],
    score: u32,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            board: [[0; 4]; 4],
            score: 0,
        };
        game.add_random_tile();
        game.add_random_tile();
        game
    }

    fn add_random_tile(&mut self) {
        let mut available = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                if self.board[i][j] == 0 {
                    available.push((i, j));
                }
            }
        }
        if let Some(&(i, j)) = available.choose(&mut rand::thread_rng()) {
            self.board[i][j] = if random::<f32>() < 0.9 { 2 } else { 4 };
        }
    }

    fn move_tiles(&mut self, direction: GameDirection) -> bool {
        let mut moved = false;
        match direction {
            GameDirection::Up => {
                for j in 0..4 {
                    for i in 1..4 {
                        if self.board[i][j] != 0 {
                            let mut row = i;
                            while row > 0 && self.board[row-1][j] == 0 {
                                self.board[row-1][j] = self.board[row][j];
                                self.board[row][j] = 0;
                                row -= 1;
                                moved = true;
                            }
                            if row > 0 && self.board[row-1][j] == self.board[row][j] {
                                self.board[row-1][j] *= 2;
                                self.score += self.board[row-1][j];
                                self.board[row][j] = 0;
                                moved = true;
                            }
                        }
                    }
                }
            },
            GameDirection::Down => {
                for j in 0..4 {
                    for i in (0..3).rev() {
                        if self.board[i][j] != 0 {
                            let mut row = i;
                            while row < 3 && self.board[row+1][j] == 0 {
                                self.board[row+1][j] = self.board[row][j];
                                self.board[row][j] = 0;
                                row += 1;
                                moved = true;
                            }
                            if row < 3 && self.board[row+1][j] == self.board[row][j] {
                                self.board[row+1][j] *= 2;
                                self.score += self.board[row+1][j];
                                self.board[row][j] = 0;
                                moved = true;
                            }
                        }
                    }
                }
            },
            GameDirection::Left => {
                for i in 0..4 {
                    for j in 1..4 {
                        if self.board[i][j] != 0 {
                            let mut col = j;
                            while col > 0 && self.board[i][col-1] == 0 {
                                self.board[i][col-1] = self.board[i][col];
                                self.board[i][col] = 0;
                                col -= 1;
                                moved = true;
                            }
                            if col > 0 && self.board[i][col-1] == self.board[i][col] {
                                self.board[i][col-1] *= 2;
                                self.score += self.board[i][col-1];
                                self.board[i][col] = 0;
                                moved = true;
                            }
                        }
                    }
                }
            },
            GameDirection::Right => {
                for i in 0..4 {
                    for j in (0..3).rev() {
                        if self.board[i][j] != 0 {
                            let mut col = j;
                            while col < 3 && self.board[i][col+1] == 0 {
                                self.board[i][col+1] = self.board[i][col];
                                self.board[i][col] = 0;
                                col += 1;
                                moved = true;
                            }
                            if col < 3 && self.board[i][col+1] == self.board[i][col] {
                                self.board[i][col+1] *= 2;
                                self.score += self.board[i][col+1];
                                self.board[i][col] = 0;
                                moved = true;
                            }
                        }
                    }
                }
            }
        }
        moved
    }
}

fn handle_move(s: &mut Cursive, dir: GameDirection) {
    let game_data = s.with_user_data(|game: &mut Game| {
        if game.move_tiles(dir) {
            game.add_random_tile();
        }
        draw_board(&game)
    });

    if let Some(board_string) = game_data {
        s.call_on_name("game_view", |view: &mut TextView| {
            view.set_content(board_string);
        });
    }
}

fn draw_board(game: &Game) -> String {
    let mut output = String::new();
    for row in &game.board {
        output.push_str("┌──────┐ ".repeat(4).as_str());
        output.push('\n');
        for &cell in row {
            let num = if cell == 0 {
                String::from(" ")
            } else {
                cell.to_string()
            };
            output.push_str(&format!("│ {:^4} │ ", num));
        }
        output.push('\n');
        output.push_str("└──────┘ ".repeat(4).as_str());
        output.push('\n');
    }
    output
}

fn main() {
    let mut siv = cursive::default();
    let game = Game::new();

    let game_view = TextView::new(draw_board(&game))
        .with_name("game_view");
    
    siv.set_user_data(game);
    
    siv.add_layer(
        Dialog::around(game_view)
            .title("2048")
            .button("Quit", |s| s.quit())
    );

    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(Key::Up, |s| handle_move(s, GameDirection::Up));
    siv.add_global_callback(Key::Down, |s| handle_move(s, GameDirection::Down));
    siv.add_global_callback(Key::Left, |s| handle_move(s, GameDirection::Left));
    siv.add_global_callback(Key::Right, |s| handle_move(s, GameDirection::Right));

    siv.run();
}

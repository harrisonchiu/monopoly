use board::BoardDisplay;
use player::{Avatar, Player};

use board_tiles::tile::Tile;
use std::io::Write;

pub struct Terminal {
    board_display: BoardDisplay,
    player_coords: Vec<(usize, usize)>,
    player_avatars: Vec<Avatar>,
}

impl Terminal {
    const PADDING_TOP: i8 = 1;
    const PADDING_LEFT: i8 = 0;

    pub fn new(board: &Vec<Tile>) -> Self {
        let board_display = BoardDisplay::new(board);

        let mut player_avatars: Vec<Avatar> = Vec::with_capacity(4);
        for id in 0..4 {
            player_avatars.push(Avatar::new(id));
        }

        Self {
            player_coords: board_display.player_coords.clone(),
            player_avatars,
            board_display,
        }
    }

    pub fn move_cursor<N: std::ops::Add<Output = N> + std::fmt::Display>(row: N, col: N) {
        print!("\x1B[{row};{col}H");
    }

    pub fn clear_display() {
        //! Clears the screen and puts the cursor at the very beginning (top left)
        //! to redraw otherwise it will redraw where it finished clearing previous texts
        print!("\x1B[2J");
        Self::move_cursor(1, 1);
    }

    pub fn display_board(&self) {
        Self::clear_display();
        Self::move_cursor(Self::PADDING_TOP, Self::PADDING_LEFT);
        print!("{}", self.board_display.ascii_board);
    }

    pub fn prompt(&self, player_id: i8) -> Option<String> {
        Self::move_cursor(
            self.board_display.ascii_board_size.1 as i8 + Self::PADDING_TOP + 1,
            Self::PADDING_LEFT,
        );
        print!(
            "[Player {}] >>> \x1b[0K", // Clears previous user input
            self.player_avatars[player_id as usize]
        );
        std::io::stdout().flush().unwrap();

        let mut input_text = String::new();
        std::io::stdin().read_line(&mut input_text).ok();
        Some(input_text.trim().to_owned())
    }

    pub fn update_tile_details(&mut self, board: &[Tile], tile: usize) {
        self.board_display
            .update_tile_details(board, tile..(tile + 1));
    }

    pub fn display_player(&mut self, player: &Player) {
        Self::move_cursor(
            self.player_coords[player.last_position].0 as i8 + Self::PADDING_TOP,
            self.player_coords[player.last_position].1 as i8 + Self::PADDING_LEFT + (2 * player.id),
        );
        print!(" ");

        Self::move_cursor(
            self.player_coords[player.position].0 as i8 + Self::PADDING_TOP,
            self.player_coords[player.position].1 as i8 + Self::PADDING_LEFT + (2 * player.id),
        );
        print!("{}", player.avatar);
    }
}

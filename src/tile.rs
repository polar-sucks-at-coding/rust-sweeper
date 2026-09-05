use crate::consts;
use crate::board;

#[derive(PartialEq)]
pub enum TileType{
    Bomb,
    Empty
}

#[derive(PartialEq)]
pub enum ClickResult{
    Explode,
    Safe
}

pub struct Tile{
    pub position: (usize, usize), // row/y first
    pub tile_type: TileType,
    pub concealed: bool,
    pub symbol: char,
}

impl Tile{
    pub fn get_symbol(&self) -> &char{
        &self.symbol
    }

    pub fn get_type(&self) -> &TileType{
        &self.tile_type
    }

    pub fn calculate_symbol(&self, board: &mut board::Board) -> char {
        if self.get_type() == &TileType::Bomb {
            return consts::BOMB_TILE_SYMBOL;
        }

        match board.get_surrounding_bomb_count(self) {
            Some(count) => std::char::from_digit(count as u32, 10)
                .unwrap_or(consts::EMPTY_TILE_SYMBOL),
            None => consts::EMPTY_TILE_SYMBOL,
        }
    }

    pub fn get_click_result(&self) -> ClickResult{
        match &self.tile_type{
            TileType::Bomb => ClickResult::Explode,
            TileType::Empty => ClickResult::Safe
        }
    }

}


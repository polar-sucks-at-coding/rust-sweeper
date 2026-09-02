use crate::consts;
use crate::board;

#[derive(PartialEq)]
pub enum TileType{
    Bomb,
    Empty
}

pub struct Tile{
    pub position: (usize, usize), // row/y first
    pub tile_type: TileType,
    pub concealed: bool,
    pub symbol: char
}

impl Tile{
    pub fn get_symbol(&self) -> &char{
        &self.symbol
    }

    fn get_type(&self) -> &TileType{
        &self.tile_type
    }

    fn get_surrounding_tiles<'a>(&self, board: &'a board::Board) -> Result<Vec<&'a Tile>, String>{
        let mut surrounding_tiles: Vec<&'a Tile> = Vec::new();

        for row in -1..2{
            for column in -1..2{
                if row == 0 && column == 0{
                    continue;
                }

                let coordinates_for_surrounding_tile: (i32, i32) = (self.position.0 as i32 + row, self.position.1 as i32 + column);
                match board.get_tile_from_coordinates(coordinates_for_surrounding_tile.0, coordinates_for_surrounding_tile.1){
                    Ok(tile) => { surrounding_tiles.push(tile); }
                    _ => {}
                }
            }
        }

        if surrounding_tiles.is_empty(){
            return Err("Error: No surrounding tiles found".to_string());
        }

        Ok(surrounding_tiles)
    }

    pub fn get_surrounding_bomb_count<'a>(&self, board: &'a board::Board) -> Option<i32>{
        let surrounding_tiles = self.get_surrounding_tiles(board).ok()?;
        let mut surrounding_bomb_count: i32 = 0;
        
        for i in surrounding_tiles{
            if let TileType::Bomb = i.get_type(){
                surrounding_bomb_count += 1;
            }
        }

        if surrounding_bomb_count < 1{
            return None;
        }

        Some(surrounding_bomb_count)
    }

    pub fn calculate_symbol(&self, board: &board::Board) -> char {
        if self.get_type() == &TileType::Bomb {
            return consts::BOMB_TILE_SYMBOL;
        }

        match self.get_surrounding_bomb_count(board) {
            Some(count) => std::char::from_digit(count as u32, 10)
                .unwrap_or(consts::EMPTY_TILE_SYMBOL),
            None => consts::EMPTY_TILE_SYMBOL,
        }
    }

}


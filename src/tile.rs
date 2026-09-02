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
    pub fn get_symbol(&self) -> char{
        self.symbol
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
                match get_tile_from_coordinates(board, coordinates_for_surrounding_tile.0, coordinates_for_surrounding_tile.1){
                    Ok(tile) => { surrounding_tiles.push(tile); }
                    _ => {}
                }
            }
        }

        if surrounding_tiles.is_empty(){
            return Err("Error: No surrounding tiles found".to_string());
        }

        return Ok(surrounding_tiles);
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



pub fn conceal_all_tiles(tiles: &mut Vec<Tile>){
    for i in tiles{
        i.concealed = true;
    }
}

pub fn get_tile_from_mouse_coordinates(board: &board::Board,  mouse_y: i32, mouse_x: i32) -> Result<&Tile, String>{
    let max_coordinates = (board.get_rows() as i32 + consts::Y_OFFSET, board.get_columns() as i32 + consts::X_OFFSET);
    if mouse_x < consts::X_OFFSET || mouse_y < consts::Y_OFFSET  ||  mouse_y > max_coordinates.0 || mouse_x > max_coordinates.1{
        return Err("Error: x or y out of bounds".to_string());
    }

    let index = (mouse_y - consts::Y_OFFSET) * board.get_columns() as i32 + (mouse_x - consts::X_OFFSET);

    match board.tiles.get(index as usize){
        Some(tile) => Ok(tile),
        None => Err("Error: tile not found at index".to_string())
    }
}

pub fn get_tile_from_coordinates(board: &board::Board, y: i32, x: i32) -> Result<&Tile, String>{
    if y < 0 || x < 0 || y >= board.get_rows() as i32 || x >= board.get_columns() as i32{
        return Err("Error: x or y out of bounds".to_string());
    }

    let index = y * board.get_columns() as i32 + x;

    match board.tiles.get(index as usize){
        Some(tile) => Ok(tile),
        None => Err("Error: tile not found at index".to_string())
    }
}

// didn't make this myself but I understand it and it works so I'm keeping it
pub fn assign_symbols_to_all_tiles(board: &mut board::Board) {
    let symbols: Vec<char> = board.tiles
        .iter()
        // .map() turns every element of the iterator into something else, in this case a char
        .map(|tile| tile.calculate_symbol(board))
        // .collect() turns the iterator into a vector
        .collect();

        // .zip() combines two iterators
    for (tile, symbol) in board.tiles.iter_mut().zip(symbols) {
        tile.symbol = symbol;
    }
}
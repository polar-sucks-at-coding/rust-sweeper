use crate::tile::{Tile, TileType};
use crate::consts;
use rand::Rng;

pub struct Board{
    pub tiles: Vec<Tile>,
    rows: usize,
    columns: usize,
}

impl Board{
    pub fn create_tiles(&self) -> Vec<Tile>{
        let mut v: Vec<Tile> = Vec::new();

        let mut row: usize = 0;
        let mut column: usize = 0;
        loop{
            let mut tile = Tile{
                position: (row, column),
                tile_type: TileType::Empty,
                concealed: false,
                symbol: consts::EMPTY_TILE_SYMBOL //temporary, will be assigned later
            };

            if (rand::thread_rng().gen_range(0..5)) == 1{
                tile.tile_type = TileType::Bomb;
            }

            tile.position = (row, column);

            v.push(tile);

            if row * column >= (self.rows - 1) * (self.columns - 1){
                break;
            }

            column += 1;
            if column == self.columns{
                column = 0;
                row += 1;
            }
        }
        v
    }
    
    pub fn new(rows: usize, columns: usize) -> Board{
        let tiles = Vec::new();
        Board{tiles, rows: rows, columns: columns}
    }

    pub fn get_rows(&self) -> usize{
        self.rows
    }

    pub fn get_columns(&self) -> usize{
        self.columns
    }

    // didn't make this myself but I understand it and it works so I'm keeping it
    pub fn assign_symbols_to_all_tiles(&mut self) {
        let symbols: Vec<char> = self.tiles
            .iter()
            // .map() turns every element of the iterator into something else, in this case a char
            .map(|tile| tile.calculate_symbol(self))
            // .collect() turns the iterator into a vector
            .collect();

            // .zip() combines two iterators
        for (tile, symbol) in self.tiles.iter_mut().zip(symbols) {
            tile.symbol = symbol;
        }
    }

    pub fn get_tile_from_mouse_coordinates(&self, mouse_y: i32, mouse_x: i32) -> Result<&Tile, String>{
        let max_coordinates = (self.get_rows() as i32 + consts::BOARD_Y_OFFSET, self.get_columns() as i32 + consts::BOARD_X_OFFSET);
        if mouse_x < consts::BOARD_X_OFFSET || mouse_y < consts::BOARD_Y_OFFSET  ||  mouse_y > max_coordinates.0 || mouse_x > max_coordinates.1{
            return Err("Error: x or y out of bounds".to_string());
        }

        let index = (mouse_y - consts::BOARD_Y_OFFSET) * self.get_columns() as i32 + (mouse_x - consts::BOARD_X_OFFSET);

        match self.tiles.get(index as usize){
            Some(tile) => Ok(tile),
            None => Err("Error: tile not found at index".to_string())
        }
    }

    pub fn get_tile_from_coordinates(&self, y: i32, x: i32) -> Result<&Tile, String>{
        if y < 0 || x < 0 || y >= self.get_rows() as i32 || x >= self.get_columns() as i32{
            return Err("Error: x or y out of bounds".to_string());
        }

        let index = y * self.get_columns() as i32 + x;

        match self.tiles.get(index as usize){
            Some(tile) => Ok(tile),
            None => Err("Error: tile not found at index".to_string())
        }
    }

}


use crate::tile::{ClickResult, Tile, TileType};
use crate::consts;
use rand::Rng;

pub struct Board{
    pub tiles: Vec<Tile>,
    rows: usize,
    columns: usize,
}

impl Board{

    pub fn get_surrounding_bomb_count(&self, tile_index: usize) -> Option<i32>{
        let surrounding_tiles = self.get_surrounding_tile_indices(tile_index);
        let mut surrounding_bomb_count: i32 = 0;
        
        for i in surrounding_tiles.unwrap(){
            if let TileType::Bomb = self.tiles[i].get_type(){
                surrounding_bomb_count += 1;
            }
        }

        if surrounding_bomb_count < 1{
            return None;
        }

        Some(surrounding_bomb_count)
    }

    pub fn get_surrounding_tile_indices<'a>(&self, tile_index: usize) -> Result<Vec<usize>, String>{
        let mut surrounding_tiles: Vec<usize> = Vec::new();

        for row in -1..2{
            for column in -1..2{
                if row == 0 && column == 0{
                    continue;
                }

                match self.tiles.get(tile_index){
                    Some(tile) => surrounding_tiles.push(self.get_index_from_coordinates(tile.position.0 as i32, tile.position.1 as i32)),
                    None => continue
                }
            }
        }

        if surrounding_tiles.is_empty(){
            return Err("Error: No surrounding tiles found".to_string());
        }

        Ok(surrounding_tiles)
    }

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

    pub fn mouse_coordinates_to_tile_coordinates(&self, mouse_y: i32, mouse_x: i32) -> Option<(i32, i32)>{
        let max_coordinates = (self.get_rows() as i32 + consts::BOARD_Y_OFFSET, self.get_columns() as i32 + consts::BOARD_X_OFFSET);
        if mouse_x < consts::BOARD_X_OFFSET || mouse_y < consts::BOARD_Y_OFFSET  ||  mouse_y > max_coordinates.0 || mouse_x > max_coordinates.1{
            return None;
        }

        let y = mouse_y - consts::BOARD_Y_OFFSET;
        let x = mouse_x - consts::BOARD_X_OFFSET;

        Some((y, x))
    }

    pub fn conceal_all_tiles(&mut self){
        for tile in &mut self.tiles{
            tile.concealed = true;
        }
    }

    pub fn get_index_from_coordinates(&self, y: i32, x:i32) -> usize{
        let index = y * self.columns as i32 + x;
        index as usize
    }

    pub fn reveal_surrounding_tiles(&mut self, tile_index: usize){
        let surrounding_indices = self.get_surrounding_tile_indices(tile_index).ok().unwrap();
        
    }
    
    pub fn reveal_tiles(&self, tiles: &mut Vec<&mut Tile>){
        for t in tiles.iter_mut(){
            (*t).concealed = false;
        }
    }

    fn get_middle_tile(&mut self) -> &Tile{
        self.tiles.get(self.columns / 2 + (self.rows * self.columns) / 2).unwrap()
    }

}


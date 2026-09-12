use crate::tile::{Tile, TileType};
use crate::consts::{self, EMPTY_TILE_SYMBOL};
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
                    Some(tile) => {
                        let index = self.get_index_from_coordinates(tile.position.0 as i32 + row, tile.position.1 as i32 + column) as i32;
                        if index < 0 || index as usize >= self.tiles.len() { continue; }

                        surrounding_tiles.push(index as usize);
                    }
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

    pub fn conceal_all_tiles(&mut self){
        for tile in &mut self.tiles{
            tile.concealed = true;
        }
    }

    pub fn get_index_from_coordinates(&self, y: i32, x:i32) -> usize{
        let index = y * self.columns as i32 + x;
        index as usize
    }

    pub fn reveal_surrounding_tiles(&mut self, tile_index: usize, reveal_self: bool){
        let surrounding_indices = self.get_surrounding_tile_indices(tile_index).ok().unwrap();
        
        if reveal_self { self.reveal_tiles(&vec![tile_index]);}

        self.reveal_tiles(&surrounding_indices);
    }
    
    pub fn reveal_tiles_random(&mut self, tile_index: usize){
        let mut to_reveal: Vec<usize> = Vec::new();
        
    }

    pub fn empty_tiles(&mut self, tiles: &Vec<usize>){
        for i in tiles.iter(){
            match self.tiles.get_mut(*i){
                Some(tile) => tile.symbol = EMPTY_TILE_SYMBOL,
                None => { continue; }
            }
        }
    }

    pub fn reveal_tiles(&mut self, tiles: &Vec<usize>){
        for t in tiles.iter(){
            match self.tiles.get_mut(*t){
                Some(tile) => {
                    if tile.tile_type == TileType::Bomb { continue; }
                    tile.concealed = false;
                }
                None => { continue; }
            }
        }
    }

    fn get_middle_tile(&mut self) -> &Tile{
        self.tiles.get(self.columns / 2 + (self.rows * self.columns) / 2).unwrap()
    }

}


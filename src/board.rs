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

    pub fn get_surrounding_tile_indices(&self, tile_index: usize) -> Option<Vec<usize>>{
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

        if surrounding_tiles.is_empty(){ return None; }

        Some(surrounding_tiles)
    }

    pub fn create_tiles(&self) -> Result<Vec<Tile>, String>{
        let mut v: Vec<Tile> = Vec::new();


        for row in 0..self.get_rows(){
            for column in 0..self.get_columns(){
                let mut tile = Tile{
                    position: (row, column),
                    tile_type: TileType::Empty,
                    concealed: false,
                    symbol: consts::EMPTY_TILE_SYMBOL //temporary, will be assigned later
                };

                //temporary hardcoded mine chance
                if (rand::thread_rng().gen_range(0..5)) == 1{
                    tile.tile_type = TileType::Bomb;
                }

                v.push(tile);
            }
        }

        if v.len() < 1{
            return Err("No tiles could be created".to_string());
        } else {
            return Ok(v);
        }

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
}


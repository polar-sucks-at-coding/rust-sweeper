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

    pub fn create_tiles(&mut self) -> Result<Vec<Tile>, String>{
        let mut v: Vec<Tile> = Vec::new();


        for row in 0..self.get_rows(){
            for column in 0..self.get_columns(){
                let mut tile = Tile{
                    position: (row, column),
                    tile_type: TileType::Empty,
                    concealed: false,
                    symbol: consts::EMPTY_TILE_SYMBOL //temporary, will be assigned later
                };

                //to add at least one bomb
                let least_bomb_coordinates = (rand::thread_rng().gen_range(0..self.get_rows()), rand::thread_rng().gen_range(0..self.get_columns()));
                let least_bomb_index = self.get_index_from_coordinates(least_bomb_coordinates.0 as i32, least_bomb_coordinates.1 as i32);
                self.tiles.get_mut(least_bomb_index).unwrap().tile_type = TileType::Bomb;
                //temporary hardcoded mine chance
                if (rand::thread_rng().gen_range(0..5)) == 1{
                    tile.tile_type = TileType::Bomb;
                }

                v.push(tile);
            }
        }

        if v.len() < 1{
            return Err("No tiles could be created".to_string());
        } 
        return Ok(v);
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

    pub fn reveal_tiles(&mut self, indices: &mut Vec<usize>){
        for t in indices.iter_mut(){
            if let Some(tile) = self.tiles.get_mut(*t){
                tile.concealed = false;
            }   else { continue; }
        }
    }

    fn get_surrounding_empty_indices(&self, middle_tile_index: usize) -> Option<Vec<usize>>{
        let mut v = Vec::new();

        if let Some(surrounding) = self.get_surrounding_tile_indices(middle_tile_index){
            for t in surrounding.iter(){
                if let Some(tile) = self.tiles.get(*t){
                    if *tile.get_symbol() == EMPTY_TILE_SYMBOL { v.push(*t); }
                }
            }
        }
        if v.len() > 0 {
            return Some(v);
        } else {
            return None;
        }
    }

    pub fn get_tiles_to_reveal_from_click(&self, index: usize) -> Option<Vec<usize>>{
        if let Some(tile) = self.tiles.get(index){
            if *tile.get_symbol() != EMPTY_TILE_SYMBOL { return None; }
        }
        let mut to_reveal = Vec::new();

        to_reveal.push(index);
        
        loop{
            let mut at_last = false;
            if let Some(empties) = self.get_surrounding_empty_indices(index){
                for i in empties.iter(){
                    to_reveal.push(*i);
                    if empties.get(empties.len() - 1) == Some(i){
                        at_last = true;
                    }
                }
            }
        }


        Some(to_reveal)
    }

}


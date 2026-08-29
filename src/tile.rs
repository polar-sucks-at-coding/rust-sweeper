use crate::consts;
use rand::Rng;

#[derive(PartialEq)]
enum TileType{
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

    fn get_surrounding_tiles<'a>(&self, tiles: &'a Vec<Tile>) -> Result<Vec<&'a Tile>, String>{
        let mut surrounding_tiles: Vec<&'a Tile> = Vec::new();

        for row in -1..2{
            for column in -1..2{
                if row == 0 && column == 0{
                    continue;
                }

                let coordinates_for_surrounding_tile: (i32, i32) = (self.position.0 as i32 + row, self.position.1 as i32 + column);
                match get_tile_from_coordinates(tiles, coordinates_for_surrounding_tile.0, coordinates_for_surrounding_tile.1){
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

    pub fn get_surrounding_bomb_count<'a>(&self, tiles: &'a Vec<Tile>) -> Option<i32>{
        let surrounding_tiles = self.get_surrounding_tiles(tiles).ok()?;
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

    pub fn calculate_symbol(&self, tiles: &Vec<Tile>) -> char {
        if self.get_type() == &TileType::Bomb {
            return consts::BOMB_TILE_SYMBOL;
        }

        match self.get_surrounding_bomb_count(tiles) {
            Some(count) => std::char::from_digit(count as u32, 10)
                .unwrap_or(consts::EMPTY_TILE_SYMBOL),
            None => consts::EMPTY_TILE_SYMBOL,
        }
    }

}

pub fn create_tiles() -> Vec<Tile>{
    let mut v: Vec<Tile> = Vec::new();

    let mut row: usize = 0;
    let mut column: usize = 0;
    loop{
        let mut tile = Tile{
            position: (row, column),
            tile_type: TileType::Empty,
            concealed: false,
            symbol:consts::EMPTY_TILE_SYMBOL //temporary, will be assigned later
        };

        if (rand::thread_rng().gen_range(0..5)) == 1{
            tile.tile_type = TileType::Bomb;
        }

        tile.position = (row, column);

        v.push(tile);

        if row * column >= (consts::ROWS - 1) * (consts::COLUMNS - 1){
            break;
        }

        column += 1;
        if column == consts::COLUMNS{
            column = 0;
            row += 1;
        }
    }
    return v;

}



pub fn conceal_all_tiles(tiles: &mut Vec<Tile>){
    for i in tiles{
        i.concealed = true;
    }
}

pub fn get_tile_from_mouse_coordinates(tiles: &Vec<Tile>, mouse_y: i32, mouse_x: i32) -> Result<&Tile, String>{
    let max_coordinates = (consts::ROWS as i32 + consts::Y_OFFSET, consts::COLUMNS as i32 + consts::X_OFFSET);
    if mouse_x < consts::X_OFFSET || mouse_y < consts::Y_OFFSET  ||  mouse_y > max_coordinates.0 || mouse_x > max_coordinates.1{
        return Err("Error: x or y out of bounds".to_string());
    }

    let index = (mouse_y - consts::Y_OFFSET) * consts::COLUMNS as i32 + (mouse_x - consts::X_OFFSET);

    match tiles.get(index as usize){
        Some(tile) => Ok(tile),
        None => Err("Error: tile not found at index".to_string())
    }
}

pub fn get_tile_from_coordinates(tiles: &Vec<Tile>, y: i32, x: i32) -> Result<&Tile, String>{
    if y < 0 || x < 0 || y >= consts::ROWS as i32 || x >= consts::COLUMNS as i32{
        return Err("Error: x or y out of bounds".to_string());
    }

    let index = y * consts::COLUMNS as i32 + x;

    match tiles.get(index as usize){
        Some(tile) => Ok(tile),
        None => Err("Error: tile not found at index".to_string())
    }
}

// didn't make this myself but I understand it and it works so I'm keeping it
pub fn assign_symbols_to_all_tiles(tiles: &mut Vec<Tile>) {
    let symbols: Vec<char> = tiles
        .iter()
        // .map() turns every element of the iterator into something else, in this case a char
        .map(|tile| tile.calculate_symbol(tiles))
        // .collect() turns the iterator into a vector
        .collect();

        // .zip() combines two iterators
    for (tile, symbol) in tiles.iter_mut().zip(symbols) {
        tile.symbol = symbol;
    }
}
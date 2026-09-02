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
        return v;
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
}


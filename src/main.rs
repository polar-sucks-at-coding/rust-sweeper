extern crate pancurses;

use pancurses::{initscr, endwin, noecho, Window, Input};
use rand::Rng;

const ROWS: i32 = 5;
const COLUMNS: i32 = 5;
const CONCEALED_TILE_SYMBOL: char = 'c';
const BOMB_TILE_SYMBOL: char = 'b';
const EMPTY_TILE_SYMBOL: char = 'e';

enum TileType{
    Bomb,
    Empty
}
struct Tile{
    position: (i32, i32), //row/y first
    tile_type: TileType,
    concealed: bool

}

fn create_tiles() -> Vec<Tile>{
    let mut v: Vec<Tile> = Vec::new();

    let mut row: i32 = 0;
    let mut column: i32 = 0;
    loop{
        let mut tile = Tile{
            position: (row, column),
            tile_type: TileType::Empty,
            concealed: false
        };

        if (rand::thread_rng().gen_range(0..2)) == 1{
            tile.tile_type = TileType::Bomb;
        }

        tile.position = (row, column);

        v.push(tile);

        if row * column >= (ROWS - 1) * (COLUMNS - 1){
            break;
        }

        column += 1;
        if column == COLUMNS{
            column = 0;
            row += 1;
        }
    }
    return v;

}

fn print_tiles(tiles: &Vec<Tile>, window: &mut Window, y_offset: i32, x_offset: i32){
    for i in tiles{
        window.mv(i.position.0 + y_offset, i.position.1 + x_offset);

        if i.concealed{
            window.printw(String::from(CONCEALED_TILE_SYMBOL));
        } 
        else{
            match i.tile_type{
                TileType::Bomb =>{
                    window.printw(String::from(BOMB_TILE_SYMBOL));
                }
                TileType::Empty =>{
                    window.printw(String::from(EMPTY_TILE_SYMBOL));
                }
            }
        }
    }
}


fn get_input() -> String{
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed = String::from(input.trim());

    trimmed
}

fn conceal_all_tiles(tiles: &mut Vec<Tile>){
    for i in tiles{
        i.concealed = true;
    }
}

fn use_char_input(tiles: &mut Vec<Tile>, window: &mut Window, c: &char){
    match c{
        'c' => { conceal_all_tiles(tiles) },
        _ => ()
    }
}

fn main() {
    let mut window = initscr();
    window.keypad(true);

    let mut tiles = create_tiles();
    let y_offset = 10;
    let x_offset = 20;

    loop{
        window.erase();
        print_tiles(&tiles, &mut window, y_offset, x_offset);

        //moving to where input will be echoed
        window.mv(y_offset + ROWS + 2, x_offset);

        match window.getch(){
            Some(Input::Character(c)) => { use_char_input(&mut tiles, &mut window, &c); },
            Some(Input::KeyDC) => break,
            Some(input) => { window.addstr(&format!("{:?}", input)); },
            None => ()
        }
    }

    endwin();
}
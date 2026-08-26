extern crate pancurses;

use pancurses::{ALL_MOUSE_EVENTS, Input::{self, KeyReference}, Window, endwin, getmouse, initscr, mousemask};
use rand::Rng;

const ROWS: usize = 5;
const COLUMNS: usize = 5;
const CONCEALED_TILE_SYMBOL: char = 'c';
const BOMB_TILE_SYMBOL: char = 'b';
const EMPTY_TILE_SYMBOL: char = 'e';
const DEBUG_COORDINATES: (u32, u32) = (30, 30);

enum TileType{
    Bomb,
    Empty
}

enum TileClickResult{
    Explode,
    Safe
}
struct Tile{
    position: (usize, usize), // row/y first
    tile_type: TileType,
    concealed: bool

}

impl Tile{
    fn get_symbol(&self) -> char{
        match self.tile_type{
            TileType::Bomb => { return BOMB_TILE_SYMBOL; },
            TileType::Empty => { return EMPTY_TILE_SYMBOL; }
        }
    }
    
    fn get_clicked(&self) -> Option<TileClickResult>{
        match self.tile_type{
            TileType::Bomb => Some(TileClickResult::Explode),
            TileType::Empty => Some(TileClickResult::Safe)
        }
    }
}

fn create_tiles() -> Vec<Tile>{
    let mut v: Vec<Tile> = Vec::new();

    let mut row: usize = 0;
    let mut column: usize = 0;
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
        window.mv(i.position.0 as i32 + y_offset, i.position.1 as i32 + x_offset);

        if i.concealed{
            window.addch(CONCEALED_TILE_SYMBOL);
            continue;
        }

        window.addch(i.get_symbol());
    }
}

fn print_tile_coordinates(window: &mut Window, y_offset: i32, x_offset: i32){
    window.mv(y_offset - 2, x_offset);
    for i in 0..COLUMNS{
        window.printw(i.to_string());
    }

    window.mv(y_offset, x_offset - 1);
    for i in 0..ROWS{
        window.mv((y_offset) + i as i32, x_offset - 2);
        window.printw(i.to_string());
    }
}

enum InputType{
    Quit,
    Character(char),
    Mouse(i32, i32) //y and x coordinates
}
fn get_input(window: &mut Window) -> Option<InputType>{
        match window.getch(){
            Some(Input::Character(c)) => {
                if c == 'q' { Some(InputType::Quit); }
                Some(InputType::Character(c))
            },

            //the delete key
            Some(Input::KeyDC) => Some(InputType::Quit),
            
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse() { Some(InputType::Mouse(mouse_event.y, mouse_event.x)) } 
                else { None }
            }

            _ => None
            
            //Some(input) => { window.addstr(&format!("{:?}", input)); },
            //None => ()
        }
}

fn conceal_all_tiles(tiles: &mut Vec<Tile>){
    for i in tiles{
        i.concealed = true;
    }
}

//wip
fn print_manual(window: &mut Window){
    window.printw("hi");
}   

fn get_tile_from_coordinates(tiles: &Vec<Tile>, y: i32, x: i32, y_offset: i32, x_offset: i32) -> Option<&Tile>{
    if x < 0 || y < 0 { return None; }
    let index = (y - y_offset) as usize * COLUMNS + (x - x_offset) as usize;
    return Some(tiles.get(index)?);
}

fn main() {
    let mut window = initscr();
    window.keypad(true);
    mousemask(ALL_MOUSE_EVENTS, None); //for pancurses to listen to all mouse events

    let mut tiles = create_tiles();
    conceal_all_tiles(&mut tiles);

    let y_offset = 10;
    let x_offset = 20;

    let mut str_for_debugging = String::new();

    //main loop
    loop{
        window.erase();
        

        window.mv(0, 0);
        print_manual(&mut window);

        print_tiles(&tiles, &mut window, y_offset, x_offset);
        print_tile_coordinates(&mut window, y_offset, x_offset);

        window.mvprintw(DEBUG_COORDINATES.0 as i32, DEBUG_COORDINATES.1 as i32, &str_for_debugging);

        match get_input(&mut window){
            Some(InputType::Character(c)) => {
                str_for_debugging = c.to_string();
            }

            Some(InputType::Mouse(y, x)) => {
                match get_tile_from_coordinates(&tiles, y, x, y_offset, x_offset){
                    Some(tile) => str_for_debugging = tile.get_symbol().to_string(),
                    None => str_for_debugging = String::from("UGHH")
                }
            }

            Some(InputType::Quit) => break,
            Option::None => {
                endwin();
                eprintln!("Invalid Input!");
                std::process::exit(1);
            }
        }
        window.refresh();
    }
    endwin();
}
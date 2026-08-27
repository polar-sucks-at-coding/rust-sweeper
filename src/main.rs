extern crate pancurses;

use std::ops::Add;

use pancurses::{ALL_MOUSE_EVENTS, Input::{self, KeyReference}, Window, endwin, getmouse, initscr, mousemask};
use rand::Rng;

const ROWS: usize = 5;
const COLUMNS: usize = 5;
const QUIT_BUTTON: char = 'q';
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
fn get_input(window: &mut Window) -> Result<InputType, String>{
        match window.getch(){
            Some(Input::Character(c)) => {
                match c {
                    QUIT_BUTTON => Ok(InputType::Quit),
                    _ => Ok(InputType::Character(c))
                }
            },
            
            Some(Input::KeyMouse) => {
                if let Ok(mouse_event) = getmouse(){
                    Ok(InputType::Mouse(mouse_event.y, mouse_event.x))
                } else { Err("Got Input::KeyMouse but can't create mouse_event".to_string()) }   
            }

            _ => Err("Error: Invalid input!".to_string())
            
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
    let mut to_print = String::from("Click on the tiles or press ");
    to_print.push(QUIT_BUTTON);
    to_print.push_str(" to quit.");
    window.printw(to_print);
}   

fn get_tile_from_coordinates(tiles: &Vec<Tile>, y_mouse: i32, x_mouse: i32, y_offset: i32, x_offset: i32) -> Result<&Tile, String>{
    let max_coordinates = (ROWS as i32 + y_offset, COLUMNS as i32 + x_offset);
    if x_mouse < x_offset || y_mouse < y_offset  ||  y_mouse > max_coordinates.0 || x_mouse > max_coordinates.1{
        return Err("Error: x or y out of bounds".to_string());
    }

    let index = (y_mouse - y_offset) * COLUMNS as i32 + (x_mouse - x_offset);

    match tiles.get(index as usize){
        Some(tile) => Ok(tile),
        None => Err("Error: tile not found at index".to_string())
    }
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
        window.refresh();

        match get_input(&mut window){
            Ok(InputType::Character(c)) => {
                str_for_debugging = c.to_string();
            }

            Ok(InputType::Mouse(y, x)) => {
                match get_tile_from_coordinates(&tiles, y, x, y_offset, x_offset) {
                    Ok(tile) => { str_for_debugging = tile.get_symbol().to_string() },
                    Err(error) => { str_for_debugging = error }
                    }
                }

            Ok(InputType::Quit) => break,
            
            Err(error) => str_for_debugging = error
        }    
    }
    endwin();
}
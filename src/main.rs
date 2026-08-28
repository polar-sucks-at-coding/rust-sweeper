extern crate pancurses;

use pancurses::{ALL_MOUSE_EVENTS, Input::{self, KeyReference}, Window, endwin, getmouse, initscr, mousemask};
use rand::Rng;

const ROWS: usize = 10;
const COLUMNS: usize = 10;
const QUIT_BUTTON: char = 'q';
const CONCEALED_TILE_SYMBOL: char = 'c';
const BOMB_TILE_SYMBOL: char = 'b';
const EMPTY_TILE_SYMBOL: char = 'e';
const DEBUG_COORDINATES: (u32, u32) = (30, 30);
const Y_OFFSET: i32 = 10;
const X_OFFSET: i32 = 20;

#[derive(PartialEq)]
enum TileType{
    Bomb,
    Empty
}

struct Tile{
    position: (usize, usize), // row/y first
    tile_type: TileType,
    concealed: bool,
    symbol: char
}

impl Tile{
    fn get_symbol(&self) -> char{
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

    fn get_surrounding_bomb_count<'a>(&self, tiles: &'a Vec<Tile>) -> Option<i32>{
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

    fn calculate_symbol(&self, tiles: &Vec<Tile>) -> char {
        if self.get_type() == &TileType::Bomb {
            return BOMB_TILE_SYMBOL;
        }

        match self.get_surrounding_bomb_count(tiles) {
            Some(count) => std::char::from_digit(count as u32, 10)
                .unwrap_or(EMPTY_TILE_SYMBOL),
            None => EMPTY_TILE_SYMBOL,
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
            concealed: false,
            symbol:EMPTY_TILE_SYMBOL //temporary, will be assigned later
        };

        if (rand::thread_rng().gen_range(0..5)) == 1{
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

fn print_tiles(tiles: &Vec<Tile>, window: &mut Window){
    for i in tiles{
        window.mv(i.position.0 as i32 + Y_OFFSET, i.position.1 as i32 + X_OFFSET);

        if i.concealed{
            window.addch(CONCEALED_TILE_SYMBOL);
            continue;
        }

        window.addch(i.get_symbol());
    }
}

fn print_tile_coordinates(window: &mut Window){
    window.mv(Y_OFFSET - 2, X_OFFSET);
    for i in 0..COLUMNS{
        window.printw(i.to_string());
    }

    window.mv(Y_OFFSET, X_OFFSET - 1);
    for i in 0..ROWS{
        window.mv((Y_OFFSET) + i as i32, X_OFFSET - 2);
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

fn get_tile_from_mouse_coordinates(tiles: &Vec<Tile>, mouse_y: i32, mouse_x: i32) -> Result<&Tile, String>{
    let max_coordinates = (ROWS as i32 + Y_OFFSET, COLUMNS as i32 + X_OFFSET);
    if mouse_x < X_OFFSET || mouse_y < Y_OFFSET  ||  mouse_y > max_coordinates.0 || mouse_x > max_coordinates.1{
        return Err("Error: x or y out of bounds".to_string());
    }

    let index = (mouse_y - Y_OFFSET) * COLUMNS as i32 + (mouse_x - X_OFFSET);

    match tiles.get(index as usize){
        Some(tile) => Ok(tile),
        None => Err("Error: tile not found at index".to_string())
    }
}

fn get_tile_from_coordinates(tiles: &Vec<Tile>, y: i32, x: i32) -> Result<&Tile, String>{
    if y < 0 || x < 0 || y >= ROWS as i32 || x >= COLUMNS as i32{
        return Err("Error: x or y out of bounds".to_string());
    }

    let index = y * COLUMNS as i32 + x;

    match tiles.get(index as usize){
        Some(tile) => Ok(tile),
        None => Err("Error: tile not found at index".to_string())
    }
}

fn handle_mouse_input(tiles: &Vec<Tile>, mouse_y: i32, mouse_x: i32) -> Result<String, String>{
    match get_tile_from_mouse_coordinates(tiles, mouse_y, mouse_x){
        Ok(tile) => {
            match tile.get_surrounding_bomb_count(tiles){
                Some(count) => Ok(format!("Surrounding bomb count: {}", count)),
                None => Ok("No surrounding bombs found".to_string())
            }
        },
        Err(error) => Err(error)
    }

}

// didn't make this myself but I understand it and it works so I'm keeping it
fn assign_symbols_to_all_tiles(tiles: &mut Vec<Tile>) {
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
fn main() {
    let mut window = initscr();
    window.keypad(true);
    mousemask(ALL_MOUSE_EVENTS, None); //for pancurses to listen to all mouse events

    let mut tiles = create_tiles();
    {
        let tiles_ref = &mut tiles;
        assign_symbols_to_all_tiles(tiles_ref);
    }
    
    //conceal_all_tiles(&mut tiles);

    let mut str_for_debugging = String::new();

    //main loop
    loop{
        window.erase();
        

        window.mv(0, 0);
        print_manual(&mut window);

        print_tiles(&tiles, &mut window);
        print_tile_coordinates(&mut window);

        window.mvprintw(DEBUG_COORDINATES.0 as i32, DEBUG_COORDINATES.1 as i32, &str_for_debugging);
        window.refresh();

        match get_input(&mut window){
            Ok(InputType::Character(c)) => {
                str_for_debugging = c.to_string();
            }

            Ok(InputType::Mouse(y, x)) => {
                match handle_mouse_input(&tiles, y, x){
                    Ok(str) => str_for_debugging = str,
                    Err(error) => str_for_debugging = error
                }
            }

            Ok(InputType::Quit) => break,
            
            Err(error) => str_for_debugging = error
        }    
    }
    endwin();
}
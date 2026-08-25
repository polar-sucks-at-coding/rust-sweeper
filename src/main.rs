extern crate pancurses;

use pancurses::{initscr, endwin, Window, Input, ALL_MOUSE_EVENTS, getmouse, mousemask};
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
    position: (i32, i32), // row/y first
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

fn print_tiles(tiles: &Vec<Tile>, window: &mut Window, y_offset: &i32, x_offset: &i32){
    for i in tiles{
        window.mv(i.position.0 + y_offset, i.position.1 + x_offset);

        if i.concealed{
            window.printw(String::from(CONCEALED_TILE_SYMBOL));
            return;
        }

        window.printw(i.get_symbol().to_string());
    }
}

fn print_tile_coordinates(window: &mut Window, y_offset: &i32, x_offset: &i32){
    window.mv(y_offset - 2, *x_offset);
    for i in 0..COLUMNS{
        window.printw(i.to_string());
    }

    window.mv(*y_offset, x_offset - 1);
    for i in 0..ROWS{
        window.mv((*y_offset) + i, x_offset - 2);
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

fn main() {
    let mut window = initscr();
    window.keypad(true);
    mousemask(ALL_MOUSE_EVENTS, None); //for pancurses to listen to all mouse events

    let tiles = create_tiles();
    let y_offset = 10;
    let x_offset = 20;

    let mut str_for_debugging = String::new();

    //main loop
    loop{
        window.erase();
        
        //moving into position for the manual
        window.mv(y_offset - 5, 0);
        print_manual(&mut window);

        print_tiles(&tiles, &mut window, &y_offset, &x_offset);
        print_tile_coordinates(&mut window, &y_offset, &x_offset);

        window.mvprintw(30, 30, &str_for_debugging);

        match get_input(&mut window){
            Some(InputType::Character(c)) => {
                str_for_debugging = c.to_string();
            }

            Some(InputType::Mouse(y, x)) => {
                let mut stringy_pingy = String::from("Mouse at ");
                stringy_pingy += &y.to_string()[..];
                stringy_pingy += " ";
                stringy_pingy += &x.to_string()[..];
                str_for_debugging = stringy_pingy;
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
extern crate pancurses;

use pancurses::{ALL_MOUSE_EVENTS, endwin, initscr, mousemask};

mod consts;
mod ui;
mod tile;
mod input;

fn main() {
    let mut window = initscr();
    window.keypad(true);
    mousemask(ALL_MOUSE_EVENTS, None); //for pancurses to listen to all mouse events

    let mut tiles = tile::create_tiles();
    tile::assign_symbols_to_all_tiles(&mut tiles);
    
    //conceal_all_tiles(&mut tiles);

    let mut str_for_debugging = String::new();

    //main loop
    loop{
        window.erase();
        
        window.mv(0, 0);
        ui::print_manual(&mut window);

        ui::print_tiles(&tiles, &mut window);
        ui::print_tile_coordinates(&mut window);

        window.mvprintw(consts::DEBUG_COORDINATES.0 as i32, consts::DEBUG_COORDINATES.1 as i32, &str_for_debugging);
        window.refresh();

        match input::get_input(&mut window){
            Ok(input::InputType::Character(c)) => {
                str_for_debugging = c.to_string();
            }

            Ok(input::InputType::Mouse(y, x)) => {
                match input::handle_mouse_input(&tiles, y, x){
                    Ok(str) => str_for_debugging = str,
                    Err(error) => str_for_debugging = error
                }
            }

            Ok(input::InputType::Quit) => break,
            
            Err(error) => str_for_debugging = error
        }    
    }
    endwin();
}
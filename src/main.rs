extern crate pancurses;

use pancurses::{ALL_MOUSE_EVENTS, endwin, initscr, mousemask};

mod consts;
mod ui;
mod tile;
mod input;
mod board;

fn main() {
    let mut window = initscr();
    window.keypad(true);
    mousemask(ALL_MOUSE_EVENTS, None); //for pancurses to listen to all mouse events

    let mut board = board::Board::new(10, 10);
    board.tiles = board.create_tiles();
    board.assign_symbols_to_all_tiles();
    board.conceal_all_tiles();

    let mut str_for_debugging = String::new();

    //main loop
    loop{
        window.erase();
        
        window.mv(0, 0);
        ui::print_manual(&mut window);

        ui::print_tiles(&board.tiles, &mut window);
        ui::print_tile_coordinates(&mut window, &board);

        let (debug_y, debug_x) = ui::get_debug_coordinates(&board);
        window.mvprintw(debug_y as i32, debug_x as i32, format!("Debug: {}", &str_for_debugging));
        window.refresh();

        match input::get_input(&mut window){
            Ok(input::InputType::Character(c)) => {
                str_for_debugging = c.to_string();
            }

            Ok(input::InputType::Mouse(y, x)) => {
                match input::handle_mouse_input(&board, y, x){
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
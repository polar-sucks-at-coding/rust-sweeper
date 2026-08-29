use crate::consts;
use pancurses::{Window};
use crate::tile;

pub fn print_tiles(tiles: &Vec<tile::Tile>, window: &mut Window){
    for i in tiles{
        window.mv(i.position.0 as i32 + consts::Y_OFFSET, i.position.1 as i32 + consts::X_OFFSET);

        if i.concealed{
            window.addch(consts::CONCEALED_TILE_SYMBOL);
            continue;
        }

        window.addch(i.get_symbol());
    }
}

pub fn print_tile_coordinates(window: &mut Window){
    window.mv(consts::Y_OFFSET - 2, consts::X_OFFSET);
    for i in 0..consts::COLUMNS{
        window.printw(i.to_string());
    }

    window.mv(consts::Y_OFFSET, consts::X_OFFSET - 1);
    for i in 0..consts::ROWS{
        window.mv((consts::Y_OFFSET) + i as i32, consts::X_OFFSET - 2);
        window.printw(i.to_string());
    }
}

pub fn print_manual(window: &mut Window){
    let mut to_print = String::from("Click on the tiles or press ");
    to_print.push(consts::QUIT_BUTTON);
    to_print.push_str(" to quit.");
    window.printw(to_print);
}   

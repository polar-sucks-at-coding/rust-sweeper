use pancurses::{Window, Input::{self}, getmouse};
use crate::{consts, tile, board};

pub enum InputType{
    Quit,
    Character(char),
    Mouse(i32, i32) //y and x coordinates
}

pub fn get_input(window: &mut Window) -> Result<InputType, String>{
        match window.getch(){
            Some(Input::Character(c)) => {
                match c {
                    consts::QUIT_BUTTON => Ok(InputType::Quit),
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

pub fn handle_mouse_input(board: &mut board::Board, mouse_y: i32, mouse_x: i32) -> Option<String>{
    let tile_coordinates = board.mouse_coordinates_to_tile_coordinates(mouse_y, mouse_x);

    if let None = tile_coordinates{
        return Some("Error: Mouse coordinates are out of bounds".to_string());
    }

    match board.tiles.get(board.get_index_from_coordinates(tile_coordinates.unwrap().0, tile_coordinates.unwrap().1)){
        Some(tile) => {
            match tile.get_click_result(){
                tile::ClickResult::Explode => Some("exploded lmao".to_string()),
                tile::ClickResult::Safe => { 
                    board.reveal_surrounding_tiles(board.get_index_from_coordinates(tile.position.0 as i32, tile.position.1 as i32));
                    return None;
                }
            }
        },
        None => Some("Error: No tile found at the given coordinates".to_string())
    }
}

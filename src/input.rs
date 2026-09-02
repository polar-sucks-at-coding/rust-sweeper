use pancurses::{Window, Input::{self}, getmouse};
use crate::consts;
use crate::board;

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

pub fn handle_mouse_input(board: &board::Board, mouse_y: i32, mouse_x: i32) -> Result<String, String>{
    match board.get_tile_from_mouse_coordinates(mouse_y, mouse_x){
        Ok(tile) => {
            match tile.get_surrounding_bomb_count(board){
                Some(count) => Ok(format!("Surrounding bomb count: {}", count)),
                None => Ok("No surrounding bombs found".to_string())
            }
        },
        Err(error) => Err(error)
    }

}

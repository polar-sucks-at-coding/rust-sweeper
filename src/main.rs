use rand::Rng;

const ROWS: u32 = 5;
const COLUMNS: u32 = 5;
const CONCEALED_TILE_SYMBOL: char = 'c';

struct Tile{
    position: (u32, u32), //row first
    symbol: char,
    concealed: bool

}

fn create_tiles() -> Vec<Tile>{
    let mut v: Vec<Tile> = Vec::new();

    let mut row: u32 = 0;
    let mut column: u32 = 0;
    loop{
        let tile = Tile{
            position: (row, column),
            symbol: '*', //temporary
            concealed: true
        };

        if (rand::thread_rng().gen_range(0..2)) == 1{
            let tile = Tile{
                symbol: 'b',
                ..tile
            };
            v.push(tile);
        }

        else {
            v.push(tile);
        }

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

fn print_tiles(tiles: &Vec<Tile>){
    let mut column = 0;
    for i in tiles{
        if column >= COLUMNS {
            print!("\n");
            column = 0;
        }

        if i.concealed{
            print!("{}", CONCEALED_TILE_SYMBOL);
        } else {
            print!("{}", i.symbol);
        }

        column += 1;
    }
    print!("\n");
}


fn get_input() -> String{
    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let trimmed = String::from(input.trim());

    trimmed
}
fn main() {
    clearscreen::clear().expect("Failed to clear screen");
    let tiles = create_tiles();
    loop{
        print_tiles(&tiles);

        let input = get_input();
        clearscreen::clear().expect("Failed to clear screen");
    }


    print!("\n");
}

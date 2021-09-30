const WIDTH: u8 = 32;
const HEIGHT: u8 = 16;
const LIFE_MINIMUM: f64 = 0.8;

const LIVE: char = 'O';
const DEAD: char = ' ';

fn main() {
    let mut cellcount = 0;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            cellcount += 1;
            let life = rand::random::<f64>() > LIFE_MINIMUM;
            // print!("{}({},{}){} ", cellcount, x, y, life);
            match life {
                true => { print!("{}", LIVE) }
                false => { print!("{}", DEAD) }
            }
            
        }
        print!("\n");
    }
}

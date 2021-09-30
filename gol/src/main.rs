const LIVE: char = 'O';
const DEAD: char = ' ';

struct Parms {
    width: u8,
    height: u8,
    life_minimum: f64,
    max_gen: u8,
}

fn main() {
    let parms = Parms {
        width: 16,
        height: 8,
        life_minimum: 0.8,
        max_gen: 3,
    };
    let mut cellcount = 0;
    for y in 0..parms.height {
        for x in 0..parms.width {
            cellcount += 1;
            let life = rand::random::<f64>() > parms.life_minimum;
            match life {
                true => {
                    print!("{}", LIVE)
                }
                false => {
                    print!("{}", DEAD)
                }
            }
        }
        print!("\n");
    }
}

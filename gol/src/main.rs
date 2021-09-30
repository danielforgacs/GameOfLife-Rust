const LIVE: char = 'O';
const DEAD: char = ' ';

struct Parms {
    width: u8,
    height: u8,
    life_minimum: f64,
    generations: u8,
}

fn main() {
    let parms = Parms {
        width: 16,
        height: 8,
        life_minimum: 0.8,
        generations: 3,
    };

    let mut cellcount = 0;
    let mut generation = 0;

    for gen in 0..parms.generations {
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
}

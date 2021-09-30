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

    let mut generation = 0;
    
    for gen in 0..parms.generations {
        generate_map(&parms);
        generation += 1;
    }
}

fn generate_map(parms: &Parms) {
    let mut cellcount = 0;

    for y in 0..parms.height {
        for x in 0..parms.width {
            cellcount += 1;
            let life = rand::random::<f64>() > parms.life_minimum;
            let cell = if life { LIVE } else { DEAD };
            print!("{}", cell)
        }
        print!("\n");
    }
}
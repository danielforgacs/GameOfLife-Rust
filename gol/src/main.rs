const LIVE: char = 'O';
const DEAD: char = ' ';

struct Parms {
    width: u8,
    height: u8,
    life_minimum: f64,
    generations: u16,
}

fn main() {
    let parms = Parms {
        width: 16,
        height: 8,
        life_minimum: 0.8,
        generations: 3,
    };

    let mut generation = 0;
    let mut map = generate_map(&parms);
    display_map(map);
    
    // for gen in 0..parms.generations {
    //     print!("generation: {}\n", generation);
    //     generation += 1;
    // }
}

fn generate_map(parms: &Parms) -> Vec<Vec<bool>> {
    let mut cellcount = 0;
    let mut map: Vec<Vec<bool>> = Vec::new();

    for y in 0..parms.height {
        let mut row: Vec<bool> = Vec::new();
        for x in 0..parms.width {
            cellcount += 1;
            let life = rand::random::<f64>() > parms.life_minimum;
            row.push(life);
            let cell = if life { LIVE } else { DEAD };
            print!("{}", cell)
        }
        map.push(row);
        print!("\n");
    }
    map
}

fn display_map(map: Vec<Vec<bool>>) {
    for y in map {
        for x in y {

            print!("{}", {if x == true {LIVE} else {DEAD}});
        }
        print!("\n");
        // for x in 0..y.len() {
        //     print!("{}", map[x]);
        // }
    }
}
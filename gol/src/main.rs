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

    let map = generate_map(&parms);
    display_map(map, &0_u16);
    
    for gen in 1..parms.generations {
        let map = generate_map(&parms);
        display_map(map, &gen);
    }
}

fn generate_map(parms: &Parms) -> Vec<Vec<bool>> {
    let mut map: Vec<Vec<bool>> = Vec::new();

    for _y in 0..parms.height {
        let mut row: Vec<bool> = Vec::new();
        for _x in 0..parms.width {
            let life = rand::random::<f64>() > parms.life_minimum;
            row.push(life);
        }
        map.push(row);
    }
    map
}

fn display_map(map: Vec<Vec<bool>>, gen: &u16) {
    print!("generation: {}\n", gen);
    for y in map {
        for x in y {
            print!("{}", {if x == true {LIVE} else {DEAD}});
        }
        print!("\n");
    }
}

fn calc_next_gen_map(map: Vec<Vec<bool>>) -> Vec<Vec<bool>>{
    let newmap: Vec<Vec<bool>> = Vec::new();
    newmap

}
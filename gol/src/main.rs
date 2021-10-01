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
    display_map(&map, &0_u16);
    
    for gen in 1..parms.generations {
        let map = calc_next_gen_map(&map);
        display_map(&map, &gen);
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

fn display_map(map: &Vec<Vec<bool>>, gen: &u16) {
    print!("generation: {}\n", gen);
    for y in map {
        for x in y {
            print!("{}", {if *x == true {LIVE} else {DEAD}});
        }
        print!("\n");
    }
}

fn calc_next_gen_map(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>>{
    let mut newmap: Vec<Vec<bool>> = Vec::new();

    for (y, row) in map.iter().enumerate() {
        let mut newrow: Vec<bool> = Vec::new();
        for (x, cell) in row.iter().enumerate(){
            // print!("[{}.{}] ", x, y);
            let neighbour_count = count_cell_neighbours(map, &x, &y);
            newrow.push(false);
            print!("{} ", neighbour_count);
        }
        newmap.push(newrow);
        // print!("\n");
    }

    newmap
}

fn count_cell_neighbours(map: &Vec<Vec<bool>>, x: &usize, y: &usize) -> u8 {
    let mut count = 0;
    let coords: Vec<i8> = vec![-1, 0, 1];

    for offset_x in &coords {
        for offset_y in &coords {
            let nx = *x as i8 + *offset_x;
            let ny = *y as i8 + *offset_y;
            
            if nx < 0 || ny < 0 || ny > { map.len() - 1} as i8 || nx > { map[0].len() -1 } as i8 {
                continue;
            }
            
            let nx = nx as usize;
            let ny = ny as usize;

            if map[ny][nx] == true {
                count += 1;
            }
        }
    }
    count
}
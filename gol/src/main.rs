const WIDTH: u8 = 8;
const HEIGHT: u8 = 2;

fn main() {
    let mut cellcount = 0;
    for y in 0..HEIGHT {
        print!("\n");
        for x in 0..WIDTH {
            cellcount += 1;
            print!("{}({},{}) ", cellcount, x, y);
        }
    }
}

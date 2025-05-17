use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut first: f32 = args[1].parse().unwrap();
    let mut second: f32 = args[2].parse().unwrap();
    let mut cycles: u32 = args[3].parse().unwrap();
    println!("{first}");
    println!("{second}");

    while cycles > 2 {
        let tmp: f32 = &first + &second;
        println!("{tmp}");
        second = first;
        first = tmp;

        cycles = cycles - 1;
    }
}

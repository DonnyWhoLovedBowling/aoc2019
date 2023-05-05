mod read_lines;
use read_lines::Lines;
use std::io;
use std::fs::File;

fn compute_fuel(mass: u128) -> u128{
    (mass/3)-2
}
fn main() -> io::Result<()> {
    let file = File::open("C:/users/pcvan/projects/aoc2019/data/ex1.txt")?;

    let mut lines = Lines::new(file);
    let mut sum: u128 = 0;
    let mut fuel_vec = Vec::new();
    while let Some(line) = lines.next() {
        let line = line?;
        let f = compute_fuel(line.parse::<u128>().unwrap());
        sum += f;
        if f >= 9 {
            fuel_vec.push(f)
        } 
    }
    let mut fuel_sum: u128 = sum;
    while fuel_vec.len() > 0{
        let f = fuel_vec.pop().unwrap();
        let f_new = compute_fuel(f);
        fuel_sum += f_new;
        if f_new >= 9{
            fuel_vec.push(f_new)
        }
    }
    println!("sum = {}", sum);
    println!("fuel sum = {}", fuel_sum);

    Ok(()) 
}

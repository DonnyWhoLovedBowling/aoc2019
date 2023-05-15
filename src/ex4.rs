use std::fs::File;
use std::io::Read;
use std::io;

fn main() -> io::Result<()> {
    let mut file: File = File::open("C:/users/pcvan/projects/aoc2019/data/ex4.txt")?;
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    let list = text.split('-').collect::<Vec<&str>>();
    println!("input as list: {:?}", list);
    let min = str::parse::<i32>(list[0]).unwrap();
    let max = str::parse::<i32>(list[1]).unwrap();
    let mut total1 = 0;
    let mut total2 = 0;

    for i in min..max{
        let mut previous: char = '-';
        let mut ok: bool = true;
        let mut ok1: bool = false;
        let mut ok2: bool = false;
        let mut seq = 1;
        for c in i.to_string().chars(){
            if c == previous{
                ok1 = true;
                seq += 1;
            }
            else{
                if seq == 2{
                    ok2 = true;
                }
                seq = 1;
            }
            if previous != '-' &&  previous.to_digit(10) > c.to_digit(10){
                ok = false;
                break;
            }
            previous = c;
        }
        if ok && ok1{
            total1 += 1;
        }
        if ok && (ok2 || (seq == 2)){
            total2 += 1;
        } 
        
    }
    println!("ans1 = {}", total1);
    println!("ans2 = {}", total2);

    Ok(())

}
    

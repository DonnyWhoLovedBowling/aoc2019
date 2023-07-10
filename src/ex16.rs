// file1.rs
use std::fs::File;
use std::io::Read;
use chrono;


fn do_run() {
    let mut file = File::open("C:/users/pcvan/projects/aoc2019/data/ex16.txt").expect("can't open the file");
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("can't read the file");

    let digits: Vec<u32> = text.chars().map(|c| c.to_digit(10).unwrap()).collect();
    println!("{:?}", text);
    println!("{:?}", digits);
    let offset = digits[..7].iter().fold(0, |n, &d| 10 * n + d) as usize;
    let suffix_len = digits.len() * 10_000 - offset;
    let mut suffix: Vec<_> = digits
        .iter()
        .copied()
        .rev()
        .cycle()
        .take(suffix_len)
        .collect();

    for _ in 0..100 {
        let mut prev = suffix[0];
        for x in &mut suffix[1..] {
            *x += prev;
            *x %= 10;
            prev = *x;
        }
    }

    let ans = suffix.iter().rev().take(8).fold(0, |n, &d| 10 * n + d);
    println!("ex2: {ans}");

    file.read_to_string(&mut text).expect("can't read the file");
    let mut phase = 0;
    let pattern: Vec<i32> = vec![0, 1, 0, -1];
    let mut operational_pattern: Vec<i32> = pattern.clone();

    let mut cur_value;
    cur_value = text.clone();
    
    while phase < 1{
        let mut new_value: String = String::new();

        for _i in 0..cur_value.len(){
            let mut output = 0;
            let mut ix_pat = 1;

            for c in cur_value.chars(){
                let c_dec = u32::from(c.to_digit(10).unwrap());
                let multiplier = operational_pattern.get(ix_pat % operational_pattern.len()).unwrap();                
                output += multiplier.wrapping_mul(i32::try_from(c_dec).unwrap());
                ix_pat += 1;
            }
            new_value.push(output.to_string().chars().last().unwrap()) ;
            let mut new_pattern: Vec<i32> = Vec::new();
            if _i > 6500{
                println!("{:?}", new_value);
                std::process::exit(0);
            }
            if _i % 100 == 0{
                println!("{:?}  {}", chrono::offset::Local::now(), _i);
            }
            for p in pattern.iter(){
                for _j in 0.._i+2{
                    new_pattern.push(*p);
                }
            }
            operational_pattern = new_pattern.clone();
        }
        operational_pattern.clear();
        operational_pattern.extend_from_slice(&pattern);
        cur_value = new_value;
        if phase > 95{
            println!("{}", cur_value);
            println!()    
        }
        phase += 1;

    }
    println!("{}", cur_value);

}

fn main(){
    do_run();
}
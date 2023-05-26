// file1.rs
use std::fs::File;
use std::io::Read;
use substring::Substring;

fn run(){

    let mut file = File::open("C:/users/pcvan/projects/aoc2019/data/ex8.txt").expect("can't open the file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    let layer_len = 150;
    let mut begin = 0;
    let mut end = begin + layer_len;
    let mut min_zeros = 170;
    let mut ans = 0;
    let mut layers: Vec<String> = vec![];
    while end <= text.len(){
        let subst = text.substring(begin, end);
        layers.push(subst.to_string());
        let zeros = subst.matches('0').count();
        if zeros < min_zeros{
            let ones = subst.matches('1').count();
            let twos = subst.matches('2').count();
            ans = ones*twos;
            min_zeros = zeros;
        } 
        begin += layer_len;
        end += layer_len;
    }
    println!("ans 1 = {}", ans);
    let mut final_layer: String = "".to_string();
    for i in 0..150{
        for l in layers.clone().into_iter(){
            if l.chars().nth(i).expect("prob with layer ") == '0'{
                final_layer.push(' ');
                break;
            }
            else if l.chars().nth(i).expect("prob with layer ") == '1'{
                final_layer.push('X');
                break;
            }

        }
    }
    println!("len string {}", final_layer.len());
    let col_size = 25;
    let row_size = 6;
    begin = 0;
    end = col_size;
    for _r in 0..row_size{
        println!("{}", final_layer.substring(begin, end));
        begin += col_size;
        end += col_size;
    }
    
}

fn main(){
    run();
}
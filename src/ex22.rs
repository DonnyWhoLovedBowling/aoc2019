use std::{fs::File, io::{BufReader, BufRead}};
use itertools::Itertools;
use modinverse::modinverse;
use mod_exp::mod_exp;


fn deal(deque: &Vec<i64>, n: usize) -> Vec<i64>{
    let mut new_deque: Vec<i64> = deque.clone();
    let mut idx = 0;
    for &card in deque.iter() {
        new_deque[idx] = card;
        idx += n;
        if idx >= deque.len() {
            idx -= deque.len();
        }
    }
    new_deque
}

fn cut(deque: &Vec<i64>, n: i32) -> Vec<i64>{
    let ix;
    if n < 0{
        println!("{n}");
        ix = usize::try_from(i32::try_from(deque.len()).unwrap()+n).unwrap();
    }
    else {
        ix = usize::try_from(n).unwrap();
    }
    [&deque[ix..], &deque[..ix]].concat().clone()
}

fn do_run(lines: Vec<String>, deque: &Vec<i64>) -> Vec<i64>{
    let mut new_deque = deque.clone();
    for line in lines{
        let len = line.len();
        if len > 18 && line[..19] == *"deal with increment"{
            new_deque = deal(&new_deque, line[20..].parse().unwrap());
        }
        else if line[..3] == *"cut"{
            new_deque = cut(&new_deque, line[4..].parse().unwrap());
        }
        else if line.contains("deal into new stack"){
            new_deque = new_deque.iter().rev().cloned().collect::<Vec<i64>>();
        }
    }
    new_deque.to_vec()
}


fn do_run_pt_2(lines: Vec<&String>, n: u64, times: u64) -> i64{
    let mut a:i64 = 1;
    let mut b:i64 = 0;
    for line in lines{
        let len: usize = line.len();
        if len > 18 && line[..19] == *"deal with increment"{
            let inc: u64 = line[20..].parse().unwrap();
            // println!("inc={inc}");
            let p = i64::try_from(mod_exp(inc, n-2, n)).unwrap();
            a *= p;
            b *= p;
            println!("a = {a}");
            println!("b = {b}");
        
        }
        else if line[..3] == *"cut"{
            let cut = line[4..].parse::<i64>().unwrap();
            // println!("cut = {cut}");
            b += cut;
        }
        else if line.contains("deal into new stack"){
            b += 1;
            a *= -1;
            b *= -1;
        }
        a %= i64::try_from(n).unwrap();
        b %= i64::try_from(n).unwrap();
    }
    let a_end = i64::try_from(mod_exp(u64::try_from(a).unwrap(), times, n)).unwrap();
    let b_end = i64::try_from(a_end - 1).unwrap() * i64::try_from(modinverse(a-1, i64::try_from(n).unwrap()).unwrap()).unwrap() * b;
    return (a_end*2020 + b_end) % i64::try_from(n).unwrap();
}


fn main() {
    let file = File::open("C:/users/pcvan/projects/aoc2019/data/ex22.txt").expect("can't open the file");
    let reader = BufReader::new(file);
    let lines:Vec<String>= reader.lines().collect::<Result<_,_>>().unwrap(); 
    let deque: Vec<i64> = (0..10007).into_iter().collect_vec();

    let new_deque = do_run(lines.clone(), &deque);
    println!("{:?}, {}", new_deque.iter().position(|&r| r == 2019).unwrap(), deque.len());

    // let file = File::open("C:/users/pcvan/projects/aoc2019/data/ex22_test.txt").expect("can't open the file");
    // let reader = BufReader::new(file);
    // let deque: Vec<i64> = (0..10).into_iter().collect_vec();
    // let lines:Vec<String>= reader.lines().collect::<Result<_,_>>().unwrap(); 
    // let new_deque: Vec<i64> = do_run(lines.clone(), &deque);
    // println!("{:?}", new_deque);

    // println!("{}", mod_exp(2, 119315717514047-2, 119315717514047));

    println!("pt2: {}", do_run_pt_2(lines.iter().rev().collect::<Vec<&String>>()
    , 119315717514047, 101741582076661));

}

// file1.rs
use std::fs::File;
use std::io::Read;


fn do_run(noun: u32, verb: u32) -> u32{

    let mut file = File::open("C:/users/pcvan/projects/aoc2019/data/ex2.txt").expect("can't open the file");
    let mut text = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    let mut v: Vec<u32> = Vec::new();
    for s in text.split(",") {
        v.push(s.trim().parse::<u32>().unwrap());
    }
    v[1] = noun;
    v[2] = verb;
    let mut ix = 0; 
    while v[ix] != 99{
        let op = v[ix];
        if op == 99{
            break;
        }
        let ix_in1: usize = usize::try_from(*v.get(ix+1).unwrap()).unwrap();
        let ix_in2 = usize::try_from(*v.get(ix+2).unwrap()).unwrap();
        let ix_out: usize = usize::try_from(*v.get(ix+3).unwrap()).unwrap();
        if op == 1{
            v[ix_out] = v[ix_in1]+v[ix_in2];
        }
        if op == 2{
            v[ix_out] = v[ix_in1]*v[ix_in2];
        }
        ix += 4;
    }
    v[0]
}

fn main(){
    println!("ex1: {}", do_run(12, 2));
    let mut br = false;
    let mut ans = 0;
    for n in 0..99{
        for v in 0..99{
             if do_run(n,v) == 19690720{
                br = true;
                ans = n*100+v;
                break
             }
        }
        if br{
            break;
        }
    }
    println!("ex2: {}", ans);

}
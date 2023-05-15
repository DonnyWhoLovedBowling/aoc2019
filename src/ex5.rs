// file1.rs
use std::fs::File;
use std::io::Read;

fn parse_op(op: i32)-> (i32, i32, i32, i32) {
    if op == 99{
        return (99,0,0,0)
    }    
    else if op < 10{
        return (op,0,0,0)
    }
    else if op < 1000{
        return(op % 100,1,0,0);
    }
    else if op < 10000{
        let real_op = op % 100;
        let par_2 = 1;
        let par_1 = op.to_string().chars().nth(1).unwrap().to_digit(10).unwrap().try_into().unwrap();
        return(real_op,par_1,par_2,0);
    }
    else{
        let real_op = op % 100;
        let par_3 = 1;
        let par_2 = op.to_string().chars().nth(1).unwrap().to_digit(10).unwrap().try_into().unwrap();
        let par_1 = op.to_string().chars().nth(2).unwrap().to_digit(10).unwrap().try_into().unwrap();
        return(real_op,par_1,par_2,par_3);

    }
}

fn do_run(input: i32){

    let mut file = File::open("C:/users/pcvan/projects/aoc2019/data/ex5.txt").expect("can't open the file");
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    let mut v: Vec<i32> = Vec::new();
    for s in text.split(",") {
        v.push(s.trim().parse::<i32>().unwrap());
    }
    let mut ix = 0; 

    while v[ix] != 99{
        let op: (i32, i32, i32, i32) = parse_op(v[ix]);
        if op.0 == 99{
            break;
        }
        let ix_in1: i32 = *v.get(ix+1).unwrap();
        let par_1: i32;
        if op.1 == 0{
            par_1 = v[usize::try_from(ix_in1).unwrap()];
        }
        else{
            par_1 = ix_in1.try_into().unwrap();
        }    
        let mut par_2: i32 = 0;
        let mut ix_out: usize = 0;
        let ix_in2: i32;
        if op.0 == 1 || op.0 == 2 || op.0 == 5 || op.0 == 6 || op.0 == 7 || op.0 == 8{
            ix_in2 = *v.get(ix+2).unwrap();
            if op.0 != 5 && op.0 != 6{
                ix_out = usize::try_from(*v.get(ix+3).unwrap()).unwrap();
                ix += 4;
            }
            if op.2 == 0{
                par_2 = v[usize::try_from(ix_in2).unwrap()];
            }
            else{
                par_2 = ix_in2.try_into().unwrap();
            }
        }
        if op.0 == 1{
            v[ix_out] = par_1+par_2;
        }
        if op.0 == 2{
            v[ix_out] = par_1*par_2;
        }
        if op.0 == 3 || op.0 == 4{
            ix += 2;
            if op.0 == 3{
                v[usize::try_from(ix_in1).unwrap()] = input;
                println!("test output mode 3 {} ", v[usize::try_from(ix_in1).unwrap()]);
            }
            else{
                println!("test output {} ", par_1);
            }
        }
        if op.0 == 5{
            if par_1 != 0{
                ix = par_2.try_into().unwrap();
            }
            else{
                ix += 3;
            }
        }
        if op.0 == 6{
            if par_1 == 0{
                ix = par_2.try_into().unwrap();
            }
            else{
                ix += 3;
            }
        }
        if op.0 == 7{
            if par_1 < par_2{
                v[ix_out] = 1
            }
            else{
                v[ix_out] = 0
            }
        }
        if op.0 == 8{
            if par_1 == par_2{
                v[ix_out] = 1
            }
            else{
                v[ix_out] = 0
            }
        }


    }
}

fn main(){
    do_run(1);
    do_run(5);
    // do_run(8);
    // do_run(9);
}
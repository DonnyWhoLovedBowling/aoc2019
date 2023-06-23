use std::collections::HashMap;
// file1.rs
use std::fs::File;
use std::io::Read;

fn get_v(v: &HashMap<i64, i64>, ix: &i64) -> i64{
    let ix_v: i64 =  *ix;
    if v.contains_key(&ix_v){
        v[&ix_v]
    }
    else{
        0
    }
}

fn parse_op(op: i64)-> (i64, i64, i64, i64) {
    if op == 99{
        return (99,0,0,0)
    }    
    else if op < 10{
        return (op,0,0,0)
    }
    else if op < 1000{
        let par_1: i64 = op.to_string().chars().nth(0).unwrap().to_digit(10).unwrap().try_into().unwrap();
        return(op % 100,par_1,0,0);
    }
    else if op < 10000{
        let real_op = op % 100;
        let par_2 = op.to_string().chars().nth(0).unwrap().to_digit(10).unwrap().try_into().unwrap();
        let par_1: i64 = op.to_string().chars().nth(1).unwrap().to_digit(10).unwrap().try_into().unwrap();
        return(real_op,par_1,par_2,0);
    }
    else{
        let real_op = op % 100;
        let par_3 = op.to_string().chars().nth(0).unwrap().to_digit(10).unwrap().try_into().unwrap();
        let par_2 = op.to_string().chars().nth(1).unwrap().to_digit(10).unwrap().try_into().unwrap();
        let par_1 = op.to_string().chars().nth(2).unwrap().to_digit(10).unwrap().try_into().unwrap();
        return(real_op,par_1,par_2,par_3);

    }
}

struct Amp{
    v: HashMap<i64, i64>,
    ix: i64,
    rel_base: i64,
    input: Vec<i64>,
    ball: (i64, i64),
    tile: (i64, i64)
}

impl Amp{
    fn get_output(&mut self) -> Option<i64>{    
        while get_v(&self.v, &self.ix) != 99{
            let op: (i64, i64, i64, i64) = parse_op(self.v[&self.ix]);
            if op.0 == 99{
                return None;
            }
            let ix_in1: i64= *self.v.get(&(self.ix+1)).unwrap();
            let par_1: i64;
            if op.1 == 0{
                par_1 = get_v(&self.v, &ix_in1);
            }
            else if op.1 == 1{
                par_1 = ix_in1.try_into().unwrap();
            }
            else{
                par_1 = get_v(&self.v, &(ix_in1+self.rel_base));
            }    
            let mut par_2: i64= 0;
            let mut ix_out = 0;
            let ix_in2: i64;
            if op.0 == 1 || op.0 == 2 || op.0 == 5 || op.0 == 6 || op.0 == 7 || op.0 == 8{
                ix_in2 = *self.v.get(&(self.ix+2)).unwrap();
                if op.0 != 5 && op.0 != 6{
                    ix_out = i64::try_from(get_v(&self.v, &i64::try_from(self.ix+3).unwrap())).unwrap();
                    if op.3 == 2{
                        ix_out = ix_out+self.rel_base
                    }
                    self.ix += 4;
                }
                if op.2 == 0{
                    par_2 = get_v(&self.v, &ix_in2);
                }
                else if op.2 == 1{
                    par_2 = ix_in2.try_into().unwrap();
                }
                else{
                    par_2 = get_v(&self.v, &(ix_in2+self.rel_base));
                }
            }
            if op.0 == 1{
                self.v.insert(ix_out, par_1+par_2);
            }
            if op.0 == 2{
                self.v.insert(ix_out, par_1*par_2);
            }
            if op.0 == 3 || op.0 == 4{
                self.ix += 2;
                if op.0 == 3{
                    if self.ball.0 < self.tile.0{
                        self.input.push(-1);
                    }
                    else if self.ball.0 > self.tile.0{
                        self.input.push(1);
                    }
                    else{
                        self.input.push(0);
                    }
                    self.v.insert(ix_in1, self.input.pop().unwrap());
                    // println!("input: {}, tile: {:?}, {}", self.v.get(&(ix_in1)).unwrap(), self.tile, self.rel_base);
                }
                else{
                    return Some(par_1);
                }
            }
            if op.0 == 5{
                if par_1 != 0{
                    self.ix = par_2.try_into().unwrap();
                }
                else{
                    self.ix += 3;
                }
            }
            if op.0 == 6{
                if par_1 == 0{
                    self.ix = par_2.try_into().unwrap();
                }
                else{
                    self.ix += 3;
                }
            }
            if op.0 == 7{
                if par_1 < par_2{
                    self.v.insert(ix_out,  1);
                }
                else{
                    self.v.insert(ix_out,  0);
                }
            }
            if op.0 == 8{
                if par_1 == par_2{
                    self.v.insert(ix_out,  1);
                }
                else{
                    self.v.insert(ix_out,  0);
                }
            }
            if op.0 == 9{
                self.rel_base += par_1;
                self.ix += 2;
            }
        }
        None
    }
    
}

fn do_loop(decoder: &mut Amp) -> HashMap<i64, HashMap<i64, i64>>{
    let mut grid: HashMap<i64, HashMap<i64, i64>> = HashMap::new();
    loop{
        let x;
        let x_result: Option<i64> = decoder.get_output();
        match x_result{
            None => break,
            Some(_) => {x = x_result.unwrap()}
        } 
        let y = decoder.get_output().unwrap();
        let b = decoder.get_output().unwrap();
        if x < 0{
            println!("{} {} {} {}", x,y,b,decoder.ix);
            print_grid(&grid);
            // decoder.ix = 0;
        }
        else if grid.contains_key(&y){
            if grid.get(&y).unwrap().contains_key(&x){
                // println!("resetting value {} {} {} was {} ", x, y, b, b_old);
                *grid.get_mut(&y).unwrap().get_mut(&x).unwrap() = b;
            }
            else{
                grid.get_mut(&y).unwrap().insert(x,b);
            }
        }
        else{
            grid.insert(y, [(x,b)].iter().cloned().collect());
        }
        if b == 3{
            decoder.tile = (x,y);
            // println!("three has changed: {} {}", x, y);
        }
        else if b == 4{
            decoder.ball = (x,y);
        }

    }
    return grid
}

fn print_grid(grid: &HashMap<i64, HashMap<i64, i64>>) -> i64{
    let mut n_blocks = 0;
    for i in 0..23{
        let mut line = "".to_string();
        for j in 0..35{ 
            let it2 = grid.get(&i).unwrap().get(&j).unwrap();
            match it2{
                0 => {line += " "},
                1 => {line += "w"},
                2 => {line += "X"; n_blocks += 1},
                3 => {line += "_"},
                4 => {line += "O"},
                _ => continue
            }
        }
        println!("{line}");
    }
    println!("");
    println!("");
    return n_blocks;
}
fn main() {
    let mut file = File::open("C:/users/pcvan/projects/aoc2019/data/ex13.txt").expect("can't open the file");
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    let mut v: HashMap<i64, i64> = HashMap::new();

    for s in text.split(",").into_iter().enumerate() {
        v.insert(i64::try_from(s.0).unwrap(), s.1.trim().parse::<i64>().unwrap());
    }
    let mut decoder: Amp = Amp{v: v.clone(), ix: 0, rel_base: 0, input: vec![], tile: (0,0), ball: (0,0)};
    let grid: HashMap<i64, HashMap<i64, i64>> = do_loop(&mut decoder);

    let n_blocks = print_grid(&grid);
    println!("{n_blocks}");
    *v.get_mut(&0).unwrap() = 2;
    decoder.v = v.clone();
    decoder.ix = 0;
    decoder.rel_base = 0;
    println!("{}", v[&0]);
    do_loop(&mut decoder);

}

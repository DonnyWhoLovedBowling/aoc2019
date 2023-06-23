use std::collections::HashMap;
use std::collections::HashSet;
// file1.rs
use std::fs::File;
use std::io::Read;
use std::process::exit;

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
    input: Vec<i64> 
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
                    self.v.insert(ix_in1+self.rel_base, self.input.pop().unwrap());
                }
                else{
                    return Some(par_1);
                }
            }
            if op.0 == 5{
                if par_1 != 0{
                    self.ix = par_2.try_into().unwrap();
                    if par_1 < 1{
                        exit(-99);
                    }
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

fn main() {
    let mut file = File::open("C:/users/pcvan/projects/aoc2019/data/ex11.txt").expect("can't open the file");
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    let mut v: HashMap<i64, i64> = HashMap::new();

    for s in text.split(",").into_iter().enumerate() {
        v.insert(i64::try_from(s.0).unwrap(), s.1.trim().parse::<i64>().unwrap());
    }
    let mut decoder = Amp{v: v.clone(), ix: 0, rel_base: 0, input: vec![]};
    let mut direction = 0;
    let mut pos = (0,0);
    let mut whites: HashSet<(i32, i32)> = HashSet::new();
    whites.insert((0,0));
    let mut blacks: HashSet<(i32, i32)> = HashSet::new();
    let mut all: HashSet<(i32, i32)> = HashSet::new();
    let mut min_y = 0;
    let mut max_x = 0;

    loop{
        if whites.contains(&pos){
            decoder.input.push(1);
        }
        else{
            decoder.input.push(0);
        }
        let o = decoder.get_output();
        all.insert(pos);
        if o == None{
            break;
        }
        else if o == Some(1){
            whites.insert(pos);
            blacks.remove(&pos);
        }
        else{
            blacks.insert(pos);
            whites.remove(&pos);
        }
        match decoder.get_output(){
            None => break,
            Some(1) => direction = (((direction + 1) % 4) + 4) % 4,
            Some(0) => direction = (((direction - 1) % 4) + 4) % 4,
            _ => break
        } 
        println!("{}, {:?}", direction, pos);

        match direction{
            0 => pos = (pos.0, pos.1 + 1),
            1 => pos = (pos.0 + 1, pos.1),
            2 => pos = (pos.0, pos.1 - 1),
            3 => pos = (pos.0 - 1, pos.1),
            _ => println!("error! {}", direction)
        }        
        if pos.0 > max_x{
            max_x = pos.0;
        }
        if pos.1 < min_y{
            min_y = pos.1;
        }
    }
    // println!("whites: {}, blacks: {}, all: {}", whites.len(), blacks.len(), all.len());  
    let base = (0..max_x+5).map(|_| " ").collect::<String>(); 
    let mut rows: Vec<String> = Vec::new();
    for _i in 0..min_y.abs()+5{
        rows.push(base.clone());
    }

    for w in whites.iter(){
        let i = usize::try_from(w.1+5).unwrap();
        let mut new_string = rows[i].clone();
        let ix = usize::try_from(w.0).unwrap();
        new_string.replace_range(ix..(ix+1), "X");
        rows[i] = new_string.to_string().clone();
    }
    for r in rows.iter(){
        println!("{}", r);
    }
}

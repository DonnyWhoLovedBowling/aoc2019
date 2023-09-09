use std::collections::HashMap;
// file1.rs
use std::fs::File;
use std::io::Read;

// use chrono::Utc;

fn get_v(v: &HashMap<i32, i32>, ix: &i32) -> i32{
    let ix_v: i32 =  *ix;
    if v.contains_key(&ix_v){
        v[&ix_v]
    }
    else{
        0
    }
}


fn parse_op(op: i32)-> (i32, i32, i32, i32) {
    if op == 99{
        return (99,0,0,0)
    }    
    else if op < 10{
        return (op,0,0,0)
    }
    else if op < 1000{
        let par_1: i32 = op.to_string().chars().nth(0).unwrap().to_digit(10).unwrap().try_into().unwrap();
        return(op % 100,par_1,0,0);
    }
    else if op < 10000{
        let real_op = op % 100;
        let par_2 = op.to_string().chars().nth(0).unwrap().to_digit(10).unwrap().try_into().unwrap();
        let par_1: i32 = op.to_string().chars().nth(1).unwrap().to_digit(10).unwrap().try_into().unwrap();
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
    v: HashMap<i32, i32>,
    ix: i32,
    rel_base: i32,
    input: Vec<i32>
}

impl Amp{
    fn get_output(&mut self) -> Option<bool>{    
        while get_v(&self.v, &self.ix) != 99{
            let op: (i32, i32, i32, i32) = parse_op(self.v[&self.ix]);
            if op.0 == 99{
                return None;
            }
            let ix_in1: i32= *self.v.get(&(self.ix+1)).unwrap();
            let par_1: i32;
            if op.1 == 0{
                par_1 = get_v(&self.v, &ix_in1);
            }
            else if op.1 == 1{
                par_1 = ix_in1.try_into().unwrap();
            }
            else{
                par_1 = get_v(&self.v, &(ix_in1+self.rel_base));
            }    
            let mut par_2: i32= 0;
            let mut ix_out = 0;
            let ix_in2: i32;
            if op.0 == 1 || op.0 == 2 || op.0 == 5 || op.0 == 6 || op.0 == 7 || op.0 == 8{
                ix_in2 = *self.v.get(&(self.ix+2)).unwrap();
                if op.0 != 5 && op.0 != 6{
                    ix_out = i32::try_from(get_v(&self.v, &i32::try_from(self.ix+3).unwrap())).unwrap();
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
                if op.0 == 3{
                    let input_int: i32 = self.input.pop().unwrap();
                    ix_out = ix_in1;
                    if op.1 == 2{
                        ix_out = ix_in1+self.rel_base
                    }
                    self.v.insert(ix_out, input_int);
                }
                else{
                    println!("{par_1}");
                }
                self.ix += 2;
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



fn main() {

    let mut file = File::open("C:/users/pcvan/projects/aoc2019/data/ex21.txt").expect("can't open the file");
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    let mut v: HashMap<i32, i32> = HashMap::new();
    for s in text.split(",").into_iter().enumerate() {
        v.insert(i32::try_from(s.0).unwrap(), s.1.trim().parse::<i32>().unwrap());
    }
    let input: Vec<i32> = Vec::new();
    let mut decoder: Amp = Amp{v: v.clone(), ix: 0, rel_base: 0,  input: input};
    
    let program: &str = "NOT A J
                             NOT B T
                             AND D T
                             OR T J
                             NOT C T
                             AND D T
                             OR T J
                             WALK
                             ";
    let program_asci = program.as_bytes();
    decoder.input = program_asci.into_iter().map(|x| i32::from(*x)).collect();
    decoder.input.reverse();
    println!("{:?}", decoder.get_output());
    let program2: &str = "OR D T
                          AND H T
                          OR T J
                          OR D T
                          AND E T
                          AND I T
                          OR T J
                          OR D T
                          AND A T 
                          AND B T
                          AND C T
                          NOT T T
                          AND T J
                          NOT A T
                          OR T J
                          RUN
                          ";
    let program_asci = program2.as_bytes();
    decoder.input = program_asci.into_iter().map(|x| i32::from(*x)).collect();
    decoder.input.reverse();
    decoder.ix = 0;
    decoder.rel_base = 0;
    decoder.v = v.clone();
    println!("{:?}", decoder.get_output());

}
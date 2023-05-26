// file1.rs
use std::fs::File;
use std::io::Read;
use std::process::exit;
use itertools::Itertools;

struct Amp{
    name: String,
    v: Vec<i64>,
    ix: usize,
    input: Vec<i64> 
}

fn parse_op(op: i64)-> (i64, i64, i64, i64) {
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

impl Amp{
    fn get_output(&mut self) -> Option<i64>{    
        while self.v[self.ix] != 99{
            let op: (i64, i64, i64, i64) = parse_op(self.v[self.ix]);
            if op.0 == 99{
                return None;
            }
            let ix_in1: i64= *self.v.get(self.ix+1).unwrap();
            let par_1: i64;
            if op.1 == 0{
                par_1 = self.v[usize::try_from(ix_in1).unwrap()];
            }
            else{
                par_1 = ix_in1.try_into().unwrap();
            }    
            let mut par_2: i64= 0;
            let mut ix_out: usize = 0;
            let ix_in2: i64;
            if op.0 == 1 || op.0 == 2 || op.0 == 5 || op.0 == 6 || op.0 == 7 || op.0 == 8{
                ix_in2 = *self.v.get(self.ix+2).unwrap();
                if op.0 != 5 && op.0 != 6{
                    ix_out = usize::try_from(*self.v.get(self.ix+3).unwrap()).unwrap();
                    self.ix += 4;
                }
                if op.2 == 0{
                    par_2 = self.v[usize::try_from(ix_in2).unwrap()];
                }
                else{
                    par_2 = ix_in2.try_into().unwrap();
                }
            }
            if op.0 == 1{
                self.v[ix_out] = par_1+par_2;
            }
            if op.0 == 2{
                self.v[ix_out] = par_1*par_2;
            }
            if op.0 == 3 || op.0 == 4{
                self.ix += 2;
                if op.0 == 3{
                    self.v[usize::try_from(ix_in1).unwrap()] = self.input.pop().unwrap();
                    // println!("op=3, output: {:?}, at index: {}", input[ix_input], ix_in1);
                    // if input[ix_input] == -4{
                    //     exit(0);
                    // }
                }
                else{
                    return Some(par_1);
                }
            }
            if op.0 == 5{
                if par_1 != 0{
                    self.ix = par_2.try_into().unwrap();
                    // println!("op=5, ix: {:?}, par_1= {}, inputbuf[]={:?}", ix, par_1, inputbuf);
                    if par_1 < 1{
                        exit(0);
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
                    self.v[ix_out] = 1
                }
                else{
                    self.v[ix_out] = 0
                }
            }
            if op.0 == 8{
                if par_1 == par_2{
                    self.v[ix_out] = 1
                }
                else{
                    self.v[ix_out] = 0
                }
            }
        }
        None
    }
    
}

fn main() {
    let mut file = File::open("C:/users/pcvan/projects/aoc2019/data/ex7.txt").expect("can't open the file");
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    let mut v: Vec<i64> = Vec::new();
    for s in text.split(",") {
        v.push(s.trim().parse::<i64>().unwrap());
    }
    let phases = vec![5,6,7,8,9];
    let mut best_perm: Vec<i64> = phases.clone();
    let mut best_output = 0;
    for perm in phases.iter().permutations(phases.len()).unique() {
        let mut amps: Vec<Amp> = vec![Amp{name: "A".to_string(), v: v.clone(), ix: 0, input: vec![0, *perm[0]]},
        Amp{name: "B".to_string(), v: v.clone(), ix: 0, input: vec![*perm[1]]},
        Amp{name: "C".to_string(), v: v.clone(), ix: 0, input: vec![*perm[2]]},
        Amp{name: "D".to_string(), v: v.clone(), ix: 0, input: vec![*perm[3]]},
        Amp{name: "E".to_string(), v: v.clone(), ix: 0, input: vec![*perm[4]]}];

        let mut ix_amp = 0;
        let mut outputs: Vec<i64> = vec![];
        loop{
            let out: Option<i64> = amps[ix_amp % 5].get_output();
            ix_amp += 1;
            if out.is_some(){
                amps[ix_amp % 5].input.insert(0,out.unwrap());
                if amps[(ix_amp-1) % 5].name == "E"{
                    println!("pushed {}", out.unwrap());
                    outputs.push(out.unwrap());
                }
            }
            else{
                if amps[(ix_amp-1) % 5].name == "E"{
                    if *outputs.iter().max().unwrap() > best_output{
                        best_output = *outputs.iter().max().unwrap();
                        best_perm = perm.into_iter().cloned().collect();
                    }
                    break;
                }
            }
        }   
    }
    println!("best output: {} at {:?}",best_output, best_perm)

    
}

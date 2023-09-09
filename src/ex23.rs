use std::collections::{HashMap, VecDeque};
// file1.rs
use std::fs::File;
use std::io::Read;

// use chrono::Utc;

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
    address: i64,
    input: Vec<i64>,
    out_msg: Vec<i64>,
}

impl Amp{
    fn get_output(&mut self) -> Option<Vec<i64>>{    
        self.out_msg = Vec::new();
        let mut do_return = false;
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
                if op.0 == 3{
                    let input_int: i64;
                    if self.input.len() > 0{
                        input_int = self.input.pop().unwrap();
                    }
                    else if do_return{
                        self.ix += 2;                        
                        return Some(vec![self.address, -1]);
                    }
                    else{
                        do_return = true;
                        input_int = -1;
                    }
                    ix_out = ix_in1;
                    if op.1 == 2{
                        ix_out = ix_in1+self.rel_base
                    }
                    self.v.insert(ix_out, input_int);
                }
                else{
                    self.out_msg.push(i64::try_from(par_1).expect("not"));
                    if self.out_msg.len() == 3{
                        self.ix += 2;
                        return Some(self.out_msg.clone());
                    }
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

    let mut file = File::open("C:/users/pcvan/projects/aoc2019/data/ex23.txt").expect("can't open the file");
    let mut text: String =
     String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    let mut v: HashMap<i64, i64> = HashMap::new();
    for s in text.split(",").into_iter().enumerate() {
        v.insert(i64::try_from(s.0).expect("s.0={s.0}"), s.1.trim().parse::<i64>().expect(s.1));
    }
    let input: Vec<i64> = Vec::new();
    let mut decoders: HashMap<u32,Amp> = HashMap::new();
    let mut msg_queue: HashMap<u32, VecDeque<Vec<i64>>> = HashMap::new();
    for i in 0..50{
        let decoder: Amp = Amp{v: v.clone(), ix: 0, rel_base: 0,  input: input.clone(), out_msg: vec![], address: i64::from(i)};
        decoders.insert(i, decoder);
        msg_queue.insert(i, vec![vec![i64::from(i)]].into());
    }
    let mut msg_address: u32;    
    let mut amp_cycle: u32 = 0;
    let mut nat = vec![0,0,0];
    let mut last_nat_y = 0;
    loop{
        let mut msg = Vec::new();
        let amp: &mut Amp = decoders.get_mut(&amp_cycle).unwrap();
        if msg_queue.get(&amp_cycle).unwrap().len() > 0{
            msg = msg_queue.get_mut(&amp_cycle).unwrap().pop_front().unwrap();
        }
        if msg.len() > 2{
            amp.input.push(*msg.get(2).unwrap());
            amp.input.push(*msg.get(1).unwrap());
        }
        else if msg.len() == 1{
            amp.input.push(*msg.get(0).unwrap());
        }
        
        let new_msg = amp.get_output().unwrap();
        if new_msg.len() > 2{
            msg_address = u32::try_from(new_msg[0]).unwrap();
            if msg_address == 255{
                println!("New nat: {:?}", new_msg);
                nat = new_msg.clone();
                *nat.get_mut(0).unwrap() = 0;
            }
            else{
                msg_queue.get_mut(&msg_address).unwrap().push_back(new_msg.clone());    
            }
        }
        let mut insert_nat = true;
        for a in msg_queue.clone(){
            if a.1.len() > 0{
                insert_nat = false;
                break;
            }
        }
        if insert_nat{
            msg_queue.get_mut(&0).unwrap().push_back(nat.clone()); 
            if last_nat_y == nat[2]{
                break;
            }
            last_nat_y = nat[2];

        }
        amp_cycle += 1;
        amp_cycle %= 50;
    }
    
}
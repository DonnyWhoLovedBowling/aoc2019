use std::collections::{HashMap, HashSet};
// file1.rs
use std::fs::File;
use std::io::Read;

use chrono::Utc;
use plotters::prelude::*;

fn get_v(v: &HashMap<i32, i32>, ix: &i32) -> i32{
    let ix_v: i32 =  *ix;
    if v.contains_key(&ix_v){
        v[&ix_v]
    }
    else{
        0
    }
}
fn test_cube(traction: &HashSet<(i32, i32)>, cube_size: i32, p: (i32, i32)) -> bool{
    if !traction.contains(&p){
        return false;
    }
    else if !traction.contains(&(p.0+(cube_size-1), p.1)){
        return false;
    }
    else if !traction.contains(&(p.0, p.1-(cube_size-1))){
        return false;
    }
    else{
        return true;
    }
}

fn print_traction(traction: &HashSet<(i32,i32)>, scan_size: i32){
    let root_area = BitMapBackend::new("/Users/pcvan/projects/aoc2019/ex19_traction.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let plot_vec_traction: Vec<(f64,f64)> = traction.iter().map(|x| (f64::from(x.0),f64::from(x.1))).collect();
    let mut ctx = ChartBuilder::on(&root_area)
    // .set_label_area_size(LabelAreaPosition::Left, 40.0)
    // .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
    // .set_label_area_size(LabelAreaPosition::Right, 40.0)
    // .set_label_area_size(LabelAreaPosition::Top, 40.0)
    .caption("Tractor beam", ("sans-serif", 40.0))
    .build_cartesian_2d(-1.0..f64::from(scan_size+10), f64::from((-1*scan_size)-10)..1.0)
    .unwrap();
    ctx.draw_series(
        plot_vec_traction.iter().map(|point| Circle::new(*point, 1.0_f64, &RED)),
    ).unwrap();

    ctx.draw_series(
        vec![(0.,0.)].iter().map(|point| Circle::new(*point, 1.0_f64, &BLACK)),
    ).unwrap();

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
    traction: HashSet<(i32,i32)>,
    x: i32,
    y: i32,
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
                    if par_1 == 1{
                        self.traction.insert((self.x,-1*self.y));
                        // println!("{:?}", (self.x,self.y));

                    }
                    return Some(par_1 == 1);
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

fn find_crates(traction: &HashSet<(i32, i32)>){
    let mut x = 5;
    let mut y = -4;
    let mut cube_size = 1;
    let mut n_found = 0;
    let mut n_empty = 0;
    let mut y_start = -4;
    let mut iter = 0;
    loop{
        iter += 1;
        let c = traction.contains(&(x,y));
        if test_cube(&traction, cube_size, (x,y)){
            println!("cube of size {} fits at: {:?}", cube_size, (x,y));
            cube_size += 1;
            iter = 0;
            y = y_start+50;
            x += 1;
            n_found = 0;
            n_empty = 0;
            continue;
        }
        if c && n_found == 0{
            y_start = y;
        }
        if c{
            n_found += 1;
        }
        else{
            n_empty += 1;
        }
        if n_empty > 50{
            y = y_start+40;
            x += 1;
            n_empty = 0;
            n_found = 0;
        }
        else{
            y -= 1;
        }
        if iter > 10000 || cube_size > 100{
            break;
        }
    }

}

fn find_crate(traction: &HashSet<(i32, i32)>, cube_size: i32){
    let mut min_x: i32 = 0;
    let mut max_y: i32 = (-1*cube_size)+2;
    loop {
        max_y -= 1;
        min_x -= 20;
        let mut iter2 = 0;
        while !traction.contains(&(min_x, max_y)) && iter2 < 10000{
            min_x += 1;
            iter2 += 1;
        }
        let max_x = min_x + (cube_size-1);
        let min_y = max_y + (cube_size-1);
        if min_y <= 0 && traction.contains(&(max_x, min_y)) {
            println!("ans: {}", min_x * 10_000 - min_y);
            break;
        }
    }
}


fn main() {
    let mut file_test = File::open("C:/users/pcvan/projects/aoc2019/data/ex19_test.txt").expect("can't open the file");
    let mut text_test: String = String::new();
    file_test.read_to_string(&mut text_test).expect("can't read the file");
    let mut x = 0;
    let mut y = 0;
    let mut test_set: HashSet<(i32,i32)> = HashSet::new();
    for c in text_test.chars(){
        if c == '\n'{
            continue;
        }
        else if c == '\r'{
            y -= 1;
            x = 0;
            continue;
        }
        else if c == '#' || c == 'O'{
            test_set.insert((x,y));
        }
        x += 1;
    }
    find_crates(&test_set);
    find_crate(&test_set, 10);

    let mut file = File::open("C:/users/pcvan/projects/aoc2019/data/ex19.txt").expect("can't open the file");
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    let mut v: HashMap<i32, i32> = HashMap::new();
    for s in text.split(",").into_iter().enumerate() {
        v.insert(i32::try_from(s.0).unwrap(), s.1.trim().parse::<i32>().unwrap());
    }
    let input: Vec<i32> = Vec::new();
    let mut decoder: Amp = Amp{v: v.clone(), ix: 0, rel_base: 0,  input: input, traction: HashSet::new(), x: 0, y:0
        };
    
    let scan_size = 1400;
    let mut n_found = 0;
    let mut n_empty = 0;
    let mut x = 4;
    let mut y = 4;
    let mut x_min = 4;
    let mut start_time = Utc::now().time();    

    loop{
        if x > scan_size{
            break;
        }
        decoder.input = vec![y,x];
        decoder.x = x;
        decoder.y = y;
        decoder.ix = 0;
        decoder.rel_base = 0;
        decoder.v = v.clone();
        let tr = decoder.get_output().unwrap();
        if tr && n_found == 0{
            x_min = x;
        }
        if tr{
            n_found += 1;
        }
        else{
            n_empty += 1;
        }
        if n_empty > 30{
            let end_time: chrono::NaiveTime = Utc::now().time();
            let diff = end_time - start_time;
            println!("Total time taken for x = {}: {} s, row_length: {}", y-1, (diff.num_microseconds().unwrap() as f64)/1000000.0, n_found);

            y += 1;
            x = x_min-20;
            n_empty = 0;
            n_found = 0;
            start_time = Utc::now().time();        
        }
        else{
            x += 1;
        }
    }
    print_traction(&decoder.traction, scan_size);
    println!("number of traction points: {}", decoder.traction.len());
    find_crates(&decoder.traction);
    find_crate(&decoder.traction, 100);

}
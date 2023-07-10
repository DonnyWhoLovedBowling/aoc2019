use std::collections::{HashMap, HashSet};
// file1.rs
use std::fs::File;
use std::io::Read;

// use plotters::prelude::*;

fn next_move(cur_pos: &(i32, i32), cur_dir: &(i32, i32), scaffolding: &HashSet<(i32, i32)>) -> String{
    let mut incr_x = cur_dir.0;
    let mut incr_y: i32 = cur_dir.1;
    let mut i = 1;
    while scaffolding.contains(&(cur_pos.0+incr_x*i, cur_pos.1+incr_y*i)){
        i += 1;
    }
    if i > 1{
        return (i-1).to_string();
    }
    incr_x = -1*cur_dir.1;
    incr_y = cur_dir.0;
    if scaffolding.contains(&(cur_pos.0+incr_x, cur_pos.1+incr_y)){
        return "L".to_string();
    }
    incr_x = cur_dir.1;
    incr_y = -1*cur_dir.0;
    if scaffolding.contains(&(cur_pos.0+incr_x, cur_pos.1+incr_y)){
        return "R".to_string();
    }
    "ERROR".to_string()
}
fn calc_score(p: (i32, i32), scaffolding: &HashSet<(i32, i32)>) -> i32{
    for dx in i32::from(-1)..2{
        for dy in i32::from(-1)..2{
            if dx.abs() + dy.abs() != 1{
                continue
            }
            let p_test = (p.0+dx, p.1+dy);
            if !scaffolding.contains(&p_test){
                return 0;
            }
        }
    }
    p.0*p.1
}

fn get_v(v: &HashMap<i32, i32>, ix: &i32) -> i32{
    let ix_v: i32 =  *ix;
    if v.contains_key(&ix_v){
        v[&ix_v]
    }
    else{
        0
    }
}

// fn print_scaffolding(scaffolding: &HashSet<(i32,i32)>, robot: &(i32, i32)){
//     let root_area = BitMapBackend::new("/Users/pcvan/projects/aoc2019/ex17_scaffolding.png", (600, 400)).into_drawing_area();
//     root_area.fill(&WHITE).unwrap();

//     let plot_vec_scaffolding: Vec<(f64,f64)> = scaffolding.iter().map(|x| (f64::from(x.0),f64::from(x.1))).collect();
//     let mut ctx = ChartBuilder::on(&root_area)
//     // .set_label_area_size(LabelAreaPosition::Left, 40.0)
//     // .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
//     // .set_label_area_size(LabelAreaPosition::Right, 40.0)
//     // .set_label_area_size(LabelAreaPosition::Top, 40.0)
//     .caption("Walls", ("sans-serif", 40.0))
//     .build_cartesian_2d(-10.0..120.0, -160.0..5.0)
//     .unwrap();
//     ctx.draw_series(
//         plot_vec_scaffolding.iter().map(|point| Circle::new(*point, 1.0_f64, &RED)),
//     ).unwrap();

//     ctx.draw_series(
//         vec![(0.,0.)].iter().map(|point| Circle::new(*point, 1.0_f64, &BLACK)),
//     ).unwrap();

//     let p_x: f64 =  f64::from(robot.0);
//     let p_y: f64 =  f64::from(robot.1);
//     ctx.draw_series(
//         vec![(p_x, p_y)].iter().map(|point| Circle::new(*point, 1.0_f64, &YELLOW)),
//     ).unwrap();


// }


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
    scaffolding: HashSet<(i32,i32)>,
    robot: (i32,i32),
    x: i32,
    y: i32,
    input: Vec<i32>
}

impl Amp{
    fn get_output(&mut self) -> Option<i32>{    
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
                    println!("getting input: {} {}",get_v(&self.v, &self.ix), get_v(&self.v, &(self.ix+1)));
                    let input_int: i32 = self.input.pop().unwrap();
                    ix_out = ix_in1;
                    if op.1 == 2{
                        ix_out = ix_in1+self.rel_base
                    }
                    self.v.insert(ix_out, input_int);
                    self.x = 0;
                    self.y = 0;
                    self.scaffolding = HashSet::new();
                }
                else{
    
                    if par_1 == 35{
                        self.scaffolding.insert((self.x,self.y));
                        self.x += 1;
                    }
                    else if par_1 == 10{
                        self.x = 0;
                        self.y -= 1;
                    }
                    else if par_1 == 46{
                        self.x += 1;
                    }
                    else if par_1 == 60 || par_1 == 62 || par_1 == 94 || par_1 == 118{
                        self.robot = (self.x, self.y);
                        self.x += 1;
                        // print_scaffolding(&self.scaffolding, &self.robot);
                        // self.scaffolding = HashSet::new();
                    }
                    else {
                        print!("{} ", par_1);
                        self.x += 1;
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


fn do_run(init_num: i32, input: Vec<i32>) {
    let mut file = File::open("C:/users/pcvan/projects/aoc2019/data/ex17.txt").expect("can't open the file");
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    let mut v: HashMap<i32, i32> = HashMap::new();
    for s in text.split(",").into_iter().enumerate() {
        v.insert(i32::try_from(s.0).unwrap(), s.1.trim().parse::<i32>().unwrap());
    }
    v.insert(0, init_num);

    let mut decoder: Amp = Amp{v: v.clone(), ix: 0, rel_base: 0, robot: (0,0), input: input, scaffolding: HashSet::new(), x: 0, y:0
        };

    decoder.get_output();
    let mut total_score = 0;
    for p in &decoder.scaffolding{
        total_score += calc_score(*p, &decoder.scaffolding);
    }
    println!("total score: {}", total_score);

    let mut moves: Vec<String> = Vec::new();
    let mut pos = decoder.robot;
    let mut dir = (0,1);
    loop{
        let m = next_move(&pos, &dir, &decoder.scaffolding);
        moves.push(m.clone());
        if m.parse::<i32>().is_ok(){
            let i = m.parse::<i32>().unwrap();
            pos = (pos.0+i*dir.0, pos.1+i*dir.1);
        }
        if m == "L"{
            let new_x = -1*dir.1;
            let new_y = dir.0;
            dir = (new_x, new_y);
        }
        if m == "R"{
            let new_x = dir.1;
            let new_y = -1*dir.0;
            dir = (new_x, new_y);
        }
        if m == "ERROR"{
            break;
        }
    }
    println!("{:?}",moves);


}

fn main() {
    do_run(1, vec![]);

    let mut input = vec![65, 44, 67, 44, 65, 44, 67, 44, 66, 44, 66, 44, 67, 44, 65, 44, 67, 44, 66, 10];
    let mut a: Vec<i32> = vec![76, 44, 49, 48, 44, 82, 44, 49, 48, 44, 76, 44, 49, 48, 44, 76, 44, 49, 48, 10];
    let mut b = vec![82, 44, 49, 50, 44, 76, 44, 49, 50, 44, 82, 44, 54, 10];
    let mut c = vec![82, 44, 49, 48, 44, 82, 44, 49, 50, 44, 76, 44, 49, 50, 10];

    input.append(&mut a);
    input.append(&mut b);
    input.append(&mut c);
    input.push(110);
    // input.push(89);
    input.push(10);

    input = input.into_iter().rev().collect();

    do_run(2, input);

}
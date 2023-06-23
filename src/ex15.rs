use std::collections::HashMap;
use std::collections::HashSet;
// file1.rs
use std::fs::File;
use std::io::Read;
use rand::Rng;
use plotters::prelude::*;
use priority_queue::PriorityQueue ;
use std::cmp::Reverse;


fn shortest_path(neighbours: &HashMap<(i32, i32), HashSet<(i32, i32)>>, start_point: (i32, i32),  end_point: (i32, i32)) ->  Vec<(i32,i32)>{
    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();
    let mut parents: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut pq: PriorityQueue <(i32, i32), std::cmp::Reverse<i32>> = PriorityQueue::new();
    pq.push(start_point, Reverse(0));
    let mut i = 0;
    while pq.len() > 0{
        if !neighbours.contains_key(&end_point){
            println!("ERROR, key not in neighbor list");
        }
        i += 1;
        let point: ((i32, i32), Reverse<i32>) = pq.pop().unwrap();
        if visited.contains_key(&end_point){
            let mut ret_vec: Vec<(i32, i32)> = Vec::new();
            let mut vtx = end_point;
            while vtx != start_point{
                ret_vec.push(vtx);
                vtx = parents[&vtx];
            }
            ret_vec.reverse();
            return ret_vec;
        }
        visited.insert(point.0, point.1.0);
        let Reverse(current_prio_origin) = point.1.clone();
        let ns: Option<&HashSet<(i32, i32)>> = neighbours.get(&point.0);
        if ns == None{
            println!("?");
        }
        for n in ns.unwrap(){
            if visited.contains_key(n){
                continue;
            }
            parents.insert(*n, point.0);
            pq.push(*n, Reverse(current_prio_origin + 1));
        }  
        if i % 10000 == 0{
            println!("{} {:?}", pq.len(), pq.peek());
        }
    }
    Vec::new()
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

fn calc_input(dx: i8, dy: i8) -> i32{
    if dy == -1{
        return 1;
    }
    if dy == 1{
        return 2;
    }
    if dx == -1{
        return 3;
    }
    if dx == 1{
        return 4;
    }
    0
}
fn calc_pos(input: i32, cur_pos: (i32,i32)) -> (i32, i32){
    if input == 1{
        return (cur_pos.0, cur_pos.1 - 1);
    }
    if input == 2{
        return (cur_pos.0, cur_pos.1 + 1);
    }
    if input == 3{
        return (cur_pos.0 -1, cur_pos.1);
    }
    if input == 4{
        return (cur_pos.0 + 1, cur_pos.1);
    }
    cur_pos

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
    wall_points: HashSet<(i32,i32)>,
    open_points: HashSet<(i32,i32)>,
    oxy_tank: (i32,i32),
    current_position: (i32, i32),
    test_position: (i32,i32),
    rng: rand::rngs::ThreadRng,    
    neighbours: HashMap<(i32,i32), HashSet<(i32,i32)>>,                                       
}

impl Amp{
    fn add_neighbor(&mut self){
        if self.neighbours.contains_key(&self.current_position){
            self.neighbours.get_mut(&self.current_position).unwrap().insert(self.test_position);
        }
        else{
            self.neighbours.insert(self.current_position, 
                vec![self.test_position].into_iter().collect());
        }   
        if self.neighbours.contains_key(&self.test_position){
            self.neighbours.get_mut(&self.test_position).unwrap().insert(self.current_position);
        }
        else{
            self.neighbours.insert(self.test_position, 
                vec![self.current_position].into_iter().collect());
        }   
        self.open_points.insert(self.test_position);
        self.current_position = self.test_position;
    }
    fn get_output(&mut self) -> Option<i32>{    
        while get_v(&self.v, &self.ix) != 99{
            let op: (i32, i32, i32, i32) = parse_op(self.v[&self.ix]);
            if op.0 == 99 || self.oxy_tank != (0,0) || self.wall_points.len() > 10000{
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
                self.ix += 2;
                if op.0 == 3{
                    let mut new_input: Vec<i32> = Vec::new();
                    let mut backup_input: Vec<i32> = Vec::new();
                    for dx in -1..2{
                        let dx = i8::from(dx);
                        for dy in -1..2{
                            let dy = i8::from(dy);
                            if dx.abs()+dy.abs() != 1{
                                continue;
                            }
                            self.test_position = self.current_position.clone();
                            self.test_position.0 = self.test_position.0.wrapping_add(dx.try_into().unwrap());
                            self.test_position.1 = self.test_position.1.wrapping_add(dy.try_into().unwrap());
                            if self.wall_points.contains(&self.test_position){
                                continue;
                            }
                            if self.open_points.contains(&self.test_position){

                                backup_input.push(calc_input(dx, dy));
                                continue
                            }
                            new_input.push(calc_input(dx,dy));
                        } 
                    }
                    if new_input.len() == 0{
                        if backup_input.len() > 0{
                            new_input = backup_input.clone();
                        }
                        else{
                            new_input.push(self.rng.gen_range(1..5))

                        }
                    }
                    let input_int = new_input[self.rng.gen_range(0..new_input.len())];
                    self.v.insert(ix_in1, input_int);
                    self.test_position = calc_pos(input_int, self.current_position);
                    println!("{} {}  {:?} {:?}", self.wall_points.len(), self.open_points.len(), self.test_position, self.current_position)
                }
                else{
                    if par_1 == 0{
                        self.wall_points.insert(self.test_position);
                    }
                    else if par_1 == 1
                    {
                        self.add_neighbor();
                    }
                    else if par_1 == 2{
                        self.open_points.insert(self.test_position);
                        self.oxy_tank = self.test_position;
                        self.add_neighbor();
                        break;
                    }
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


fn main() {
    let mut file = File::open("C:/users/pcvan/projects/aoc2019/data/ex15.txt").expect("can't open the file");
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    let mut v: HashMap<i32, i32> = HashMap::new();

    for s in text.split(",").into_iter().enumerate() {
        v.insert(i32::try_from(s.0).unwrap(), s.1.trim().parse::<i32>().unwrap());
    }
    let mut decoder: Amp = Amp{v: v.clone(), ix: 0, rel_base: 0, 
        wall_points: HashSet::new(), open_points: HashSet::new(), 
        oxy_tank: (0,0), current_position: (0,0), test_position: (0,0),
        rng: rand::thread_rng(), neighbours: HashMap::new()};

    decoder.get_output();
    let sp_ex1 = shortest_path(&decoder.neighbours, (0,0), decoder.oxy_tank);

    let plot_vec_walls: Vec<(f64,f64)> = decoder.wall_points.iter().map(|x| (f64::from(x.0),f64::from(x.1))).collect();
    let plot_vec_tested: Vec<(f64,f64)> = decoder.open_points.iter().map(|x| (f64::from(x.0),f64::from(x.1))).collect();
    let plot_vec_oxy_tank:Vec<(f64,f64)> = vec![(decoder.oxy_tank.0.into(), decoder.oxy_tank.1.into())];
    let plot_vec_origin:Vec<(f64,f64)> = vec![(0., 0.)];
    let sp_ex1: Vec<(f64,f64)> = sp_ex1.iter().map(|x| (f64::from(x.0),f64::from(x.1))).collect();

    let root_area = BitMapBackend::new("/Users/pcvan/projects/aoc2019/ex15_walls.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
    // .set_label_area_size(LabelAreaPosition::Left, 40.0)
    // .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
    // .set_label_area_size(LabelAreaPosition::Right, 40.0)
    // .set_label_area_size(LabelAreaPosition::Top, 40.0)
    .caption("Walls", ("sans-serif", 40.0))
    .build_cartesian_2d(-30.0..30.0, -30.0..30.0)
    .unwrap();
    ctx.draw_series(
        plot_vec_walls.iter().map(|point| Circle::new(*point, 4.0_f64, &RED)),
    ).unwrap();
    ctx.draw_series(
        plot_vec_tested.iter().map(|point| Circle::new(*point, 4.0_f64, &BLUE)),
    ).unwrap();
    ctx.draw_series(
        plot_vec_oxy_tank.iter().map(|point| Circle::new(*point, 6.0_f64, &BLACK)),
    ).unwrap();
    ctx.draw_series(
        plot_vec_origin.iter().map(|point| Circle::new(*point, 6.0_f64, &BLACK)),
    ).unwrap();
    ctx.draw_series(
        LineSeries::new(sp_ex1.clone(), &BLACK)
    ).unwrap();

    println!("{:?}", sp_ex1.len());
    let mut max = 0;
    let mut max_path = Vec::new();
    for p in decoder.open_points{
        let sp = shortest_path(&decoder.neighbours, decoder.oxy_tank, p);
        if sp.len() > max{
            max = sp.len();
            max_path = sp;
        }
    }
    let sp_ex2: Vec<(f64,f64)> = max_path.iter().map(|x| (f64::from(x.0),f64::from(x.1))).collect();

    ctx.draw_series(
        LineSeries::new(sp_ex2.clone(), Into::<ShapeStyle>::into(&YELLOW).stroke_width(3))
    ).unwrap();
    println!("{}",max)

}

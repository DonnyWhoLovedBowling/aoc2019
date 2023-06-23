mod read_lines;
use read_lines::Lines;
use std::collections::HashMap;
use std::io;
use std::fs::File;
use std::cmp::Ordering;

fn calc_angle(base: (f32, f32), target: (f32, f32)) -> f32{
    let mut angle = 0.;
    if base.0 == target.0 && base.1 == target.1{
        return angle;
    }
    else if base.0 == target.0{
        if (base.1 - target.1) > 0. {
            angle = std::f32::consts::PI/2.;
        }
        else{
            angle = 1.5 * std::f32::consts::PI;
        }
    }
    else if base.1 == target.1{
        if base.0 > target.0{
            angle = std::f32::consts::PI;                    
        }
        else{
            angle = 0.;
        }
    }
    else if base.0 >= target.0 && base.1 >= target.1{
        // quadrant upper-left
        angle = std::f32::consts::PI - f32::from((target.1-base.1).abs()/(target.0-base.0).abs()).atan();
    }
    else if base.0 >= target.0 && base.1 <= target.1{
        // quadrant lower-left
        angle = f32::from((target.1-base.1).abs()/(target.0-base.0).abs()).atan()+std::f32::consts::PI;
    }
    else if base.0 <= target.0 &&  base.1 >= target.1{
        // quadrant upper-right
        angle = f32::from((target.1-base.1).abs()/(target.0-base.0).abs()).atan();
    }
    else if base.0 <= target.0 &&  base.1 <= target.1{
        // quadrant lower-right
        angle = 2.*std::f32::consts::PI - f32::from((target.1-base.1).abs()/(target.0-base.0).abs()).atan();
    }
    angle
}

fn cmp_f32(a: &f32, b: &f32) -> Ordering {
    if a.is_nan() {
        return Ordering::Greater;
    }
    if b.is_nan() {
        return Ordering::Less;
    }
    if a < b {
        return Ordering::Less;
    } else if a > b {
        return Ordering::Greater;
    }
    return Ordering::Equal;
}
fn decr_cmp_f32(a: &f32, b: &f32) -> Ordering{
    match cmp_f32(a, b){
        Ordering::Greater => Ordering::Less,
        Ordering::Less => Ordering::Greater,
        Ordering::Equal => Ordering::Equal,
    }
}

fn angle_to_vert(a: f32) -> f32{
    if a > std::f32::consts::PI/2.{
        a-std::f32::consts::PI/2.
    }
    else if (a-(std::f32::consts::PI/2.)).abs() < 0.0001{
        2.*std::f32::consts::PI
    }
    else{
        a+std::f32::consts::PI*1.5
    }
}


fn vert_to_angle(v: f32) -> f32{
    if v <= std::f32::consts::PI*1.5{
        v+std::f32::consts::PI/2.
    }
    else if (v-std::f32::consts::PI).abs() < 0.0001{
        std::f32::consts::PI/2.
    }
    else{
        v-std::f32::consts::PI*1.5
    }
}

fn main() -> io::Result<()> {
    let file = File::open("C:/users/pcvan/projects/aoc2019/data/ex10.txt")?;

    let mut lines = Lines::new(file);
    let mut astroid_vec = Vec::new();
    let mut line_nr: f32 = 0.;
    while let Some(line) = lines.next() {
        let mut col_nr: f32 = 0.; 
        for c in line?.parse::<String>().unwrap().chars().into_iter(){
            if c == '#'{
                astroid_vec.push((col_nr, line_nr));
            }
            col_nr += 1.;
        }
        line_nr += 1.;
    }
    let mut max = 0;
    let mut max_base = (-1.,-1.);    
    for base in astroid_vec.iter(){
        let mut angles: Vec<f32> = Vec::new();
        for target in astroid_vec.iter(){
            let angle = calc_angle(*base, *target);
            let mut found = false;
            if *base == *target{
                continue;
            }
            for ex_angle in angles.iter(){
                if (ex_angle-angle).abs() < 0.0001{
                    found = true;
                    break;
                }
            }
            if !found{
                angles.push(angle);
            }
        
        }
        if angles.len() > max{
            max = angles.len();
            max_base = base.clone();
        }
    }
    println!("{}", max);
    println!("{:?}", max_base);
    let mut polar_map: HashMap<String, Vec<f32>> = HashMap::new();
    let mut max_angles: Vec<f32> = Vec::new();

    
    max_angles.sort_by(decr_cmp_f32);
    for target in astroid_vec.iter(){
        if *target == max_base{
            continue;
        }
        let angle = calc_angle(max_base, *target);
        let r1 = (max_base.0-target.0).powi(2);
        let r2 = (max_base.1-target.1).powi(2);
        let r12 = r1+r2;

        let r = f32::sqrt(r12);
        let v = angle_to_vert(angle);
        if !max_angles.contains(&v){
            max_angles.push(v);
        }

        let angle_str: String = format!("{:.4}", v);
        if polar_map.contains_key(&angle_str){
            polar_map.get_mut(&angle_str).unwrap().push(r);
        }
        else{
            polar_map.insert(angle_str.clone(), vec![r]);
        }
    }
    max_angles.sort_by(decr_cmp_f32);
    for k in polar_map.clone().keys(){
        polar_map.get_mut(k).unwrap().sort_by(decr_cmp_f32);
    }
    let mut i = 0; 
    for a in max_angles.iter(){
        let a_str = format!("{:.4}", a);
        let r = polar_map.get_mut(&a_str).unwrap().pop().unwrap();
        if polar_map[&a_str].len() == 0{
            polar_map.remove(&a_str);
        }
        i += 1;
        
        let a_corr = vert_to_angle(*a); 
        let x = a_corr.cos()*r;
        let y = a_corr.sin()*r; 
        // println!("{},{},{}", i, x , y);
        println!("{},{},{}", i, max_base.0+x , max_base.1-y);
        if i == 200{
            println!("{}", (max_base.0+x) *100. + max_base.1-y );
            break;
        }
}

    Ok(()) 
}

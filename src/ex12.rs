// file1.rs
mod read_lines;
use read_lines::Lines;
use substring::Substring;
use std::fs::File;

fn change_vel(pos: &Vec<Vec<i32>>, vel: &mut Vec<Vec<i32>>, ix_first: usize, ix_second: usize, ix_dim: usize){
    if pos[ix_first][ix_dim] > pos[ix_second][ix_dim]{
        vel[ix_first][ix_dim] -= 1;
        vel[ix_second][ix_dim] += 1;
    }
    else if pos[ix_first][ix_dim] < pos[ix_second][ix_dim]{
        vel[ix_first][ix_dim] += 1;
        vel[ix_second][ix_dim] -= 1;        
    }
}

fn main() {
    let fname = "C:/users/pcvan/projects/aoc2019/data/ex12.txt";
    println!("{fname}");
    let file: File = File::open(fname).expect("error opening file");
    let mut lines: Lines<File> = Lines::new(file);
    let mut moons_pos: Vec<Vec<i32>> = Vec::new();
    let mut moons_vel: Vec<Vec<i32>> = Vec::new();
    while let Some(line) = lines.next() {
        let line: &str = line.expect("prob");
        let mut ix = line.find("x=").unwrap();
        let mut ix2 = line.find(",").unwrap();
        let x = line.to_string().substring(ix+2,ix2).parse::<i32>().unwrap();
        ix = line.find("y=").unwrap();
        ix2 = line.find("z=").unwrap();
        let y = line.substring(ix+2,ix2-2).parse::<i32>().unwrap();
        ix = line.find("z=").unwrap();
        ix2 = line.find(">").unwrap();
        let z = line.substring(ix+2,ix2).parse::<i32>().unwrap();
        moons_pos.push(vec![x,y,z]);
        moons_vel.push(vec![0,0,0]);

    }
    let init_pos = moons_pos.clone();
    let init_vel = moons_vel.clone();
    println!("{:?}",moons_pos);
    println!("{:?}",moons_vel);
    let mut last_hit = 0;
    for _i in 0..10000000{
        for i in 0..moons_pos.len(){
            for j in i+1..moons_pos.len(){
                for k in 0..3{
                    change_vel(&moons_pos, &mut moons_vel, i,j, k);
                }
            }
        }
        for ix_moon in 0..moons_pos.len(){
            for ix_dim in 0..3{
                moons_pos[ix_moon][ix_dim] += moons_vel[ix_moon][ix_dim];
            }
        }
        let ix_moon = 3;
        let ix_dim = 0;
        
        if moons_pos[ix_moon][ix_dim] == init_pos[ix_moon][ix_dim] && moons_vel[ix_moon][ix_dim] == init_vel[ix_moon][ix_dim]{
            println!("{_i}, {}: {:?}, {:?}", _i-last_hit, moons_pos[ix_moon], moons_vel[ix_moon]);
            last_hit = _i;
        }
        if moons_pos[ix_moon] == init_pos[ix_moon] && moons_vel[ix_moon] == init_vel[ix_moon]{
            println!("{_i}, {}: {:?}, {:?}", _i-last_hit, moons_pos[ix_moon], moons_vel[ix_moon]);
            break;
        }

    }
    println!("{:?}",moons_pos);
    println!("{:?}",moons_vel);
    let mut tot_energy: i32 = 0;

    for moon_i in 0..moons_pos.len(){
        let mut pot_energy = 0;
        let mut kin_energy = 0;    
        for dim_i in 0..3{
            pot_energy += moons_pos[moon_i][dim_i].abs();
            kin_energy += moons_vel[moon_i][dim_i].abs();
        }
        tot_energy += pot_energy*kin_energy;
    }
    println!("{}", tot_energy);


}

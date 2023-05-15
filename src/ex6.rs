use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;

use std::fs::File;
use std::io;
mod read_lines;
use read_lines::Lines;

fn count_orbits(planet: &String, depth: i32, orbit_map: &HashMap<String, Vec<String>>) -> i32{
    let mut new_count: i32 = depth;
    if orbit_map.contains_key(planet){
        if orbit_map[planet].len() > 1{
            println!("vec length: {}", orbit_map[planet].len());
        }
        for p in orbit_map[planet].iter(){
            new_count += count_orbits(p, depth+1, orbit_map);
        }
    }
    return new_count
}

fn find_lineage(planet: &String, orbit_map: &HashMap<String, Vec<String>>, lineage: &mut Vec<String>) -> Vec<String>{
    let last_planet = lineage.last().unwrap().to_string();
    if lineage.contains(&planet){
        return lineage.to_vec();
    }
    else{
        if !orbit_map.contains_key(&last_planet){
            return lineage.to_vec();
        }
        for p in orbit_map[&last_planet].iter(){
            let mut new_lineage: Vec<String> = lineage.to_vec();
            new_lineage.push(p.to_string());
            let new_lineage: Vec<String> = find_lineage(planet, orbit_map, &mut new_lineage);
            if new_lineage.contains(&planet){
                return new_lineage;
            }
        }    
    }    
    lineage.to_vec()
}

fn main() -> io::Result<()> {
    let mut orbit_map: HashMap<String, Vec<String>> = HashMap::new();
    let file: File = File::open("C:/users/pcvan/projects/aoc2019/data/ex6.txt")?;
    let mut lines: Lines<File> = Lines::new(file);
    while let Some(line) = lines.next() {
        let line: &str = line.expect("prob");
        let lst: Vec<&str> = line.split(')').collect();
        let a: String = lst[0].to_string();
        let b: String = lst[1].to_string();
        if orbit_map.contains_key(&a){
            orbit_map.get_mut(&a).unwrap().push(b.to_string());
        }
        else{
            orbit_map.insert(a.to_string(), vec![b.to_string()]);
        }
    }
    let total_orbits: i32 = count_orbits(&String::from("COM"), 0, &orbit_map);
    println!("ans 1 = {} ", total_orbits);
    let mut lineage: Vec<String> = vec!["COM".to_string()];
    let you_path = find_lineage(&String::from("YOU"), &orbit_map, &mut lineage);
    let san_path = find_lineage(&String::from("SAN"), &orbit_map, &mut lineage);

    println!("you path = {:?} ", you_path);
    println!("santa path = {:?} ", san_path);

    let you_set: HashSet<String> = HashSet::from_iter(you_path);
    let san_set: HashSet<String> = HashSet::from_iter(san_path);

    let union: HashSet<&String> = you_set.union(&san_set).collect();
    let inters: HashSet<&String> = you_set.intersection(&san_set).collect();
    let shortest_path: HashSet<&&String> = union.difference(&inters).collect();
    println!("shortest path = {:?} ", shortest_path);

    print!("shortest path length to santa: {} ", shortest_path.len());
    Ok(())
    
}
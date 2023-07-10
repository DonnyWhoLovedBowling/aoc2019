use std::collections::HashMap;
use std::collections::HashSet;
// file1.rs
use std::fs::File;
mod read_lines;
use read_lines::Lines;
use priority_queue::PriorityQueue ;
use std::cmp::Reverse;

fn get_neighbours(point: (i32, i32), points: &HashMap<(i32, i32), char>) -> HashSet<(i32, i32)>{
    let mut neighbours: HashSet<(i32, i32)> = HashSet::new();
    for dx in -1..2{
        for dy in -1..2{
            if i32::from(dx+dy).abs() != 1{
                continue;
            }
            let p_test = ((point.0 + dx), (point.1 + dy));
            if points.contains_key(&p_test){
                neighbours.insert(p_test);                    
            } 
        }
    }
    neighbours
}

fn shortest_path(start_point: (i32, i32),  end_point: (i32, i32), points: &HashMap<(i32,i32), char>) ->  Vec<(i32,i32)>{
    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();
    let mut parents: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut pq: PriorityQueue <(i32, i32), std::cmp::Reverse<i32>> = PriorityQueue::new();
    pq.push(start_point, Reverse(0));
    let mut i = 0;
    while pq.len() > 0{
        if !points.contains_key(&end_point){
            println!("ERROR, key not in neighbor list");
        }
        i += 1;
        let point: ((i32, i32), Reverse<i32>) = pq.pop().unwrap();
        if visited.contains_key(&end_point){
            break;
        }
        visited.insert(point.0, point.1.0);
        let Reverse(current_prio_origin) = point.1.clone();
        let ns: HashSet<(i32, i32)> = get_neighbours(point.0, &points);
        for n in ns{
            if visited.contains_key(&n){
                continue;
            }
            parents.insert(n, point.0);
            pq.push(n, Reverse(current_prio_origin + 1));
        }  
        if i % 10000 == 0{
            println!("{} {:?}", pq.len(), pq.peek());
        }
    }
    let mut ret_vec: Vec<(i32, i32)> = Vec::new();
    let mut vtx = end_point;
    while vtx != start_point{
        ret_vec.push(vtx);
        vtx = parents[&vtx];
    }
    ret_vec.reverse();
    return ret_vec;

}



fn main() {
    let file = 
    File::open("C:/users/pcvan/projects/aoc2019/data/ex18_test.txt").expect("can't open the file");
    let mut lines: Lines<File> = Lines::new(file);
    let mut x = 0;
    let mut y = 0;
    let mut all_points: HashMap<(i32, i32), char> = HashMap::new();
    let mut keys: HashMap<(i32, i32), char> = HashMap::new();
    let mut route_lengths: HashMap<(char, char), i32> = HashMap::new();
    let mut route_needs: HashMap<(char, char), HashSet<char>> = HashMap::new();

    let mut doors: HashMap<(i32, i32), char> = HashMap::new();
    while let Some(line) = lines.next(){
        for c in line.unwrap().chars(){
            if c != '#'{
                all_points.insert((x, y), c);
            }
            if (c.is_alphabetic() && c.is_lowercase()) || c == '@'{
                keys.insert((x, y), c);
            }
            else if c.is_alphabetic(){
                doors.insert((x, y), c);
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }
    println!("neighbours of x: {:?}", get_neighbours((79,29), &all_points));
    for s in keys.clone(){
        for e in keys.clone(){
            if s == e{
                continue;
            } 
            let path = shortest_path(s.0, e.0, &all_points);
            // let mut add = true;
            let mut needs = HashSet::new();
            let path_len = i32::try_from(path.len()).unwrap();
            for p in path{
                // if keys.contains_key(&p) && p != e.0 && p != s.0{
                //     // Deze route moet worden opgebroken;
                //     add = false;
                //     break;
                // }
                if doors.contains_key(&p){
                    let c: &char = doors.get(&p).unwrap();
                    needs.insert(*c);
                }
            }
            // if add{
            route_lengths.insert((s.1, e.1), path_len);
            route_needs.insert((s.1, e.1), needs);
            // }
        }

    }
    println!("route needs: {:?}", route_needs);

    let mut keys_str: Vec<char> = keys.values().copied().collect();
    keys_str.retain(|&x| x != '@');
    
    // let mut pq: PriorityQueue <(Vec<char>, char, Vec<char>), std::cmp::Reverse<i32>> = PriorityQueue::new();
    let mut pq: Vec<(Vec<char>, char, Vec<char>, i32)> = Vec::new();
    // pq.push((keys_str.clone(), '@', Vec::new()), Reverse(0));
    pq.push((keys_str.clone(), '@', Vec::new(), 0));

    let mut min_length = 1000000;
    let mut shortest_path: Vec<char> = Vec::new();
    loop{
        let state: Option<(Vec<char>, char, Vec<char>, i32)> = pq.pop();
        if state == None{
            break;
        }
        // let state_unwrapped: ((Vec<char>, char, Vec<char>), Reverse<i32>) = state.unwrap();
        let state_unwrapped: (Vec<char>, char, Vec<char>, i32) = state.unwrap();
        
        let unvisited: Vec<char> = state_unwrapped.0.clone();
        let s = state_unwrapped.1;
        for e in unvisited.clone(){
            if e == s{
                println!("start = end! {}", e);
                continue;
            }
            let mut visited = state_unwrapped.2.clone();
            visited.push(e);
            let mut new_unvisited = unvisited.clone();
            new_unvisited.retain(|&x| x != e);
            let r = (s,e);
            let mut needs_ok = true;
            if !route_needs.contains_key(&r){
                continue;
            }
            for c in route_needs.get(&r).unwrap(){
                let cl = c.to_lowercase().collect::<Vec<_>>().get(0).unwrap().clone();
                if unvisited.contains(&cl){
                    needs_ok = false;
                    // println!("route {:?} needs: {}, which is unvisited: {:?}", r, cl, unvisited);
                    break;
                }
            }
            if needs_ok{
                // let Reverse(current_prio_origin) = state_unwrapped.1.clone();
                let current_prio_origin = state_unwrapped.3;
                let new_dist = current_prio_origin + *route_lengths.get(&r).unwrap();  
                if new_unvisited.len() > 0{
                    pq.push((new_unvisited,e, visited, new_dist));
                }
                else{
                    // println!("found full path! {}: {:?}", new_dist, visited);
                    if new_dist < min_length{
                    println!("found shorter path! {}: {:?}", new_dist, visited);

                        shortest_path = visited;
                        min_length = new_dist;
                    }
                }
            }    
        }
    }
    println!("min_length: {}: {:?}", min_length, shortest_path);

}


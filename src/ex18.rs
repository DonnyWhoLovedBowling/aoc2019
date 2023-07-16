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

fn get_accessible_routes(starts: &Vec<char>, has_keys: &Vec<char>, all_keys: &Vec<char>, route_needs: &HashMap<(char, char), HashSet<char>>, quadrants: &HashMap<char, char>) -> Vec<(char, char)>{
    let mut ret_vec: Vec<(char, char)> = Vec::new();
    let mut pt2 = false;
    if starts.len() > 1{
        pt2 = true;
    }
    for e in all_keys{
        for start in starts{
            let r = (*start, *e);
            let mut needs_ok = true;
            if has_keys.contains(&e){
                continue;
            }
            if pt2 && (quadrants.get(e) != quadrants.get(start)){
                continue;
            }
            for c in route_needs.get(&r).unwrap(){
                let cl = c.to_lowercase().collect::<Vec<_>>().get(0).unwrap().clone();
                if !has_keys.contains(&cl){
                    needs_ok = false;
                    // println!("route {:?} needs: {}, which is unvisited: {:?}", r, cl, unvisited);
                    break;
                }
            }
            if needs_ok{
                ret_vec.push((*start, *e));
            }
        }
    }
    ret_vec
}

fn next_step(start: &Vec<char>, route: &Vec<char>, memmap: &mut HashMap<Vec<char>, (Vec<char>,i32)>,
    route_needs: &HashMap<(char, char), HashSet<char>>, route_lengths: &HashMap<(char, char), i32>, 
    all_keys: &Vec<char>, quadrants: &HashMap<char, char>) -> (Vec<char>, i32){
    let mut key = route.clone();
    key.sort();
    if start.len() == 1{
        key.push(*start.get(0).unwrap());
    }
    else{
        for ix in 0..4{
            key.push(*start.get(ix).unwrap());
        }
    }
    let l: Option<&(Vec<char>, i32)> = memmap.get(&key);
    if l != None{
        let nl = l.unwrap();
        return nl.clone();

    }
    if route.len() == all_keys.len(){
        let nr = route.to_owned();
        return (nr, 0);
    }
    let mut min_dist = 1000000;
    let mut min_path: Vec<char> = Vec::new();
    let a_keys: Vec<(char, char)> = get_accessible_routes(start, route, all_keys, route_needs, &quadrants);
    for r in a_keys{
        let mut new_starts = start.clone();
        new_starts.retain(|&x| x != r.0);
        new_starts.push(r.1);

        let mut new_route: Vec<char> = route.clone();
        new_route.push(r.1);
        let new_dist = route_lengths.get(&r).unwrap();
        let ret = next_step(&new_starts, &new_route, memmap, &route_needs, &route_lengths, &all_keys, &quadrants);
        if (ret.1+new_dist) < min_dist{
            min_dist = ret.1+new_dist;
            min_path = ret.0;
        }
    }
    memmap.insert(key, (min_path.clone(), min_dist));
    (min_path, min_dist)
}

fn main() {
    let file = 
    File::open("C:/users/pcvan/projects/aoc2019/data/ex18.txt").expect("can't open the file");
    let pt2 = true;

    let mut lines: Lines<File> = Lines::new(file);
    let mut x = 0;
    let mut y = 0;
    let mut all_points: HashMap<(i32, i32), char> = HashMap::new();
    let mut keys: HashMap<(i32, i32), char> = HashMap::new();
    let mut route_lengths: HashMap<(char, char), i32> = HashMap::new();
    let mut route_needs: HashMap<(char, char), HashSet<char>> = HashMap::new();
    let mut quadrants: HashMap<char, char> = HashMap::new();
    let mut start_point: (i32,i32) = (0,0);
    let mut doors: HashMap<(i32, i32), char> = HashMap::new();
    while let Some(line) = lines.next(){
        for c in line.unwrap().chars(){
            if c != '#'{
                all_points.insert((x, y), c);
            }
            if (c.is_alphabetic() && c.is_lowercase()) || c == '@'{
                if c == '@' && pt2{
                    let translations = vec![(1,1,'1'),(1,-1,'2'),(-1,1,'3'),(-1,-1,'4')];
                    for t in translations{
                        keys.insert((x+t.0, y+t.1), t.2);
                    }
                    start_point = (x, y);
                }
                else if c == '@'{
                    keys.insert((x, y), c);
                }
                else{
                    keys.insert((x, y), c);
                }
                
            }
            else if c.is_alphabetic(){
                doors.insert((x, y), c);
            }
            x += 1;
        }
        y += 1;
        x = 0;
    }
    for s in keys.clone(){
        for e in keys.clone(){
            if s == e{
                continue;
            } 
            let path = shortest_path(s.0, e.0, &all_points);
            let mut needs = HashSet::new();
            let path_len = i32::try_from(path.len()).unwrap();
            for p in path{
                if doors.contains_key(&p){
                    let c: &char = doors.get(&p).unwrap();
                    needs.insert(*c);
                }
            }
            route_lengths.insert((s.1, e.1), path_len);
            route_needs.insert((s.1, e.1), needs);
        }
        if s.0.0 >= start_point.0 && s.0.1 >= start_point.1{
            quadrants.insert(s.1, '1');
        }
        else if s.0.0 >= start_point.0 && s.0.1 <= start_point.1{
            quadrants.insert(s.1, '2');
        }
        else if s.0.0 <= start_point.0 && s.0.1 >= start_point.1{
            quadrants.insert(s.1, '3');
        }
        else if s.0.0 <= start_point.0 && s.0.1 <= start_point.1{
            quadrants.insert(s.1, '4');
        }

    }

    let mut keys_vec: Vec<char> = keys.values().copied().collect();
    let starts: Vec<char>;
    if pt2{
        starts = vec!['1', '2','3', '4'];
    }
    else{
        starts = vec!['@']
    }
    keys_vec.retain(|&x| !starts.contains(&x));
    let mut memmap = HashMap::new();
    println!("{:?}", next_step(&starts, &Vec::new(), &mut memmap, &route_needs, &route_lengths, &keys_vec, &quadrants));
}


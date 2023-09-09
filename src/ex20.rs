use std::collections::HashMap;
use std::collections::HashSet;
// file1.rs
use std::fs::File;
use std::io::Read;
use priority_queue::PriorityQueue ;
use std::cmp::Reverse;

fn create_path_from_parents(start_point: &(i32, i32), end_point: &(i32, i32), parents: &HashMap<(i32, i32), (i32, i32)>, for_portals: bool) -> Vec<(i32, i32)> {
    let mut ret_vec: Vec<(i32, i32)> = Vec::new();
    let mut vtx = end_point;
    let mut last_vtx = &(-1,-1);
    while vtx != start_point{
        if for_portals && (((last_vtx.0 - vtx.0) > 1) ||  ((last_vtx.1 - vtx.1) > 1)){
            return Vec::new();
        }
        ret_vec.push(*vtx);
        last_vtx = vtx;
        vtx = &parents[&vtx];
    }
    ret_vec.reverse();
    return ret_vec;
}

fn shortest_path(neighbours: &HashMap<(i32, i32), HashSet<(i32, i32)>>, start_point: (i32, i32),  end_point: (i32, i32), for_portals: bool) ->  Vec<(i32,i32)>{
    if !neighbours.contains_key(&end_point){
        println!("ERROR, key not in neighbor list");
    }

    let mut visited: HashMap<(i32, i32), i32> = HashMap::new();
    let mut parents: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    let mut pq: PriorityQueue <(i32, i32), std::cmp::Reverse<i32>> = PriorityQueue::new();
    pq.push(start_point, Reverse(0));
    let mut i = 0;
    while pq.len() > 0{
        i += 1;
        let point: ((i32, i32), Reverse<i32>) = pq.pop().unwrap();
        visited.insert(point.0, point.1.0);
        let end_found = visited.contains_key(&end_point); 
        if end_found{
            return create_path_from_parents(&start_point, &end_point, &parents, for_portals);
        }
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

fn is_edge(p: (i32, i32), max: &(i32, i32)) -> bool{
    p.0 == 0 || p.1 == 0 || p.0 == max.0 || p.1 == max.1
}
fn shortest_path_pt2(neighbours: &HashMap<(i32, i32), HashSet<(i32, i32)>>, start_point: (i32, i32),  end_point: (i32, i32), 
                     portals: &HashSet<(i32, i32)>, max: &(i32, i32)) ->  i32{
    if !neighbours.contains_key(&end_point){
        println!("ERROR, key not in neighbor list");
    }
    let mut memory: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut pq: PriorityQueue <(i32, i32, i32), std::cmp::Reverse<i32>> = PriorityQueue::new();
    pq.push((start_point.0, start_point.1 ,0), Reverse(0));
    let mut depths: HashSet<i32> = HashSet::new();
    while pq.len() > 0{
        let point: ((i32, i32, i32), Reverse<i32>) = pq.pop().unwrap();
        let Reverse(current_prio_origin) = point.1.clone();
        let depth = point.0.2;
        depths.insert(depth);
        let key = (point.0.0, point.0.1, depth);
        let ret_val = memory.get(&key);
        if ret_val != None{
            continue;
        }
        memory.insert(key);

        let ns: Option<&HashSet<(i32, i32)>> = neighbours.get(&(point.0.0, point.0.1));
        if ns == None{
            println!("?");
        }
        if !depths.contains(&depth){
            println!("checking: {:?}", (key));
        }
        for n in ns.unwrap(){
            
            let delta = delta_depth(&(point.0.0, point.0.1), n, portals, max); 
            if n == &end_point && depth == 0{
                return current_prio_origin+1;
            }
            if depth == 0 && is_edge((point.0.0, point.0.1), max) && portals.contains(&(point.0.0, point.0.1)) && portals.contains(n){
                continue;
            }
            if (depth + delta) < 0{
                continue;
            }
            else{
                pq.push((n.0,n.1,depth+delta), Reverse(current_prio_origin + 1));    
            }
        }
    }
    10000000
}

fn insert_neighour(neighbours: &mut HashMap<(i32,i32), HashSet<(i32,i32)>>, p1: &(i32, i32), p2:&(i32,i32)){
    if neighbours.contains_key(p1){
        neighbours.get_mut(p1).unwrap().insert(*p2);
    }
    else{
        neighbours.insert(*p1, HashSet::from_iter(vec![*p2].iter().cloned()));
    }
    if neighbours.contains_key(p2){
        neighbours.get_mut(p2).unwrap().insert(*p1);
    }
    else{
        neighbours.insert(*p2, HashSet::from_iter(vec![*p1].iter().cloned()));
    }

}

fn delta_depth(p: &(i32, i32), p2: &(i32, i32), portals: &HashSet<(i32, i32)>, max: &(i32, i32)) -> i32{
        if portals.contains(p) && portals.contains(p2){
            println!("did a portal! {:?}, {:?}",  p,p2);
            if p.0 == 2 || p.1 == 2 || p.0 == max.0 || p.1 == max.1{
                return -1;

            }
            else{
                return 1;
            }
        }
    0
}

fn main() {
    let mut file = File::open("C:/users/pcvan/projects/aoc2019/data/ex20.txt").expect("can't open the file");
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("can't read the file");
    let mut neighbours: HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let mut points: HashSet<(i32, i32)> = HashSet::new();
    let mut letters: HashMap<(i32,i32), char> = HashMap::new();
    let mut portals: HashMap<String, HashSet<(i32,i32)>> = HashMap::new();
    let mut all_portals: HashSet<(i32,i32)> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    let mut start_point = (0,0);
    let mut end_point = (0,0);
    let mut max = (0,0);
    for c in text.chars(){
        if c == '\n'{
            x = 0;
            y += 1;
        }
        else if c == '#'{
            x += 1;
        }
        else if c == '.'{
            points.insert((x,y));
            if x > max.0{
                max.0 = x;
            }
            if y > max.1{
                max.1 = y;
            }
            x += 1;
        }
        else if c.is_alphabetic(){
            letters.insert((x,y), c);
            x += 1;
        }
        else if c == ' '{
            x += 1;
        }
    }
    for l1 in letters.keys(){
        let mut s = String::new();
        s.push(*letters.get(l1).unwrap());
        let mut p = (-1,-1);
        let mut l2 = (l1.0+1,l1.1);
        if letters.contains_key(&l2){
            s.push(*letters.get(&l2).unwrap());            
        }         
        else{
            l2 = (l1.0, l1.1+1);
            if letters.contains_key(&l2){
                s.push(*letters.get(&l2).unwrap());            
            }         
        }
        if s.len() == 1{
            continue;
        }
        for d in [(-1,0), (0,1), (1,0), (0,-1)]{
            let mut p_test = (l1.0+d.0, l1.1+d.1);
            if points.contains(&p_test){
                p = p_test;
                break;
            }
            p_test = (l2.0+d.0, l2.1+d.1);
            if points.contains(&p_test){
                p = p_test;
                break;
            }
        }
        all_portals.insert(p);
        if s == "AA"{
            start_point = p;
        }
        else if s == "ZZ"{
            end_point = p;
        }
        if portals.contains_key(&s){
            let other_portal =  portals.get_mut(&s).unwrap();
            insert_neighour(&mut neighbours, &p, other_portal.iter().nth(0).unwrap());
            other_portal.insert(p);

        }
        else{
            portals.insert(s, HashSet::from_iter(vec![p].iter().cloned()));
        }
    }
    for p in points.clone(){
        for d in [(-1,0), (0,1), (1,0), (0,-1)]{
            let p_test = (p.0+d.0, p.1+d.1);
            if points.contains(&p_test){
                insert_neighour(&mut neighbours,&p,&p_test);
            }
        }
    }    
    let mut total_len = 0;
    for n in neighbours.clone(){
        total_len += n.1.len();
        for nb in n.1{
            if !points.contains(&nb){
                println!{"point not contained!: {:?}", nb};
            }
        }
    }
    println!("neighbours: {}", total_len);
    let sh = shortest_path(&neighbours, start_point, end_point, false);
    println!("shortest path: {:?}, len: {}", sh, sh.len());
    let sh2 = shortest_path_pt2(&neighbours, start_point, end_point, &all_portals,  &max);
    println!("shortest path: {}", sh2);

}

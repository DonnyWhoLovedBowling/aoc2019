use std::fs::File;
use std::io;
mod read_lines;
use read_lines::Lines;

fn split_first_char(s: &str) -> Option<(char, &str)> {
    let mut chars = s.chars();
    match chars.next() {
        Some(c) => Some((c, chars.as_str())),
        None => None,
    }
}
struct Segment{
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32
}

impl Segment{
    fn direction(&self) -> &str{
        if self.x1 == self.x2{
            return "vertical";
        }
        "horizontal"
    }
    fn new(x1: i32,x2: i32, y1: i32, y2: i32) -> Segment{
        Segment{x1: x1, x2: x2, y1: y1, y2: y2}
    }
    fn x_max(&self) -> i32{
        std::cmp::max(self.x1,self.x2)
    }
    fn y_max(&self) -> i32{
        std::cmp::max(self.y1,self.y2)
    }
    fn x_min(&self) -> i32{
        std::cmp::min(self.x1,self.x2)
    }
    fn y_min(&self) -> i32{
        std::cmp::min(self.y1,self.y2)
    }
}

fn calc_length(segments: &Vec<Segment>, intersect: &(i32, i32)) -> i32{
    let mut length = 0;
    let mut sgs = 0;
    for s in segments.iter(){
        sgs += 1;
        if s.direction() == "vertical"{
            if s.x1 == intersect.0 && s.y_max() > intersect.1 && s.y_min() < intersect.1{
                length += (s.y1-intersect.1).abs();
                break
            }
            else{
                length += (s.y2-s.y1).abs();
            }
        }
        else{
            if s.y1 == intersect.1 && s.x_max() > intersect.0 && s.x_min() < intersect.0{
                length += (s.x1-intersect.0).abs();
                break
            }
            else{
                length += (s.x2-s.x1).abs();
            }
        }
    }
    println!("after {} iterations, found: {}", sgs, length);
    length
}

fn create_line(line: &str) -> Vec<Segment>{
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut segments: Vec<Segment> = Vec::new();
    for s in line.split(",") {
        let splitted: Option<(char, &str)> = split_first_char(s);
        if splitted.is_some(){
            let direction: char = splitted.unwrap().0;
            let value: i32 = splitted.unwrap().1.trim().parse::<i32>().unwrap();
            let y_old: i32 = y;
            let x_old: i32 = x;
            if direction == 'U'{
                y += value;
                segments.push(Segment::new(x, x, y_old, y));
            }
            if direction == 'D'{
                y -= value;
                segments.push(Segment::new(x, x, y_old, y));
            }
            if direction == 'L'{
                x -= value;
                segments.push(Segment::new(x_old, x, y, y));
            }
            if direction == 'R'{
                x += value;
                segments.push(Segment::new(x_old, x, y, y));
            }
        }
        else{
            println!("Kon string niet splitten!");
            break
        }
    }
    segments
}

fn find_intersections(vs: &Vec<Segment>, hs: &Vec<Segment>) -> (Vec<(i32, i32)>, i32){
    let mut intersections: Vec<(i32, i32)> = Vec::new();
    let mut min: i32 = 999999;
    for v in vs.iter(){
        if v.direction() != "vertical"{
            continue;
        }
        for h in hs.iter(){
            if h.direction() != "horizontal"{
                continue;
            }    
            if v.x1 > h.x_min() && v.x1 < h.x_max() && h.y1 > v.y_min() && h.y1 < v.y_max(){
                let man_dist: i32 = v.x1.abs() + h.y1.abs();
                intersections.push((v.x1, h.y1));
                if (man_dist) < min  && man_dist != 0{
                    min = man_dist;
                }
            }
        }    
    }
    (intersections, min)
}

fn main() -> io::Result<()> {
    let file: File = File::open("C:/users/pcvan/projects/aoc2019/data/ex3.txt")?;

    let mut lines: Lines<File> = Lines::new(file);

    let mut _first: &str = lines.next().expect("probleem").expect("probleem met eerste regel inlezen");
    let s1: Vec<Segment> = create_line(_first);
    
    let mut _second: &str = lines.next().expect("probleem").expect("probleem met tweede regel inlezen");
    let s2: Vec<Segment> = create_line(_second);

    let (mut intersections1, dist1) = find_intersections(&s1, &s2);
    let (mut intersections2, dist2) = find_intersections(&s2, &s1);

    intersections2.append(&mut intersections1);
    println!("ans 1 = {}", std::cmp::min(dist1, dist2));

    let mut min: i32 = 999999999;
    for i in intersections2.iter(){
        let length1 = calc_length(&s1, i);
        let length2 = calc_length(&s2, i);
        if (length1 + length2) < min{
            min = length1 + length2;
        }
    }
    println!("ans 2 = {}", min);
    Ok(())

}
    

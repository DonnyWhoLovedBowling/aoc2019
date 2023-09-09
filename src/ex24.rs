use std::{fs::File, io::{BufReader, BufRead}, collections::{HashMap, HashSet}};

fn calc_score(card: &HashMap<(u8, u8), u8>) -> u32{
    let mut score: u32 = 0;
    let base: u32 = 2;
    for (k,v) in card.iter(){
        score += base.pow(u32::from((k.1*5)+k.0))*u32::from(*v);
    }
    score
}

fn print_card(card: &HashMap<(u8, u8), u8>){
    for y in 0..5{
        let mut line: String = "".to_string();
        for x in 0..5{
            if card.get(&(x, y)).unwrap() == &u8::from(0){
                line.push('.');
            }
            else{
                line.push('#');
            }
        }
        println!("{line}");
    }
    println!("");
}


fn sum_upper_row(card: &HashMap<(u8, u8), u8>) -> u8{
    let mut tot = 0;
    for x in 0..5{
        tot += card.get(&(x, 0)).unwrap();
    }
    tot
}

fn sum_lower_row(card: &HashMap<(u8, u8), u8>) -> u8{
    let mut tot = 0;
    for x in 0..5{
        tot += card.get(&(x, 4)).unwrap();
    }
    tot
}

fn sum_left_col(card: &HashMap<(u8, u8), u8>) -> u8{
    let mut tot = 0;
    for y in 0..5{
        tot += card.get(&(0, y)).unwrap();
    }
    tot
}

fn sum_right_col(card: &HashMap<(u8, u8), u8>) -> u8{
    let mut tot = 0;
    for y in 0..5{
        tot += card.get(&(4, y)).unwrap();
    }
    tot
}

fn calc_new_card(card: &HashMap<(u8, u8), u8>) -> HashMap<(u8, u8), u8>{
    let mut new_card: HashMap<(u8, u8), u8> = HashMap::new();
    let incs:Vec<i8> = vec![-1,0,1];
    for (k,v) in card.iter(){
        let mut n_bugs = 0;
        for dx in incs.iter(){
            for dy in incs.iter(){
                if dx.abs() + dy.abs() != 1 {
                    continue;
                }
                let new_x = i8::try_from(k.0).unwrap()+dx;
                let new_y = i8::try_from(k.1).unwrap()+dy;
                if new_x < 0 || new_x > 4 || new_y < 0 || new_y > 4{
                    continue;
                }

                n_bugs += card.get(&(u8::try_from(new_x).unwrap(),u8::try_from(new_y).unwrap())).unwrap();
            }
        }
        let mut new_v = *v;
        if *v == 0 && (n_bugs == 1 || n_bugs == 2){
            new_v = 1;
        }
        if *v == 1 && n_bugs != 1{
            new_v = 0;
        }
        new_card.insert(*k, new_v);
    }

    new_card
}

fn calc_new_cards(cards: HashMap<i16, HashMap<(u8, u8), u8>>) -> HashMap<i16, HashMap<(u8, u8), u8>>{
    let mut new_cards: HashMap<i16, HashMap<(u8, u8), u8>> = HashMap::new();
    let incs:Vec<i8> = vec![-1,0,1];
    for (card_ix, card) in cards.iter(){
        let mut new_card: HashMap<(u8, u8), u8> = HashMap::new();
        for (k,v) in card.iter(){
            let mut n_bugs = 0;
            if *k == (2, 2){
                new_card.insert(*k,0);
                continue;
            }
            for dx in incs.iter(){
                for dy in incs.iter(){
                    if dx.abs() + dy.abs() != 1 {
                        continue;
                    }
                    let new_x = i8::try_from(k.0).unwrap()+dx;
                    let new_y = i8::try_from(k.1).unwrap()+dy;
                    if (new_x,new_y) == (2,2) && cards.contains_key(&(card_ix-1)){
                        let sub_card = cards.get(&(card_ix-1)).unwrap();
                        match k{
                            (2,1) => n_bugs += sum_upper_row(&sub_card),
                            (1,2) => n_bugs += sum_left_col(&sub_card),
                            (3,2) => n_bugs += sum_right_col(&sub_card),
                            (2,3) => n_bugs += sum_lower_row(&sub_card),
                            _     => panic!("unmatched coordinates around centre subcard!"),
                        }
                    }
                    else if(new_x == -1 || new_x == 5 || new_y == -1 || new_y == 5) && 
                        cards.contains_key(&(card_ix+1)){

                        let sub_card = cards.get(&(card_ix+1)).unwrap();
                        if new_x == -1{
                            n_bugs += sub_card.get(&(1,2)).unwrap();
                        }
                        if new_x == 5{
                            n_bugs += sub_card.get(&(3,2)).unwrap();
                        }
                        if new_y == -1{
                            n_bugs += sub_card.get(&(2,1)).unwrap();
                        }
                        if new_y == 5{
                            n_bugs += sub_card.get(&(2,3)).unwrap();
                        }

                    }
                    if 0 <= new_x && new_x < 5 && 0 <= new_y && new_y < 5{
                        let coor = (u8::try_from(new_x).unwrap(),u8::try_from(new_y).unwrap());
                        let is_bug = card.get(&coor).unwrap();
                        // if *card_ix == 0{
                        //     println!("coordinate = {:?}", coor);
                        //     println!("regular case: k = {:?}, v = {}, new_x = {}, new_y = {}, is_bug = {}", *k,v,new_x,new_y, is_bug);
                        // }
                        n_bugs += is_bug;
                    }
                }
            }
            let mut new_v = *v;
            if *v == 0 && (n_bugs == 1 || n_bugs == 2){
                new_v = 1;
            }
            if *v == 1 && n_bugs != 1{
                new_v = 0;
            }
            new_card.insert(*k, new_v); 
            // if *card_ix == 0{
            //     println!("k = {:?}, old_v = {}, new_v = {}, n_bugs = {}",k,*v,new_v,n_bugs );   
            // }
        }   
        new_cards.insert(*card_ix, new_card.clone());

    }
    new_cards
}
fn new_card(lines: Vec<String>, empty_card: bool) -> HashMap<(u8, u8), u8>{
    let mut card: HashMap<(u8, u8), u8> = HashMap::new();
    let mut x = 0;
    let mut y = 0;    

    for line in lines{
        for c in line.chars(){
            if empty_card{
                card.insert((x,y), 0);
                x += 1;
            }
            else if c == '#'{
                card.insert((x,y), 1);
                x += 1;
            }
            else if c == '.'{
                card.insert((x,y), 0);
                x += 1;
            }
        }
        x = 0;
        y += 1;
    }
    card
}
fn main() {
    let file = File::open("C:/users/pcvan/projects/aoc2019/data/ex24.txt").expect("can't open the file");
    let reader = BufReader::new(file);
    let lines:Vec<String>= reader.lines().collect::<Result<_,_>>().unwrap(); 
    let mut card: HashMap<(u8, u8), u8> = new_card(lines.clone(), false);
    let mut scores: HashSet<u32> = HashSet::new();
    loop{
        let score = calc_score(&card);
        if scores.contains(&score){
            println!("pt1: {score}");
            break;
        }
        scores.insert(score);
        card = calc_new_card(&card);
    }
    card = new_card(lines.clone(), false);
    let empty_card = new_card(lines, true);
    let mut cards: HashMap<i16, HashMap<(u8, u8), u8>> = HashMap::new();
    cards.insert(0, card.clone());
    let mut max_card: i16 = 0;
    let mut min_card: i16 = 0;

    for _i in 0..200{
        // print_card(cards.get(&0).unwrap());
        max_card += 1;
        min_card -= 1;
        cards.insert(max_card, empty_card.clone());
        cards.insert(min_card, empty_card.clone());
        cards = calc_new_cards(cards);
        // print_card(cards.get(&-1).unwrap());
        // print_card(cards.get(&0).unwrap());
        // print_card(cards.get(&1).unwrap());

    }
    let mut tot: u32 = 0;
    for (_, card) in cards.iter(){
        for (_,v) in card{
            tot += u32::from(*v);
        }
    }
    print_card(cards.get(&0).unwrap());
    println!("pt2: {:?}", tot);
}


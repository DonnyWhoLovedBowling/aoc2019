use std::collections::HashMap;
// file1.rs
use std::fs::File;
mod read_lines;
use read_lines::Lines;
use num_integer;

fn do_loop(factories: &HashMap<String, Factory>, inv: &mut HashMap<String, u64>, i: &mut u32, step_size: u32){
    loop{
        let mut inv_new  = inv.clone();
        count_ore(&factories, &"FUEL".to_string(), step_size.into(), &mut inv_new); 
        if inv_new.get("ORE").unwrap() > &1000000000000{
            println!("{}", i);
            return;
        }
        *inv = inv_new;
        *i += step_size;
    }
}

#[derive(Debug)]
struct Factory{
    inputs: HashMap<String, u64>,
    output: (String, u64)
}

fn count_ore(factories: &HashMap<String, Factory>, material: &String, req: u64, inv: &mut HashMap<String, u64>){
    let f = factories.get(material);
    let factory =  f.expect("material not found");
    let req_corrected; 
    if req <= inv[material]{
        *inv.get_mut(material).unwrap() -= req;
        req_corrected = 0;
    }
    else{
        req_corrected = req - inv[material];
        *inv.get_mut(material).unwrap() = 0;
    }
    let n_batch = num_integer::div_ceil(req_corrected, factory.output.1);
    *inv.get_mut(material).unwrap() += (n_batch*factory.output.1)-req_corrected;

    for mat in factory.inputs.iter(){
        if mat.0 == "ORE"{
            *inv.get_mut(mat.0).unwrap() += n_batch*mat.1;
            return;
        }

        count_ore(factories, &mat.0, *mat.1*n_batch, inv);
    }
}

fn main() {    
    let file = File::open("C:/users/pcvan/projects/aoc2019/data/ex14.txt").expect("can't open the file");
    let mut lines: Lines<File> = Lines::new(file);
    let mut factories = HashMap::new();
    let mut inventory: HashMap<String, u64> = HashMap::new();
    while let Some(line) = lines.next(){
        let line = line.unwrap();
        let first_split: Vec<&str> = line.split(" => ").collect();
        let o: Vec<&str> = first_split[1].trim().split(" ").collect();
        let o_name = o[1].to_string().clone();
        inventory.insert(o_name.clone(), 0);
        let o_num = o[0].to_string().clone().parse::<u64>().unwrap();
        let mut is_map = HashMap::new();
        for i in first_split[0].split(","){
            let i_split: Vec<&str> = i.trim().split(" ").collect();
            let i_name = i_split[1].to_string().clone();
            let i_num = i_split[0].to_string().clone().parse::<u64>().unwrap();
            is_map.insert(i_name.clone().to_string(),i_num);
        }
        factories.insert(o_name.clone(), Factory{inputs: is_map, output: (o_name.clone(), o_num)});
    }
    inventory.insert("ORE".to_string().clone(),0);
    count_ore(&factories, &"FUEL".to_string(), 1, &mut inventory); 
    println!("{}", inventory["ORE"]);
    let mut i: u32 = 1; 
    do_loop(&factories, &mut inventory, &mut i, 1000000);
    println!("{}", i);
    do_loop(&factories, &mut inventory, &mut i, 100000);
    println!("{}", i);
    do_loop(&factories, &mut inventory, &mut i, 100000);
    println!("{}", i);
    do_loop(&factories, &mut inventory, &mut i, 10000);
    println!("{}", i);
    do_loop(&factories, &mut inventory, &mut i, 1000);
    println!("{}", i);
    do_loop(&factories, &mut inventory, &mut i, 100);
    println!("{}", i);
    do_loop(&factories, &mut inventory, &mut i, 10);
    println!("{}", i);
    do_loop(&factories, &mut inventory, &mut i, 1);
    println!("{}", i);

}

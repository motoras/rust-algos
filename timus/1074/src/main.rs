use std::io::stdin;

fn clean_exp(line:&str)->&str{
    let mut has_exp = line.find('e');
    if has_exp.is_none(){
        has_exp = line.find('E');
    }
    if has_exp.is_none(){
        line
    }else{
        let char_vec: Vec<char> = line.chars().collect();    
        let mut res: Vec<char> = Vec::with_capacity(char_vec.len() + 10);
        for (i, c) in char_vec.iter().enumerate() {
            
        }
        
        line
    }
    
}

fn main() {
    let reader = stdin();
    let mut nr_line = String::new();
    let mut fr_line = String::new();
    loop {
        reader.read_line(&mut nr_line).unwrap();
        if nr_line.trim() == "#" {
            break;
        }
        reader.read_line(&mut fr_line).unwrap();
        let number_res = nr_line[0..nr_line.len()-1].parse::<f64>();
        if number_res.is_err() {
            println!("Not a floating point number");
        } else {
            let decs = fr_line[0..fr_line.len()-1].parse::<u32>();
            println!("{}->{}", number_res.unwrap(),clean_exp(&nr_line[0..nr_line.len()-1]));
        }
        nr_line.clear();
        fr_line.clear();
    }
}

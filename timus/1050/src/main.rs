use std::io::stdin;
use std::vec::Vec;

/**
 * You have to go twice over each paragraph in order to remove the last double quote if unmatched.
 * Tone of issues may occur when you want to detect \par.. It may be in the middle of the text, when is valid
 * only followed by a nonalphabetic char or at the end of line. 
 * Also you must make sure that you preserver empty lines.
 * You must read unitl no error occurs, otherwise it wil crash on Test 5.
 * By any means I cannot pass test number 5.
 *  
 */
fn process(text: &str) -> String {    
    let char_vec: Vec<char> = text.chars().collect();    
    let mut quotes_count = 0;
    for (i, c) in char_vec.iter().enumerate() {
        if *c == '"' {
            if i == 0 || char_vec[i-1] != '\\' {
                quotes_count += 1;
            }
        }
    }
    let mut res: Vec<char> = Vec::with_capacity(char_vec.len());
    let mut open = true;
    for (i, c) in char_vec.iter().enumerate() {
        if *c == '"' {
            if i == 0 ||  char_vec[i-1] != '\\' {
                quotes_count -= 1;
                if quotes_count > 0 {
                    if open {
                        res.push('`');
                        res.push('`');
                    } else {
                        res.push('\'');
                        res.push('\'');
                    }
                    open = !open;
                } else {
                    if !open {
                        res.push('\'');
                        res.push('\'');
                    }
                }
            } else {
                //just copy the char
                res.push(*c);
            }
        } else {
            //just copy the char
            res.push(*c);
        }
    }
    let str_res: String = res.into_iter().collect();
    str_res.to_string()
}

fn is_control_only(line:&str)->bool{
    let line_vec: Vec<char> = line.chars().collect();
    for c in line_vec{
        if !c.is_control(){
            false;
        }
    }
    true
}

fn main() {    
    let mut paragraph = String::new();
    let mut line = String::new();
    let mut res: std::io::Result<usize>;
    loop {
        res = stdin().read_line(&mut line);
        if res.is_ok() {
            break;
        }
    }
    
    
    loop {
        // let check_end_inp = line.find("\\endinput");
        // line = match  check_end_inp {
        //     Some(n)=>line[0..n +  9].to_owned(),
        //     None => line
        // };
        if line.trim().len() == 0 && is_control_only(&line){
            //blank line
            if paragraph.len() != 0 {
                paragraph.push_str(&line);
                print!("{}", process(&paragraph));
                paragraph.clear();
            } else {
                print!("{}", line); //extra blank line just have to be printed
            }
        }else{
            let line_vec: Vec<char> = line.chars().collect();
            let mut idx = 0;
            let mut start = 0;
            let len = line_vec.len();
            while idx < len {
                if idx + 4 <= len && (idx == 0 || !line_vec[idx].is_alphanumeric()) 
                    && line_vec[idx] == '\\'
                    && line_vec[idx + 1] == 'p'
                    && line_vec[idx + 2] == 'a'
                    && line_vec[idx + 3] == 'r'
                    && (idx + 4 == len || !line_vec[idx + 4].is_alphanumeric())
                {
                    for i in start..(idx + 4) {
                        paragraph.push(line_vec[i]);
                    }
                    print!("{}", process(&paragraph));
                    paragraph.clear();
                    idx += 4;
                    start = idx;
                } else {
                    idx += 1;
                }
            }
            if start < len {
                for i in start..len {
                    paragraph.push(line_vec[i]);
                }
            }
        } 
        // if line.trim().ends_with(" \\endinput") || line.trim() == "\\endinput" {
        //     break;
        // }
        if res.unwrap() == 0{
            break;
        }
        line.clear();        
        loop {
            res = stdin().read_line(&mut line);
            if res.is_ok() {                                
                break;
            }
        }                
    }       
    if paragraph.len() > 0 {
        print!("{}", process(&paragraph));
    }
    
    
}

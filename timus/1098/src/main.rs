/**
 * Again brute force will fail. Use the recursive soluton for Josephus problem. In order to find it
 * look to cases at least from 1 to 10.
 */
use std::io::{ Read, Write};
use std::io::stdin;
use std::io::stdout;


const NO_COMMENTS: &str = "No comments";
const YES: &str = "Yes";
const NO: &str = "No";



fn answer_dp(len:usize)-> usize{
    let k = 1999;
    let mut res:usize = 0;
    for i in 2..(len+1){
        res = (res + k ) % i;
    }
    res
}

fn main() {
    let mut reader = stdin();    
    let mut question = String::new();
    let mut character = [0];
    while let Ok(_) = reader.read_exact(&mut character) {            
            let crt:char = character[0] as char;                        
            if crt != '\n' && crt !='\r'{
                question.push(crt);
            }            
    }
     let idx = answer_dp(question.len());
     let res = match question.chars().nth(idx).unwrap() {
           '?' =>YES,
           ' ' =>NO,
           _ =>NO_COMMENTS,
      };
     let writer = &mut stdout();    
     writeln!(writer, "{}",res).unwrap();        
}


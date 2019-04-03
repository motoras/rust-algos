use std::io::stdin;
fn main() {
    let in_stream = stdin();
    let mut players_count_line = String::new();
    in_stream.read_line(&mut players_count_line).unwrap();
    let players_count: usize = players_count_line.trim_right().parse().unwrap();
    let mut players = [[0f64; 2]; 100];
    for i in 0..players_count {
        let mut player_line = String::new();
        in_stream.read_line(&mut player_line).unwrap();
        
        let mut res = player_line.split_whitespace();
        let p1:f64 = res.next().unwrap().parse().unwrap();
        let p2:f64 = res.next().unwrap().parse().unwrap();
        let p3:f64 = res.next().unwrap().parse().unwrap();        
        players[i] = [p1/p3, p2/p3];
    }    
}
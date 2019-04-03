use std::f64::consts::PI;
/**
 * Just use the formula's from wikipeida for great circle distance and be carefull
 * as the degrees could be sometimes one digit only.
 */
use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;
const DEGREES: f64 = PI / 180.0;
const RADIUS: f64 = 6875.0 / 2.0;

fn lat(ship_lat_desc: &str) -> f64 {
    let mut split = ship_lat_desc.split(&['^', '\'', '"', ' '][..]);
    let l1 = split.next().unwrap().parse::<f64>().unwrap();
    let l2 = split.next().unwrap().parse::<f64>().unwrap();
    let l3 = split.next().unwrap().parse::<f64>().unwrap();
    split.next();
    let l_dir = split.next().unwrap();
    if l_dir.starts_with('N') {
        (l1 + l2 / 60.0 + l3 / 3600.0) * DEGREES
    } else {
        -(l1 + l2 / 60.0 + l3 / 3600.0) * DEGREES
    }
}

fn long(ship_long_desc: &str) -> f64 {
    let mut split = ship_long_desc.split(&['^', '\'', '"', ' '][..]);
    split.next();
    let l1 = split.next().unwrap().parse::<f64>().unwrap();
    let l2 = split.next().unwrap().parse::<f64>().unwrap();
    let l3 = split.next().unwrap().parse::<f64>().unwrap();
    split.next();
    let l_dir = &split.next().unwrap();
    if l_dir.starts_with('E') {
        (l1 + l2 / 60.0 + l3 / 3600.0) * DEGREES
    } else {
        -(l1 + l2 / 60.0 + l3 / 3600.0) * DEGREES
    }
}

fn main() {
    let reader = BufReader::new(stdin());
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();

    let ship_lat = lat(&lines[3]);
    let ship_long = long(&lines[4]);
    let ice_lat = lat(&lines[6]);
    let ice_long = long(&lines[7]);
    let delta_l = (ship_long - ice_long).abs();
    let delta_s =
        (ship_lat.sin() * ice_lat.sin() + ship_lat.cos() * ice_lat.cos() * delta_l.cos()).acos();
    let distance = delta_s * RADIUS;
    println!("The distance to the iceberg: {:.2} miles.", distance);
    if distance < 99.995 {
        println!("DANGER!");
    }
}

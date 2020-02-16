extern crate elapsed;
use elapsed::measure_time;
use std::collections::HashMap;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs::File;
use std::num::ParseIntError;

fn main()  {
    let mut vector: Vec<u64> = vec![];
    if let Ok(lines) = read_lines("src/yahtzee-upper.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let s = line.unwrap().trim().parse::<u64>().unwrap();
            vector.push(s);
        }
    }

    let (elapsed, sum) = measure_time(|| {
        let value:u64 = yahtzee_upper(vector.as_slice(), vector.len());
        println!("\nvalue {}", value);
    });
    println!("elapsed = {}", elapsed);

}


fn yahtzee_upper(v:&[u64], dice_sides:usize) -> u64 {
    let mut counter:HashMap<u64, u64> = HashMap::with_capacity(dice_sides);
    for idx in 0..v.len() {
        let num = v[idx];
        let val = match counter.get(&num) {
            Some(x) => x,
            None => &0
        };
        counter.insert(num, val + num);
    }
   *counter.values().max_by(|x, y| x.cmp(y)).unwrap()
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}





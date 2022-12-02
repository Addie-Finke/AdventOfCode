use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str;

fn main() -> io::Result<()>{
    let file = File::open("input.txt")?;
    let iter = BufReader::new(file);

    let mut curr_total = 0;
    let mut curr_max = 0;
    for line in iter.lines(){
        let l = line?.clone();
        if l.eq(""){ 
            if curr_total > curr_max{
                curr_max = curr_total;
            }
            curr_total = 0;
        }
        else{
            curr_total += l.parse::<i32>().unwrap();
        }
    }

    println!("{}", curr_max);
    Ok(())
}

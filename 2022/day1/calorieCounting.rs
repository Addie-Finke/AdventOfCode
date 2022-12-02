use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()>{
    let file = File::open("input.txt")?;
    let iter = BufReader::new(file);

    let mut curr_totals =[0, 0, 0];
    let mut curr_num = 0;
    for line in iter.lines(){
        let l = line?.clone();
        if l.eq(""){ 
            if curr_num > curr_totals[0]{
                curr_totals[2] = curr_totals[1];
                curr_totals[1] = curr_totals[0];
                curr_totals[0] = curr_num;
            }
            else if curr_num > curr_totals[1]{
                curr_totals[2] = curr_totals[1];
                curr_totals[1] = curr_num;
            }
            else if curr_num > curr_totals[2]{
                curr_totals[2] = curr_num;
            }
            curr_num=0;
        }
        else{
            curr_num += l.parse::<i32>().unwrap();
        }
    }
    let max = curr_totals[0] + curr_totals[1] + curr_totals[2];
    println!("{}", max);
    Ok(())
}

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()>{
    let file = File::open("input.txt")?;
    let iter = BufReader::new(file);

    let mut curr_score = 0;
    for line in iter.lines(){
        let l = line?.clone();
        let opponent_play = l.chars().nth(0).unwrap();
        let round_end = l.chars().nth(2).unwrap();
        let x = round_end as u32 - 88;
        let y = opponent_play as u32 -65;
        if x == 0{
            if y!=0{
                curr_score += y;
            }
            else{
                curr_score+=3;
            }
        }
        else if x == 1{
            curr_score += 3 + y+1;
        }    
        else{
            if y == 2{
                curr_score += 7;
            }
            else{
                curr_score += y+8;
            }
        }
    }
    println!("{}", curr_score);
    Ok(())
}

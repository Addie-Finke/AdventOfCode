use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()>{
    let file = File::open("input.txt")?;
    let iter = BufReader::new(file);

    let mut curr_score = 0;
    for line in iter.lines(){
        let l = line?.clone();
        let opponent_play = l.chars().nth(0).unwrap();
        let our_play = l.chars().nth(2).unwrap();
        let x = our_play as u32 -88;
        let y = opponent_play as u32 -65;
        if x == y{
            curr_score += 3;
        }
        else if ((x+1)%3)!= y{
            curr_score += 6;
        }
        curr_score += x+1;
    }
    println!("{}", curr_score);
    Ok(())
}

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;


//converts x to its score value
fn score(x: char) -> u32{
    let mut y = x as u32;
    if x.is_lowercase(){
        y = y - 96;
        return y;
    }
    else{
        y = y-38;
        return y;
    }
}


fn main() -> io::Result<()>{
    let file = File::open("input.txt")?;
    let iter = BufReader::new(file);
    let mut s = 0;
    for line in iter.lines(){
        let l = line?.clone();
        let mut mapx = HashMap::new();
        let mut mapy = HashMap::new();
        let mut x = 0;
        let mut y = l.len()/2;
        while x < l.len()/2{
            if l.chars().nth(x).unwrap() == l.chars().nth(y).unwrap(){
                s += score(l.chars().nth(x).unwrap());
                break;
            }
            else if mapy.contains_key(&l.chars().nth(x).unwrap()){
                s += score(l.chars().nth(x).unwrap());
                break;
            }
            else if mapx.contains_key(&l.chars().nth(y).unwrap()){
                s += score(l.chars().nth(y).unwrap());
                break;
            }
            else{
                mapx.insert(l.chars().nth(x).unwrap(), true);
                mapy.insert(l.chars().nth(y).unwrap(), true);
            }
            x+=1;
            y+=1;
        }
    }
    println!("{}", s);
    Ok(())
}

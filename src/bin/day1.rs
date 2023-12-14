use std::i32;
use std::io::{ BufReader, BufRead};
use std::fs::{File};
use std::path::{Path,Display};
fn main() {
    let mut sum :i32 = 0;
    
    //readfile
    let path  = Path::new("./src/data/Input_data_day1.txt");
    let file = match File::open(&path){
        Err(why) => panic!("Error happend {} {}",path.display(),why),
        Ok(file) =>file
    };
    let reader = BufReader::new(file);
    for line in reader.lines(){
        let line = match line {
            Ok(line) => line,
            Err(why) => panic!("couldn't read line: {}", why),
        };
        sum += find_digits(&line);
    }
   println!("THe answer is {}",sum);
}

fn find_digits(line: &String) -> i32{
    let mut firstDigit = String::new();
    let mut lastDigit  = String::new();
    for letter in line.chars(){
        if firstDigit != ""{
            firstDigit = if letter.is_numeric(){ letter.to_string()}else{firstDigit};
        }else{
            lastDigit = if letter.is_numeric(){ letter.to_string()}else{lastDigit};
        }
    }
    let mut number = (firstDigit + &lastDigit).parse::<i32>().unwrap();
    number
}
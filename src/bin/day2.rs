use std::{io::{BufReader, BufRead}
    ,path::Path,
    fs::File};
use std::collections::HashMap;

// struct max_condition{
//     color:&str,
//     max:i32
// }


pub fn run() -> i32{
    let path  = Path::new("./src/data/Input_data_day2.txt");
    let file = match File::open(&path){
        Err(why) => panic!("Error happend {} {}",path.display(),why),
        Ok(file) =>file
    };
    let reader = BufReader::new(file);
    //let max_conditions:Vec<max_condition>;

    let mut max_conditions:HashMap<&str, i32> = HashMap::new();
    max_conditions.insert("red",12);
    max_conditions.insert("green", 13);
    max_conditions.insert("blue", 14);

    let mut answer = 0;
    for line in reader.lines() {
        let line = match line {
            Err(why) => panic!("Error happend {}",why),
            Ok(line) => line
        };
        answer += check_possible_or_not(line, &max_conditions);
    }
    //println!("Answer is {}",answer);
    answer
}

fn check_possible_or_not(line: String,conditions :&HashMap<&str,i32>) -> i32{
    //["Game 1:", "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"]  the below line has this value
    let game_line: Vec<&str> = line.split(":").collect();

    //game number, in this case 1
    let game_number:i32 = match game_line[0].split(" ").collect::<Vec<&str>>()[1].parse(){
        Err(why) => panic!("{}",why),
        Ok(game_number) => game_number,
    };

    //["3 blue, 4 red"," 1 red, 2 green, 6 blue" ,"2 green"]
    let game_data: Vec<&str> = game_line[1].split(";").collect();

    //"3 blue,4 red"
    for each_round in game_data.iter(){
        //[" 3 blue"," 4 red"]
        let each_set:Vec<&str> = each_round.split(",").collect();
        //"3 blue"
        for each_color in each_set.iter(){
            //["3","blue"]
            let each_set_of_color:Vec<&str> = each_color.split(" ").collect();
            let max_condition_for_this_color = match conditions.get(each_set_of_color[2]){
                Some(max_condition_for_this_col) => *max_condition_for_this_col,
                None => panic!(" \nKey not found in conditions,{} & {} & {} \n",each_set_of_color[0],each_set_of_color[1],each_set_of_color[2]),
            };
            let number_of_balls: i32 = match each_set_of_color[1].parse::<i32>() {
                Ok(number) => number,
                Err(why) => panic!("{}", why),
            };
            if max_condition_for_this_color < number_of_balls  {
                return 0;
            }
            
        }
    }
    return game_number;
}
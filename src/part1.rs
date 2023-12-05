use std::env;
use std::fs;
use std::fs::read_to_string;

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    let mut counter = 1;
    let mut first_int = 0;
    let mut last_int = 0;
    //let mut first_int_string ="";
   // let mut last_int_string = "";
    let mut sum_total = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_string) = line {
                println!("{}", line_string);
                for character in line_string.chars() {
                        if(character.is_numeric())
                        {
                            first_int = character.to_string().parse::<i32>().unwrap();
                            break;
                        }
                    }
                for character in line_string.chars().rev().collect::<String>().chars() {
                    if(character.is_numeric())
                    {
                        last_int = character.to_string().parse::<i32>().unwrap();
                        break;
                    }
                }
        //println!("Line# {} F: {} L:{}",counter,first_int,last_int );
        println!("Line# {} F: {} L:{} T:{}",counter,return_int_instance(line_string.clone()), return_int_instance(line_string.clone().chars().rev().collect::<String>()),return_merdged_number(first_int,last_int));
        sum_total = sum_total + return_merdged_number(first_int,last_int);
                }//if line string ok
                
                counter = counter+ 1;
            }
        }
        println!("{}",sum_total);

    }


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn return_int_instance(line: String) -> i32{
    for character in line.chars() {
        if(character.is_numeric()) {
        return character.to_string().parse::<i32>().unwrap();
    }
}
    println!("error from looking for int");
    return 999; //can return eror?
}

fn return_merdged_number(first: i32, last: i32) -> i32 {
    //String::from(first.to_string()+last.to_string());
    let first_string = String::from(first.to_string());
    let last_string = String::from(last.to_string());
    return String::from(first_string + &last_string).parse::<i32>().unwrap();
  //  return 19;
    
}

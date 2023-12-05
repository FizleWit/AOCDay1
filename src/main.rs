use std::env;
use std::fs;
use std::fs::read_to_string;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File hosts.txt must exist in the current path
    let mut counter = 0;
    let mut first_int = 0;
    let mut last_int = 0;
    let mut reversed = String::new();
    //let mut first_int_string ="";
   // let mut last_int_string = "";
    let mut sum_total = 0;
   // let mut vec = Vec::new();
    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line_string) = line {
                println!("{}", line_string);
                let reg_normal = Regex::new(r"one|two|three|four|five|six|seven|eight|nine|zero|[0-9]").unwrap();
                let reg_weird = Regex::new(r"eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|orez|[0-9]").unwrap();
                 let caps_normal = reg_normal.captures(line_string.as_str()).unwrap();
                let reversed = line_string.chars().rev().collect::<String>();
                 let caps_weird = reg_weird.captures(reversed.as_str()).unwrap();

               
               // let caps2 = reg_normal.captures(line_string.as_str()).unwrap();
               counter = counter+ 1;
               // println!("first catch {}", caps_normal[0].to_owned());
                println!("Line: {} F: {} L: {}, F1:{} L1:{}",counter, caps_normal[0].to_owned(),  caps_weird[0].to_owned(), return_format_number(caps_normal[0].to_owned()), return_format_number(caps_weird[0].to_owned()));
               sum_total = sum_total + return_string_number(return_format_number(caps_normal[0].to_owned()), return_format_number(caps_weird[0].to_owned()));
               //assert_eq!("toady", &caps2[0]);
                }
        //println!("Line# {} F: {} L:{}",counter,first_int,last_int );
       // println!("Line# {} F: {} L:{} T:{}",counter,return_int_instance(line_string.clone()), return_int_instance(line_string.clone().chars().rev().collect::<String>()),return_merdged_number(first_int,last_int));
        sum_total = sum_total + return_merdged_number(first_int,last_int);
                }//if line string ok
                
                
            }
        
        println!("Sum Total: {}",sum_total);
    }

fn return_format_number(num_data: String) -> String{
    match num_data.as_str(){
        "one" | "eno" | "1" => return String::from("1"),
        "two" | "owt" | "2" => return String::from("2"),
        "three" | "eerht" | "3" => return String::from("3"),
        "four" | "ruof" | "4" => return String::from("4"),
        "five" | "evif" | "5" => return String::from("5"),
        "six" | "xis" | "6" => return String::from("6"),
        "seven" | "neves" | "7" => return String::from("7"),
        "eight" | "thgie" | "8" => return String::from("8"),
        "nine" | "enin" | "9" => return String::from("9"),
        "zero" | "orez" | "0" => return String::from("0"),
        _=> return String::from("1000")//error catch here
    }

    return String::from("1000");//error catch here
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
        if character.is_numeric() {
        return character.to_string().parse::<i32>().unwrap();
    }
}
    println!("error from looking for int");
    return 999; //can return eror?
}

fn return_string_number(first_string:String,last_string:String) ->i32{
    return String::from(first_string + &last_string).parse::<i32>().unwrap();
}
fn return_merdged_number(first: i32, last: i32) -> i32 {
    //String::from(first.to_string()+last.to_string());
    let first_string = String::from(first.to_string());
    let last_string = String::from(last.to_string());
    return String::from(first_string + &last_string).parse::<i32>().unwrap();
  //  return 19;
    
}

/*fn return_data_from_string(data: String) -> String {
    if(data == "one" |&&"eno" | "1")
    {
        return String::from("1");
    }
    if(data == "two" | "owt" | "2")
    {
        return "2";
    }
    else 
    return String::from("woohoo");
}
*/
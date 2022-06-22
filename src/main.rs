use std::io::stdin;
use rand::prelude::*;
fn main() {
    println!("How many rounds would you like to play?");
    let mut round_string = String::new();
    stdin().read_line(&mut round_string)
        .ok()
        .expect("Failed to read line");
    
    let mut range_of= round_string.trim_end().parse::<u32>().unwrap();
    let mut rng = rand::thread_rng();
    let mut first_num = rng.gen::<u32>();
    let mut points = 0;
    while range_of>0{
        let mut second_num = rng.gen::<u32>();
        let mut answer = String::new();
        println!("This is the first number:{}. Is the second number higher(h) or lower(l)?", first_num);
        stdin().read_line(&mut answer)
            .ok()
            .expect("Failed to read line");
        if "h" == answer.trim_end() && second_num > first_num{
            points += 1;
            println!("Correct!")
        } else if "l" == answer.trim_end() && second_num <= first_num{
            points += 1;
            println!("Correct!")
        } else{
            println!("Incorrect!")
        }
        range_of = range_of - 1;
        // second_number=first_num;
        first_num = second_num;
    }
    println!("Given {} round(s) you scored {} point(s)", round_string, points);
}

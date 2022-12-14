use std::fs;

fn main() {
    let contents = fs::read_to_string("src/puzzle_input.txt").expect("wasnt able to read file");

    let contents: Vec<&str> = contents.split('\n').collect();

    let max;
    let mut count = 0;

    let mut values = vec![];

    for i in &contents {
        let j: u32 = match i.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                values.push(count);
                count = 0;
                continue;
            }
        };
        count += j;
    }

    values.sort();
    //Get max calories
    max = values[values.len() - 1];
    //Print max calories
    println!("Max Calories : {}", max);

    //Calculate Top 3
    let top_three = values[values.len() - 1] + values[values.len() - 2] + values[values.len() - 3];
    //Print Top 3
    println!("Top Three : {}", top_three);
    
}

use std::fs;

fn main() {
    let contents = fs::read_to_string("src/puzzle_input.txt").expect("couldn't read file");

    let contents: Vec<&str> = contents.split('\n').collect();

    let mut score: u32 = 0;

    for i in &contents{
        let j: String = match i.trim().parse() {
            Ok(string) => string,
            Err(_) => {
                println!("Couldnt make a string");
                continue;
            }
        };

        // part_1(j, &mut score);

        part_2(j, &mut score);
        
    }

    println!("{}", score);
}

#[allow(dead_code)]
fn part_1(x: String, score: &mut u32){
    // A is Rock B is Paper C is Scissors
    // X is Rock Y is Paper Z is Scissors

    match x.as_str() {
        "A Y" => *score += 8,
        "B Z" => *score += 9,
        "C X" => *score += 7,
        "B X" => *score += 1,
        "C Y" => *score += 2,
        "A Z" => *score += 3,
        "A X" => *score += 4,
        "B Y" => *score += 5,
        "C Z" => *score += 6,
        _ => println!("I dont know what happened"),
    }
}

fn part_2(x: String, score: &mut u32){
    // A is Rock B is Paper C is Scissors
    // X is Lose Y is Tie   Z is Win

    match x.as_str() {
        "A Y" => *score += 4,
        "B Z" => *score += 9,
        "C X" => *score += 2,
        "B X" => *score += 1,
        "C Y" => *score += 6,
        "A Z" => *score += 8,
        "A X" => *score += 3,
        "B Y" => *score += 5,
        "C Z" => *score += 7,
        _ => println!("I dont know what happened"),
    }
}
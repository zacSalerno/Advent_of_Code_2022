use std::fs;

fn main() {
    let contents = fs::read_to_string("src/puzzle_input.txt").expect("couldnt read file");

    let mut count: usize = 0;

    let amount_of_char: usize = 14;

    let mut temp: Vec<u8> = Vec::new();

    for _ in 1..=contents.as_bytes().len() - 3{

        for i in 0..amount_of_char{
            temp.push(contents.as_bytes()[count + i])
        }

        if !(1..temp.len()).any(|i| temp[i..].contains(&temp[i - 1])){
            break;
        } else {
            temp.clear();
        }
        
        count += 1;
    }
    
    println!("{}", count + amount_of_char);

}

use std::fs;

fn main() {
    let contents = fs::read_to_string("src/puzzle_input.txt").expect("couldnt read file");

    let mut count: usize = 0;

    println!("{:?}\n", contents.as_bytes());

    for _ in 1..=contents.as_bytes().len() - 3{

        let temp: [u8; 4] = [contents.as_bytes()[count], contents.as_bytes()[count+ 1], contents.as_bytes()[count + 2], contents.as_bytes()[count + 3]];

        if !(1..temp.len()).any(|i| temp[i..].contains(&temp[i - 1])){
            break;
        }
        
        count += 1;
    }
    
    println!("{:?}", count + 4);

}

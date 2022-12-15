use std::fs;

fn main() {
    let contents = fs::read_to_string("src/puzzle_input.txt").expect("couldnt read file");

    let mut count: usize = 0;

    let mut temp: Vec<u8> = Vec::new();

    println!("{:?}\n", contents.as_bytes());

    for _ in 1..=contents.as_bytes().len() - 3{

        for i in 0..14{
            temp.push(contents.as_bytes()[count + i])
        }

        // println!("{:?}", temp);

        // let temp: [u8; 4] = [contents.as_bytes()[count], contents.as_bytes()[count + 1], contents.as_bytes()[count + 2], contents.as_bytes()[count + 3]];

        if !(1..temp.len()).any(|i| temp[i..].contains(&temp[i - 1])){
            break;
        } else {
            temp.clear();
        }
        
        count += 1;
    }
    
    println!("{}", count + 14);

}

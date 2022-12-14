use std::fs;

fn main() {
    let contents = fs::read_to_string("src/puzzle_input.txt").expect("Couldnt read file");
    let contents: Vec<&str> = contents.split("\n").collect();

    let mut num_of_pairs: u32 = 0;

    for x in &contents {
        let i: String = match x.trim().parse(){
            Ok(string) => string,
            Err(_) => {
                println!("Couldnt make a string");
                continue;
            }
        };
        let v: Vec<u8> = i.split(&[',', '-']).map(|x| x.parse::<u8>().unwrap()).collect();
        

        println!("V : {:?}", v);

        if v[0] <= v[2] && v[1] >= v[3]{
            num_of_pairs += 1;
            println!("Num of Pairs + 1");
        } else if v[2] <= v[0] && v[3] >= v[1]{
            num_of_pairs += 1;
            println!("Num of Pairs + 1");
        }

    }

    println!("{}", num_of_pairs);
}

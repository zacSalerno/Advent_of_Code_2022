use std::fs;
 
fn main() {
    const KEY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    
    let contents = fs::read_to_string("src/puzzle_input.txt").expect("couldnt read file");

    let contents: Vec<&str> = contents.split('\n').collect();

    let mut values: Vec<String> = Vec::new();

    let mut counter = 0;
    let mut counter_2: u32;
    let mut priorities: u32 = 0;

    for x in &contents {
        let j: String = match x.trim().parse() {
            Ok(str) => str,
            Err(_) => {
                continue;
            }
        };
        values.push(j);
        
    }

    for _ in 0..100 {
        
        let list = [&values[counter], &values[1 + counter], &values[2 + counter]];
        println!("{:?}", list);

        for i in KEY.chars(){
            if list[0].contains(i) && list[1].contains(i) && list[2].contains(i){
                counter_2 = u32::from(i.to_string().as_bytes()[0]);
                if counter_2 >= 97{
                    priorities += counter_2 - 96;
                } else {
                    priorities += counter_2 - 38;
                }

            }
            
        }
        counter += 3;
    }

    println!("{}", priorities);

}
use std::fs;
 
fn main() {
    const KEY: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    let mut counter = 0;
    let mut counter_2: u32;
    let mut priorities: u32 = 0;

    let contents = fs::read_to_string("src/puzzle_input.txt").expect("couldnt read file");

    let contents: Vec<&str> = contents.split('\n').collect();

    for x in &contents {
        let v = x.split_at(&contents[counter].len() / 2);
        let list: [&str; 2] = [v.0, v.1];
        println!("{:?}", list);

        for i in KEY.chars(){
            if list[0].contains(i) && list[1].contains(i){
                counter_2 = u32::from(i.to_string().as_bytes()[0]);
                if counter_2 >= 97{
                    priorities += counter_2 - 96;
                } else {
                    priorities += counter_2 - 38;
                }

            }
            
        }

        counter += 1;
    }

    println!("{}", priorities);
}
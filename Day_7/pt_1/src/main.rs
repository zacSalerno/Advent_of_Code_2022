use std::fs;

fn main() {
    let contents = fs::read_to_string("src/puzzle_input.txt").expect("Couldnt read file");

    let contents: Vec<&str> = contents.split('\n').collect();

    let mut current_directory: Vec<&str> = Vec::new();

    let mut size: Vec<u32> = Vec::new();

    let mut size_of_file: u32 = 0;

    for x in &contents {
        let j = x.trim();
        
        if j.starts_with('$') {
            let command = j.split_at(2);
            
            run_command(command.1, &mut current_directory);

        } else if !j.starts_with('$') && !j.starts_with("dir"){
            let i: Vec<&str> = j.split(' ').collect();

            let list = i[0].parse::<u32>().unwrap();

            let current_directory_len: u32 = current_directory.len() as u32 - 1;

            // println!("{}", list);
            
            size_of_file = list * current_directory_len;
            println!("{}", size_of_file);
        }

        if !size.contains(&size_of_file){
            size.push(size_of_file);
        }
        
        // println!("{:?}", size);
    }
    let size: u32 = size.iter().sum();

    println!("{}", size);

}

fn run_command<'a>(run: &'a str, current: &mut Vec<&'a str>) {
    if run == "cd .." {
        // println!("Go back a dir");
        current.pop();
    } else if run == "ls" {
        // println!("List values in dir");
    } else {
        let dir: Vec<&str> = run.split(' ').collect();
        // println!("{:?}", dir);
        current.push(dir[1]);
    }

}
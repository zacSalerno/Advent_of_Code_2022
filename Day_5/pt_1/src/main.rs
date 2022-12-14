use std::fs;

fn main() {
    let contents = fs::read_to_string("src/puzzle_input.txt").expect("couldnt read file");

    let contents: Vec<&str> = contents.split("\n\r").collect();

    let crates: [Vec<&str>; 9] = [vec!["D", "L", "J", "R", "V", "G", "F"], vec!["T", "P", "M", "B", "V", "H", "J", "S"], vec!["V", "H", "M", "F", "D", "G", "P", "C"], vec!["M", "D", "P", "N", "F"], vec!["J", "L", "H", "N", "F"], vec!["N", "F", "V", "Q", "D", "G", "T", "Z"], vec!["F", "D", "B", "L"], vec!["M", "J", "B", "S", "V", "D", "N"], vec!["G", "L", "D"]];
    let mut instructions: Vec<&str> = contents[1].split('\n').collect();

    instructions.remove(0);

    // println!("{:?}", instructions);

    for line in instructions {
        let mod_line = line.trim();

        let mut j: Vec<&str> = mod_line.split(' ').collect();

        j.remove(0);
        j.remove(1);
        j.remove(2);

        let computed_instructions = compute_instructions(j[0], j[1], j[2]);

        // println!("{:?}", computed_instructions);

        let amount_to_move = computed_instructions.0;
        let move_from = computed_instructions.1;
        // let move_to = computed_instructions.2;
        
        for _ in 1..=amount_to_move {
            // let temp = crates[move_from - 1][crates[move_from - 1].len() - 1];
            
            println!("{}", crates[move_from - 1].len());

            // if crates[move_from - 1].len() != 0{
            //     crates[move_from - 1].remove(crates[move_from - 1].len() - 1);
            // }
            

            // crates[move_to - 1].push(temp);
        }

         
    }

    for x in crates {
        println!("{:?}", x[x.len() - 1]);
    }
    

}

fn compute_instructions(amount: &str, from: &str, to: &str) -> (usize, usize, usize){
    (amount.parse().unwrap(), from.parse().unwrap(), to.parse().unwrap())
}

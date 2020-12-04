fn main() {
    let file_path = "src/day_1_input.txt";
    let content = std::fs::read_to_string(file_path).expect("could not read file");

    let mut num_counter_1 = 0;
    let mut num_counter_2 = 0;
    let mut num_counter_3 = 0;

    let mut vars = [0, 0, 0];

    // faster approaches:
    // sort - and then iterate through 2 or 3 at a time until you get 2020

    // improvements:
    // make directories for each day
    // make CLI arguments for --demo and 

    for line in content.lines() {
        let num1 = line.parse::<i32>().unwrap();

        for line2 in content.lines() {
            let num2 = line2.parse::<i32>().unwrap();

            for line3 in content.lines() {
                let num3 = line3.parse::<i32>().unwrap();

                if num_counter_1 != num_counter_2
                    && num_counter_1 != num_counter_3
                    && num_counter_2 != num_counter_3
                {
                    let sum = num1 + num2 + num3;
                    if sum == 2020 {
                        vars[0] = num1;
                        vars[1] = num2;
                        vars[2] = num3;
                    }
                }
                num_counter_3 += 1
            }
            num_counter_2 += 1;
        }
        num_counter_1 = 1;
    }

    println!("num1: {}", vars[0]);
    println!("num2: {}", vars[1]);
    println!("num3: {}", vars[2]);
    println!("product: {}", vars[0] * vars[1] * vars[2]);
}

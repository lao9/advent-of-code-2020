// use ferris_says::say;
// use std::io::{stdout, BufWriter};

// a comment

fn main() {
    let file_path = "src/demo_day_1_input.txt";
    let content = std::fs::read_to_string(file_path)
        .expect("could not read file");


    let thing = [];
    
    for (line in content.lines()) {
        let string = line.read_to_string;
        
    }

    for x in 0..item_count {
        println!("value of iterator is: {}", thing[x]);
      }
    
    // for line in content.lines() {
    //     println!("{}", line);
    // }

    // let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());
    // say(message.as_bytes(), width, &mut writer).unwrap();
}





fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut sizes: 
    let mut vals: Vec<String> = Vec::new();
    let mut not_stable: i32 = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap().to_string();
        // Show the line and its number.
        //println!("{}. {}", index + 1, line);
        if index == 0 {
            _n = line.parse::<i32>().unwrap();
        } else {
            vals.push(line);
        }
    }
}

use std::io::Write;
fn main() {
    let mut file = std::fs::File::create("results.txt").expect("create failed");
    for i in 2..5000000 {
        let mut number: i128 = i;
        file.write_all(i.to_string().as_bytes()).expect("write failed");
        file.write_all(": ".as_bytes()).expect("write failed");
        while number != 1 {
            if number % 2 == 0 {
                number = number /2;
                file.write_all(number.to_string().as_bytes()).expect("write failed");
                file.write_all(" ".as_bytes()).expect("write failed");
            } else {
                number = (number*3)+1;
                file.write_all(number.to_string().as_bytes()).expect("write failed");
                file.write_all(" ".as_bytes()).expect("write failed");
            }
        }
        file.write_all("\n".as_bytes()).expect("write failed");
        println!("{}", i);
    }
}
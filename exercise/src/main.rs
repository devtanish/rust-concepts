

fn main() {
    let file = std::fs::read_to_string("hello.txt");
    match file {
        Ok(content) => println!("{:?}", content),
        Err(err) => println!("Error while reding file: {}", err)
    }
}
fn main() {
    let var = add(1, 2);
    println!("{:?}", var);
}

fn add<T: std::ops::Add<Output = T>>(num1: T, num2: T) -> T{
    num1 + num2
}
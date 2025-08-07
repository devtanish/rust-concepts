fn main(){
    //without using mutables you can you change the value of a variable, you can asign a mutable variable using sign 'mut'
    // if you dont use 'mut' sign then you can not change the value of a variable
    // if you try to change the value of a variable without using 'mut' sign then
    // you will get an error: cannot assign twice to immutable variable `number`
    // so you have to use 'mut' sign before the variable name to make it mutable
    // it default if you asign a variable lt suppose number of 14 so at default it is immutable variable you can not change the value of it
    let mut number: i32 = 42;

    println!("The value of number is {number}");
    number = 43;
    println!("The value of number is {number}");
}
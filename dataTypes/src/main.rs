fn main() {
    /* i32 represent 32 byte signed integer which have value form -int to +int
        32-bit signed integers can represent values from -2,147,483,648 to 2,147,483,647.
        The i32 type is commonly used for counting, indexing, and other operations that require
        a signed integer value. It is a fixed-size type, meaning it always occupies 4 bytes
        
        Example of more signed integer types in Rust are i8, i16, i64, and i128.
        The unsigned integer types in Rust are u8, u16, u32, u64, and u128,
        which can represent values from 0 to 2^n - 1, where n is the number of bits.
      

      
        The u32 type is commonly used for operations that require an unsigned integer value,
        such as counting, indexing, and other operations that do not require negative values.
        The u32 type is a fixed-size type, meaning it always occupies 4 bytes.
        Example of more unsigned integer types in Rust are u8, u16, u64, and u128.
      
      
      
        The f64 type is commonly used for floating-point operations that require
        a double-precision floating-point value. It is a fixed-size type, meaning it
        always occupies 8 bytes. Example of more floating-point types in Rust are f32.
    */ 
       
    // &str represent a string slice which is a reference to a string
    // String is a growable string type, but here we are using &str for a
    // fixed string slice
    // Note: Rust is a statically typed language, so types are known at compile time
    // and do not change at runtime.
    // The types are inferred by the compiler based on the values assigned to them.
    // The following code demonstrates the use of these data types in Rust
    let number: i32 = 42;

    // if  you asigned a variable and not using it so you have to use '_' sign before the variable name
    let _unsigned_number: u32 = 100;
    let _float_number: f64 = 3.14;
    let _boolean_value: bool = true;
    let _character: char = 'A';
    let _string: &str = "Hello, Rust!";

    println!("inter value is {}", number);
}

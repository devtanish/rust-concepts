/*
    // Structs in rust let you structure data together. Similar to objects in javascript
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };

        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );
    }

    fn area(rectangle: &Rectangle) -> u32 {
        rectangle.width * rectangle.height
    }
*/


#[derive(Clone)]

//Structs in rust let you structure data together. Similar to objects in javascript
struct Book {
    title: String,
    author: String,
    pages: u32,
    price: u32
}

impl Book {
    fn book_info(&self){
        println!("title of the book is: {}\nauthor of the book is: {}\npages of book are: {}\nprince is only: {}", self.title, self.author, self.pages, self.price);
    }

    fn is_expensive(&self) -> bool{
        if self.price > 500 {
            true
        } else {
            false
        }
    }

    fn update_price(&mut self, new_price: u32){
        self.price = new_price;
    }
}

fn main() {
    let mut book1: Book = Book {
        title: String::from("Rich Dad Poor Dad"),
        author: String::from("Ram Lakhan"),
        pages: 400,
        price: 450
    };

    let book2: Book = Book {
        title: String::from("Rust Book"),
        author: String::from("Davin jonson"),
        pages: 1300,
        price: 1550
    };

    book1.book_info();
    book1.update_price(550);
    book1.book_info();
    println!("high price: {:?}", book2.is_expensive());
}
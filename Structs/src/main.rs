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
struct User {
    active: bool,
    _sign_in_count: u64,
    _username: String,
}

fn main() {
    let user1 = User {
        active: true,
        _sign_in_count: 1,
        _username: "harkirat".to_string()
    };

    change_name(user1.clone());
    print!("User 1 username: {}", user1.active); // Error - can not use borrowed value
}

fn change_name(user1: User) {
    print!("User 1 username: {:?}", user1.active);
}
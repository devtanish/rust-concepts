
// this is a struct which holds the hight, width 
struct Rect{
	height: f32,
	width: f32,
}

// this is a implentation of Rect, you have to declare same name you made for "Rect" to use with Rect struct
impl Rect{
	fn area(&self) -> f32 {
		self.height * self.width
	}
	fn square(&self, size: f32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
	fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
	// you can define a vaiable of struct using this format and also get the full access of all implementation of Rect
	let rectangle = Rect {
		height: 20.05,
		width: 1.05,
	};
	let rectangle2 = Rect {
		height: 20.05,
		width: 1.05,
	};

	// you can use the implementation and struct using object like notation which we use's in javaScript or in Typescript
	println!("the area of rectangle is: {}", rectangle.area());
	println!("{:?}", rectangle2.can_hold(&rectangle));
	println!("the area of rectangle is: {}", rectangle.square(2.20).area());
}
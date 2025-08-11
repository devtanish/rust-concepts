fn main(){
	// References mean giving the address of a string rather than the ownership of the string over to a function
	// if we use value directly in borrow so the compiler will throw a error of ownership so we have to use "&" when we
	// want to take a dynimic variable reference "&" mean you are borrowing the value of value so it look like 
	// " borrow -> " points to  " Address of value and value points to the address of where data present into the heap"
	// and if we deleted the borrow data so the only thing delet is the borrow pointed address to the value    
	/*
		let value: String = String::from("Tanish ");
		let borrow: &String = &value;

		println!("{:?}", value);
		println!("{:?}", borrow);
	*/

	/*
		RULES
		   1. There can me many immutable references at the same time
		   2. There can be only one mutable reference  at a time
		   3. If there is a mutable reference , you can’t have another immutable reference either.

	*/

	// and if you want to pass a mutible string so you can write like " variable_name: &mut data_type "
	// There can be only one mutable reference  at a time
	// If there is a mutable reference , you can’t have another immutable reference either.
	let mut value: String = String::from("Tanish ");
	let borrow: &mut String = &mut value;
	add(borrow, "Vishwakarma");
	println!("{}", value);


	// immutable Reference 
	// There can me many immutable references at the same time
	/* 
		let value: String = String::from("Tanish");
		let borrow1: String = &value;  
		let borrow2: String = &value;
		let borrow3: String = &value;
		let borrow4: String = &value;
	*/

}

//" variable_name: &mut data_type" Mutable references
fn add(reference: &mut String, data: &str){
	reference.push_str(data);
}
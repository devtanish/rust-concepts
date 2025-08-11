fn main() {
    // these are the variable which is signed integer not a String or vector which is dynmic in nature so they are present
    // into stack in RAM not in heap this programe not throw a ownership error
    /*
        let number: i32 = 5;
        let num: i32 = number;

        println!("{:?}", number);
        println!("{:?}", num);
    */


    // These are the variables which have a String datatype and String is dynimic in nature because they take a dynimic 
    // size memory because String is a array of charecters and each charecter contain 1 byte memory/space so as the 
    // lenth of string increse the number of byte also incresses, so the string goes into a heap memory which is in
    // the RAM because heap deal with dynamic memory allocation

        // Owner of "name" Address is "name"
        let name: String = String::from("Tanish");
        // now Owner of "name" is "change"
        let change: String = name; // to resolve this you can do "name.clone()" or use references and borrowing
        
        // println!("{:?}", name);    // this line code will throw an ownership error because the owner of "name" is "change"
        println!("{:?}", change);  // this is the owner of name address 
}


fn main(){
    let mut vec: Vec<i8> = vec![9, 5, 7, 4, 2, 1, 3, 6, 8];

    println!("unsorted array: {:?}", vec);

    sort_array(&mut vec);

    println!("Sorted array: {:?}", vec);

}

fn sort_array(vec: &mut Vec<i8>){
    vec.sort();
}
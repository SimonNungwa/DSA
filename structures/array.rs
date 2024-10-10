let mut array: [i32; 3] = [0; 3];

array[1] = 1;
array[2] = 2;

assert_eq!([1, 2], &array[1..]);


for x in array {
    print!("{x} ");
}


/*
    // Rust Arrays
    // data type
    // ! data type
    // default values
*/


// data type

fn main() {
    // initialize array with data type
    let numbers : [i32 ; 5] = [1, 2, 3, 4, 5];
    prinln("Array of numbers = {:?}", numbers)

}

// ! data type

fn main() {
    // initialize array without data type
    let numbers = [1, 2, 3, 4, 5];
    prinln("Array of numbers = {:?}", numbers)

}


// default values

fn main() {
    // initialize array with default values
    let numbers : [i32 ; 5] = [3, 5];
    prinln("Array of numbers = {:?}", numbers)

}


fn main() {
   //creating an empty, new vector

   let v: Vec<i32> = Vec::new(); // we annotate because we're not putting any values inside as parameters

   let v = vec![1, 2, 3]; // this vector contains new values
                            // because we have given initial type i32 values, Rust can infer that the type of 'v' is Vec<i32> and the type annotation is not neccessary.


    // Updating a Vector

    let mut v = Vec::new(); // making this variable mutable using the keyword 'mut' - allows us to chenge it later, when we want to.
    
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // the numbers inside are all 'i32' - Rust infers this from the data, we don't need to the 'Vec<i32>' annotation.
    
    // Reading Elements of Vectors - there are two ways to reference a value inside a vector - 1) indexing OR 2) using the 'get' method

    // we annotate the type of values returned from these functions for extra clarity
    
    let v = vec![1, 2, 3, 4, 5, 6];

    let third: &i32 = &v[2];
    printlin!("The third element is {third}");

    let third: Option<i32> = v.get(2);
    match third {
        Some(third) => printlin!("The third element is {third}"),
        None => printlin!("The is no third element. "),
    }

    // Rust provides these two methods to reference an element so you can 
    // choose how the program behaves when you try to use an index valueoutside the range of existing elements.
    
    // LETS's see what happens when we have a vector od 5 elements and we try and access an element at index 100

    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    // when we run this code, the first [] method will cause the program to panic because it references a non-existent element.
    // This methos is best used when you want your program to crash if there's an attempt to access an element past the end of a vector.

    // When the get method is passed an index that is outside the vector, it returns 'None' without panicking.You would use this methodif accessing an element beyond that range of the vector may happen occasionally under normal circumstances. The code will then have logic to handle having either 'Some(&element) or 'None' - this index could be coming from a person entering a number.
    // If they accidentally enter a number that's too large and the program gets a 'None' value - you could tell a use
    // many items are in the current vectorand give them another chance to try rather than crashing the program.
}

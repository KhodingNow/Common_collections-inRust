There are two ways to reference a value stored in a vector : via indexing or by using the 'get' method as already mentioned.

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
Notice a few details here. We use the index value of 2 to get third element because vector are indexed by number, starting from zero. Using '&' and '[]' gives us a reference to the element at the index value. When we use the 'get' method with the index passed as an argument, we get an 'Option<&T>' that we can use with 'match'.

Rust provides these two ways to reference an element so you can choose how the program behaves when you try to use an index value outside the range of existing elements. As an example, let's see what happens when we have a vector of five elements and then we try to access an element at index '100' with each technique;

let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100); 

// Attempting to access an indexed element at '100' in a vector containing five elements//

When we run this code, the first '[]' method will cause the program to panic because it references a nonexistant element. This method is best used when you want your program to crash if there's an attempt to access an element past the end of a the vector.

When the 'get' method is passed an an index that is outside the vector, it returns 'None' without panicking. You would use this method if accessing an element beyond the range of the vector, it may happen occassionally under normal circumstances. The code will then have logic to handle having either 'Some(&element)' or 'None'.

When the program has a valid reference, the borrow checker enforces the ownership and borrowing rules to ensure that this reference and any other references to the contentsof the vector remain valid. Remember, the rules states that you can'y have mutable and immutable refrences in the same scope.

 This rule applies here: 

     let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {first}");

// We hold an immutable reference to the first element in a vectorand try to add an element to the end. This program won't work if we also try to refer to that element later in the function.

This program looks like it should work---why should a reference to the first element care about changes at the end of the vector? This error is due to the way vectors work: because vector put the values next to each other in memory, adding a new element onto the end of the vector might require allocating new memory and copying the old elements to the new space, if there isn't enough room to put all the elements next to each other where the vector is currently stored. In that case, the reference to the first element would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation. 




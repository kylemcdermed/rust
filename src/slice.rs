// THE SLICE TYPE
//
// slices let you refernce a contiguous sequence of elements in a collection. a slice is a kind of
// reference so it does not have ownership
//
// here is a small programming problem, write a function that takes a string of words separated by
// spaces and returns the first word it finds in that string. if the function doesnt find a space
// in the string, the whole string must be one word, so the entire string should be returned
//
// lets work through how wed write the signature of this function without using slices, to
// understand the problem that slices will solve:
//
// fn first_word(s: &String) -> ?
//
// the first_word function has a parameter of type &String. we dont need ownership, so this is
// fine, but what should we return? we dont really have a way to talk about part of a string.
// however, we could return the index of the end of the word, indicated by a space, lets try that.
//
// because we need to go through the String element by element and check whether a value is a
// space, well convert our String to an array of bytes using the as_bytes method
//
// let bytes = s.as_bytes();
//
// next we create an iterator over the array of bytes using the iter method:
//
// for (i, &item) in bytes.iter().enumerate() {}
//
// iter is a method that returns each element in a collection and that enumerate wraps the result
// of iter and returns each element as part of a tuple instead. the first element of the tuple
// returned from enumerate is the index, and the second element is a reference to the element. this
// is a bit more convenient than calculating the index ourselves.
//
// because the enumerate method returns a tuple, we can use patterns to destructure that tuple. in
// a for loop, we specify a pattern that has i for the index in the tuple and &item for the single
// byte in the tuple. because we get a reference to the element from .iter().enumerate(), we use
// the & in the pattern.
//
// inside the for loop, we search for the byte that represents the space by using the byte literal
// syntax. if we find a space, we return the position. otherwise, we return the length of the
// string by using s.len() :
//
// if item == b' ' {
//      return i;
//  }
//
//  s.len()
//
// we now have a way to find out the index of the end of the first word in the string, but theres a
// problem. were returning usize on itws own, but ots only meaningful number in the context of the
// &String. in other words, because its a separate value from the String, theres no guarantee that
// it will still be valid in the future.
//
// this program compiles without any errors and would also do so if we used word after calling
// s.clear(). because word isnt connected to the state of s at all, word still contains the value
// 5. we could use that value 5 with the variable s to try to extract the first word out, but this
// would be a bug because the contents of s have changed since we saved 5 in word
//
// having to worry about the index in word getting out of sync with the data in s is tedious and
// error prone! managing these indices is even more brittle if we write a second_word function. its
// signature would have to look like this:
//
// fn second_word(s: &String) -> (usize, usize) {}
//
// now were tracking a starting and an ending index, and we have even more values that were
// calculated from data in particular state but arent tied to that state at all. we have three
// unrelated variables floating around that need to be kept in sync
//
// luckily, Rust has a solution to this problem: string slices
//
//
// STRING SLICES 
//
// a string slice is a reference to a contiguous sequence of elements of a String and it looks like
// this:
//
// let s = String::from("hello");
// let hello = &s[0..5];
// let world = &s[6..11];
//
// rather than reference to the entire String, hello is a reference to a portion of the String,
// specified in the extra [0..5] bit. we create slices using a range within brackets by specifying
// [starting_index..ending_index], where starting_index is the first position in the slice and
// ending_index is one more than the last position in the slice. internally, the slice dats
// structure stores the starting position and the length of the slice, which corresponds to
// ending_index minus starting_index. so, in the case of let world = &s[6..1]; world would be a
// slice that contains a pointer to the byte at index 6 of s with a length value of 5
//
// with Rusts' .. range syntax, if you want to start at index 0, you can drop the value before the
// two periods. in other words, these are equal:
//
// let s = String::from("hello");
// let slice = &s[0..2];
// let slice = &s[..2];
//
// by the same token, if your slice includes the last byte of the String, you can drop the trailing
// number. that means these are equal:
//
// let s = String::from("hello");
// let len = s.len();
// let slice = &s[3..len];
// let slice = &s[3..];
//
// you can also drop both values to take a slice of the entire string, so these are equal:
//
// let len = s.len();
// let slice = &s[0..len];
// let slice &s[..];
//
// with all this information in mind, lets rewrite first_word to return a slice. the type that
// signifies 'string slice' is written as &str:
//
// we get the index for the end of the word the same way we did before, by looking for the first
// occurrence of a space. when we find a space, we return a string using the start of the string
// and the index of the space as the starting and ending indices
//
// now when we call first_word, we get back a single value that is tied to the underlying data. the
// value is made up of a reference to the starting point of the slice and the numbe rof elements in
// the slice.
//
// returning a slice would also work for a second_word function: 
//
// fn second_word(s: &String) -> &str {}
//
// we now a straightfoward API thats much harder to mess up because the compiler will ensure the
// references into the String remain valid. Remember the bug in the program before? when we got the
// index to the end of the first word but then cleared the string so our index was invalid? the
// code was logically incorrect but didnt show any immediate errors. the problems would show up
// later if we kept trying to use the first word index with an emptied string. slices make this bug
// impossible and let us know we have a problem with our code much sooner. using the slice version
// of first_word will throw a compile time error
//
// recall from the borrowing rules that if we have an immutable reference to something, we cannot
// also take a mutable refernce. because clear needs to truncate the String, it needs to get
// mutable reference. the println! after the call to clear uses the reference in word, so the
// immutable reference must still be active at that point. Rust disallows that mutable reference in
// clear and the immutable reference in word from exitisting at the same time and compilatio fails.
// Not only has Rust made our API easier to use, but it also has elimainated an entire class of
// errors at compile time!
//
//
// STRING LITERALS AS SLICES
//
// recall that we talked about string literals being stored inside the binary. now that we know
// about slices, we can properly understand string literals:
//
// let s = 'hello, world!';
//
// the type of s here is &str: its a slice pointing to that specific point of the binary. this is
// also why strig literals immutable; &str is immutable reference
//
//
// STRING SLICES AS PARAMETERS 
//
// knowing that you can take slices of literals and String values lead up to one more improvement
// on first_word, and that its signature:
//
// fn first_word(s: &String) -> &str {}
//
// a more experienced Rustacean would write the signature shown below because it allows us to use
// the same function on both &String values andn &str values
//
// fn first_word(s: &str) -> &str {}
//
// if we have a string slice, we can pass that directly. if we have a String, we can pass a slice
// of the String or reference to the String. this flexibility takes advantage of deref coercions
//
// defining a function to take a string slice instead of a reference to  a String makes our API
// more general and useful without losing any functionality:
//
// 
// OTHER SLICES 
//
// string slices, as you might imagine, are specific to strings. but theres more general slice type
// too. consider this array:
//
// let a = [1,2,3,4,5];
//
// just as we might want to refer to part of a string, we might want to refer to part of an array,
// wed do so like this:
//
// let a = [1,2,3,4,5];
// let slice = &a[1..3];
// assert_eq!(slice, &[2..3]);
//
// this slice has the type &[i32]. it works the same way as string slices do, by storing a
// reference to the first element and a length. youll use this kind of slice for all sorts of other
// collections. well discuss these collections in detail whn we talk more about vectors
//
//
// SUMMARY 
//
// the concept of ownership, borrowing and slices ensure memory safety in Rust programs at compile
// time. the Rust language gives you control over your memory usage in the same way as other system
// languages, but having the owner of data automatically clean up that data when the owner goes out
// of scope means you dont have to write and debug extra code to get this control
//
// owernship affects how lots of other parts of rust work, so well talk about these concepts
// further

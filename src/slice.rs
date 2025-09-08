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
//

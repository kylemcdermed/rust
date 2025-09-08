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

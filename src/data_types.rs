// Data Types
//
// Every value in Rust is of a certain data type
//
// this tells Rust what kind of data is being specified so it knows how to work with  that data 
//
// theres 2 data type subsetes: scalar and compound
//
// Rust is a statically typed language, which means it must know the types of all variables at
// compile time 
//
// The compiler can infer what type we want to use based on the value and how we use it 
//
// In cases where many type are possible, such as when we converted a String to a numeric type
// using parse like this :
// 
// let guess: u32 = "42".parse().expect("Not a number!");
//
// if we dont add the : u32 type annotation shown in the preceding code, Rust will display the
// following error, "type must be known as this point"
//
//
// SCALAR TYPES
//
// A scalar type represents a single value, Rust has four primary scalar types: integers, floating
// point numbers, Booleans and characters. 
//
// Integer Types --
//
// An integer is a number without fractional component. 
// Type Declaration indicates that the value is associated with either an unsigned or signed
// integer type (signed integers start with `i` instead of `u`).
//
//
// Integer Types in Rust
// Length:  Signed  Unsigned
// 8-bit    i16     u16
// 32-bit   i32     u32
// 64-bit   i64     u64
// 128-bit  i128    u128
// architecture dependent   isize   usize
//
//
// Each variant can be either signed or unsigned and has explicit size.
// Signed and unsigned refer to whether it's possible for a number to be negative (signed) or
// whether it will only ever be positive (unsigned)
//
//
// Floating Point Types --
//
// Rust has two primitive types for floating point numbers 
// Rust's floating point types are f32 and f64, which are 32 bits ans 64 bits in size, respectively
// The default type is f64
//
// 
// Numeric Operations --
//
// Rust supports basic mathematical operations for all number types:
// addition, subtraction, multiplication, division, remainder
//
// Integer division truncates toward zero to the nearest integer.
//
//
// The Boolean Type --
//
// In most other languages, Boolean type in Rust has two possible values: true and false
// Booleans are one byte in size. Boolean type in Rust is specified using bool
//
// The Character Type 
//
// Rusts char type is the most primitive alphabetic type.
//
// We specify char literals with single quotes, as opposed to string literals which use double
// quotes
//
// Rust's char type is four bytes in size
//
//
// Compound Types --
//
// Compound types can group multiple values into one type. Rust has two primitive compound types:
// tuples and arrays
//
// The Tuple Type --
//
// A tuple is a general way of group together a number of values with a variety of types into one
// compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size
//
// We create a tuple by writing a comma-seperated list of values inside parentheses. Each position
// in the tuple has a type, and types of different values in the tuple dont have to be the same
//
// the variable tup binds to the entire tuple because tuple is considered a single compound
// element. To get the individual values of a tuple, we can use pattern matching to destructure a
// tuple value
//
// the program first creates a tuple and binds it to the variable tup. it then uses a pattern with
// let to take tup and turn it into three seperate variables x,y,z. this is called destructuring
// because it breaks the single tuple into three parts. finally the program prints the value of y,
// which is 6.4
//
// we can also access a tuple element directly using a period (.) followed by the index of the
// value we want to access
//
// this program creates the tuple x and then accesses each element of the tuple using their
// respective indices. as with most programming languages, the first index in a tuple is 0.
//
// the tuple without any values has a special name, unit. this value and its corresponding type are
// both written () and represent an empty value or an empty return type. Expressions implicitly
// return the unit value if they dont return any other value
//
//
// The Array Type --
//
// Another way to have a collection of multiple values is with an arra. unlike a tuple, every
// element of an array must hav ethe same type. un like arrays in some other language, arrays in
// rust has a fixed length.
//
// we write the values in an array as a comma-seperated list inside square brackets
//
// arrays are useful when you want your data allocated on the stack, the same as the other types we
// have seen.
//
// rather than the heap, or when you want to ensure you always have a fixed number of elements. an
// array isnt as flexible as the vector type.
//
// a vector is a similar collection type provided by the standard lobrary that IS ALLOWED to grow
// or shrink in size because its contents live on the heap. if youre unsure whether to use an array
// or a vector, chances are you should use a vector
//
// however, arrays are more useful when you know the number of elements will not need to change.
// for exmaple lets say you were using the names of the month in a program, you would probably use
// an array rather than a vector becasue you know it will always contain 12 elements
//
// you write an arrays type using square brackets with the type of each element, a semicolon and
// then the number of elements in the array
//
// here, i32 is the type of each element, after the semi colon, the number 5 indicates teh array
// contains 5 elements
//
// you can also initialize an array to contain the same value for each element by specifying the
// initial value, followed by a semi colon, then the length of the array in square brackets as
// shown here:
//
// let a = [3; 5];
//
// the array named a will contain 5 elements that will all be set to the value 3 initially. this is
// the same as writing let a = [3,3,3,3,3]; but in a more concise way
//
// Accessing Array Elements --
//
// an array is a single chunk of memory of known, fixed size that can be allocated on the stack.
// you can access elements of an array using indexing like this 
//
// in this example the variable named first will get the value of 1 because that is the value at
// the index [0] in the array. the variable named second will get the value 2 from the index [1] in
// the array
//
//
// Invalid Array Element Access --
//
// lets see what happens if you try to access an element of an array that is past the end of the
// array, say you run this code similar to guessing game code in chapter 2, to get an array index
// from the user
//
// this code compiles successfully. if you run this code using `cargo run` and enter 0,1,2,3,4 the
// program will print out th ecorresponding value at that index in the array if you enter a number
// past the end of the array such as 10, youll see output like this:
// "index out of bounds: len is 5 but the index is 10"
//
// the program resulted in a runtime erorr at the point of using an invalud value in the indexing
// operation.
//
// the program exited with an error message and did not excute the final println! statement.
//
// when you attemp tto access an element using indexing, Rust will check that the index youve
// specified is less than the array length. if the index is greater than or equal to the length,
// Rust will panic. this check has to happen at runtime, especially in this case, because the
// compiler cant possibly know what the value a user will enter whne they run the code later.
//
// this is an example of Rust's memory safety principles in action. Rust protexts you against any
// kind of error by immediately exiting instead of allowing the memory access and continuing.

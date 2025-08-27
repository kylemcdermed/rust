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

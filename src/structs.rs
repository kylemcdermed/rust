// USING STRUCTS TO STRUCTURE RELATED DATA
//
// a struct, or structure, is a custom data type that lets you package together and name multiple
// related values that make up a meaningful group. a struct is like an objects data attributes.
//
// well compare and contract tuples with structs to build on what you already know and demonstrate
// when structs are better way to group data.
//
//
//// DEFINING AN INSTANTIATING STRUCTS
//
// structs are similar to tuples, that they both hold multiple related values. like tuples, the
// pieces of a struct can be different types. unlike tuples, a struct youll name each piece of data
// so its clear what the values mean
//
// adding names means that structs are more flexible than tuples: you dont have to rely on the
// order of the data to specify or access the values of an instance
//
// to define a struct we enter the keyword struct and name the entire struct. a structs name should
// describe the significance of data being grouped together.
//
// then inside the curly brackets we define names and types of data we call fields
//
// struct User {
//      active: bool,
//      username: String,
//      email: String,
//      sing_in_count: u64,
// }
//
// like above
//
// to use a struct after we have defined it, we create an instance of that struct by specifying
// concrete values for each of the fields. we can create an instance by stating the name of the
// struct and then add curly brackets containing `key:value` pairs, where the keys are the names of
// the fields and the values are the daa we want to store in those fields.
//
// to get specific value from a struct we use dot notation. for example, to access this users email
// address we use --> user1.email
//
// if the instance is mutable, we can change a value by using the dot notation and assigning into a
// particular field
//
// note that the entire instance is mutable; Rust doesnt allow us to mark only certain fields as
// mutable, as with any expression, we can construct a new instance of the struct as the last
// expression in the function body to implicitly return that new instance
//
// lets create a build_user function that returns a User instance with the given email and
// username. the active field gets the value of true and the sing_in_count gets a value of 1
//
// it makes sense to name the function parameters with the same name as the struct fields but
// having to repeat the email an dusername field names and variables is a bit tedious. if the
// struct had more fields repeating each name would get annoying luckily theres a shorthand
//
//
// USING THE FIELD INIT SHORTHAND 
//
// because the paraneter names and struct fields names are exactly the same, we can use the field
// init shorthand syntax to rewrite build_user so it behaves exactly like the same but doesnt have
// the repitition of username and email
//
// here we are creating a new instance of the User struct, which has a field name email. we want to
// set the email field value in the email parameter of the build_user function. because the email
// field and the email parameter have the same name, we only need to write email rather than email:
// email
//
//
// CREATING INSTANCES FROM OTHER INSTANCES WITH STRUCT UPDATE SYNTAX 
//
// its often useful to create a new instance of a struct that icludes most of the values from
// another instance using the same type, but changes some. you can do this using struct update
// syntax
//
// first we show how to create a new user instance in user 2 regularly, without the update syntax.
// we set a new value for email but otherise use the same values from user1 
//
// using struct update syntax we can achieve the sam effect with less code, the syntax `..`
// specifies that the remaining fields not explicitly set should have the same value as the fields
// in the given instance
//
// the code creates an instance in user2 that has a different value for email but has the same
// values for username, active, sign_in_count fields from user1
//
// the ..user1 must come las tto specify that any remaining fields should get their values from the
// corresponding fields in user1, but we can choose to specify values for as many fields as we want
// in any order, regardless of the order of the fields in the structs definition
//
// note that the struct update syntax uses `=` like an assignmentl this is because it moves the
// data, just as we saw in the variables and data interacting with move section.
//
// in this example we can no longer use user1 afer creating user2 because the String in the
// username field of user1 was moved into user2, if we had given user2 new String values for obth
// email and username, and this only used the active and sign_in_count values from user1, then
// user1 would still be valid after creating user2. both active and sign_in_count are types that
// implemenet the Copy train, so the behavior we discussed in Copy sectio would apply. we can also
// still use user1.email in this example because its value was note moved out of user1
//
//
// USING TUPLE STRUCTS WITHOUT NAMED FIELDS TO CREATE DIFFERENT TYPES 
//
// Rust also supports structs that look similar to tuples called tuple structs, tuple structs have
// the added meaning the struct name provides but doesn thave names associated with their fields;
// rather, they just have the types of the fields. tuple structs are useful when you want to give
// the whole tuple a name and make the tuple a different type from other tuples, and when naming
// each field as in a regular struct would be verbose or redundant 
//
// to define a tuple struct, stat with the struct keyword and the struct name followed by the types
// in the tuple
//
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
//
// fn main() {
//      let black = Color(0,0,0);
//      let origin = Point(0,0,0);
// }
//
// note that black and origin values are different types because theyre instances of different
// tuple structs, each struct you define is its own type, even though the fields within the struct
// might have the same types. for example, a function that takes a parameter of type Color cannot
// take a Point as an argument, even though both types are made up of three i32 values. otherwise
// tuple structs instances are similar to tuples in that you can destructre them into their
// individual value. unlike tuples, tuple structs require you to name the type of the struct when
// you destruct them. for example, we would write let Point(x,y,z) = origin; to destructure the
// values in the oprigin point into variables named x,y,z
//
//
// UNIT LIKE STRUCTS WITHOUT ANY FIELDS 
//
// you can define structs that dont have any fields! these are called unit-like structs because
// they behave similarly to `()`, the unit type we mentioned in tuple type section. unit like
// structs can be useful when you need to implement a trait on some type but dont have any data
// that you wan tto store in the type itself.
//
// struct AlwaysEqual;
//
// fn main() {
//      let subject = AlwaysEqual;
// }
//
// to define AlwaysEqual, we use the struct keyword, the name we want, and then a semi colon. No
// need to curly brackets or parenthesis! then we can get an instance of AlwaysEqual in the subject
// variable in a similar eway: using the name we defined, without any curly bracketes or
// parentheses. imagine that later well implement behavior for this type such that every instance
// of AlwaysEqual is always equal to every instance of any other type, perhaps to have a known
// results for testing purposes.
//
//
// OWNERSHIP OF STRUCT DATA
//
// in the user struct, we used the owned String type rather than the &str string slice type. this
// is delibrate choice because we want each instance of this struct to own all of its data for that
// data to be valid for as long as the entire struct is valid.
//
// its also possible for structs t store references to data owned by something else, but to do so
// requires the use of lifetimes, a Rust feature. Lifetimes ensure that the data referenced by a
// struct is valid for as long as the struct is. Lets say you try to store a reference in a struct
// without specifying lifetimes, like the following this wont work
//
// 
// AN EXMAPLE PROGRAM USING STRUCTS
//
// to understand when we might wan tto use structs lets write a progream tha tcalculates the area
// of a rectangle. well start by using single variables, then refractor the program until were
// using structs instead
//
// lets make a new binary project with cargo called rectangles that will take the width and height
// of a rectangle specified in pixels and calculate the area of the rectangle
//
// the code succeeds in figuring out the area of the rectangle by calling thee area function with
// each dimension but we can do more to make this code clear and readble
//
// the issue with this code is evident in the signature of area:
//
// fn area(width: u32, height: u32) -> u32 {}
//
// the area function is suppose to calculate the area of one rectangle but the function we wrote
// has two parameters, and its not clear anywhere in our program that the parameters are relatred.
// it would be more readable and more manageable to group width and height together, as we have
// disucssed one wya to do that in tuples
//
//
// REFRACTORING WITH TUPLES
//
//

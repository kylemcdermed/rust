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
// in one way this program is better. the tuples let us add a bit of structure and now were passing
// just one argument but in another way, this version is less clear: tuples dont name their
// elements, so we have to index into the parts of the tuple, mkain gour calculation less obvious.
//
// mixing up the width and height wouldnt matter for the area calculation, but if we wan tto draw
// the rectangle on the screen, it would matter! we would have ot keep in mind that width is the
// tuple index 0 and height is the tuple index 1. this would be even harder for someone else to
// figure out and keep in mind if they were to use our code. because we havent conveyed the
// meaning of our data in our code its now eaier to introduce errors.
//
//
// REFRACTORING WITH STRUCTS: ADDING MORE MEANING 
//
// we use strcuts to add meaning by labeling the data. we transform the tiple were using into a
// struct with a name for the whole as well as names for parts
//
// were we have defined a struct and named it Rectangle. inside the curly brackets, we defined the
// fields as width and height, both of which have type u32. then in main we created a particular
// instance of Rectangle that has a width of 30 and a height of 50.
//
// our area function is now defined with one parameter, which is named rectange, whose type is an
// immutable borrow of struct rather than take ownership of it. this way main retains its ownership
// and can continue using rect1 which is the reason we use the & in the function signature and
// where we call the function
//
// the area function accesses the width and height fields of the rectangle instance (note that
// accessing fields of borrowed struct instance does not move the field values, which is why you
// can often see borrows of structs). our function signature for area now says exactly what we
// mean: calculate the area of Rectangle using its width and height fields. this conveys that the
// width and height are related to each other, and it gives descriptive names to the values rather
// than using the tuple index values of 0 and 1 . this is a win for clarity.
//
//
// ADDING USEFUL FUNCTIONALITY WITH DERIVED TRAITS
//
// its be useful to be able to print an instance of Rectangle while were debugging our program and
// see the values for all its fields. 
//
// struct Rectangle {
//      width: u32,
//      height: u32,
//  }
//
//  fn main() {
//      let rect1 = Rectangle {
//          width: 30,
//          height: 50,
//      };
//      println!("rect1 is {rect1}");
//  }
//
//  when this code compiles we get an eror saying Rectangle doesnt implement std::fmt::Display
//
//  the println! macro can do many kinds of formatting, and by default the curly brackets tell
//  println! to use formatting known as display: output intended for direct end user consumption.
//  this primitive types weve seen so far implement Display by default because theres only one way
//  youd wan tto show a 1 or any other primitive type to a user. but the structs, the way println!
//  should format the output is less clear because there are more display probabilities: do you wan
//  tcommans or not? do you wan tto print the curly bracket? shold all field be shown? due to this
//  ambiguity, Rust doesnt try to guess what we want, and structs dont have provided implementation
//  of Display to use the println! and the {} placeholder
//
//  using the ? inside --> {rect1:?} this is known as pretty print to debug and a useful way devs
//  can see its value while were debugging 
//
//  however when we compile we come across this errors trait 'debug' is not implemented for
//  'rectangle' -- add '#[derive(Debug)]' to 'rectangle'
//
//  Rust does include functionality to print out debugging information but we have to explicitly
//  opt in to make that functionalty available for our struct. to do that, we add the outer
//  atttribute #[derive(debug)] just before the struct definition
//
// when we run the program we dont get erros and see the code saay 'rect1 is Rectangle {width: 30,
// height: 50}
//
// Nice! its not the prettiest output but it shows the values of all the fields for this instance,
// which would definitly help during debugging. when we have larger structs, its useful to have
// output thats a bit easier to read; in those cases we can use {:#?} instead of {:?} in the
// println! string.
//
// another way to print out a value using the Debug format is to use the dbg! macro, whichs takes
// ownership of an expression (as opposed to println!, which takes a reference), prints the file
// and line number of where that dbg! macro call occurs in your code along with the resultant value
// of that expression, and returns ownership of that value
//
// Note: calling dbg! macro prints to the standard error console stream (stderr) as opposed to the
// println! , which prints to the standard output console stream (stdout)
//
// we can see that first bit of output came from line 10 where we are debugging the expression 30
// *scale , and its resultant value is 60. the dbg! call outputs the value of &rect1, which is the
// REctangle struct. this output uses the pretty debug formatting of the Rectangle type. the dbg!
// macro can be really helpful when your trying to digure out what your code is doing.
//
// in addition to the Debug trait, Rust has provided a number of traits for us to use with the
// derive attribute that can add useful behavior to our custom types. those traits and their
// behaviors are listed in appendix c. 
//
// Our area function is very specific it only computes the area of rectangles. it would be helpful
// to tie this behavior more closely to our recntalge struct because it wont work wiht any other
// type. lets look at how we can continue to refactor this code by turning the area into an area
// metod defined on our rectangle type.
//
// 
// METHOD SYNTAX
//
// methods are similar to functions: we declare them with the fn keyword and a name, they can have
// parameters and return a value, and they can contain some code thats run when the method is
// called from somewhere else. unlike functions, methods are defined within a context of a struct,
// and their first parameter is always self, which represents the instance of thre struct the
// method is being called on
//
//
// DEFINING METHODS 
//
// lets change the area function that has a Rectangle instance as a parameter instead make an area
// method defined on the Rectangle struct
//
//

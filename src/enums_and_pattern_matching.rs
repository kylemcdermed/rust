// ENUMS AND PATTERN MATCHING 
//
// DEFINING AN ENUM 
//
// where structs give you a way of grouping together related fields and data, like a Rectangle with
// its width and height, enums give you a way of saying a value is one of a possible set of values.
// for example, we may want to say that Rectangle is one of a set of possible shapes that also
// includes Circle and Triangle. To do this, Rust allows us to encode these possbilities as enum.
//
// Say we are working with IP addresses. Currently, the two major standards for IP addresses are :
// IPV4, IPV6.
//
// Because there are only two possibilities for an IP address that our program will come across, we
// can enumerate all possible variants, which is where enumeration gets its name. 
//
// Because IP addresses can only be one of the variants (IPV4, IPV6), enums data structure are best
// appropriate for this. 
//
// Both IPV4 and IPV6 should be treated as the same type when code is handling situations that
// apply to any kind of IP addresses.
//
// We can express this concept in code by defining an IpAddrKind enumeration and listing the
// possible kinds of IP address can be V4 and V6:
//
// enum IpAddrKind {
//      V4,
//      V6,
// };
//
// IpAddrKin is now a custom data type that we can use else where in our code 
//
//
// ENUM VALUES 
//
// We can create instances of each of the two variants of IpAddrKind like this:
// let four = IpAddrKind::V4;
// let six = IpAddrKing::V6;
//
// note that the variants of enum are namespaced under its identifier, and we use a double colon to
// separate the two. This is useful because no both values ::V4 and ::V6 are of the same type:
// IpAddrKind. We can then define a function that takes any IpAddrKind:
//
// fn route(ip_kind: IpAddrKind) {}
//
// we can also call this function with either variant:
//
// route(IpAddrKind::V4);
// route(IpAddrKind::V6);
//
// using enums has even more advantages. Thinking more about our IP address type, at the moment we
// dont have a way to store the actual IP address data; we only know what kind it is...
//
// here we defined a struct IpAddr that has two fields: a kind field that is of type IpAddrKind and
// an address field of type String. We have two instances of this struct. The first is home, and it
// has the value IpAddrKind::V4 as its kind with associated address of 127.0.0.1 and the second
// instance is loopback. It has the other variant of IpAddrKind as its kind value, V6, and has
// address ::1 associated with it. Weve used a struct to bundle the kind and address values
// together, so now the variant is associated with the value.
//
// However, representing the same concept using just an enum is more concise: rather than an enum
// inside a struct, we can put data directly into each enum variant. This new definition of the
// IpAddr enum says that both V$ and V6 variants will have an associated String values:
//
// we attach data to each variant of the enum directly, so there is no need for an extra struct.
// here, it is also easier to see another detail of how enums work: the name of each enum variant
// that we define also becomes a function that constructs an instance of the enum. That is,
// IpAddr::V4() is a function call that takes a string argument and returns an instance of the
// IpAddr type. We automatically get this consutrctor function defined as a result of defining the
// enum.
//
// Theres another advantage to using an enum rather than a struct: each variant can have different
// types and amount of associated data. Version four IP addresses will always have four numeric
// components that will have values between 0 and 255. if we wanted to store V4 addresses as four
// u8 values but still express V6 addresses as one String value, we wouldnt be able to with a
// struct. Enums handle this case with ease:
//
// enum IpAddr {
//      V4(u8, u8, u8, u8),
//      V6(String),
// }
//
// let home = IpAddr:V4(127, 0, 0, 1);
// let loopback = IpAddr::V6(String::from("::1");
//
// As it turns out, wanting to store IP addresses and encode which kind they are is so common that
// the STL has a definition we can use. How the STL defins IpAddr: it has the exact enum variants
// that weve defined and used but it embeds the address data inside the variants in the form of two
// different structs, which are defined differently for each variant:
//
// the code illustrates that you can put any kind of data inside an enum variant: strings, numeric
// types, or structs, for exmaple. you can even include another enum! also with STL types are often
// not much more complicated than what you might come up with.
//
// not that even though the STL contains a definition for IpAddr, we can still create and use our
// own definition without confliect because we havent brought the STL definition into our scope. 
//
// lets look at another example of enum, this has wide varity of types embedded in its variants.
//
// enum Message {
//      Quit,
//      Move {x: i32, y: i32},
//      Write(String),
//      ChangeColor(i32, i32, i32),
// }
//
// this enum has four variants with different types:
// Quit - has no data associated with it at all
// Move - has named fields like a struct does 
// Write - includes a single String 
// ChangeColor - includes three i32 values
//
// defining an enum with variants like we have above is similar to defining deifferent kinds of
// struct definitions, except the enum doesnt use the struct keyword and all the variant are
// grouped together under the Message type. The following structs could hold the same data that the
// preceding enum variants hold:
//
// struct QuiteMessage; // unit struct 
// struct MoveMessage {
//      x: i32,
//      y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct 
//
// but if we used the different structs, each of which has its own type, we couldnt easily define a
// function to take any of these kinds of messages as we could with the Message enum defined.
//
// there is no more similarity between enums and structs: just as were able to define methods on
// structs using impl, were also able to define methods on enums. heres a method named call that we
// could define on our Message enum:
//
// impl Message {
//      fn call(&self) {
//          // method body would be defined here
//      }
// }
//
// let m = Message::Write(String::from("hello"));
// m.call();
//
// the body of the method would use self to get the value that we called the method on. in this
// example, weve created a variable m that has the value Message::Write(String::from("hello")), and
// that is what self will be in the body of the call method when m.call() runs.
//
//
// THE OPTION ENUM AND ITS ADVANTAGES OVER NULL VALUES 
//
// this section explores a case study of Option, which is another enum defined by the standard
// library. The Option type encodes the very common scenario in which a value could be something or
// it could be nothing.
//
// for example, if you request the first item in a non empty list, you would get a value. if you
// request the first item in an empty you would get nothing.expressing this concept in terms of the
// type system means the compiler can check whetehr youve handled all the cases you should be
// handling; this functionality can prevent bugs that are extremely common in other programming
// languages. 
//
// Rust doesnt have the null feature that others do. Null is a value that means there is no value
// there, in languages with null, variables can always be in one of two states, null or non null.
//
// the problem with null values si that if you try to use a null value as a not null value, youll
// get an error of some kind. because this null or not null propert is pervasive, its extremely
// easy to make this kind of error.
//
// however, the concept that null is trying to express is still a sueful one, a null is a value
// that is currently invalid or absent for some reason
//
// the prolem isnt really with the concept but with the particular implementation. as such, Rust
// DOES NOT HAVE NULLS, but it does have an enum that can encode the concept of a value being
// present or absent. This enum is Option<T>, defined as follows:
//
// enum Option<T> {
//      None,
//      Some(T),
// }
//
// the Option<T> enum is so useful that its even included in the prelude; you dont need to bring it
// into scope explicily. its variants are also included in teh prelude: you can use Some and None
// directly without the Option:: prefix. the Option<T> enum is still just a regular enum, and
// Some(T) and None are still variants of type Option<T>.
//
// the <T> syntax is a feature of Rust we havent talked about, its a generic type parameter. For
// now, what you need to know is that <T> means that Some variant of the Option enum can hold one
// piece of data of any type, and that each concrete type that gets used in place of T makes the
// overall Option<T> type a different type. here are some examples of using Option values to hold
// number types and char types:
//
// let some_number=  Some(5);
// let some_char = Some('e');
// let absent_number: Option<i32> = None;
//
// the type of some_number is Option<i32>. the type of some_char is Option<char>, which is a
// different type. Rust can infer these types because weve specified a value inside the Some
// variant. For absent_number, Rust requires us to annotate the overall Option type: the compiler
// cant infer the type that corresponding Some variant will hold by looking only at a None value.
// Here, we tell Rust we mean for absent_number to be of type Option<i32>.
//
// when we have Some value, we know that a value is present and the value is held within the Some.
// When we have a None value, in some sens eit means the same thing as null: we dont have a valid
// value. So why having Option<T> any better than having null?
//
// in short, because Option<T> and T, where T can be any type, are different types, the compiler
// wont let us use an Option<T> value as if it were definitely a valid value. for example, this
// code wont compile because its trying to add an i8 to an Option<i8>:
//
// let x: i8 = 5;
// let y: Option<i8> = Some(5);
// leet sum = x + y;
//
// you get an error! intense!
//
// this error message means that Rust does not understand how to add an i8 and an Option<i8>
// because they are different types. the compiler ensures that we always have a valid value so we
// can confidently proceed without having the possibility of the value being of type null or None.
//
// in order words, you have to convert an Option<T> to a T before you can perform T operations
// with it. generally, this helps catch one of the most common issues with null: assuming that
// something isnt null when it actually is.
//
// eliminating the risk of incorrectly assuming a not null value helps you to be more confident in
// your code. in order to have a value that can possily be null, you must explicitly opt in by
// making the type of that value Option<T>. then, when you use that value, you are required to
// explicitly handle the case when the value is null. everywhere thata  value has a type that isnt
// an Option<T>, you can safely assume that the value isnt null. this was deliberate design
// decision for Rust to limit null's pervasiveness and increase the safety of Rust code.
//
// So how do you get the T value out of Some variant when you have  avalue of type Option<T> so
// that you can use that value? the Option<T> enum has a larger number of methods that are useful
// in a variety of situations;
//
// in general, in order to use an Option<T> value, you want to have code that will handle each
// variant. You want some code that will run only when you have Some(T) value, and this code is
// allowed to use the inner T. you want some other ocde to run only if you have a None value, and
// that code doesnt have a T value avilable. The match expression is a control flow construct that
// does just this when used with enums: it will run different code depending on which variant of
// the enum it has, and that code can use the data inside of the matching value.
//
//
// THE MATCH CONTROL FLOW CONSTRUCT 
//
// Rust has na extremely powerful control flow construct called match that allows you to compare a
// value against a series of patterns and then execute code based on which pattern matches.
// patterns can be made up of literal values, variable names, wildcards, and other things; the
// power of match comes from the expressiveness of the patterns and the fact that the compiler
// confirms that all possible cases are handled.
//
// think of a match expression as being like a coin sorting machine: coins slide down a track with
// variously sized holes along the way, and each coin falls through the first hole it encounters
// that it fits into. in the same way, values go through each pattern in a match, and at the first
// pattern the value fits, the value falls into the associated code block to be used during
// execution
//
// lets use this coin expression as an example using match. writing a function that takes an
// unknown us coin and determines which coin it is and returns the value in cents
//
// lets break down the match in value_in_cents function. first we list the match keyword followed
// by an expression, which in this case is the value coin. this is like an if statement but a big
// different with if statement is the condition needs to evaluate to a Boolean value, but here it
// can be any type. The type of coin in this example is the Coin enum that we defined on the first
// line 
//
// next are the match arms. an arm has two parts: a pattern and some code. the first arm here has a
// pattern that is the value of Coin::Penny and then the => operator that separates the pattern and
// the code to run. the code in this case is just the value 1. each arm is separated from the next
// with a comma.
//
// when the match expression executes, it compares the resultant value against the pattern of each
// arm, in order. if a pattern matches the value, the code associated with that pattern is
// executed. if that pattern doesnt match the value, execution continues to the next arm, much as
// in a coin sort machine. we can have as many arms as we need.
//
// the code associated with each arm is an expression, and the resultant value of the expression in
// the matching arm is the value that gets returned for the entire match expression
//
// we typically dont use curly brackets if the match arm code is short, if you want to run multiple
// lines of code in a match arm, you must use curly brackets, and the comma following the arm is
// then optional. for example, the following code prints "lucky penny" everytime the method is
// called with a Coin::Penny, but still returns the last value of the block, 1:
//
//
//
//
//
//
//
//
//

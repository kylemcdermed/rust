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
// PATTERNS THAT BIND TO VALUES 
//
// another useful feature of match arms is that they can bind to the parts ot eh values that match
// the pattern. this is how we can extract out enum variants.
//
// so for example, lets change one of our enum variants to hold data inside of it from 1999 to
// 2008, the usa minted quarters with different designs for each of the 50 states on one side. no
// other coins got state designs, so only quarters have this extra value. we can add this
// information to our enum by changing the Quarter variant to include  UsState value stored inside
// it
//
// lets imagine someone is collecting all 50 quarters. while we sort our loose change by coin type,
// well also call out the name of the state associated with each quarter so that if one of our
// friend doesnt have they can add it to their collection
//
// in the match expression for this code, we add variable called state to the pattern that matches
// values of the variant Coin::Quarter. when a Coin::Quart matches, the state variable will bind to
// the value of that quarters state. then we can use state in the code for that arm like so:
//
// if we were to call value_in_cents(Coin::Quarter(UsState::Alaska)), coin would be
// Coin::Quarter(UsState::Alaska). when we compare value with each of the match arms, none of them
// match until we each Coin::Quarter(state). at that point the binding for state will be the value
// UsState::Alaska. we can then use that binding in the println! expresion, thus getting the inner
// state value out of the Coin enum variant for Quarter
//
//
// MATCHING WITH OPTION<T>
//
// in the previous section we wanted to get the inner T value out of the Some case when using
// Option<T>; we can also handle Option<T> using match, as we did with the Coin enum! instead of
// comparing coins, well compare the variants of Option<T>, but the way the match expression works
// remains the same.
//
// lets say we want to write a function that takes an Option<i32> and, if theres a value inside,
// adds 1 to the value. if there isnt a value inside, the function should return None value and not
// attempt to perform any operations
//
// this function is very easy to write thanks to match
//
// lets examine the first execution of plus_one in more detail. when we call plus_one(five), the
// variable x in the body of plus_one will have the value Some(5). the variable x in the body of
// plus_one will have the value Some(5). we then compare that against each match arm: None => None,
//
// the Some(5) value doesnt match the pattern None, so we continue to the next arm: Some(5) =>
// Some(i+1),
//
// does some(5) match Some(i)? it does! we have the same variant, the i binds to the value containe
// in Some, so i takes the value 5. the code in the match arm is then executed, so we add 1 to the
// value of i and create Some value with our total 6 inside.
//
// Now lets consider the second call of plus_one where x is None. we enter the match and compre to
// the first arm: None => None,
//
// it matches! theres no value to add to, so the program stops and returns the None value on the
// right side of =>, nbecause the first arm is matched, no other arms are compared.
//
// combining match and enum is usefil in many situations. youll see this pattern alot in Rust code:
// match against an enum, bind a variable to the data inside, and then execute code basedd on it.
// its a bit tricky at first, but once you get used to it, youll wish you had it in all languages.
// its consistnently a user favorite
//
//
// MATCHES ARE EXHUASTIVE 
//
// theres one other aspect of match we need to discuss: the arms pattern must cover all
// possibilities. consider this version of our plus_one function, which has a bug and wont compile:
//
// Rust knows that we didnt cover every possible case, and even knows which pattern we forgot!
// Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to
// be valid. especially in the case of Option<T>, when Rust prevents us from forgetting to
// explicitly handle the None case, it protects us from assuming that we have a valud when we might
// have null, ths making the billion dollar mistake discussed earlier impossible.
//
//
//
// CATCH ALL PATTERNS AND THE _ PLACEHOLDER
//
// using enums we can also take special actions for a few particular values, but for all other
// values take one default action, imagine were implementing a game where, if you roll a 3 on a
// dice roll, your player doesnt move, but instead gets a new fancy hat. if you roll a 7, your
// player loses a fancy hat. for all other values, your player moves that number of spaces on the
// game board. heres a match that implements that logic, with the result of the dice roll hardcoded
// rather than a random value, and all other logic represented by functions without bodies because
// actually implementing them is out of scope for this example:
//
// for the firs two arms, the pattern are the literal values 3 and 7. for the last arm that covers
// every other possible value, the pattern is the variable weve chosen to name other. the code that
// runs for the other arm uses the variable by passing it to the move_player function.
//
// this code compiles, even though we havent listed all possible values a u8 can have, because the
// last pattern will match all values not specifically listed. this catch all pattern meets the
// requirement that match must be exhaustive. note that we have to put the catch all arm last
// because the patterns are evaluated in order. if we put the catch all arm earlier, the other arms
// would never run, sp Rust will warn us if we add arms after a catch all!
//
// Rust also has a pattern we can use when we want a catch all but dont want to use the value in
// the catch all pattern: `_` is a special pattern that matches any value and does not bind to that
// value. this tells Rust we arent going to use the value. so Rust wont warn up about an unused
// variable.
//
// lets change the rules of the game now, if you roll anything other than a 3 or 7, you must roll
// again, we no longer need to use a catch all value so we can change our code to use _ instead of
// the variable named other:
//
// this exmaple also meets the exhaustiveness requirement because were explicitly ignoring all
// other values in the last arm; we havent forgotten anything.
//
// finally well change the rules of the game on emore time so that nothing else happens on your
// turn if you roll anything other than a 3 or a 7. we cna epxress that by using the unit value
// (the empty tuple type we mentioned in The Tuple type section) as code that goes with the _ arm:
//
// Here were telling rust explicitly that we arent going to use any other value that doesnt match
// the pattern in an earlier arm, and we dont want to run any code in this case.
//
//
// 
// CONCISE CONTROL FLOW WITH IF LET AND LET ELSE 
//
// the if let syntax lets you combine if and let into a less verbose way to handle values that
// match one pattern while ignoring the rest. consider the program that matches on an Option<u8>
// value in the config_max variable but only wants to execute code if the value is the Some
// variant.
//
// if the value is Some, we print out the value in the Some variant by binding the value to the
// variable max in the pattern. We dont want to do anything with the None value. to satisfy the
// matchc expression we have to add _ => () after processing just one variant, which is annoying
// boilerplate code to add
//
// instead we could write this in a short way using if let. the code behaves the same as the match
//
// the syntax if let takes a pattern and an expression separated by an equal sign. it works the
// same way as a match where the expression is given to the match and the pattern is its first arm.
// in this case, the pattern is Some(max) and the max binds to the value inside the some. we can
// use max in th body of the if let block in the same way we used max in the corresponding match
// arm. the code in the if let block only runs if the value matches the pattern.
//
// using if let means less typing, less indentation, and less boilerplate code. however you lose
// the exhaustive checking match enforces that ensures you arent foregetting to handle any cases.
// choosing between match and if let depends on what your doing in your particular situaion and
// whether gaining conciseness is an appropriate trade off for losing exhaustive checking 
//
// in other words you can think of if let as syntax sugar for a match that runs code when the value
// matchres one pattern and then ignores all other values 
//
// we cna include an else with an if let. the block of code that goes with the else is the same as
// the block of code that would go with the _ case in the match expression that is equivalent to
// the if let and else. recall the Coin enum definition where the quarter variant also held a
// UsState value. if we wanted to count all non quarter coins we see while also announcing the
// state of the quarters, we could do that with a match expression
//
// or we could use an if let and else expression 
//
//
// 
// STAYING ON THE HAPPY PATH WITH LET...ELSE
//
// the common pattern is to perform some computation when a value is present and return a default
// value otherwise. continuing on with our example of coins with a UsState value, if we wanted to
// say something funny depending on how old the state on the quarter was we might introduce a
// method on UsState to check the age of a state like so:
//
//
//

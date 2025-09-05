// REFERENCES AND BORROWING 
//
// the issue with the tuple code is that we have to return the String to the calling function so we
// can still use the String after the call to calculate_length, because the String was moved into
// calculate_length. instead we can provide a reference to the String value. a reference is like a
// pointer in that its an address we can follow to access the data stored at that address; that
// data is owned by some other variable. unlike a pointer, a reference is guaranteed to point to a
// valid value of some particular type for the life of that reference 
//
// here is how you would define and use calculate_length function that has a reference to an object
// as a parameter instead of taking ownership of a value:
//
// first, notice that all tuple code in the variable declaration and the function return value is
// gone. 
// second, note that we pass &s1 into calculate_length and in its definition, we take &String
// rather than String. these ampersands represent references, and they allow you to refer to some
// value without taking ownership of it 
//
// note: the opposite of referencing by using `&` is defreferencing, which accomplished with the
// defreference operator `*`
//
// lets take a close look at the function call here:
//
// let s1 = String::from("hello");
// let len = calculate_length(&s1);
//
// the &s1 syntax lets us create a reference that refers to the value of s1 but does not own it.
// because the reference does not own it, the value it points to will not e dropped when the
// reference stops being used
//
// likewise, the signature of the function uses & to indicate that the type of the parameter s is a
// reference. lets add some explanatory annotations:
//
// fn calculate_length(s: &String) -> usize { // s is a reference to a String
//      s.len()
// } // here, s goes out of scope, but because s does not have ownership of what
// // it refers to, the String is not dropped
//
// the scope in which the variable s is valud is the same as any function parameters scope, but the
// value pointed to by the reference is not dropped when s stops being used, because s doesnt have
// ownership. when functions have references as parameters instead of the actual values, we wont
// need to return the values in order to give back ownership, because we never had ownership
//
// we call the action of creating a reference borrowing. as in real life, if a person borrows
// somethings, they are borrowing it from someone who owns something. when your done, you give it
// back, you dont own it.
//
// what happens if we modify something were borrowing? spoiler alert: it doesnt work!
//
// just as variables are immutable by default, so are references. were not allowed to modify
// something we have a reference to
//
//
// MUTABLE REFERENCES
//
// we can fix the code to allow us to modify a borrowed value with just a few small tweaks that
// instead a mutable reference:
//
// first we change s to be mut. then we create a mutable reference with &mut s where we call the
// change function, and update the function signature to accept a mutable reference with
// some_string: &mut String. this makes it very clear that the change function will mutate the
// value it borrows
//
// mutable references have one big restriction: if you have a mutable reference to a value, you can
// have no other references to that value. this code attempts to create two mutable reference to s
// will fail:
//
// this error says that this code is invalid because we cannot borrow s as mutable more than once
// at a time. the first mutable borrow is in r1 and must last until its used in println!, but
// between the creation of that mutable reference and its usage, we tried to create another mutable
// reference in r2 that borrows the same data as r1
//
// the restriction preventing multiple mutable references to the same data at the same time allows
// for mutation but in a very controlled fashoin. its something that new Rustaceans struggle with
// because most languages let you mutate whenever youd like. the benefit of having this restriction
// is that Rust can prevent data races at compile time. a data race is similar to a race condition
// and happens when these three behaviors occur:
//
// - two or more pointers access the same data at the same time 
// - at least one of the two pointers is being used to write to the data
// - theres no mechanism being used to synchronize access to the data 
//
// data races cause undefined behavior and can be difficult to diagnose and fix when youre trying
// to track them down at runtimel Rust prevents this problem by refusing to compile code with data
// races!
//
// we can use curly brackets to create a new scope, allowing for multiple references, just not
// simultaneous ones 
//
// let mut s = String::from("hello");
// {
//      let r1 = &mut s;
// } // r1 goes out of scope here, so we can make a new reference with no problems
// let r2 = &mut s;
//
// Rust enforces a similar rule for combining mutable and immutable references. this code results
// in an error:
//
// let mut s = String::from("hello");
// let r1 = &s // no problem
// let r2 = &s; // no problem
// let r3 = &mut s; // BIG PROBLEM
// println!("{r1}, {r2}, {r3}");
//
// we also cannot have mutable reference while we have an immutable one to the same value 
//
// users of an immutable reference dont expect the value to suddenly change out from under them!
// however, multiple immutable references are allowed because no one who is just reading the data
// has the ability to affect anyone elses reading of the data
//
// note that a references scope starts from where it is introduced and continues through the last
// time that reference is used. for instance, this code will compile because the last usage of the
// immutable references is in println!, because the mutable reference is introduced:
//
// let mut s = String::from("hello");
// let r1 = &s // no problem
// let r2 = &s; // no problem
// println!("{r1}, {r2}, {r3}"); // variables r1 and r2 will not be used after this point
//
// let r3 = &mut s; // no problem
// println!("{r3}");
//
// the scopes of the immutable references r1 and r2 end after the println! where they are last
// used, which is before the mutable reference r3 is created. these scopes dont overlap, so this
// code is allowed: the compiler can tell that the reference is no longer being used at a point
// before the end of the scope.
//
// even though borrowing errors may be frustrating at times, remember that its the Rust compiler
// pointing out a potential bug early (at compile time rather than at runtime) and showing you
// exactly where the problem is. then you dont have to track down why your data isnt what you
// thought it was.
//
//
// DANGLING REFERENCES 
//
// in languages with pointers, its easy to erroneously create a dangling pointer -- a pointer that
// references a location in memory that may have been given to someone else -- by freeing some
// memory while preserving a pointer to that memory. in Rust, by contrast, the compiler guarantees
// that references will never be dangling references: if you have a reference to some data, the
// compiler will ensure that the data will not go out of scope before the reference to the data
// does.
//
// lets create a dangling reference to see how rust prevents them with some compile-time error:
//
// this error message refers to a feature we havent covered yet: lifetimes. well discuss lifetimes
// later. but if you disregard the parts about lifetimes, the message does contain the key to why
// this code is a problem.
//
// 'this functions return type contains a borrows value, but there is no value for it to be
// borrowed from'
//
// lets take a closer look at exactly what is happening at each stage of our dangle code:
//
// fn dangle() -> &String { // dangle returns a reference to a String
//      let s = String::from("hello"); // s is a new String
//      &s // we a reference to the String, s 
// } // here s goes out of scope, and is dropped., so its memory goes away, danger!
//
// because s is created inside dangle, when the code of dangle is finished, s will be deallocated.
// but we tried to return a reference to it. that means this reference would be pointing to an
// invalid String. thats no good! Rust wont let us do this.
//
// the solution is to return the string by value directly:
//
// fn no_dangle() -> String {
//      let s = String::from("hello");
//      s
// }
//
// this works without any problems, ownership is moved out, and nothing is deallocated.
//
//
// THE RULES OF REFERENCES 
//
// lets recap about references:
// - at any given time, you can either have on mutable reference or any number of immutable
// references
// - references must always be valid
//
// now lets take a look at a different kid of references, slices.
// 

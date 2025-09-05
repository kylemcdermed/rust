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
//

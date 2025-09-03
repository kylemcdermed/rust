// UNDERSTANDING OWNERSHIP
//
// onwership is Rusts most unique feature and has deep implications for the res of the language. It
// enables Rust to make memory safety guarantees without needing a garbage collector, so its
// important to understand how ownership works
//
//
// WHAT IS OWNERSHIP?
//
// ownership is a set of rules that govern how a Rust program manages memory. all programs have to
// manage the way they use a computers memory while running. some languages have a garbage
// collections tha tregularly looks for no longer used memory as the program runs; other langugaes
// the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory
// is managed through a system of ownership with a set of rules that the compiler checks. if any
// rules are violated, the program wont compile. None of the features ow ownership will shlow down
// your program while its running
//
// 
// STACK AND THE HEAP
//
// Whether a value is on the stack or the heap affects how the language behaves and why you have to
// make certain decisions. parts of ownership will be described in relation to the stack and the
// heap later in the chapter
//
// both the stack and the heap are parts of memory available to your code to use at runtime, they
// are structured in different ways. the stack stores values in order it get them and removes the
// values in the opposite order. this is referred to as last in, first out. adding data is called
// pushing onto the stack and removing data is called popping off the stack. all data stored on the
// stack must have a known fixed size. data with an unknown size at compiler time or a size that
// might change must be stored on the heap instead.
//
// the heap is less organized: when you put data on the heap, you required a certain amount of
// space. the memory allocator finds an empty spot in the heap that is big enough, marks it as
// being in use, and returns a pointer, which is the address of that location. this process is
// called allocating on the heap and is sometimes abbreviated as just allocating. because the
// pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you
// want the actual data, you must follow the pointer. 
//
// pushing to the stack is faster thanallocating on the heap because the allocator never has to search for a place to store new data; that location is always on top of the stack. comparatively, allocating space on the heap requires more work because the allocator must first find a big enough space to hold th// the dat and ten perform bookkeeping to prepare for the next allocation.
//
// accessing data in the heap is generally slower than aceessing data on the stack because you have
// to follow a pointer to get there. contemporary processors are faster if they jump around less in
// memory.
//
// when your code calls a function, the values passed into the function (including, potentially,
// pointers to data on the heap) and functions local variables get pushed onto the stack. when the
// function is over, those values get popped off the stack
//
// keeping track of what parts of code are using what data on the heap, minimizing the amount of
// duplicate data on the heap, and cleaning up unuseed data on the heap so you dont run out space
// are all problems that ownership addresses. once you understand ownership you wont need to think
// about stack and heap very often, but knowing the main purpose of ownership is to manage heap
// data can help explain why it works the way it does
//
//
// OWNERSHIP RULES 
//
// lets look at the ownership rules. keep these rules in mind as we work through the examples and
// illustrate them:
//
// - each value in Rust has an owner 
// - there can only be one owner at a time 
// - when the owner goes out of scope, the value will be dropped 
//
//
// VARIABLE SCOPE 
//
// now that we past basic Rust syntax, we wont include the fn main(){} code in exmaples the code
// will be much more percise and concise.
//
// as a first example of ownership, well look at the scope of some variables. a scope is the range
// within a program for which an item is valid
//
// take the following variable:
//
// let s = "hello";
//
// the variable s refers to a string literal, where the value of the string is hard coded into the
// text of our program. the variable is value from the point at which its declared until the end of
// the current scope. 
//
// in other words, there are two important points in time here:
//
// - when s comes into scope, it is valid 
// - it remains valud until it goes out of scope 
//
// at this point, the relationship between scopes and when variables are valid is similar to that
// in other languages. now well build on top of this understanding by introducing the `String` type 
//
//
// THE STRING TYPE 
//
// to illustrate the rules of ownership, we need a data type that is more complex than those
// covered in data types. the types covered previously are of a known size, can be stored on the
// stack and popped off the stack when their scope is over, and can be quickly and trivially copied
// to make a new, independent instance if another part of code needs to use the value in a
// different scope. but we want to look at data that is stored on the heap, and explore how Rust
// knows when to clean up that data, and the String type is a great example.
//
// weve already seen string literals, where a string value is hard coded into our program. string
// literals are convenient, but they arent suitable for every situation in which we may want to use
// text. one reason is that their immutable. another is that not every string value can be known
// when we write our code: for example, what if we want to take user input and store? for these
// situations, Rust has a second string type, String. this type manages data allocated on the heap
// and as such is able tostore an amount of text that is unknown to us at compile time. 
//
// you can create a String from a string literal using the from function:
//
// let s = String::from("hello");
//
// the double colon :: operator allows us to namespace this particular `from` function under the
// `String` type rather than using some sort of name like `string_from`
//
// this kind of string can be mutated:
//
// let mut s = String::from("hello");
// s.push_str(", world!"); // push_str() appends a literal to a String 
// println!("{s}"); // this will print "hello, world!"
//
// so, what the difference here? why can `String` be mutated but literals cannot? the difference is
// in how these two types deal with memory
//
// 
// MEMORY AND ALLOCATION
//
// in the case of a string literal, we know the contents at compile time, so the text is hard coded
// directly into the final executable, this is why string literals are fast and efficient. but
// these properties only come from the string literals immutability. unfornately we cant put a blob
// of memory into the binary for each piece of text whose size is unknown at compile time and whose
// size might change while running the program
//
// with the string type, in order to support the mutable, growing piece of text, we need to
// allocate an amount of memory on the heap, unknown at compile time, to hold the contents, this
// means:
// - the memory must be requested from the memory allocator at runtime 
// - we need a way of returning this memory to the allocator when we are done with our string
//
// that first part is done by us, when we call String::from, its implementation requests the memory
// it needs.
//
// however, the second part is different. in languages with a garbage collector, the GC keeps track
// of and cleans up memory that isnt being used anymore, and we dont need to think about it. in
// most languages without a garbage collector its our responsibility to identify when memory is
// longer being used and to call code explicitly free it, just as did to request it. doing this
// correctly has historically been a difficult programming problem. if we forget, well waste
// memory. if we do it too early, well have an invalid variable. if we do it twice, thats a bug
// too. we need to pair exactly one allocate with exactly one free
//
// Rust takes a different path, the memory is automatically returned once the variable that owns it
// goes out of scope, here is an example using a string instead of a string literal:
//
// {
//      let s = String::from("hello"); // s is valid from this point forward
//      // do stuff with s 
// }                                  // this scope is now over, annd s is no longer valid
//
// there is a natural point at which we can return the memory our String needs to the allocator:
// when s goes out of scope. when a variable goes out of scope, Rust calls a special function for
// us, this function is called drop, and its where the author of String can put the code to reutrn
// the memory. Rust calls drops automatically at the closing curly bracket
//
// this pattern has a profound impact on the way Rust code is written. it may seem simple right
// now, but the behavior of code can be unexpected in more complicated situations when we want to
// have multiple variables use data weve allocated on the heap. 
//
//
// VARIABLES AND DATA INTERACTING WITH MOVE 
//
// multiple variables can interact with the same data in different ways in Rust
// for example:
//
// let x = 5;
// let y = x;
//
// you can probably guess what this is doing: "bind the value 5 to x; then make a copy of the value
// in x and bind it to y". we now have two variables, x and y, and both equal 5. this is indeed
// what is happening, because integers are simple values with a known fixed size and these two 5
// values are pushed onto the stack
//
// lets look at the string version:
//
// let s1 = String::from("hello");
// let s2 = s1;
//
// this looks very similar so we might assume that the way it works would be the same. that is, the
// second line, would make a copy of the value in s1 and bind it to s2. but this isnt quite what
// happens
//
// the figure on screen shows what is happening. a String is made up of three parts, shown on the
// left: a pointer to the memory that holds the contents of the string, a length, and a capacity.
// this group of data is stored on the stack. on the right is the memory on the heap that holds te
// contents.
//
// the length is how much memory, in bytes, the contents of the String are currently using. the
// capacity is the total amount of memory, in bytes, that the String has recieved from the
// allocator. The difference between length and capacity matters, but not in this context, so for
// now, ignore the capacity.
//
// when we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and
// capcity that are on the stack. we do not copy the data on the heap that pointer refers to.
//
// earlier we said that when a variable goes out of scope, Rust automatically calls the drops
// function and cleans up the the heap memory for that variable. figure 4-2 shows both data
// pointers pointing to the same location. this is a problem: when s2 and s1 go out of scope, they
// will both try to free the same memory location. this is known as a double free error and is one
// the memory safety bugs we mentioned previously. freeing memory twice can lead to memory
// corruption, which can potentially lead to security vulnerabilities.
//
// the ensure memory safety, after the line: let s2 = s1; Rust considers s1 no longer valid.
// Therefore, Rust does not need to free anything when s1 goes out of scope. Check out what happens
// when you try to use s1 after s2 is created; it wont work...
//
// if youve heard the terms shallow copy and deep copy while working in other languages, the
// concept of copying the pointer, length, and capacity without copying the data probably sounds
// like making a shallow copy. but because Rust also invalidates the first variable, instead of
// being called a shallow copy, this is known as a move. in this example, we would say that s1 was
// moved into s2.
//
// that solves our problem! with only s2 valid, when it goes oout of scope it alone will free the
// memory, and were done.
//
// in addition, theres a design choice that implies: Rust will never automatically create "deep"
// copies of your data. therefore, any automatic copying can be assumed to be inexpensive in terms
// of runtime performance
//
//
// SCOPE AND ASSIGNMENT 
//
// the inverse of this is true for the relationship between scoping, ownership and memory being
// freed via the drop function as well. when you assign a completely new value to an existing
// variable, Rust will call `drop` and free the original values memory immediately. consider this
// code:
//
// let mut s = String::from("hello");
// s = String::from("ahoy");
// println!("{s}, world!");
//
// we initially declare a variable s and bind it to a String with the value "hello". then we
// immediately create a new String with the value "ahoy" and assign it to s. at this point, nothing
// is referring to the original value on the heap at all.
//
// the original string thus immediately goes out of scope. Rust will run the drop function on it
// and its memory will be freed right away. when we print the value at the end, it will be "ahoy,
// world!"
//
//
// VARIABLES ARE DATA INTERACTING WITH CLONE 
//
// if we do want to deeply copy the heap data of the String, not just the stack data, we can use a
// common  method called clone. here is an example:
//
// let s1 = String::from("hello");
// let s2 = s1.clone();
// println!("s1 = {s1}, s2 = {s2}");
//
// this works just fine and explicitly produces the behavcior shown where the heap data does get
// copied
//
// when you see a call to clone, you know that some aribtrary code is being executed and that code
// may be expensive. 
//
// 
// STACK ONLY DATA: COPY
//
// theres another wrinkly we havent talked about yet, this code using integers - part of which
// works and is valid:
//
// let x = 5;
// let y = x;
// println!("x = {x}, y = {y}");
//
// but this code seems to contradict what we just learned: we dont have a call to clone, but x is
// still valid and wasnt moved into y.
//
// the reason is that types such as integers that have a known size at compile time are stored
// entirely on the stack, so copies of the actual values are quick to make. that means theres no
// reason we would want to prevent x from being valid after we create the variable y. in other
// words, theres no difference between deep and shallow copying here, so calling clone wouldnt do
// anything different from the usual shallow copying, we can leave it out
//
// Rust has special annotatios called the Copy trait that we can plac on types that are stored on
// the stack, as integers are. if a type implements the copy trait, variables that use it do not
// move, but rather are trivally copied, making them still invalid after assignment to another
// variable.
//
// Rust wont let us annotate a type with copy if the type or any of its parts are implemented the
// Drop trait. if the type needs something special to happen when the value goes out of scope and
// we add the Copy annotatoin to that type, well get a compiler time error. 
//
// so what types implement the copy trait? here are some of the types that implement copy:
// - all integer types, such as u32
// - the boolean type, bool, with values true and false
// - all the floating point types, such as f64
// - the char type, char
// - tuples if they only contain types that also implement Copy. 
//
//
// OWNERSHIP AND FUNCTIONS
//
// the mechanics of passing a value to a function are similar to those when assigning a value to a
// variable. passing a value to a function will move or copy, just as assignment does...
//
//

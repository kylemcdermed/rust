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

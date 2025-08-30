// CONTROL FLOW
//
// the most common constructs that let you control the flow of execution of Rust code are if
// expressions and loops
//
//
// IF EXPRESSIONS
//
// an if expression allows you to branch your code depending on conditions, you provide a condition
// and then state, if this condition is met, run this block of code, if the condition is not met,
// do not run this block of code
//
// all if expressions start with the keyword if, followed by the condition. in this case, the
// condition checks whether or not the variable number has a value less than 5. we place the block
// of code to execute if the condition is true immediately after the condition inside curly
// brackets. blocks of code associated with the conditions in if expressions are sometimes called
// 'arms', just like the arms in the match expressions
//
// optionally we can also include an else expression which we chose to do here, to give the program
// an alternative block of code to execute should the condition evalute to false. if you dont
// provide an else expression and the condition is false, the program will just skip the if block
// and move onto the next bit of code
//
// we can change the number variable to a value that makes the condition false 
//
// let number = 7;
//
// its also worth noting that the condition in this code must be a ool. if the condition isnt a
// bool, well get an error
//
// if the condition evaluates to a value, Rust throws an error 
//
// the erorr indicates that Rust expected a bool but got an integer, Rust does not automatically
// try to conver non boolean types to a boolean. you must explicitly and always provide if with a
// boolean as its condition. if we want the if code block to run only when a number is not equal to
// 0, for example, we can change the if statement to say
//
// if number != 0

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
//
//
// HANDLING MULTIPLE CONDITIONS WITH ELSE IF 
//
// you can use multiple conditions by combining if and else in an else if expression
//
// the program has 4 possible paths it can take
//
// when this program executes, it checks each if expression in turn and executes the first body for
// which the conidition evaluates to true. note that even though 6 is divisible by 2, we dont see
// the output number is divisible by 2, nor do we see the number is not divisible by 4, 3, or 2
// text from the else block. thats because Rust only executes the block for the first true
// condition, once it finds one, it doesnt even check the rest.
//
// using too many else if expressions can clutter your code, so if you have more than one, you
// might want to refactor your code
//
//
// USING IF IN A LET STATEMENT 
//
// because if is an expression, we can use it on the right side of a let statement to assign the
// outcome to a variable 
//
// the number variable will be bound to a value based on the outcome of the if expression
//
// remember that blocks of code evaluate to the last expression in them, and numbers by themselves
// are also expressions. in this case, the value of the whole if expression depends on which block
// of code executes. This means the values that have the potential to be rsults from each arm of
// the if must be the same type. the results of both the if arm and the else arm were i32 integers.
// if the types are mismatched, well get an error
//
//
//
//
//
//
//
//
//
//
//
//

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
// the expression in the if block evaluates to an integer, and the expression in the else block
// evaluates to a string. this wont work because variables must have a single type, and Rust needs
// to know at compile time what type the `number` variable is, definitively. knowing the type of
// number lets the compiler verify the type is valid everywhere we use number. Rust wouldnt be able
// to do that if the type of number was only determined at runtime; the compiler would be more
// complex and would make fewer guarantees about the code if it had to keep track of multiple
// hypothetical types for any variable
//
//
// REPEITION WITH LOOPS
//
// its often useful to execute a block of code more than once. for this task, Rust provides several
// loops, which will run through the code inside the loop body to the end and then start
// immediately back at the beginning. to experiment with loops, lets make a new project called
// loops.
//
// Rust has three kinds of loops: loop, while, and for. lets try each one
//
//
// REPEATING CODE WITH LOOPS
//
// the `loop` keyword tells Rust execute a block of code over and over again forever or until you
// expicitly tell it to stop
//
// when we run this program, well see 'again!' printed over and over continously until we stop the
// program manually. most terminals support the keyboard shortcut ctrl-c to interrupt a program
// that is stuck in a infinite loop
//
// fortunately, Rust also provides a way to break out of a loop using code, you can place a break
// keyword within the loop to tell the program when to stop executing the loop.
//
// we also used continue in the guessing game, which in a loop tells the program to skip over any
// remaining code in this iteration of the loop and go to the next iteratio
//
//
// RETURNING VALUES FROM LOOPS
//
// one of the uses of a loop is to retry an operation you might fail, sich as checking whether a
// thread has completed its job. you might also need to pass the result of that operation out of
// the loop to the rest of your code. to do this, you can add the value you want returned after the
// break expression you use to stop the loop; that value will be returned out of the loop so you
// can use it
//
// 
// LOOP LABELS TO DISAMBIGUATE BETWEEN MULTIPLE LOOPS 
//
//
// if you have loops within loops, break and continue apply to the innermost loop at that point.
// you can optionally specify a loop label on a loop that you can then use with a break or continue
// to specify that those keywords apply to the labeled loop instead of the innermost loop. loop
// labels must begin with a single quote
//
// the outer loop has the lbale 'counting_up, and it will count from 0 to 2. the inner loop without
// a label counts down from 10 to 9. the first break that doesnt specify a label will exit the
// inner loop only. the break 'counting_up; statement will exit the outer loop
//
//
// CONDITIONAL LOOPS WITH WHILE 
//
//
// a program will often need to evaluate a condition within a loop. while the condition is true,
// the loop runs. when the condition ceases to be true, the program calls break, stopping the loop.
// its possible to implement behavior like this using a combination of loop, if, else, and break;
// you could try that now in a program. however this pattern is so common that Rust has built in a
// language construt for it, called a while loop. 
//
// the construct eliminate a lot of nesting that would be necessayr if you used loop, if, else and
// break, and its clearer. while a condition evaluates to true, the code runs; otherwise, it exits
// the loop
//
//
// LOOPING THROUGH A COLLECTION WITH FOR 
//
// you can choose to use the while construct to loop over the elements of a collection, such as an
// array
//
//
// the code counts up through the elements in the array, it starts at index 0, and then loops until
// it reaches the final index int the array (that is, when index< 5 is no longer true)
//
// all five array values appear in the terminal, as expected. even thoug index will reach a value
// of 5 at some point, the loop stops executing before trying to fetch the 6th value from the
// array.
//
// however this approach is error pronel we could cause the program to panic if the index value or
// test condition is incorrect. for exmaple, if you changed the definition of the a array to have
// four elements but forgot to update the condition to while index < 4, the code could panic, its
// also slow, because the comiler adds runtime code to perform the conditional check of whether the
// index is within the bounds of the array on every iteration through the loop
//
// a more concise alternative, you can use a for loop and execute some code for each item in a
// collection
//
// when we run this code we have now increased the safety of the code, and eliminated the chance of
// bugs that might result from going beyond the end of the array or ont going far enough and
// missing some items. machine code generated from for loops can be more effiicent as well, because
// the index doesnt need to be compared to the length of an array on every iteration
//
// using the for loop you wouldnt need to remember to change any other code if you changed the
// number of values in the array
//
// safety and consciouness of for loops make them the most commonly used loop construct in Rust.
// Even in situations in which you wan tto run some code a certain number times, in the countdow
// example using a while loop. most Rustaceans would use a for loop. the way to do that would to
// use a `Range`, provided by the standard library, which generates all numbers in sequence
// starting from one number and ending before another number
//
//
//
//
//
//
//
//

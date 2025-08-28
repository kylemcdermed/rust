// FUNCTIONS 
//
// the most important function in the Rust language is the `main` function, which is the entry
// point of many programs
//
// youve aso seen the `fn` keyword, which allows you to declare a new function
//
// Rust uses snake_case as the convential style for function and variable names, all letters are
// lower case and underscores separate words
//
// we define a function in Rust by entering the fn followed by a function name and a set of
// parenthese. the curly brackets tell the compiler where the function body begins and ends
//
// we can call any function weve defined by entering its name followed by a set of parentheses.
//
// because `another_function` is defined in the program, it can be called from inside the `main`
// function.
//
// note that we defined `another_function` after the main function in the source code;
//
// we could have defined it before as well, Rust doesnt care where you define yours functions, only
// that theyre defined somewhere in a scope that can be seen by th caller
//
// were going to create a new binary project called functions to explore further
//
// the lines execute in the order in which they appear in the main function, first the "hello
// world" message prints and then another_function is called and is printed
//
//
// PARAMETERS --
//
// we can define functions to have parameters, which are special variables that are part of a
// functions signature.
//
// when a function has parameters, you can provide it with concrete values for those parameters.
// technically, the concrete values are called arguments, but in casual conversation, people tend
// to use the words parameters and arguments interchangeable for either the variables in a function
// definition or the concrete values passed in when you call a function
//
// lets add another parameter inside our function
//
// the declaration of another_function has one parameter named x, the type of x is specified to
// i32. when we pass 5 in to another function, the println! macro puts 5 where the pair of curly
// brackets containing x was in th format string
//
// in function signatures, you must declare the type of each parameter. this is a delivrate
// decision in Rust's design: requiring type annotations in function definitions means the compiler
// almost never needs you to use them elsewhere in the code to figure out what type you mean. 
//
// the compiler is also able to give more helpeful error messages if it knows what types the
// function expects
//
// when defining multiple parameters, separate the parameter declarations with commas, like this
//
//
// this example creates a function named print_labeled_measurement with two parameters. the first
// parameter is named value and is an i32, the second is named unit_label and it type char. 
//
// the function then prints text containing both the value and the unit_label
//
// because we called the function with 5 as the value and 'h' as the value for unit_label, the
// program output contains those values 
//
//
// STATEMENTS AND EXPRESSIONS
//
// function bodies are made up of a series of statements optionally ending in an expression. 
//
// so far, the functions weve covered havent included an ending expression, but you have seen an
// expressoin as part of a statement 
//
// because Rust is an expression based language, this is an important distinction to understand.
// other languages dont have the same distinctions so lets look at what statements and expressions
// are and how their differences affect the bodies of the functions
//
// -- statements are instructions that perform some action and do not return a value
// -- expression evaluate to a resultant value 
//
// lets look at some examples
//
// weve actually already used statements and expressoin. creating a variable and assinging a value
// to it with the `let` keyword is a statement. 
//
// let y = 6; // is a statement 
//
//
// function definitions are also statements; the entire preceding example is a statement in itself
//
// statements do not reutrn values. therefore, you cant assign a `let` statement to another
// variable, as the following code tries to dol youll get an erorr 
//
//
// the `let y = 6` statement does not return a value, so there isnt anything for x to bind to. 
//
// this is differnet from what happens in other languages such as C, Ruby, where the assignment
// returns the value of the assignment. in those languages, you can write x = y = 6 and have both x
// and y have the value 6; that is not the case in Rust.
//
// expressions evaluate to a value and make up most of the rest of the code thar youll write in
// Rust.
//
// consider a math operation such as 5+6, which is an expression that evaulates to the value 11.
// expressions can be part of statements, the 6 in the statement let y = 6; is an expressoin that
// evaulates to the value of 6. calling a function is an expression, calling a macro is an
// expression. a new scope block created with curly brackets is an expression
//
//
// this expression:
// {
//      let x = 3;
//      x + 1
// }
//
// is a block that, in this case, evaluates to 4. that value gets bound to y as part of the let
// statement. note that the x + 1 line doesnt have a semi colon at the end, which is unlike most of
// the lines youve seen so far. expression do not include ending semi colons. if you add a semi
// colon to the end of an expression, you turn it into a statement, and it will then not return a
// value. keep this in mind as you explore function return values and expressions next.
//
// 
// FUNCTIONS WITH RETURN VALUES 
//
// functions can return values to the code that calls them. we dont name return values, but we
// must declare their type after an arrow (->). in Rust, the return value of the function is
// synonymous with the value of the final expression in the block of the body of a function. you
// can return early from a function by using the `return` keyword by specifying a value, but most
// functions return the last expression implicitly. 
//

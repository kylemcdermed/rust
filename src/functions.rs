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

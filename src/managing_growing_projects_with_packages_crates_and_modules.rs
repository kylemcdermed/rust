// MANAGING GROWING PROJECTS WITH PACKAGES, CRATES, AND MODULES 
//
//
//
// as you write large programs, organzing your code will become increasingly important. by grouping
// related functionality and separating code with distinct features youll clarify where to find
// code that implements a particular feature and where to go to change how a feature works 
//
// for very large projects comprising of a set of interrlated packages that evolve together, cargo
// provides workspaces
//
// you can create scopes and change which names are in or out of scope. you cant have two items
// with the same name in the same scope; tools are available to resolve name conflicts 
//
// these features sometimes collectively are referred to as the module system that include :
// packages, crates, modules and use, paths 
//
//
// PACKAGES AND CRATES 
//
// a crate is the smallest amount of code that the Rust compiler consideers at a time. even if you
// run rustc rather than cargo and pass a single source code file, the compiler considers that file
// to be a crate. crates can contain modules, and the modules may be defined in other files that
// get compiled with the crate/
//
// a crate can come in two forms: a binary crate or a library crate. a binary crate are programs
// you can compile to an executable that you can run, such as a command line program or a server.
// each must have a function called main that defines what happens when the executable runs. all
// crates weve created so far have been binary crates.
//
// library crates dont have a main function, and they dont compile to an executable. instead they
// define functionality intended to be shared with multiple projects. for exmaple, the rand crate
// we used, provides functionality that generates random numbers. most of the time when rustaceans
// say crate they mean library crate and they use crate interchangeably with the general
// programming concept of a library 
//
// the crate root is a source file that the Rust compiler starts from and makes up the root modules
// of your crate 
//
// a package is a bundle of one or more crates that provides a set of functionality. a package
// contains a Cargo.toml file that describes how to build those crates. cargo is actually a package
// that contains the binary crate for the command line tool youve been using to build your code.
// the cargo package also contains a library crate that the binary crate depends on. other projects
// depend on the cargo library crate to use the same logic the cargo command line tool uses.
//
// a package can contain as many binary crates as you like, but at most only one library crate. a
// package must contain at least one crate, which thats a library or a binary crate.
//
// after we create cargo new my-project and after we run cargo new my-project, we use ls to see
// what cargo creates. in the project directory, a Cargo.toml file giving us a package. theres also
// a src directory that contains main.rs. open Cargo.toml in your text editor, and note theres no
// mention of src/main.rs. Cargo follows a convention that src/main.rs is the crate root of a
// binary crate with the same name as the package.
//
// likewise, cargo knows that if the package directory contains src/lib.rs, the package contains a
// library crate with the same name as the package, and src/lib.rs is its crate root. cargo passes
// the crate root files to rustc to build the library or binary
//
// here we have a package that only contains src/main.rs meaning it only contains a binary crate
// named my-project. if a package contains src/main.rs and src/lib.rs it has two crate: a binary
// and a library, both with the same name as the package. a package can have multiple binary crates
// by placing files in the src/bin directory: each file will be a separate binary crate.
//
//

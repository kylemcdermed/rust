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
// DEFINING MODULES TO CONTROL SCOPE AND PRIVACY
//
// well talk about modules and other parts of the modules system, namely paths, which allow you to
// name items; the use keyword that brings a path into scope; and the pub keyword to make items
// public. well also discuss as keyword, external packages, and the glob operator
//
//
// MODULES CHEAT SHEET 
//
// here is a cheat sheet on how modules, paths, the use keyword, the pub keyword work in the
// compiler, and how most devs organize their code. 
//
// -- start from the crate root: when compiling a crate, the compiler first looks in the crate root
// file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to
// compile
//
// -- declaring modules: in the crate root file you can declare new modules; say you declare a
// 'garden' module with mod garden; the compuler will look for modules code in these places:
// 1. inline, within curly brackets that replace the semicolon following mod garden
// 2. in the file src/garden.rs
// 3. in the file src/garden/mod.rs
//
// -- declaring submodules: in any file other than the crate root, you can declare submodules. for
// exmaple you might declare mod vegetables; in src/garden.rs. the compiler will look for the
// submodules code within the directory named for the parent modules in these places:
// 1. inline, directly following mod vegetables, within curly brackets instead of the semi colon
// 2. in the file src/garden/vegetables.rs
// 3. in the file src/garden/vegetables/mod.rs
//
// -- paths to code in modules: once a module is part of your crate, you can refer to code in that
// module from anywhere else in that same crate, as long as the privacy rules allow, using the path
// to the code. for exmaple, an asparagus type in the garden vegetables modules would be found at
// crate::garden::vegetables::asparagus
//
// -- private vs public: code within a module is private from its parent modules by default. to
// make a module public, declare it with pub mod instead of mod. to make items within a public
// modules public as well, use pub before their declarations
//
// -- the use keyword: within a scope, the use keyword creates shortcuts to items to reduce
// repitition of long paths. in any scope that can refer to crate::garden::vegetable::asparagus,
// you can create a shortcut with use crate::garden::vegetables::asparagus; and from then on you
// only need to write asparagus to make use of that type in the scope
//
// the crate root file in this case is src/main.rs and it contains some code inside of it and the
// pub mod garden; line tells the compiler to include the code it finds in src/garden.rs, which is 
// pub mod vegetables;
//
// here pub mod vegetables; means the code in src/garden/vegetables.rs is included too, that code
// is:
//
// #[derive(Debug)]
// pub struct Asparagus {}
//
//
// GROUPING RELATED CODE IN MODULES 
//
// modules let us organize code within a crate for readability and easy reuse. modules also allow
// us to control the privacy of items because code within a modules is private by default. private
// items are internal implementation details not available for outside use. we can choose to make
// modules and the items within them public, which exposes them to external code to use and depends
// on them.
//
// lets write some library crate that provides functionality of a restuarant.
//
// in the restuarant industry, some parts of a resutatant are referred to as front of house and
// others as back of house. front of house is where the customers are; this encompasses where the
// hosts seat customers, servers take orders and payment, and bartenders make drinks. back of house
// is where the chefs and cooks work in the kitchen, dishwashers clean up and managers do
// administrative work.
//
// lets organzie our functions into nested modules, create a new library names restaurant by
// running cargo new restaurant --lib. then enter the code for the front of house section...
//
//
//

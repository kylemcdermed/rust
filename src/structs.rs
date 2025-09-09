// USING STRUCTS TO STRUCTURE RELATED DATA
//
// a struct, or structure, is a custom data type that lets you package together and name multiple
// related values that make up a meaningful group. a struct is like an objects data attributes.
//
// well compare and contract tuples with structs to build on what you already know and demonstrate
// when structs are better way to group data.
//
//
//// DEFINING AN INSTANTIATING STRUCTS
//
// structs are similar to tuples, that they both hold multiple related values. like tuples, the
// pieces of a struct can be different types. unlike tuples, a struct youll name each piece of data
// so its clear what the values mean
//
// adding names means that structs are more flexible than tuples: you dont have to rely on the
// order of the data to specify or access the values of an instance
//
// to define a struct we enter the keyword struct and name the entire struct. a structs name should
// describe the significance of data being grouped together.
//
// then inside the curly brackets we define names and types of data we call fields
//
// struct User {
//      active: bool,
//      username: String,
//      email: String,
//      sing_in_count: u64,
// }
//
// like above
//
// to use a struct after we have defined it, we create an instance of that struct by specifying
// concrete values for each of the fields. we can create an instance by stating the name of the
// struct and then add curly brackets containing `key:value` pairs, where the keys are the names of
// the fields and the values are the daa we want to store in those fields.
//
// to get specific value from a struct we use dot notation. for example, to access this users email
// address we use --> user1.email
//
// if the instance is mutable, we can change a value by using the dot notation and assigning into a
// particular field
//
//

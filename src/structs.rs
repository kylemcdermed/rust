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
// note that the entire instance is mutable; Rust doesnt allow us to mark only certain fields as
// mutable, as with any expression, we can construct a new instance of the struct as the last
// expression in the function body to implicitly return that new instance
//
// lets create a build_user function that returns a User instance with the given email and
// username. the active field gets the value of true and the sing_in_count gets a value of 1
//
// it makes sense to name the function parameters with the same name as the struct fields but
// having to repeat the email an dusername field names and variables is a bit tedious. if the
// struct had more fields repeating each name would get annoying luckily theres a shorthand
//
//
// USING THE FIELD INIT SHORTHAND 
//
// because the paraneter names and struct fields names are exactly the same, we can use the field
// init shorthand syntax to rewrite build_user so it behaves exactly like the same but doesnt have
// the repitition of username and email
//
// here we are creating a new instance of the User struct, which has a field name email. we want to
// set the email field value in the email parameter of the build_user function. because the email
// field and the email parameter have the same name, we only need to write email rather than email:
// email
//
//
// CREATING INSTANCES FROM OTHER INSTANCES WITH STRUCT UPDATE SYNTAX 
//
// its often useful to create a new instance of a struct that icludes most of the values from
// another instance using the same type, but changes some. you can do this using struct update
// syntax
//
// first we show how to create a new user instance in user 2 regularly, without the update syntax.
// we set a new value for email but otherise use the same values from user1 
//
// using struct update syntax we can achieve the sam effect with less code, the syntax `..`
// specifies that the remaining fields not explicitly set should have the same value as the fields
// in the given instance
//
//

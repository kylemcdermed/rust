


fn main() {
    let my_string = String::from("hello world");

    // first_word function works on slices of Strings, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // first_word also works on references to Strings which are equivalent 
    // to the whole slices of Strings
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    //  first_word works on slices of string literals, whether partial or whole 
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // because string literals *are* string slices already, this works too, without any slice
    // syntax!
    let word = first_word(my_string_literal);
}

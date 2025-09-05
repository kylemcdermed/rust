// REFERENCES AND BORROWING 
//
// the issue with the tuple code is that we have to return the String to the calling function so we
// can still use the String after the call to calculate_length, because the String was moved into
// calculate_length. instead we can provide a reference to the String value. a reference is like a
// pointer in that its an address we can follow to access the data stored at that address; that
// data is owned by some other variable. unlike a pointer, a reference is guaranteed to point to a
// valid value of some particular type for the life of that reference 
//
// here is how you would define and use calculate_length function that has a reference to an object
// as a parameter instead of taking ownership of a value:
//
//

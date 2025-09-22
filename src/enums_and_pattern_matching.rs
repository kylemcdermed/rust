// ENUMS AND PATTERN MATCHING 
//
// DEFINING AN ENUM 
//
// where structs give you a way of grouping together related fields and data, like a Rectangle with
// its width and height, enums give you a way of saying a value is one of a possible set of values.
// for example, we may want to say that Rectangle is one of a set of possible shapes that also
// includes Circle and Triangle. To do this, Rust allows us to encode these possbilities as enum.
//
// Say we are working with IP addresses. Currently, the two major standards for IP addresses are :
// IPV4, IPV6.
//
// Because there are only two possibilities for an IP address that our program will come across, we
// can enumerate all possible variants, which is where enumeration gets its name. 
//
// Because IP addresses can only be one of the variants (IPV4, IPV6), enums data structure are best
// appropriate for this. 
//
// Both IPV4 and IPV6 should be treated as the same type when code is handling situations that
// apply to any kind of IP addresses.
//
// We can express this concept in code by defining an IpAddrKind enumeration and listing the
// possible kinds of IP address can be V4 and V6:
//
// enum IpAddrKind {
//      V4,
//      V6,
// };
//
// IpAddrKin is now a custom data type that we can use else where in our code 
//
//
// ENUM VALUES 
//
// We can create instances of each of the two variants of IpAddrKind like this:
// let four = IpAddrKind::V4;
// let six = IpAddrKing::V6;
//
// note that the variants of enum are namespaced under its identifier, and we use a double colon to
// separate the two. This is useful because no both values ::V4 and ::V6 are of the same type:
// IpAddrKind. We can then define a function that takes any IpAddrKind:
//
// fn route(ip_kind: IpAddrKind) {}
//
// we can also call this function with either variant:
//
// route(IpAddrKind::V4);
// route(IpAddrKind::V6);
//
// using enums has even more advantages. Thinking more about our IP address type, at the moment we
// dont have a way to store the actual IP address data; we only know what kind it is...

// Unlike structs, enums define the *possible* values something could take. This
// allows us to define one function that can take either type, but still have
// the same signature.
enum IpAddr {
    V4(u8, u8, u8, u8), // V4 variant
    V6(String), // V6 variant
}
// let home = IpAddr::V4(127, 0, 0, 0);
//
// let loopback = IpAddr::V6(String::from("::1"));

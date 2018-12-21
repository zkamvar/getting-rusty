// Unlike structs, enums define the *possible* values something could take. This
// allows us to define one function that can take either type, but still have
// the same signature.
enum IpAddrKind {
    V4, // V4 variant
    V6, // V6 variant
}
// let V4 = IpAddrKind::V4;
// let V6 = IpAddrKind::V6;

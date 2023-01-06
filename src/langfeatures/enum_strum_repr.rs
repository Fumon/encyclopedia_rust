use strum::FromRepr;

#[allow(unused)]

#[derive(Debug, PartialEq, FromRepr)]
#[repr(u32)]
enum PType {
    Start = 0x0,
    Command = 0x1,
    Stop = 0x2,
    Else,
}

#[test]
fn test_from() {
    assert_eq!(PType::Start, PType::from_repr(0x0).unwrap());
    assert_eq!(PType::Command, PType::from_repr(0x1).unwrap());
    //assert_eq!(PType::Else, PType::from_repr(96).unwrap());
}
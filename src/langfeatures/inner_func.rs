
#[test]
fn inner_functions_exist() {
    assert_eq!(9, add_two_with_inner_func(7));
}

#[test]
fn inner_functions_can_be_returned_as_function_pointers() {
    assert_eq!(14, get_either_adder(AdderSpec::ThatOne)(7));
    assert_eq!(7, get_either_adder(AdderSpec::TheOtherOne)(7));
}



fn add_two_with_inner_func(i: u8) -> u8 {
    fn add_two(e: u8) -> u8 {
        e + 2
    }

    add_two(i)
}

enum AdderSpec {
    ThatOne,
    TheOtherOne,
}

fn get_either_adder(spec: AdderSpec) -> fn(u8) -> u8 {
    fn add_seven(i: u8) -> u8 {
        i + 7
    }

    fn add_nothing(i: u8) -> u8 {
        i
    }

    match spec {
        AdderSpec::ThatOne => add_seven,
        AdderSpec::TheOtherOne => add_nothing,
    }
}
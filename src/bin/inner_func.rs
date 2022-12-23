

fn main() {
    println!("{} plus 2 is: {}", 7, add_two_with_inner_func(7));

    println!("{} using ThatOne: {}", 7, get_either_adder(AdderSpec::ThatOne)(7));
    println!("{} using TheOtherOne: {}", 7, get_either_adder(AdderSpec::TheOtherOne)(7));
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
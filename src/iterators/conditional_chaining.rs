#![allow(unused)]

fn div_two_iter(a: impl Iterator<Item = i32>) -> impl Iterator<Item = i32> {
    a.map(|n| n / 2)
}

enum Either<Plain, Fancy>
where
    Plain: Iterator<Item=i32>,
    Fancy: Iterator<Item=i32>,
{
    L(Plain),
    R(Fancy),
}

impl<Plain: Iterator<Item=i32>, Fancy: Iterator<Item=i32>> Iterator for Either<Plain, Fancy> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Either::L(a) => a.next(),
            Either::R(b) => b.next(),
        }
    }
}

fn cond_chain(num: i32, div_two: bool) -> Vec<i32> {
    let start = (0..=num).map(|n| n + 3);

    match div_two {
        true => Either::R(div_two_iter(start)),
        false => Either::L(start),
    }.collect()
}

#[cfg(test)]
mod test {
    use crate::iterators::conditional_chaining::cond_chain;

    #[test]
    fn cond_chain_it() {
        assert_eq!(cond_chain(3, false), vec![3, 4, 5, 6]);
        assert_eq!(cond_chain(3, true), vec![1, 2, 2, 3]);
    }
}

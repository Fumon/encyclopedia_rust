
#[test]
fn flatting_a_result_iterator_produces_only_the_oks() {
    let mut ex = (0..)
        .map(|n| if n % 5 == 0 { Err("div5") } else { Ok(n) })
        .flatten();

    assert_eq!(Some(1), ex.next());
    assert_eq!(Some(2), ex.next());
    assert_eq!(Some(3), ex.next());
    assert_eq!(Some(4), ex.next());
    assert_eq!(Some(6), ex.next());
}

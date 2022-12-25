

#[test]
fn iterating_over_negative_end_range_produces_none() {
    assert_eq!(None, (0..-1).next())
}

#[test]
fn chaining_a_none_iterator_will_produce_values() {
    assert_eq!(Some(1), (0..-1).chain(1..3).next())
}
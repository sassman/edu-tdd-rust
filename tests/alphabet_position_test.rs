use dev_challenge_47::alphabet_position;

#[test]
fn it_should_replace_the_a_with_1() {
    let replaced = alphabet_position("a");
    assert_eq!(replaced, "1");
}

#[test]
fn it_should_replace_the_capital_a_with_1() {
    let replaced = alphabet_position("A");
    assert_eq!(replaced, "1");
}

#[test]
fn it_should_ignore_non_characters() {
    let replaced = alphabet_position("'a a. 2");
    assert_eq!(replaced, "1 1");
}

#[test]
fn it_should_replace_the_sentence() {
    let replaced = alphabet_position("The sunset sets at twelve o' clock.");
    assert_eq!(
        replaced,
        "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11"
    );
}

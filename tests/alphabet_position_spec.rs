use dev_challenge_47::alphabet_position;
use speculate::speculate;

speculate! {
    describe "alphabet_position" {
        it "should replace 'a' with 1" {
            assert_eq!(alphabet_position("a"), "1");
        }
        it "should replace 'A' with 1" {
            assert_eq!(alphabet_position("A"), "1");
        }
        it "should ignore non characters" {
            assert_eq!(alphabet_position("'a a. 2"), "1 1");
        }
        it "should replace the full test sentence" {
            assert_eq!(
                alphabet_position("The sunset sets at twelve o' clock."),
                "20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11"
            );
        }
    }
}

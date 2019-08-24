# [Daily Challenge #47 - Alphabets][1]

> In today's challenge, you are asked to replace every letter with its position in the alphabet for a given string where 'a' = 1, 'b'= 2, etc.
> For example:
> alphabet_position("The sunset sets at twelve o' clock.") should return 20 8 5 19 21 14 19 5 20 19 5 20 19 1 20 20 23 5 12 22 5 15 3 12 15 3 11 as a string.

## solution in rust

I want to illustrate the solution written in rust and focus on the testing aspect.
The [Rust book already talks][2] about testing and how to organise the code for it, so I will repeat a few pieces here.

For all friends of rpsec I want to go an extra mile and show how we can adapt the tests in a bdd style and write some specs for it.

## a crate for specs in rust

I choose [the crate speculate][3] caused by their simplicity and their good documentation with examples.

[1]: https://dev.to/thepracticaldev/daily-challenge-47-alphabets-4cbn
[2]: https://doc.rust-lang.org/book/ch11-03-test-organization.html
[3]: https://crates.io/crates/speculate

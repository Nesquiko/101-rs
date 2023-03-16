/// Very naive implementation of FizzBuzz
pub fn fizz_buzz(i: u32) -> String {
    let fizz = i % 3 == 0;
    let buzz = i % 5 == 0;

    if fizz && buzz {
        "FizzBuzz".to_owned()
    } else if buzz {
        "Buzz".to_owned()
    } else if fizz {
        "Fizz".to_owned()
    } else {
        format!("{i}")
    }
}

// TODO Write a unit test, using the contents of `fizzbuzz.out` file
// to compare.
// You can use the `include_str!()` macro to include file
// contents as `&str` in your artifact.

#[test]
fn test_fizz_buzz() {
    let expected: Vec<String> = include_str!("../fizzbuzz.out")
        .split("\n")
        .map(|s| s.to_string())
        .collect();

    for i in 1..1001 {
        let actual = fizz_buzz(i);
        let exp = expected.get((i - 1) as usize).unwrap().to_owned();

        assert_eq!(actual, exp);
    }
}

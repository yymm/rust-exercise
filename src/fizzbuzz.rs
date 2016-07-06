pub fn fizzbuzz_match(n: i32) -> String {
    match n {
        n if n % 3 == 0 && n % 5 == 0 => "fizzbuzz".to_string(),
        n if n % 3 == 0 => "fizz".to_string(),
        n if n % 5 == 0 => "buzz".to_string(),
        _          => n.to_string()
    }
}

pub fn fizzbuzz_match_effective_pattern(n: i32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "fizzbuzz".to_string(),
        (0, _) => "fizz".to_string(),
        (_, 0) => "buzz".to_string(),
        (_, _) => n.to_string()
    }
}

pub fn fizzbuzz_if(n: i32) -> String {
    if n % 3 == 0 && n % 5 == 0 { "fizzbuzz".to_string() }
    else if n % 3 == 0          { "fizz".to_string()     }
    else if n % 5 == 0          { "buzz".to_string()     }
    else                        { n.to_string()          }
}

#[test]
fn test_match() {
    assert_eq!("1",        fizzbuzz_match(1));
    assert_eq!("fizz",     fizzbuzz_match(3));
    assert_eq!("buzz",     fizzbuzz_match(5));
    assert_eq!("fizzbuzz", fizzbuzz_match(15));
}

#[test]
fn test_match_effective_pattern() {
    assert_eq!("1",        fizzbuzz_match_effective_pattern(1));
    assert_eq!("fizz",     fizzbuzz_match_effective_pattern(3));
    assert_eq!("buzz",     fizzbuzz_match_effective_pattern(5));
    assert_eq!("fizzbuzz", fizzbuzz_match_effective_pattern(15));
}

#[test]
fn test_if() {
    assert_eq!("1",        fizzbuzz_if(1));
    assert_eq!("fizz",     fizzbuzz_if(3));
    assert_eq!("buzz",     fizzbuzz_if(5));
    assert_eq!("fizzbuzz", fizzbuzz_if(15));
}

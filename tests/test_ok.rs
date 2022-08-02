use ctrl_macros::{ok_or, ok_or_break, ok_or_continue, ok_or_return};

#[test]
fn test_ok_or_return() {
    let mut value = 1;
    returns_early(Ok(42), &mut value);
    assert_eq!(value, 42);

    let mut value = 1;
    returns_early(Err("Failure".to_owned()), &mut value);
    assert_eq!(value, 1);
}

fn returns_early(input: Result<i32, String>, value: &mut i32) {
    let res = ok_or_return!(input);
    *value = res;
}

#[test]
fn test_ok_or_return_with_value() {
    assert_eq!(returns_early_with_value(Ok(42)), 42);

    assert_eq!(returns_early_with_value(Err("Failure".to_owned())), 44);
}

fn returns_early_with_value(input: Result<i32, String>) -> i32 {
    let res = ok_or_return!(input, 44);
    res
}

#[test]
fn test_ok_or_with_return() {
    assert_eq!(ok_or_return_err(Ok(1), 2), 1);
    assert_eq!(ok_or_return_err(Err("Failure".to_owned()), 2), 2);
}

fn ok_or_return_err(input: Result<i32, String>, if_err: i32) -> i32 {
    let res = ok_or!(input, return if_err);
    res
}

#[test]
fn test_ok_or_continue() {
    let mut sum = 0;
    for i in 0..5 {
        let x = if i > 2 { Ok(i) } else { Err("Failure") };

        let x = ok_or_continue!(x);
        sum += x;
    }
    assert_eq!(3 + 4, sum);
}

#[test]
fn test_ok_or_with_continue() {
    let mut sum = 0;
    for i in 0..5 {
        let x = if i > 2 { Ok(i) } else { Err("Failure") };

        let x = ok_or!(x, continue);
        sum += x;
    }
    assert_eq!(3 + 4, sum);
}

#[test]
fn test_ok_or_break() {
    let mut sum = 0;
    for i in 0..5 {
        let x = if i > 2 { Err("Failure") } else { Ok(i) };

        let x = ok_or_break!(x);
        sum += x;
    }
    assert_eq!(1 + 2, sum);
}

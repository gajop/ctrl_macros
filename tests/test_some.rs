use ctrl_macros::{some_or, some_or_break, some_or_continue, some_or_return};

#[test]
fn test_some_or_return() {
    let mut value = 1;
    returns_early(Some(42), &mut value);
    assert_eq!(value, 42);

    let mut value = 1;
    returns_early(None, &mut value);
    assert_eq!(value, 1);
}

fn returns_early(input: Option<i32>, value: &mut i32) {
    let res = some_or_return!(input);
    *value = res;
}

#[test]
fn test_some_or_with_return() {
    assert_eq!(some_or_return_err(Some(1), 2), 1);
    assert_eq!(some_or_return_err(None, 2), 2);
}

fn some_or_return_err(input: Option<i32>, if_err: i32) -> i32 {
    let res = some_or!(input, return if_err);
    res
}

#[test]
fn test_some_or_continue() {
    let mut sum = 0;
    for i in 0..5 {
        let x = if i > 2 { Some(i) } else { None };

        let x = some_or_continue!(x);
        sum += x;
    }
    assert_eq!(3 + 4, sum);
}

#[test]
fn test_some_or_with_continue() {
    let mut sum = 0;
    for i in 0..5 {
        let x = if i > 2 { Some(i) } else { None };

        let x = some_or!(x, continue);
        sum += x;
    }
    assert_eq!(3 + 4, sum);
}

#[test]
fn test_some_or_break() {
    let mut sum = 0;
    for i in 0..5 {
        let x = if i > 2 { None } else { Some(i) };

        let x = some_or_break!(x);
        sum += x;
    }
    assert_eq!(1 + 2, sum);
}

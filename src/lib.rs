#[macro_export]
macro_rules! ok_or {
    ( $e:expr, $err_case:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => $err_case,
        }
    };
}

#[macro_export]
macro_rules! ok_or_return {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return,
        }
    };
    ( $e:expr, $return_value:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => return $return_value,
        }
    };
}

#[macro_export]
macro_rules! ok_or_continue {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => continue,
        }
    };
}

#[macro_export]
macro_rules! ok_or_break {
    ( $e:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => break,
        }
    };
    ( $e:expr, $return_value:expr ) => {
        match $e {
            Ok(x) => x,
            Err(_) => break $return_value,
        }
    };
}

#[macro_export]
macro_rules! some_or {
    ( $e:expr, $err_case:expr ) => {
        match $e {
            Some(x) => x,
            None => $err_case,
        }
    };
}

#[macro_export]
macro_rules! some_or_return {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return,
        }
    };
    ( $e:expr, $return_value:expr ) => {
        match $e {
            Some(x) => x,
            None => return $return_value,
        }
    };
}

#[macro_export]
macro_rules! some_or_continue {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => continue,
        }
    };
}

#[macro_export]
macro_rules! some_or_break {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => break,
        }
    };
    ( $e:expr, $return_value:expr ) => {
        match $e {
            Some(x) => x,
            None => break $return_value,
        }
    };
}

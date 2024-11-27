/// debug println
/// print:
/// ```
/// [DEBUG]: ${TEXT}
/// ```
#[macro_export]
macro_rules! dmsg {
    ( $( $s:expr ),* ) => {
        {
            let result = vec![
                $( format!("{}", $s) ),*
            ].join("");
            println!("\x1b[33m[DEBUG]\x1b[0m {}", result);
        }
    };
}


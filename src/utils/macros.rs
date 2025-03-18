#[macro_export]
macro_rules! debug {
    ($any:expr) => {{
        eprintln!(
            "[{}:{}:{}] -> {:?}",
            file!(),
            line!(),
            column!(),
            $any
        );
        $any
    }};
}

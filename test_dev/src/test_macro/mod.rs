#[macro_export]
macro_rules! skip {
    ($res:expr) => {
        match $res {
            true => {
                continue;
            }
            false => {}
        }
    }
}
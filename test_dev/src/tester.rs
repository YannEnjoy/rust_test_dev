pub fn test_function(){
    println!("in tester");
}

pub fn get_text() -> String{
    "Word".to_string()
}

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
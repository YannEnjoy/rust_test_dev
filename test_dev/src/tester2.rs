use crate::tester::*;

use macro_list::skip;


pub fn print_test(){
    println!("{}",get_text());
}

pub fn loop_test(){
    for i in 1..10{
        if i >5 {
            skip!(true);
            print_test();
        }
        println!("loop now {}",i);
    
    }
    println!("break loop");
}
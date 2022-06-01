mod test_macro;

fn main(){
    for i in 1..10{
        if i >5 {
            skip!(true);
        }
        println!("loop now {}",i);
    
    }
    println!("break loop");
}
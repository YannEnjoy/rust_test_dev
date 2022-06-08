use std::ops::Deref;

// xの中身をバイト列として見るための関数
fn as_raw_bytes<'a, T:?Sized>(x: &'a T) -> &'a [u8] {
    unsafe {
        std::slice::from_raw_parts(
            x as *const T as *const u8,
            std::mem::size_of_val(x))
    }
}

pub struct S<T:?Sized> {
    pub x: u8,
    pub y: T,
}

fn main() {
    let strslice = "a";
    let strslice2 = "a".to_string();
    println!("strslice = {:?}", as_raw_bytes(strslice));
    println!("&strslice = {:?}", as_raw_bytes(&strslice));
    println!("&&strslice = {:?}", as_raw_bytes(&&strslice));
    
    // println!("&strslice2 = {:?}", as_raw_bytes(strslice2));
    println!("&strslice2 = {:?}", as_raw_bytes(&strslice2));
    println!("&&strslice2 = {:?}", as_raw_bytes(&&strslice2));
}
use std::slice;

fn main() {
    test_bare_pointer();
    test_unsafe_split();
    test_basic_macro();
    test_vec_macro();
}

fn test_bare_pointer() {
    println!("====== test_bare_pointer ======");
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 value = {:x}", r1 as usize);
        println!("r1 = {}", *r1);
        println!("r2 = {}", *r2);
        *r2 = 10;
        println!("------ modified ------");
        println!("r2 = {}", *r2);
    }

    let addr = 0xdfba0ff804usize;
    let r = addr as *const i32;

    println!("------ get some value ------");
    unsafe {
        // println!("addr = {}", *r);
    }
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let length = values.len();
    let val_ptr = values.as_mut_ptr();
    assert!(mid <= length);

    unsafe {
        (
            slice::from_raw_parts_mut(val_ptr, mid),
            slice::from_raw_parts_mut(val_ptr, length - mid)
        )
    }
}

fn test_unsafe_split() {
    println!("====== test_unsafe_split ======");
    let mut arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let (arr_1, arr_2) = split_at_mut(&mut arr, 7);

    // println!("arr => {:?}", arr);

    println!("arr_1 => {:?}", arr_1);
    println!("arr_1 => {:?}", arr_2);
}

macro_rules! bouble {
    ($x:expr) => {
        2*$x
    };
}

fn test_basic_macro() {
    println!("====== test_basic_macro ======");
    let x = 5;
    let y = bouble!(x);
    println!("y = {}", y);
}

macro_rules! mvec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn test_vec_macro() {
    println!("====== test_vec_macro ======");
    let v = mvec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);
}


use std::slice;

// Those superpowers include the ability to:

// Dereference a raw pointer
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions

unsafe fn dangerous() {}

fn another_unsafe_split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    let ptr = values.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();

    assert!(mid <= len);

    //(&mut values[..mid], &mut values[mid..]) // can't borrow as mutable multiple times even with unsafe function



    (
        slice::from_raw_parts_mut(values.as_mut_ptr(), mid),
        slice::from_raw_parts_mut(values.as_mut_ptr().add(mid), len - mid),
    )
  

}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let address = 0x012345usize; // some random address
    let r = address as *const i32; // converting address to raw pointer

    println!("r1 is: {:?}", r1);
    println!("r2 is: {:?}", r2);
    println!("r is: {:?}", r);

   // dangerous(); // this will give error because it is unsafe function
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        dangerous();
    }
    // both example are same but the difference is that r1 is immutable and r2 is mutable
    // and we can do it

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}
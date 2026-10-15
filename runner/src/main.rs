unsafe extern "C" {
    fn add_numbers(a: i32, b: i32) -> i32;
    fn sub_numbers(a: i32, b: i32) -> i32;
}

fn main() {
    // SAFETY: add_numbers uses the C ABI with two 32-bit integers and no pointers.
    // These inputs also stay within the C3 signed integer range when added.
    let result = unsafe { add_numbers(5, 7) };
    assert_eq!(result, 12);
    println!("The result of adding 5 and 7 is: {}", result);
    // SAFETY: sub_numbers uses the C ABI with two 32-bit integers and no pointers.
    // These inputs also stay within the C3 signed integer range when subtracted.
    let result_sub = unsafe { sub_numbers(10, 3) };
    assert_eq!(result_sub, 7);
    println!("The result of subtracting 3 from 10 is: {}", result_sub);
}

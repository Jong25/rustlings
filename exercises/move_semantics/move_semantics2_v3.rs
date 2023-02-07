// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

///
/// In general, borrowing should be used in three cases.
///
/// First is when some value should be passed in and then out. For example, your fill_vec function could be written as such:
/// ```ignore
/// fn fill_vec(vec: &mut Vec<i32>) {
///     vec.push(22);
///     vec.push(44);
///     vec.push(66);
/// }
/// ```
/// From the low-level point of view, this is equivalent to the original function,
/// the difference is only with ownership semantics.
/// 
/// Second is when some value can't be cloned, but should be read from multiple places.
/// In this case, you pass references to it around, while storing the value itself somewhere in the parent function
/// (for example, in the Box).
/// 
/// And the third case is when you find out that your code is too slow and want to optimize it.
/// For example, large vector may be, while clonable, fairly costly to clone on every action.
/// In this case, you can borrow the vector and pass around this borrow
/// - which, thanks to so-called "deref coercions",
/// will be transparently converted into reference to slice whenever it is necessary.
/// 

fn main() {
    let mut vec0 = Vec::new();
    
    fill_vec(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) {
    vec.push(22);
    vec.push(44);
    vec.push(66);
}

#[rustfmt::skip]
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    if let Some(val) = my_option {
        println!("{val:?}");
    }

    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {my_arr:?}");

    let my_empty_vec: Vec<i32> = vec![1, 2, 3, 4, 5];
    let my_empty_vec: Vec<i32> = my_empty_vec.into_iter().take(0).collect();
    println!("This Vec is empty, see? {my_empty_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // ✅ Correct way to swap
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}

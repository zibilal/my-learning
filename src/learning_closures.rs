#[test]
fn test_non_move_closures() {
    let y = 10;
    let add_y = |x| x + y;
    let copy_of_add_y = add_y;     // This closure is `Copy`, so ...
    assert_eq!(add_y(copy_of_add_y(22)), 42);  // ... we can call both
}


#[test]
fn test_move_closures() {
    let mut x = 0;
    let mut add_to_x = |n: i32| { x += n; x };

    // let copy_of_add_to_x = add_to_x;     // this moves, rather than copies
    // assert_eq!(add_to_x(copy_of_add_to_x(1)), 2);    // error: used a moved value
    // assert_eq!(add_to_x(add_to_x(1)), 2);            // error: cannot borrow more than once
    assert_eq!(add_to_x(1), 1);
    assert_eq!(add_to_x(1), 2);
    assert_eq!(x, 2);
}
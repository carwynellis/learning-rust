pub fn references_and_borrowing() {
    println!("References and Borrowing\n");

    // Immutable references can be borrowed as follows.
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let answer = sum_two_vecs(&v1, &v2);

    println!("Sum of v1 and v2 is {}", answer);

    // By default references are immutable.

    // Mutable references are also supported. For example...
    let mut x = 5;

    // Note that the following only works because we introduce an extra scope.
    {
        let y = &mut x;
        *y += 1;
    }

    println!("x is now {}", x);

    println!();
}

// Folds over a vec to compute the sum.
fn sum_vec(v: &Vec<i32>) -> i32 {
    return v.iter().fold(0, |a, &b| a + b);
}

fn sum_two_vecs(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    sum_vec(v1) + sum_vec(v2)
}

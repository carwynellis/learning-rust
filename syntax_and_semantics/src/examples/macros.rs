pub fn macros() {
    println!("Macros\n");

    macro_rules! vecExample {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    let x = vecExample![1, 2, 3];

    println!("vec macro generated vector with {} elements", x.capacity());

    println!();
}

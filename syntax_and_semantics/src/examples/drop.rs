#![allow(unused_variables)]

pub fn drop() {
    println!("Drop\n");

    // Rust provides a trait, Drop, that will run some code when a value goes
    // out of scope.

    // This is useful for handling and cleanup, freeing of resources etc.. when
    // a value is no-longer in use.

    struct Firework {
        strength: i32,
    }

    impl Drop for Firework {
        fn drop(&mut self) {
            println!("BOOM times {}!!!", self.strength);
        }
    }

    fn create_fireworks() {
        println!("Declaring firework strength: 1");
        let firecracker = Firework { strength: 1 };
        println!("Declaring firework strength: 100");
        let tnt = Firework { strength: 100 };
        // Both instances fall out of scope here, triggering the drop code.
    }

    // Note that drop invocations occur in the reverse order of declaration.
    // Calling create_fireworks will output 100 and then 1, which is the
    // reverse of the order of declaration.
    create_fireworks();

    // This makes sense since later declarations may depend on earlier
    // declarations. So, the cleanup order should the reverse of the
    // declaration order.

    println!();
}

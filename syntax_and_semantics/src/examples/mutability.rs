use std::cell::Cell;
use std::cell::RefCell;
use std::sync::Arc;

pub fn mutability() {
    println!("Mutability\n");

    // Variables are immutable by default.
    // Mutability can be declared with the mut keyword.
    let mut x = 1;
    println!("x is {}", x);

    x = 2;
    println!("x is {} after assignment", x);

    // A reference to a value must also be declared mutable if that reference
    // is to be used to modify the value.
    {
        // Note that ref_x is immutable in that it cannot be changed to point to
        // another resource.
        let ref_x = &mut x;
        *ref_x = 12;
    }

    println!("x is now {} after modification via mutable reference", x);

    // Interior vs Exterior Mutability

    // Immutability refers to the exterior mutability of a resource. The 
    // resource itself can still have mutable interior state. For example.
    let arc = Arc::new(5);
    let arc_clone = arc.clone();
    println!("arc_clone is {}", arc_clone);
    // The Arc (reference counting ponter) instance referred to by x is still
    // able to update its internal state to allocate a reference to y, even
    // though the variable binding, x, is immutable.

    // In Rust, you can think of immutability as 'is this safe to have more
    // than one pointer to it?'.

    // The following example shows interior mutability, that is, obtaining a
    // mutable reference to a value held within another instance.
    // An immutable reference to a RefCell...
    let ref_cell = RefCell::new(42);
    {
        // We can borrow a mutable reference as follows...
        let mut ref_to_cell = ref_cell.borrow_mut();
        // which can be used to modify the value.
        *ref_to_cell = 24;
    }
    println!("ref_cell is now {}", ref_cell.into_inner());

    // Field Level Mutability

    // Mutability is either a property of a borrow or a binding.

    // This means that you cannot have a struct for example with a mixture of
    // immutable and mutable fields.

    // However a cell can be used to emulate field level mutability as follows.
    struct Point {
        a: i32,
        b: Cell<i32>,
    }

    let point = Point { a: 5, b: Cell::new(6) };

    point.b.set(7);

    println!("point after setting mutable field b: Point({}, {:?})", 
        point.a, point.b
    );

    println!();
}

pub fn create_box() {
    // Allocate an integer on the heap
    let _box1 = Box::new(3i32);
}

pub fn main() {
    // Allocate an interger on the heap
    let _box2 = Box::new(5i32);

    // A nested scope
    {
        // Allocate an integer on the heap
        let _box3 = Box::new(4i32);
        // _box3 is destroyed here and memory is freed
    }

    for _ in 0u32..1_000 {
        create_box();
    }
}

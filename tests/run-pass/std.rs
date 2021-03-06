use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

fn rc_cell() -> Rc<Cell<i32>> {
    let r = Rc::new(Cell::new(42));
    let x = r.get();
    r.set(x + x);
    r
}

// TODO(solson): also requires destructors to run for the second borrow to work
// TODO(solson): needs StructWrappedNullablePointer support
// fn rc_refcell() -> i32 {
//     let r = Rc::new(RefCell::new(42));
//     *r.borrow_mut() += 10;
//     let x = *r.borrow();
//     x
// }

fn arc() -> Arc<i32> {
    let a = Arc::new(42);
    a
}

fn true_assert() {
    assert_eq!(1, 1);
}

fn main() {
    assert_eq!(*arc(), 42);
    assert_eq!(rc_cell().get(), 84);
    true_assert();
}

// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the "cons list", a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: The value of the current item and
// the next item. The last item is a value called `Nil`.

// TODO: Use a `Box` in the enum definition to make the code compile.
#[derive(PartialEq, Debug)]

enum List {
    // `Nil` is the end of the list.
    Nil,
    // `Cons` is a non-empty list with a value and a pointer to the next item.
    Cons(i32, Box<List>),
}

// TODO: Create an empty cons list.
fn create_empty_list() -> List {
    // The cons list should be empty, represented by `Nil`.
    List::Nil
}

// TODO: Create a non-empty cons list.
fn create_non_empty_list() -> List {
    // The cons list should contain the values 1, 2, and 3.
    // The last item should be `Nil`.
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))))
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}

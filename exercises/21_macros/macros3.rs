
// TODO: Fix the compiler error without taking the macro definition out of this
// module.
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    pub fn call_macro() {
        my_macro!();
    }
}

fn main() {
    macros::call_macro();
}

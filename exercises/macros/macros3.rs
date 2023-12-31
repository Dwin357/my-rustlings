// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

// I AM NOT DONE

// "old way" (per: https://stackoverflow.com/questions/26731243/how-do-i-use-a-macro-across-module-files)
/*
#[macro_use]
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
*/


// "new way"(since Rust 1.32, 2019-01-17), note how order dependency is now not an issue

fn main() {
    macros::my_macro!();
}

mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }

    pub(crate) use my_macro; // Neat!
}


#[macro_export]
macro_rules! group_attr {
    // interior rule to hoist an unaliased extern crate
    (@inner $name:ident ($($krates:ident)*) #[cfg($attr:meta)] $(#[$krattr:meta])* extern crate $krate:ident; $($yes:tt)*) => {
        // hoist the extern crate statement
        #[cfg($attr)] $(#[$krattr])* extern crate $krate;

        // collect the crate name and continue processing
        group_attr!(@inner $name ($($krates)* $krate) #[cfg($attr)] $($yes)*);
    };

    // interior rule to hoist an aliased extern crate
    (@inner $name:ident ($($krates:ident)*) #[cfg($attr:meta)] $(#[$krattr:meta])* extern crate $krate:ident as $alias:ident; $($yes:tt)*) => {
        // hoist the extern crate statement
        #[cfg($attr)] $(#[$krattr])* extern crate $krate as $alias;

        // collect the alias (not the real name) and continue processing
        group_attr!(@inner $name ($($krates)* $alias) #[cfg($attr)] $($yes)*);
    };

    // interior rule for outputting items, after extern crates are done
    (@inner $name:ident ($($krates:ident)*) #[cfg($attr:meta)] $($yes:item)*) => {
        // a curious inner module
        #[cfg($attr)]
        mod $name {
            // glob import gets all pub items from enclosing module
            #[allow(unused_imports)] use super::*;

            // explicitly import all the collected extern crates
            #[allow(unused_imports)] use super::{$($krates),*};

            // output the rest of the items
            $($yes)*
        }
    
        // re-export everything that the inner module produced
        #[cfg($attr)]
        pub use $name::*;
    };

    // entry point for default inner module name
    (#[cfg($attr:meta)] $($yes:tt)*) => { group_attr!(__internal #[cfg($attr)] $($yes)*); };

    // entry point with a name given for the inner module
    ($name:ident #[cfg($attr:meta)] $($yes:tt)*) => { group_attr!(@inner $name () #[cfg($attr)] $($yes)*); };
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

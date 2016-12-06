#[macro_export]
macro_rules! group_attr {
    // interior rule for outputting items
    (@inner $name:ident ($($krates:ident)*) #[cfg($attr:meta)] $($yes:item)*) => {
        #[cfg($attr)]
        mod $name {
            $($yes)*
        }
    };

    // entry point for default inner module name
    (#[cfg($attr:meta)] $($yes:tt)*) => { group_attr!(__internal #[cfg($attr)] $($yes)*); };

    // entry point with a name given for the inner module
    ($name:ident #[cfg($attr:meta)] $($yes:tt)*) => { group_attr!(@inner $name () #[cfg($attr)] $($yes)*); };
}


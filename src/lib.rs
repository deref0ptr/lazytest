//! Provides the [lazytest] macro to reduce the amount of code needed to write unit tests.

/// The core macro of the crate.
/// 
/// This macro converts blocks of the form:
/// ```ignore
///     test_name {
///         test_body
///     }
/// ```
/// into unit test functions and places them in a `#[cfg(test)]` module.
/// 
/// Note:
/// - The tests are automatically marked with `#[test]`. Other attributes (e.g. `#[ignore]`) can be added as usual above each test block.
/// - The test module includes a `use super::*;` import.
/// - Only `()` return types are currently supported.
#[macro_export]
macro_rules! lazytest {
    (
        $(
            $(#[$attr:meta])*
            $test_name:ident
                $body:block
        )*
    ) => {
        #[cfg(test)]
        mod tests {
            #[allow(unused_imports)]
            use super::*;

            $(
                $(#[$attr])*
                #[test]
                fn $test_name()
                    $body
            )*
        }
    };
}

lazytest! {

    // Basic use
    identity {
        assert_eq!(1, 1);
    }

    // With an attribute
    #[ignore]
    sine {
        let sin_half_pi = std::f64::consts::FRAC_PI_2.sin();
        assert!((sin_half_pi - 1.0).abs() < 1e-10);
    }

}

#[macro_export]
macro_rules! lazytest {
	(
		$(	$(#[$attr:meta])*
			$test_name:ident
				$body2:block
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
					$body2
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
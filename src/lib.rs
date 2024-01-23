

macro_rules! impl_factorial {
	($t:ty, $n:ident) => {
		#[inline]
		pub fn $n(n: $t) -> $t {
			(1..=n).fold(1, |acc, v| acc * v)
		}
	};
}

impl_factorial!(u8, factorial_u8);
impl_factorial!(u16, factorial_u16);
impl_factorial!(u32, factorial_u32);
impl_factorial!(u64, factorial_u64);
impl_factorial!(usize, factorial_usize);

#[cfg(test)]
#[test]
fn test_factorial() {
	assert_eq!(factorial_u32(0), 1);
	assert_eq!(factorial_u32(1), 1);
	assert_eq!(factorial_u32(2), 2);
	assert_eq!(factorial_u32(3), 6);
	assert_eq!(factorial_u32(4), 24);
	assert_eq!(factorial_u32(5), 120);
	assert_eq!(factorial_u32(6), 720);
	assert_eq!(factorial_u32(7), 5040);
	assert_eq!(factorial_u32(8), 40320);
	assert_eq!(factorial_u32(9), 362880);
	assert_eq!(factorial_u32(10), 3628800);
}


macro_rules! impl_double_factorial {
	($t:ty, $n:ident) => {
		#[inline]
		pub fn $n(mut n: $t) -> $t {
			// iterators are slower here...
			let mut acc = 1;
			let start = n%2+1;
			while n >= start {
				acc *= n;
				n -= 2;
			}
			acc
		}
	};
}

impl_double_factorial!(u8, double_factorial_u8);
impl_double_factorial!(u16, double_factorial_u16);
impl_double_factorial!(u32, double_factorial_u32);
impl_double_factorial!(u64, double_factorial_u64);
impl_double_factorial!(usize, double_factorial_usize);

#[cfg(test)]
#[test]
fn test_double_factorial() {
	assert_eq!(double_factorial_u32(0), 1);
	assert_eq!(double_factorial_u32(1), 1);
	assert_eq!(double_factorial_u32(2), 2);
	assert_eq!(double_factorial_u32(3), 3);
	assert_eq!(double_factorial_u32(4), 8);
	assert_eq!(double_factorial_u32(5), 15);
	assert_eq!(double_factorial_u32(6), 48);
	assert_eq!(double_factorial_u32(7), 105);
	assert_eq!(double_factorial_u32(8), 384);
	assert_eq!(double_factorial_u32(9), 945);
	assert_eq!(double_factorial_u32(10), 3840);
}



macro_rules! impl_sub_factorial {
	($t:ty, $n:ident) => {
		#[inline]
		pub fn $n(n: $t) -> $t {
			match n {
				0 => return 1,
				_ => {},
			}

			let fac = |i| (1..=i).fold(1, |acc, v| acc * v);

			let f = fac(n);
			let mut acc = f;

			let mut mode = -1;

			for i in 1..n {
				let m = f / fac(i);
				acc += m * mode;
		
				mode = -mode;
			}

			acc + mode
		}
	};
}

impl_sub_factorial!(i8, sub_factorial_i8);
impl_sub_factorial!(i16, sub_factorial_i16);
impl_sub_factorial!(i32, sub_factorial_i32);
impl_sub_factorial!(i64, sub_factorial_i64);
impl_sub_factorial!(isize, sub_factorial_isize);

#[cfg(test)]
#[test]
fn test_sub_factorial() {
	assert_eq!(sub_factorial_i32(0), 1);
	assert_eq!(sub_factorial_i32(1), 0);
	assert_eq!(sub_factorial_i32(2), 1);
	assert_eq!(sub_factorial_i32(3), 2);
	assert_eq!(sub_factorial_i32(10), 1_334_961);
}



macro_rules! impl_super_factorial_sloane {
	($t:ty, $n:ident) => {
		#[inline]
		pub fn $n(n: $t) -> $t {
			let fac = |i| (1..=i).fold(1, |acc, v| acc * v);

			(1..=n).fold(1, |acc, v| acc * fac(v))
		}
	};
}

impl_super_factorial_sloane!(u8, super_factorial_sloane_u8);
impl_super_factorial_sloane!(u16, super_factorial_sloane_u16);
impl_super_factorial_sloane!(u32, super_factorial_sloane_u32);
impl_super_factorial_sloane!(u64, super_factorial_sloane_u64);
impl_super_factorial_sloane!(usize, super_factorial_sloane_usize);

#[cfg(test)]
#[test]
fn test_super_factorial_sloane() {
	assert_eq!(super_factorial_sloane_u32(0), 1);
	assert_eq!(super_factorial_sloane_u32(1), 1);
	assert_eq!(super_factorial_sloane_u32(2), 2);
	assert_eq!(super_factorial_sloane_u32(3), 12);
	assert_eq!(super_factorial_sloane_u32(4), 288);
	assert_eq!(super_factorial_sloane_u32(5), 34_560);
	assert_eq!(super_factorial_sloane_u32(6), 24_883_200);
}


macro_rules! impl_super_factorial_pickover {
	($t:ty, $n:ident) => {
		#[inline]
		pub fn $n(n: $t) -> $t {
			let fac = |i| (1..=i).fold(1, |acc, v| acc * v);
			let pow = |base, ex| (1..ex).fold(base, |acc, _| acc * base);
		
			let nf = fac(n);
			(1..nf).fold(nf, |acc, _| pow(nf, acc))
		}
	};
}

impl_super_factorial_pickover!(u8, super_factorial_pickover_u8);
impl_super_factorial_pickover!(u16, super_factorial_pickover_u16);
impl_super_factorial_pickover!(u32, super_factorial_pickover_u32);
impl_super_factorial_pickover!(u64, super_factorial_pickover_u64);
impl_super_factorial_pickover!(usize, super_factorial_pickover_usize);

#[cfg(test)]
#[test]
fn test_super_factorial_pickover() {
	assert_eq!(super_factorial_pickover_u32(1), 1);
	assert_eq!(super_factorial_pickover_u32(2), 4);
	// impossible to use beyond 2
}



macro_rules! impl_exponential_factorial {
	($t:ty, $n:ident) => {
		#[inline]
		pub fn $n(n: $t) -> $t {
			let pow = |base, ex| (1..ex).fold(base, |acc, _| acc * base);

			(0..n-1).rev().fold(1, |acc, i| pow(n-i, acc))
		}
	};
}

impl_exponential_factorial!(u8, exponential_factorial_u8);
impl_exponential_factorial!(u16, exponential_factorial_u16);
impl_exponential_factorial!(u32, exponential_factorial_u32);
impl_exponential_factorial!(u64, exponential_factorial_u64);
impl_exponential_factorial!(usize, exponential_factorial_usize);

#[cfg(test)]
#[test]
fn test_exponential_factorial() {
	assert_eq!(exponential_factorial_u32(1), 1);
	assert_eq!(exponential_factorial_u32(2), 2);
	assert_eq!(exponential_factorial_u32(3), 9);
	assert_eq!(exponential_factorial_u32(4), 262_144);
}



macro_rules! impl_hyper_factorial {
	($t:ty, $n:ident) => {
		#[inline]
		pub fn $n(n: $t) -> $t {
			let pow = |base, ex| (1..ex).fold(base, |acc, _| acc * base);

			(0..n).fold(1, |acc, i| { let v = n-i; acc * pow(v, v) })
		}
	};
}

impl_hyper_factorial!(u8, hyper_factorial_u8);
impl_hyper_factorial!(u16, hyper_factorial_u16);
impl_hyper_factorial!(u32, hyper_factorial_u32);
impl_hyper_factorial!(u64, hyper_factorial_u64);
impl_hyper_factorial!(usize, hyper_factorial_usize);

#[cfg(test)]
#[test]
fn test_hyper_factorial() {
	assert_eq!(hyper_factorial_u32(1), 1);
	assert_eq!(hyper_factorial_u32(2), 4);
	assert_eq!(hyper_factorial_u32(3), 108);
	assert_eq!(hyper_factorial_u32(4), 27_648);
}



macro_rules! impl_swinging_factorial {
	($t:ty, $n:ident) => {
		#[inline]
		pub fn $n(n: $t) -> $t {
			let half = n / 2;

			(half+1..=n).fold(1, |acc, v| acc * v)
			/ (1..=half).fold(1, |acc, v| acc * v)
		}
	};
}

impl_swinging_factorial!(u8, swinging_factorial_u8);
impl_swinging_factorial!(u16, swinging_factorial_u16);
impl_swinging_factorial!(u32, swinging_factorial_u32);
impl_swinging_factorial!(u64, swinging_factorial_u64);
impl_swinging_factorial!(usize, swinging_factorial_usize);



#[cfg(test)]
#[test]
fn test_swinging_factorial() {
	assert_eq!(swinging_factorial_u32(0), 1);
	assert_eq!(swinging_factorial_u32(1), 1);
	assert_eq!(swinging_factorial_u32(2), 2);
	assert_eq!(swinging_factorial_u32(3), 6);
	assert_eq!(swinging_factorial_u32(4), 6);
	assert_eq!(swinging_factorial_u32(5), 30);
	assert_eq!(swinging_factorial_u32(6), 20);
	assert_eq!(swinging_factorial_u32(7), 140);
	assert_eq!(swinging_factorial_u32(8), 70);
	assert_eq!(swinging_factorial_u32(9), 630);
}

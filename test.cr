type Test = root::test

pub type External = package::Type

type Alias = super::super::test

struct Testing {
	a: self::Alias,
	b: test
}

static S: i32 = 5

const C: u8 = 2

pub fn inc(x: f32) -> f32 | x + 1
pub fn dec(x: f32) -> f32 | x - 1

pub fn func() -> f32 {
	loop {
		let a: f32 = test(5 + 5)
	};

	continue;

	break;

	break 5;

	let x: _ = ::Test::test(5);

	while true {
	 	let a: f32 = test(5 + 5);
	};

	if 2 == 5 {
		let a: *f32 = &test(5 + 5);
		a
	};

	let x: ::a::i32 = 0;
	let a: i32 = if x == 0 { 
		5
	} else { 
		6
	};

	let a: f32 = test(5 + 5);
	
	a + a
}

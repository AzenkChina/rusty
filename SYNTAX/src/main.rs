#![allow(non_snake_case)]

/// cargo build --release
/// cargo run --release

mod hello;
use hello::hello_external;

fn main() {

	{
		hello_internal();

		//mod hello;
		hello::hello_external();

		//use hello::hello_external;
		hello_external();
	}

	{
		let x = true;
		let y: bool = !x;

		println!("x is {}, y is {}", x, y);
	}

	{
		let x = 'x';
		let y: char = 'y';

		println!("x is {}, y is {}", x, y);
	}

	{
		let x = "Hello, World!";
		let y: &str = "Isn’t it a wonderful life?!";

		println!("x is {}, y is {}", x, y);
	}

	{
		let x = String::from("Hello, World!");
		let y: String = String::from("Isn’t it a wonderful life?!");

		println!("x is {}, y is {}", x, y);
	}

	{
		let x = [1, 2, 3];
		let y: [i32; 3] = [4, 5, 6];

		println!("x is {}, y is {}", x[0], y[0]);
	}

	{
		let x = [1, 2, 3];
		let y: usize = x.len();

		println!("x is {}, y is {}", x[0], y);
	}

	{
		let x = vec![1, 2, 3];
		let y: Vec<i32> = [4, 5, 6].to_vec();

		println!("x is {}, y is {}", x[0], y[0]);
	}

	{
		let x = (5, '6');
		let y: (i32, char) = (7, '8');

		println!("x is {}, y is {}", x.1, y.0);
	}

	{
		let x = '5';

		if x == '5' {
			println!("X is the char '5'!");
		} else if x == '6' {
			println!("X is the char '6'!");
		} else {
			println!("I don’t know what x is.");
		}

		let y = if x == '5' { 5 } else if x == '6' { 6 } else { -1 };

		println!("y is {}", y);
	}

	{
		loop {
			println!("Looping!");
			break;
		}
	}

	{
		let mut x = 0;

		while x != 5 {
			print!("x: {}", x);
			x += 1;
		}
		
		print!("\n");
	}

	{
		let x = vec!["Hello", ",", "World", "!", "\n"];

		for element in &x {
			print!("{}", element);
		}
	}

	{
		let mut x :Vec<i32> = vec![1, 2, 3];

		push_element(&mut x);

		println!("{:?}", x);
	}

	{
		let mut c = Complex::new(3, 5);

		println!("The multiply is {}", c.multiply());
	}

	// loop {
		// println!("Hello, world!");
		// std::thread::sleep(std::time::Duration::from_secs(2));
	// };
}

fn hello_internal() {
	println!("Hello, World!");
}

fn push_element(v: &mut Vec<i32>) {
    v.push(5);
}

struct Complex {
    x: i32,
    y: u32,
	z: f32,
}

impl Complex {
    fn new(vx: i32, vy: u32) -> Complex {
        Complex {
            x: vx,
            y: vy,
			z: 0.0,
        }
    }

    fn multiply(&mut self) -> f32 {
        let vx = self.x as f32;
		let vy = self.y as f32;
        self.z = vx * vy;
		self.z
    }
}
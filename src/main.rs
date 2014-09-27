
#![feature(globs)] 
extern crate serialize;
extern crate num;

use std::num::FloatMath;

use std::from_str::FromStr;
mod sim;

fn unwrap_arg<T: FromStr>(index: uint, def:T) -> T {
	let arg: Option<T>  = FromStr::from_str(std::os::args()[index].as_slice());
	arg.unwrap_or(def)
}
fn main() {
	let args = std::os::args();
	
	/*let (a0, fin) = sim::run(unwrap_arg(1,10),unwrap_arg(2,1.0),unwrap_arg(3,10), unwrap_arg(4,6f64));
	println!("{}", fin)
	return;
	let (a0, fin) = sim::run(100,1.0,5000, 100.0);
	println!("{}", fin)
return;*/
		let mut i = 1f64;
		let tries = 20;
		while i <= 100f64 {
			let multi = true;
			let n = 30;
			let mut ctr = if multi {0u} else {tries - 1};
			let mut sum0 = 0.0f64	;
			let mut min = 2f64;
			let mut max = 0.0f64;
			while ctr < tries {
				let (a0, fin) = sim::run(n,1.0, 50000, i);
				let mut sum = 0.0;
				for  el in fin.iter() {
					sum+=el.phase;
				}
				sum = sum / n as f64;
				let (mut s, mut c) = (0f64, 0f64);
				for el in fin.iter() {
					s+=(el.phase * 3.14159/180.0).sin();
					c+=(el.phase * 3.14159/180.0).cos();
				}

				if multi {
					let f = (s*s + c*c).sqrt()/n as f64	;
					sum0 +=f;
					min = if f < min {f} else {min};
					max = if f > max {f} else {max};
				} else {
				 println!("{} {}", i, (s*s + c*c).sqrt()/100.0 as f64	);					
				}
				ctr += 1;	
			}
			if multi {println!("{} {} {} {}", i, sum0/tries as f64, min, max);}
			i += 10f64;

		}

}
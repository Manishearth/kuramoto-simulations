
use graphics::*;
use opengl_graphics::Gl;

pub fn run() {
	let mut gl = Gl::new();
	let c = Context::new(); // Back-end independent context.
	let d = c.trans(10.0, 10.0);
	d.rect(0.0, 0.0, 200.0, 100.0).rgb(0.0,0.0,0.0).draw(&mut gl);
	d.ellipse(0.0, 0.0, 200.0, 100.0).rgb(0.0,0.0,0.0).draw(&mut gl);
	let d = c.trans(20.0, 20.0);
}
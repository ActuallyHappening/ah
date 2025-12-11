use ah_tcita::veciksi;

fn main() {
	println!("Hello, world!");
}

#[veciksi(lojban = "mu'a", glico = "Example")]
pub struct Example {
	_abc: u32,
}

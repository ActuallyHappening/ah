use ah_tcita::{Ka_tcita, tcita};

fn main() {
	println!("The tcita is: {}", Example::TCITA);
}

#[tcita("su'u tcita")]
struct Example {
	pub _field1: String,
}

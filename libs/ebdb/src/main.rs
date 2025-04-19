use crate::tokenizer::Tokenizer;

pub mod error;
pub mod model;

mod tokenizer;
mod partials;

const STRING: &'static str = "table {
	id int(64),
	name char[]? \"ds\",
	price uint? null,
	is_table bool[6]
}";

fn main() -> anyhow::Result<()> {
	let tokenizer = Tokenizer::new(STRING).parse()?;
	let partial = partials::parse(tokenizer)?;
	let structs = partials::link(partial)?;
	for structure in structs {
		dbg!(&structure.get_name());
		for (name, type_info) in structure.get_fields() {
			dbg!(&name);
			dbg!(&type_info);
		}
	}
	Ok(())
}
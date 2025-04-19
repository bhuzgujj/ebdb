use crate::tokenizer::Tokenizer;

pub mod error;
pub mod model;

mod tokenizer;
mod partials;

fn main() -> anyhow::Result<()> {
	let tokenizer = Tokenizer::new("table {
	id int(64)!,
	name char[]?,
	price uint,
	is_table bool,
}").parse()?;
	let partial = partials::parse(tokenizer)?;
	for (key, value) in partial {
		dbg!(&key);
		for (k, v) in value.fields {
			dbg!(&k);
			dbg!(&v);
		}
	}
	Ok(())
}
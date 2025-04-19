pub mod error;
pub mod model;

mod tokenizer;
mod partials;

use crate::error::EbdbResult;
use crate::model::Structure;
use crate::tokenizer::Tokenizer;

pub fn to_string() {

}

pub fn from_string(str: String) -> EbdbResult<Vec<Structure>> {
	let tokens = Tokenizer::new(str.as_str()).parse()?;
	let partial = partials::parse(tokens)?;
	let structs = partials::link(partial)?;
	Ok(structs)
}
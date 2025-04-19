use crate::error::EbdbResult;
use crate::tokenizer::Token;
use std::collections::HashMap;

pub struct Structure {
	name: String,
	fields: HashMap<String, Type>
}

pub enum Type {
	// Numbers
	Integer {
		amount: usize,
		size: usize,
		default: Defaults<i128>,
		nullable: bool
	},
	UnsignedInteger {
		amount: usize,
		size: usize,
		default: Defaults<u128>,
		nullable: bool
	},
	Float {
		amount: usize,
		size: usize,
		default: Defaults<f64>,
		nullable: bool
	},

	// Text
	Text {
		amount: usize,
		default: Defaults<String>,
		nullable: bool
	},

	Boolean {
		amount: usize,
		default: Defaults<bool>,
		nullable: bool
	},

	Datatype {
		amount: usize,
		types: String,
		nullable: bool
	}
}

impl Type {
	fn from_tokens(tokens: &[Token]) -> EbdbResult<Type> {
		if let Token::Identifier(name) = &tokens[0] {

		}
		todo!()
	}
}

pub enum Defaults<T> {
	Value(T),
	Null,
	Required
}

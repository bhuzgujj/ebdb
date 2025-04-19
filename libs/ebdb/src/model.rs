use std::collections::HashMap;

pub struct Structure {
	name: String,
	fields: HashMap<String, Type>
}

impl Structure {
	pub fn new(name: String, fields: HashMap<String, Type>) -> Self {
		Self { name, fields }
	}

	pub fn get_field(&self, name: &str) -> Option<&Type> {
		self.fields.get(name)
	}

	pub fn get_fields(&self) -> &HashMap<String, Type> {
		&self.fields
	}

	pub fn get_name(&self) -> &str {
		&self.name
	}
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum Defaults<T> {
	Value(T),
	Null,
	Required
}

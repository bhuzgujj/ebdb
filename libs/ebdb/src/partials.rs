use crate::error::{EbdbError, EbdbResult};
use crate::model::{Defaults, Structure, Type};
use crate::tokenizer::Token;
use std::collections::HashMap;
use std::str::FromStr;

pub fn parse(tokens: Vec<Token>) -> EbdbResult<HashMap<String, PartialStructure>> {
	let mut partials = HashMap::new();
	let mut sname: Option<String> = None;
	for token in tokens {
		match token {
			Token::Identifier(name) => {
				if sname.is_some() {
					return Err(EbdbError::Custom(format!("Cannot parse multiple name for Structs: {}", name)));
				} else {
					sname = Some(name.clone())
				}
			}
			Token::Curly(curly) => {
				if let Some(n) = sname {
					partials.insert(n, PartialStructure::from_tokens(curly)?);
					sname = None;
				} else {
					return Err(EbdbError::Custom(format!("Structs starts with a name")));
				}
			}
			_ => return Err(EbdbError::Custom(format!("Invalid token for creation of struct: {:?}", token))),
		}
	}
	Ok(partials)
}

pub fn link(partials: HashMap<String, PartialStructure>) -> EbdbResult<Vec<Structure>> {
	let mut structures = Vec::new();
	for (name, partial) in &partials {
		let mut fields = HashMap::new();
		for (field, types) in &partial.fields {
			let new_type = match parse_type_info(&partials, field, &types) {
				Ok(value) => value,
				Err(value) => return value,
			};
			fields.insert(field.clone(), new_type);
		}
		structures.push(Structure::new(name.clone(), fields));
	}
	Ok(structures)
}

fn parse_type_info(partials: &HashMap<String, PartialStructure>, field: &String, types: &&PartialType) -> Result<Type, EbdbResult<Vec<Structure>>> {
	Ok(match types.types.as_str() {
		"int" => Type::Integer {
			amount: types.amount,
			size: types.size.unwrap_or(0),
			default: match &types.default {
				None => Defaults::Required,
				Some(default) => {
					if let Some(val) = default {
						let result = i128::from_str(val.as_str()).expect("TODO");
						Defaults::Value(result)
					} else {
						Defaults::Null
					}
				}
			},
			nullable: types.nullable,
		},
		"uint" => Type::UnsignedInteger {
			amount: types.amount,
			size: types.size.unwrap_or(0),
			default: match &types.default {
				None => Defaults::Required,
				Some(default) => {
					if let Some(val) = default {
						let result = u128::from_str(val.as_str()).expect("TODO");
						Defaults::Value(result)
					} else {
						Defaults::Null
					}
				}
			},
			nullable: types.nullable,
		},
		"float" => Type::Float {
			amount: types.amount,
			size: types.size.unwrap_or(0),
			default: match &types.default {
				None => Defaults::Required,
				Some(default) => {
					if let Some(val) = default {
						let result = f64::from_str(val.as_str()).expect("TODO");
						Defaults::Value(result)
					} else {
						Defaults::Null
					}
				}
			},
			nullable: types.nullable,
		},
		"char" => {
			if types.size.is_some() {
				return Err(Err(EbdbError::Custom(format!("String uses square braquets for they size: {}", field))));
			}
			Type::Text {
				amount: types.amount,

				default: match &types.default {
					None => Defaults::Required,
					Some(default) => {
						if let Some(val) = default {
							Defaults::Value(val.clone())
						} else {
							Defaults::Null
						}
					}
				},
				nullable: types.nullable,
			}
		},
		"bool" => {
			if types.size.is_some() {
				return Err(Err(EbdbError::Custom(format!("String uses square braquets for they size: {}", field))));
			}
			Type::Boolean {
				amount: types.amount,
				default: match &types.default {
					None => Defaults::Required,
					Some(default) => {
						if let Some(val) = default {
							let result = bool::from_str(val.as_str()).expect("TODO");
							Defaults::Value(result)
						} else {
							Defaults::Null
						}
					}
				},
				nullable: types.nullable,
			}
		},
		val => {
			if partials.contains_key(&val.to_string()) {
				Type::Datatype {
					amount: types.amount,
					types: val.to_string(),
					nullable: types.nullable,
				}
			} else {
				return Err(Err(EbdbError::Custom(format!("Type '{}' has no been defined", val))))
			}
		}
	})
}

pub struct PartialStructure {
	pub fields: HashMap<String, PartialType>,
}

impl PartialStructure {
	fn from_tokens(tokens: Vec<Token>) -> EbdbResult<Self> {
		let mut fields = HashMap::new();
		let mut begining = 0;
		let mut current = begining;
		let mut names: Option<String> = None;
		while current < tokens.len() {
			if names.is_none() {
				if let Token::Identifier(name) = &tokens[current] {
					names = Some(name.clone());
					begining += 1;
				} else {
					return Err(EbdbError::Custom("Name not found for the structs".to_string()));
				}
			} else if let Token::Commas = &tokens[current] {
				let partial_type = PartialType::from_tokens(&tokens[begining..current])?;
				if let Some(partial_type) = partial_type {
					fields.insert(names.unwrap().clone(), partial_type);
				}
				names = None;
				begining = current + 1;
			}
			current += 1;
		}
		if current > begining {
			let partial_type = PartialType::from_tokens(&tokens[begining..current])?;
			if let Some(partial_type) = partial_type {
				fields.insert(names.unwrap().clone(), partial_type);
			}
		}
		Ok(PartialStructure { fields })
	}
}

#[derive(Debug, Clone)]
pub struct PartialType {
	types: String,
	size: Option<usize>,
	default: Option<Option<String>>,
	amount: usize,
	nullable: bool
}

impl PartialType {
	fn from_tokens(tokens: &[Token]) -> EbdbResult<Option<Self>> {
		if tokens.is_empty() {
			return Ok(None);
		} else if let Token::Identifier(types) = &tokens[0] {
			let mut size: Option<usize> = None;
			let mut default: Option<Option<String>> = None;
			let mut amount: usize = 1;
			let mut nullable: bool = false;
			for token in &tokens[1..] {
				match token {
					Token::Parenthesis(size_token) => {
						if size_token.len() > 1 {
							return Err(EbdbError::Custom(format!("To many tokens for Parenthesis in types {}", types)));
						} else if size_token.len() == 1 {
							if size.is_some() {
								return Err(EbdbError::Custom(format!("To many Parenthesis in types {}", types)));
							}
							if let Token::Identifier(size_identifier) = &size_token[0] {
								match usize::from_str(size_identifier.as_str()) {
									Ok(value) => {
										size = Some(value)
									}
									Err(_) => {
										return Err(EbdbError::Custom(format!("Parenthisis is not a usize in types {}", types)));
									}
								}
							}
						} else if size.is_none() {
							size = Some(0);
						} else {
							return Err(EbdbError::Custom(format!("Cannot set size twice in types {}", types)));
						}
					},
					Token::Square(amount_token) => {
						if amount_token.len() > 1 {
							return Err(EbdbError::Custom(format!("To many tokens for Square in types {}", types)));
						} else if amount_token.len() == 1 {
							if let Token::Identifier(amount_identifier) = &amount_token[0] {
								match usize::from_str(amount_identifier.as_str()) {
									Ok(value) => {
										amount = value
									}
									Err(_) => {
										return Err(EbdbError::Custom(format!("Square is not a usize in types {}", types)));
									}
								}
							}
						} else {
							amount = 0;
						}
					}
					Token::ExclamationMark => {
						if nullable {
							return Err(EbdbError::Custom(format!("One ExclamationMark maximum in types {}", types)));
						} else {
							// ..
						}
					},
					Token::QuestionMark => {
						if default.is_some() {
							return Err(EbdbError::Custom(format!("One QuestionMark maximum in types {}", types)));
						} else {
							nullable = true;
						}
					},
					Token::Quoted(default_token) => {
						default = Some(Some(default_token.clone()))
					},
					Token::Identifier(str) => {
						match str.to_lowercase().as_str() {
							"null" => {
								default = Some(None);
							}
							_ => {}
						}
					}
					_ => return Err(EbdbError::Custom(format!("Invalid token for creation of types {}: {:?}", types, token))),
				}
			}
			Ok(Some(Self {
				types: types.clone(),
				size,
				default,
				amount,
				nullable
			}))
		} else {
			Err(EbdbError::Custom("The first token should be an identifier".to_string()))
		}
	}
}
use std::{collections::HashMap, fmt::format};

/// Represents a pronumeral in an equation.
/// Guarenteed to be unique.
pub struct Pronumeral {
		pub human_char: char,
		id: Id,
}

pub struct PronumeralStore {
		pronumerals: HashMap<Id, Pronumeral>
}

mod store;

use id::Id;
mod id {
	#[derive(Hash, Eq, PartialEq)]
	pub(super) struct Id(String);

	impl Id {
			fn new(id: &str) -> Self {
					Self(format!("<{}>", id))
			}
	}
}
use std::collections::HashMap;

use maplit::hashmap;

use super::PronumeralStore;

impl Default for PronumeralStore {
	fn default() -> Self {
		let pronumerals = hashmap! {
			// Id::new("constant-e-2.7") => 
		};

		Self {
			pronumerals,
		}
	}
}
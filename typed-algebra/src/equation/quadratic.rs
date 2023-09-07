use crate::fraction::FractionI32;

/// A quadratic, generic over its form.
pub struct Quadratic<Form> {
	form: Form
}

/// ax^2 + bx + c = 0
pub struct StandardForm {
	second_degree_coefficient: FractionI32,
	first_degree_coefficient: FractionI32,
	constant: FractionI32,
}
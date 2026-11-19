//! `postcard_schema::Schema` implementation tests.

use crate::postcard_schema::Schema;
use crate::si::f32::{ElectricCurrent, Length};

#[test]
fn quantity_schema_is_the_stored_value_schema() {
    assert_eq!(Length::SCHEMA, f32::SCHEMA);
    assert_eq!(ElectricCurrent::SCHEMA, f32::SCHEMA);
}

use super::base::Base;

use super::super::enums::TaskStatus;
use super::super::traits::create::Create;
use super::super::traits::edit::Edit;
use super::super::traits::get::Get;

impl Create for Pending {}
impl Get for Pending {}
impl Edit for Pending {}

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING,
        };
        return Self { super_struct: base };
    }
}

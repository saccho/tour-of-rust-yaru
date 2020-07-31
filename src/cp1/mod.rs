use crate::runner::Runner;

mod arrays;
mod basic_type_conversion;
mod basic_types;
mod changing_variables;
mod constants;
mod variables;

pub struct Capter1;

impl Runner for Capter1 {
    fn run(&self) {
        variables::run();
        changing_variables::run();
        basic_types::run();
        basic_type_conversion::run();
        constants::run();
        arrays::run();
    }
}

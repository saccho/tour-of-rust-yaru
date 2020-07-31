use crate::runner::Runner;

mod arrays;
mod basic_type_conversion;
mod basic_types;
mod changing_variables;
mod constants;
mod functions;
mod multiple_return_values;
mod returning_nothing;
mod variables;

pub struct Chapter1;

impl Runner for Chapter1 {
    fn run(&self) {
        variables::run();
        changing_variables::run();
        basic_types::run();
        basic_type_conversion::run();
        constants::run();
        arrays::run();
        functions::run();
        multiple_return_values::run();
        returning_nothing::run();
    }
}

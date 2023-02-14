use derive_more::{Deref, Display};

#[derive(Clone, Deref, Debug, Display)]
pub struct CourseCode(String);

impl CourseCode {
    pub fn new(code: String) -> CourseCode {
        CourseCode(code.to_uppercase())
    }
}

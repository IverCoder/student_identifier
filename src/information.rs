use std::fmt;

/// Types of genders available for a student.
/// 
/// Male, female, and non-binary options are offered.
/// 
/// # Example
/// ```rust
/// use student_identifier::information::Gender;
/// let student_gender = Gender::NonBinary;
/// ```
pub enum Gender {
    Male, Female, NonBinary,
}

impl fmt::Display for Gender {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Self::Male => write!(f, "Male"),
            Self::Female => write!(f, "Female"),
            Self::NonBinary => write!(f, "Non-binary"),
        }
    }
}
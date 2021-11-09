use std::fmt;

pub struct PetsError;

impl fmt::Display for PetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "An Error Occurred, Please Try Again!") // user-facing output
    }
}

impl fmt::Debug for PetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

impl From<sled_extensions::Error> for PetsError {
    fn from(_error: sled_extensions::Error) -> Self {
        PetsError
    }
}

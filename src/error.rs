pub enum ParseError {
    InvalidLength { expected: usize, actual: usize },
    FieldOutOfBound { field: &'static str, position: (usize, usize), max: usize }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidLength { expected, actual }=> write!(f, "Invalid length parser, expected {expected}, got {actual}"),
            ParseError::FieldOutOfBound { field, position, max } => write!(f, "Field {field} out of bound: {position:?}/{max}"),
        }
    }
}

impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}


impl std::error::Error for ParseError {}

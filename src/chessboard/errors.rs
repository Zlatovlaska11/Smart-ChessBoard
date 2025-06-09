pub mod chess_errors {
    use std::fmt::{self};

    #[derive(Debug, Clone)]
    pub enum ErrorType {
        InvalidMove,
        InvalidMoveStructure,
        WrongAnswearPuzzle,
    }

    #[derive(Debug, Clone)]
    pub struct MoveError {
        pub error_type: ErrorType,
    }

    impl fmt::Display for MoveError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self.error_type {
                ErrorType::InvalidMove => write!(f, "Invalid Move"),
                ErrorType::InvalidMoveStructure => write!(f, "Invalid Move structure"),
                ErrorType::WrongAnswearPuzzle => write!(f, "Wrong answer"),
            }
        }
    }
}

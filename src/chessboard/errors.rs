pub mod chess_errors {
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct MoveStructureError;

    impl fmt::Display for MoveStructureError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Invalid Move structure")
        }
    }
}

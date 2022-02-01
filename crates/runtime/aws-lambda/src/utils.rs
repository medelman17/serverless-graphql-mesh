pub mod environment {
    pub fn is_lambda() -> bool {
        std::env::var("AWS_LAMBDA_RUNTIME_API").is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_detects_lambda() {
        std::env::set_var("AWS_LAMBDA_RUNTIME_API", "yes");
        let result = environment::is_lambda();
        assert_eq!(result, true);
    }

    #[test]
    fn it_detects_not_lambda() {
        std::env::remove_var("AWS_LAMBDA_RUNTIME_API");
        let result = environment::is_lambda();
        assert_eq!(result, false);
    }
}

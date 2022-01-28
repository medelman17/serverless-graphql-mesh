const AWS_LAMBDA_RUNTIME_API_ENV_VAR_KEY: &str = "AWS_LAMBDA_RUNTIME_API";

pub fn is_lambda() -> bool {
    std::env::var(AWS_LAMBDA_RUNTIME_API_ENV_VAR_KEY).is_ok()
}

#[cfg(test)]
mod tests {
    use super::is_lambda;

    #[test]
    fn it_detects_lambda_runtime_from_env_when_set() {
        std::env::set_var(super::AWS_LAMBDA_RUNTIME_API_ENV_VAR_KEY, "yes");
        assert!(is_lambda());
    }

    #[test]
    fn it_detects_lambda_runtime_from_env_when_unset() {
        std::env::remove_var(super::AWS_LAMBDA_RUNTIME_API_ENV_VAR_KEY);
        assert!(!is_lambda());
    }
}

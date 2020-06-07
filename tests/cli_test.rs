extern crate assert_cli;

mod cli_test {
    #[test]
    fn default_invocation() {
        assert_cli::Assert::main_binary()
            .stdout()
            .satisfies(|p| p.chars().count() == 16, "Invocation successful")
            .unwrap();
    }

    #[test]
    fn successful_invocation_with_custom_length() {
        assert_cli::Assert::main_binary()
            .with_args(&["-l", "24"])
            .stdout()
            .satisfies(|p| p.chars().count() == 24, "Custom invocation successful")
            .unwrap();
    }

    #[test]
    fn max_length_override_test() {
        assert_cli::Assert::main_binary()
            .with_args(&["-l", "420", "-m", "1024"])
            .stdout()
            .satisfies(|p| p.chars().count() == 420, "Max length overriden")
            .unwrap();
    }

    #[test]
    fn invocation_defaults_to_max_length() {
        assert_cli::Assert::main_binary()
            .with_args(&["-l", "420"])
            .stdout()
            .satisfies(|p| p.chars().count() == 128, "Default max length")
            .unwrap();
    }
}

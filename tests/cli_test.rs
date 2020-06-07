use std::process::Command;

mod cli_test {
    #[test]
    fn proper_invocation() {
        let output = std::process::Command::new("./target/release/pgen")
            .output()
            .expect("Failed to invoke process");
        if let Ok(pwd) = String::from_utf8(output.stdout) {
            println!("{:#?}", (&pwd, pwd.chars().count()));
            assert_eq!(pwd.chars().count(), 16);
        } else {
            panic!("Invalid output generated!");
        };        
    }
}

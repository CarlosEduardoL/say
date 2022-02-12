#[cfg(test)]
mod test {
    use assert_cmd::Command;
    use std::process::Command as CMD;
    use predicates::prelude::predicate;
    static MESSAGE: &str = "\\nHello \\v\\t World!\\a\\c please stop";


    #[test]
    fn say_new_line_no_interpretation() {
        let echo = CMD::new("echo")
            .arg(MESSAGE)
            .output()
            .expect("failed running echo");
        let mut say = Command::cargo_bin("say").unwrap();
        say.arg(MESSAGE).assert().success().stdout(predicate::eq(echo.stdout.as_slice()));
    }

    #[test]
    fn say_no_new_line_no_interpretation() {
        let echo = CMD::new("echo")
            .arg("-n")
            .arg(MESSAGE)
            .output()
            .expect("failed running echo");
        let mut say = Command::cargo_bin("say").unwrap();
        say.arg("-n").arg(MESSAGE).assert().success().stdout(predicate::eq(echo.stdout.as_slice()));
    }
    #[test]
    fn say_new_line_interpretation() {
        let echo = CMD::new("echo")
            .arg("-e")
            .arg(MESSAGE)
            .output()
            .expect("failed running echo");
        let mut say = Command::cargo_bin("say").unwrap();
        say.arg("-e").arg(MESSAGE).assert().success().stdout(predicate::eq(echo.stdout.as_slice()));
    }

    #[test]
    fn say_no_new_line_interpretation() {
        let echo = CMD::new("echo")
            .arg("-e")
            .arg("-n")
            .arg(MESSAGE)
            .output()
            .expect("failed running echo");
        let mut say = Command::cargo_bin("say").unwrap();
        say.arg("-e").arg("-n").arg(MESSAGE).assert().success().stdout(predicate::eq(echo.stdout.as_slice()));
    }
}


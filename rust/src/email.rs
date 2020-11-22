pub mod email {}
#[cfg(test)]
mod tests {
    mod email;
    use std::io;
    #[test]
    fn test_send_mail() {
        let mail = Mail {
            to: String::from("Swann"),
            from: String::from("Moi"),
            subject: String::from("test"),
            body: String::from("test-body"),
        };
        Mailer {}.send_email(mail);
        let mut buffer = String::new();
        let reader = io::stdin();
        reader.read_to_string(&mut buffer);
        println!("{}", buffer);
        let expected_result = String::from(
            "Sending email to : Swann\nsubject: test\nBody: test-body\n-------------------------",
        );
        assert_eq!(buffer, expected_result);
    }
}

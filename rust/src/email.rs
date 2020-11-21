use std::io;

mod email {

    struct Mailer {}

    struct Mail {
        to: String,
        from: String,
        subject: String,
        body: String,
    }

    impl Mail {
        fn send_email(&self) {
            println!("Sending email to : {}", self.to);
            println!("subject: {}", self.subject);
            println!("Body: {}", self.body);
            println!("-------------------------");
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_send_mail() {
        let mail = Mail {
            to: String::from("Swann"),
            from: String::from("Moi"),
            subject: String::from("test"),
            body: String::from("test-body"),
        };
        mail.send_email();
        let mut buffer = String::new();
        let reader = io::stdin();
        reader.read_to_string(&mut buffer);
        println!("{}", buffer);
    }
}

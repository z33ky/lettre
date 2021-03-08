use lettre::{transport::smtp::authentication::Credentials, message::Mailbox, Message, SmtpTransport, Transport};

fn main() {
    tracing_subscriber::fmt::init();

    let email = Message::builder()
        .from("NoBody <nobody@domain.tld>".parse::<Mailbox>().unwrap())
        .reply_to("Yuin <yuin@domain.tld>".parse::<Mailbox>().unwrap())
        .to("Hei <hei@domain.tld>".parse::<Mailbox>().unwrap())
        .subject("Happy new year")
        .body(String::from("Be happy!"))
        .unwrap();

    let creds = Credentials::new("smtp_username".to_string(), "smtp_password".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}

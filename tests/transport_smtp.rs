#[cfg(test)]
#[cfg(all(feature = "smtp-transport", feature = "builder"))]
mod test {
    use lettre::{message::Mailbox, Message, SmtpTransport, Transport};

    #[test]
    fn smtp_transport_simple() {
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse::<Mailbox>().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse::<Mailbox>().unwrap())
            .to("Hei <hei@domain.tld>".parse::<Mailbox>().unwrap())
            .subject("Happy new year")
            .body(String::from("Be happy!"))
            .unwrap();
        SmtpTransport::builder_dangerous("127.0.0.1")
            .port(2525)
            .build()
            .send(&email)
            .unwrap();
    }
}

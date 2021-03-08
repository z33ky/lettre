#[cfg(test)]
#[cfg(all(feature = "sendmail-transport", feature = "builder"))]
mod test {
    use lettre::{transport::sendmail::SendmailTransport, message::Mailbox, Message};

    #[cfg(feature = "tokio02")]
    use tokio02_crate as tokio;

    #[test]
    fn sendmail_transport() {
        use lettre::Transport;
        let sender = SendmailTransport::new();
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse::<Mailbox>().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse::<Mailbox>().unwrap())
            .to("Hei <hei@domain.tld>".parse::<Mailbox>().unwrap())
            .subject("Happy new year")
            .body(String::from("Be happy!"))
            .unwrap();

        let result = sender.send(&email);
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[cfg(feature = "async-std1")]
    #[async_std::test]
    async fn sendmail_transport_asyncstd1() {
        use lettre::{AsyncSendmailTransport, AsyncStd1Executor, AsyncTransport};

        let sender = AsyncSendmailTransport::<AsyncStd1Executor>::new();
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse::<Mailbox>().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse::<Mailbox>().unwrap())
            .to("Hei <hei@domain.tld>".parse::<Mailbox>().unwrap())
            .subject("Happy new year")
            .date("Tue, 15 Nov 1994 08:12:31 GMT".parse().unwrap())
            .body(String::from("Be happy!"))
            .unwrap();

        let result = sender.send(email).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }

    #[cfg(feature = "tokio02")]
    #[tokio::test]
    async fn sendmail_transport_tokio02() {
        use lettre::{AsyncSendmailTransport, Tokio02Executor, Tokio02Transport};

        let sender = AsyncSendmailTransport::<Tokio02Executor>::new();
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse::<Mailbox>().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse::<Mailbox>().unwrap())
            .to("Hei <hei@domain.tld>".parse::<Mailbox>().unwrap())
            .subject("Happy new year")
            .date("Tue, 15 Nov 1994 08:12:31 GMT".parse().unwrap())
            .body(String::from("Be happy!"))
            .unwrap();

        let result = sender.send(email).await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}

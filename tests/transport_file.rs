#[cfg(test)]
#[cfg(all(feature = "file-transport", feature = "builder"))]
mod test {
    use lettre::{transport::file::FileTransport, message::Mailbox, Message};
    use std::{
        env::temp_dir,
        fs::{read_to_string, remove_file},
    };

    #[cfg(feature = "tokio02")]
    use tokio02_crate as tokio;

    #[test]
    fn file_transport() {
        use lettre::Transport;
        let sender = FileTransport::new(temp_dir());
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse::<Mailbox>().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse::<Mailbox>().unwrap())
            .to("Hei <hei@domain.tld>".parse::<Mailbox>().unwrap())
            .subject("Happy new year")
            .date("Tue, 15 Nov 1994 08:12:31 GMT".parse().unwrap())
            .body(String::from("Be happy!"))
            .unwrap();

        let result = sender.send(&email);
        let id = result.unwrap();

        let eml_file = temp_dir().join(format!("{}.eml", id));
        let eml = read_to_string(&eml_file).unwrap();

        assert_eq!(
            eml,
            concat!(
                "From: NoBody <nobody@domain.tld>\r\n",
                "Reply-To: Yuin <yuin@domain.tld>\r\n",
                "To: Hei <hei@domain.tld>\r\n",
                "Subject: Happy new year\r\n",
                "Date: Tue, 15 Nov 1994 08:12:31 GMT\r\n",
                "Content-Transfer-Encoding: 7bit\r\n",
                "\r\n",
                "Be happy!"
            )
        );
        remove_file(eml_file).unwrap();
    }

    #[test]
    #[cfg(feature = "file-transport-envelope")]
    fn file_transport_with_envelope() {
        use lettre::Transport;
        let sender = FileTransport::with_envelope(temp_dir());
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse::<Mailbox>().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse::<Mailbox>().unwrap())
            .to("Hei <hei@domain.tld>".parse::<Mailbox>().unwrap())
            .subject("Happy new year")
            .date("Tue, 15 Nov 1994 08:12:31 GMT".parse().unwrap())
            .body(String::from("Be happy!"))
            .unwrap();

        let result = sender.send(&email);
        let id = result.unwrap();

        let eml_file = temp_dir().join(format!("{}.eml", id));
        let eml = read_to_string(&eml_file).unwrap();

        let json_file = temp_dir().join(format!("{}.json", id));
        let json = read_to_string(&json_file).unwrap();

        assert_eq!(
            eml,
            concat!(
                "From: NoBody <nobody@domain.tld>\r\n",
                "Reply-To: Yuin <yuin@domain.tld>\r\n",
                "To: Hei <hei@domain.tld>\r\n",
                "Subject: Happy new year\r\n",
                "Date: Tue, 15 Nov 1994 08:12:31 GMT\r\n",
                "Content-Transfer-Encoding: 7bit\r\n",
                "\r\n",
                "Be happy!"
            )
        );

        assert_eq!(
            json,
            "{\"forward_path\":[\"hei@domain.tld\"],\"reverse_path\":\"nobody@domain.tld\"}"
        );

        let (e, m) = sender.read(&id).unwrap();

        assert_eq!(&e, email.envelope());
        assert_eq!(m, email.formatted());

        remove_file(eml_file).unwrap();
        remove_file(json_file).unwrap();
    }

    #[cfg(feature = "async-std1")]
    #[async_std::test]
    async fn file_transport_asyncstd1() {
        use lettre::{AsyncFileTransport, AsyncStd1Executor, AsyncTransport};

        let sender = AsyncFileTransport::<AsyncStd1Executor>::new(temp_dir());
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse::<Mailbox>().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse::<Mailbox>().unwrap())
            .to("Hei <hei@domain.tld>".parse::<Mailbox>().unwrap())
            .subject("Happy new year")
            .date("Tue, 15 Nov 1994 08:12:31 GMT".parse().unwrap())
            .body(String::from("Be happy!"))
            .unwrap();

        let result = sender.send(email).await;
        let id = result.unwrap();

        let eml_file = temp_dir().join(format!("{}.eml", id));
        let eml = read_to_string(&eml_file).unwrap();

        assert_eq!(
            eml,
            concat!(
                "From: NoBody <nobody@domain.tld>\r\n",
                "Reply-To: Yuin <yuin@domain.tld>\r\n",
                "To: Hei <hei@domain.tld>\r\n",
                "Subject: Happy new year\r\n",
                "Date: Tue, 15 Nov 1994 08:12:31 GMT\r\n",
                "Content-Transfer-Encoding: 7bit\r\n",
                "\r\n",
                "Be happy!"
            )
        );
        remove_file(eml_file).unwrap();
    }

    #[cfg(feature = "tokio02")]
    #[tokio::test]
    async fn file_transport_tokio02() {
        use lettre::{AsyncFileTransport, AsyncTransport, Tokio02Executor};

        let sender = AsyncFileTransport::<Tokio02Executor>::new(temp_dir());
        let email = Message::builder()
            .from("NoBody <nobody@domain.tld>".parse::<Mailbox>().unwrap())
            .reply_to("Yuin <yuin@domain.tld>".parse::<Mailbox>().unwrap())
            .to("Hei <hei@domain.tld>".parse::<Mailbox>().unwrap())
            .subject("Happy new year")
            .date("Tue, 15 Nov 1994 08:12:31 GMT".parse().unwrap())
            .body(String::from("Be happy!"))
            .unwrap();

        let result = sender.send(email).await;
        let id = result.unwrap();

        let eml_file = temp_dir().join(format!("{}.eml", id));
        let eml = read_to_string(&eml_file).unwrap();

        assert_eq!(
            eml,
            concat!(
                "From: NoBody <nobody@domain.tld>\r\n",
                "Reply-To: Yuin <yuin@domain.tld>\r\n",
                "To: Hei <hei@domain.tld>\r\n",
                "Subject: Happy new year\r\n",
                "Date: Tue, 15 Nov 1994 08:12:31 GMT\r\n",
                "Content-Transfer-Encoding: 7bit\r\n",
                "\r\n",
                "Be happy!"
            )
        );
        remove_file(eml_file).unwrap();
    }
}

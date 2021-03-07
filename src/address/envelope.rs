#[cfg(feature = "builder")]
use std::convert::TryFrom;

use super::Address;
#[cfg(feature = "builder")]
use crate::message::header::{self, Headers};
#[cfg(feature = "builder")]
use crate::message::{header::To, Mailboxes};
use crate::Error;
use hyperx::header::Header as _;

/// Simple email envelope representation
///
/// We only accept mailboxes, and do not support source routes (as per RFC).
#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Envelope {
    /// The envelope recipient's addresses
    ///
    /// This can not be empty.
    forward_path: Vec<Address>,
    /// The envelope sender address
    reverse_path: Option<Address>,
}

impl Envelope {
    /// Creates a new envelope, which may fail if `to` is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// # use lettre::Address;
    /// # use lettre::address::Envelope;
    ///
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sender = Address::from_str("from@email.com")?;
    /// let recipients = vec![Address::from_str("to@email.com")?];
    ///
    /// let envelope = Envelope::new(Some(sender), recipients);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// If `to` has no elements in it.
    pub fn new(from: Option<Address>, to: Vec<Address>) -> Result<Envelope, Error> {
        if to.is_empty() {
            return Err(Error::MissingHeader { header_name: To::header_name() });
        }
        Ok(Envelope {
            forward_path: to,
            reverse_path: from,
        })
    }

    /// Gets the destination addresses of the envelope.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// # use lettre::Address;
    /// # use lettre::address::Envelope;
    ///
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sender = Address::from_str("from@email.com")?;
    /// let recipients = vec![Address::from_str("to@email.com")?];
    ///
    /// let envelope = Envelope::new(Some(sender), recipients.clone())?;
    /// assert_eq!(envelope.to(), recipients.as_slice());
    /// # Ok(())
    /// # }
    /// ```
    pub fn to(&self) -> &[Address] {
        self.forward_path.as_slice()
    }

    /// Gets the sender of the envelope.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// # use lettre::Address;
    /// # use lettre::address::Envelope;
    ///
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let sender = Address::from_str("from@email.com")?;
    /// let recipients = vec![Address::from_str("to@email.com")?];
    ///
    /// let envelope = Envelope::new(Some(sender), recipients.clone())?;
    /// assert!(envelope.from().is_some());
    ///
    /// let senderless = Envelope::new(None, recipients.clone())?;
    /// assert!(senderless.from().is_none());
    /// # Ok(())
    /// # }
    /// ```
    pub fn from(&self) -> Option<&Address> {
        self.reverse_path.as_ref()
    }

    /// Check if any of the addresses in the envelope contains non-ascii chars
    pub(crate) fn has_non_ascii_addresses(&self) -> bool {
        self.reverse_path
            .iter()
            .chain(self.forward_path.iter())
            .any(|a| !a.is_ascii())
    }
}

#[cfg(feature = "builder")]
impl TryFrom<&Headers> for Envelope {
    type Error = Error;

    fn try_from(headers: &Headers) -> Result<Self, Self::Error> {
        let from = match headers.get::<header::Sender>() {
            // If there is a Sender, use it
            Some(header::Sender(a)) => Some(a.email.clone()),
            // ... else try From
            None => match headers.get::<header::From>() {
                Some(header::From(a)) => {
                    if a.len() > 1 {
                        return Err(Error::TooManyFrom);
                    }
                    a.iter().next().map(|mailbox| mailbox.email.clone())
                }
                None => None,
            },
        };

        fn add_addresses_from_mailboxes(
            addresses: &mut Vec<Address>,
            mailboxes: Option<&Mailboxes>,
        ) {
            if let Some(mailboxes) = mailboxes {
                for mailbox in mailboxes.iter() {
                    addresses.push(mailbox.email.clone());
                }
            }
        }
        let mut to = vec![];
        add_addresses_from_mailboxes(&mut to, headers.get::<header::To>().map(|h| &h.0));
        add_addresses_from_mailboxes(&mut to, headers.get::<header::Cc>().map(|h| &h.0));
        add_addresses_from_mailboxes(&mut to, headers.get::<header::Bcc>().map(|h| &h.0));

        Self::new(from, to)
    }
}

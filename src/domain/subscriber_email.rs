use validator::validate_email;

#[derive(Debug)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<Self, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{s} is not a valid email"))
        }
    }
}

#[cfg(test)]
mod test {
    use claim::assert_err;

    use super::SubscriberEmail;

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_owned();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "nameexample.com".to_owned();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn email_missing_domain_is_rejected() {
        let email = "name@".to_owned();
        assert_err!(SubscriberEmail::parse(email));
    }
}

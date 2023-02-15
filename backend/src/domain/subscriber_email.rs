use validator::validate_email;

#[derive(Debug, Clone)]
pub struct SubscriberEmail(String);

impl SubscriberEmail {
    pub fn parse(s: String) -> Result<SubscriberEmail, String> {
        if validate_email(&s) {
            Ok(Self(s))
        } else {
            Err(format!("{} is not valid subscriber email.", s))
        }
    }
}

impl AsRef<str> for SubscriberEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::SubscriberEmail;
    use claims::assert_err;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: quickcheck::Gen>(g: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(g);
            Self(email)
        }

        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            quickcheck::empty_shrinker()
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed_successfully(valid_email: ValidEmailFixture) -> bool {
        SubscriberEmail::parse(valid_email.0).is_ok()
    }

    #[test]
    fn empty_emails_are_rejected() {
        let email = "".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn emails_missing_at_symbol_are_rejected() {
        let email = "jakevalink.io".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }

    #[test]
    fn emails_missing_subject_are_rejected() {
        let email = "@valink.io".to_string();
        assert_err!(SubscriberEmail::parse(email));
    }
}

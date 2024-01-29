use unicode_segmentation::UnicodeSegmentation;

pub struct NewSubscriber {
    pub name: SubscriberName,
    pub email: String,
}

#[derive(serde::Deserialize, Debug)]
pub struct SubscriberName(String);

impl SubscriberName {
    pub fn parse(s: String) -> Result<Self, String> {
        let is_empty = s.trim().is_empty();
        let is_too_long = s.graphemes(true).count() > 256;
        let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
        let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty || is_too_long || contains_forbidden_characters {
            Err(format!("Invalid name provided: {s}"))
        } else {
            Ok(Self(s))
        }
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod test {
    use claim::{assert_ok, assert_err};

    use super::SubscriberName;

    #[test]
    pub fn a_256_grapheme_long_name_is_valid() {
        let name = "å".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    pub fn a_name_longer_than_256_grapheme_is_rejected() {
        let name = "a".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }
}

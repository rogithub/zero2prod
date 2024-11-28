use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug)]
pub struct SubscriberName(String);

#[derive(Debug)]
pub struct NewSubscriber {
    pub email: String,
    pub name: SubscriberName
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SubscriberName {
/*
    pub fn inner(self) -> String {
        // The caller gets the inner string
        // but they do not have a SubscriberName anymore!
        // That's because ´inner´ takes ´self´ by value,
        // consuming it according to move semantics.
        self.0
    }

    pub fn inner_mut(&mut self) -> &mut String {
        // The caller gets a mutable reference to the inner string.
        // This allows them to perform *arbitrary* changes to 
        // value itself, potentially breaking our invariants!
        &mut self.0
    }

    // same as AsRef<str> avove
    pub fn inner_ref(&self) -> &str {
        // The caller gets a shared reference to the inner string.
        // Thid gives the caller **read-only** access,
        // they have no way to compromise our invariants
        &self.0
    }
*/

    /// Returns an instance of SubscriberName if the input satisfies all
    /// our validation constraints on subscriber names.
    pub fn parse(s: String) -> Result<SubscriberName, String> {
        let is_empty_or_whitespace = s.trim().is_empty();

        // A grapheme is defined by the unicode standard as "user-percived"
        // character: Ä is a single grapheme, but it is composed of two characters.
        //
        // graphemes returns an iterator over the graphemes in the input s.
        // true specifies that we want to use the extended grapheme definition set,
        // the recommended one.
        let is_too_long = s.graphemes(true).count() > 256;

        // Iterate over all chars in the input s to check if any of them matches
        // one of the characters in the forbidden array.
        let forbidden_characters = ['/','(', ')', '"', '<', '>', '\\', '{', '}' ];
        let contains_forbidden_characters = s
            .chars()
            .any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            Err(format!("{} is not a valid subscriber name.", s))
        } else {
            Ok(Self(s))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberName;
    use claims::{assert_err, assert_ok};

    #[test]
    fn a_256_grapheme_long_name_is_valid() {
        let name = "ë".repeat(256);
        assert_ok!(SubscriberName::parse(name));
    }

    #[test]
    fn a_name_longer_than_256_grapheme_is_rejected() {
        let name = "ë".repeat(257);
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn white_space_only_names_are_rejected() {
        let name = " ".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn empty_string_is_rejected() {
        let name = "".to_string();
        assert_err!(SubscriberName::parse(name));
    }

    #[test]
    fn names_containing_an_invalid_character_are_rejected() {
        for name in &['/','(', ')', '"', '<', '>', '\\', '{', '}' ] {
            let name = name.to_string();
            assert_err!(SubscriberName::parse(name));    
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Úrsula Le Guin".to_string();
        assert_ok!(SubscriberName::parse(name));
    }
}
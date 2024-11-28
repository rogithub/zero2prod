use std::string;

use unicode_segmentation::UnicodeSegmentation;

pub struct SubscriberName(String);

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
        let is_too_long = s.graphemes(true).count() > 255;

        // Iterate over all chars in the input s to check if any of them matches
        // one of the characters in the forbidden array.
        let forbidden_characters = ['/','(', ')', '"', '<', '>', '\\', '{', '}' ];
        let contains_forbidden_characters = s
            .chars()
            .any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace || is_too_long || contains_forbidden_characters {
            panic!("{} is not a valid subscriber name.", s)
        } else {
            Ok(Self(s))
        }
    }
}
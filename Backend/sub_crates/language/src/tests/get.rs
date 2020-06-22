#[cfg(test)]
mod tests {
    use crate::domain_value::Language;
    use crate::material::Dictionary;
    use crate::tools::Get;
    use crate::tools::Register;

    #[test]
    #[should_panic]
    fn key_not_registered() {
        let dictionary = Dictionary::default();
        dictionary.get("Test", Language::English);
    }

    #[test]
    #[should_panic]
    fn language_not_registered() {
        let dictionary = Dictionary::default();
        dictionary.register("Test", Language::English, "test");
        dictionary.get("Test", Language::German);
    }

    #[test]
    #[should_panic]
    fn language_not_registered_because_empty() {
        let dictionary = Dictionary::default();
        dictionary.register("Test", Language::Japanese, "test");
        dictionary.get("Test", Language::English);
    }

    #[test]
    fn language_exists() {
        let dictionary = Dictionary::default();
        dictionary.register("Test", Language::Japanese, "test");
        assert_eq!(dictionary.get("Test", Language::Japanese), "test")
    }
    #[test]
    #[should_panic(expected = "Value is empty for key 'Test' and language 0")]
    fn has_empty_translation() {
        let dictionary = Dictionary::default();
        dictionary.register("Test", Language::English, "");
        dictionary.get("Test", Language::English);
    }

    /*
    Tests derived by input space partitioning with the following characteristics:
    empty/nonempty key
    key pure ASCII or with at least one non-ASCII character
    0, 1, >1 translations
    translation for requested is available or not
    translation is valid or not (empty string or nonempty string)
     */
    #[test]
    #[should_panic(expected = "Key '' is not registered")]
    fn keyEmpty_0Translations() {
        let dictionary = Dictionary::default();
        dictionary.get("", Language::English);
    }

    #[test]
    #[should_panic(expected = "Key 'äöüß' is not registered")]
    fn keyNonEmpty_nonAscii_0Translations() {
        let dictionary = Dictionary::default();
        dictionary.get("äöüß", Language::English);
    }

    #[test]
    #[should_panic(expected = "Key 'Test' is not registered")]
    fn keyNonEmpty_ascii_0Translations() {
        let dictionary = Dictionary::default();
        dictionary.get("Test", Language::English);
    }

    #[test]
    fn keyEmpty_1Translations_available_valid() {
        let dictionary = Dictionary::default();
        dictionary.register("", Language::English, "test");
        assert_eq!(dictionary.get("", Language::English), "test");
    }

    #[test]
    #[should_panic(expected = "Key 'Test' is not registered for language 0")]
    fn keyNonEmpty_ascii_1Translations_NotAvailable() {
        let dictionary = Dictionary::default();
        dictionary.register("Test", Language::Japanese, "");
        dictionary.get("Test", Language::English);
    }

    #[test]
    #[should_panic(expected = "Value is empty for key 'Test' and language 0")]
    fn keyNonEmpty_ascii_2Translations_available_invalid() {
        let dictionary = Dictionary::default();
        dictionary.register("Test", Language::Japanese, "test_jp");
        dictionary.register("Test", Language::English, "");
        dictionary.get("Test", Language::English);
    }

    #[test]
    #[should_panic(expected = "Value is empty for key '' and language 0")]
    fn keyEmpty_2Translations_available_invalid() {
        let dictionary = Dictionary::default();
        dictionary.register("", Language::Japanese, "test_jp");
        dictionary.register("", Language::English, "");
        dictionary.get("", Language::English);
    }

    #[test]
    #[should_panic(expected = "Value is empty for key 'äöü' and language 0")]
    fn keyNonEmpty_nonAscii_2Translations_available_invalid() {
        let dictionary = Dictionary::default();
        dictionary.register("äöü", Language::Japanese, "test_jp");
        dictionary.register("äöü", Language::English, "");
        dictionary.get("äöü", Language::English);
    }

    #[test]
    #[should_panic(expected = "Key 'Test' is not registered for language 0")]
    fn keyNonEmpty_ascii_2Translations_notAvailable() {
        let dictionary = Dictionary::default();
        dictionary.register("Test", Language::Japanese, "test_jp");
        dictionary.register("Test", Language::German, "test_ge");
        dictionary.get("Test", Language::English);
    }

    #[test]
    fn keyNonEmpty_ascii_2Translations_available_valid() {
        let dictionary = Dictionary::default();
        dictionary.register("Test", Language::Japanese, "test_jp");
        dictionary.register("Test", Language::English, "test_en");
        assert_eq!(dictionary.get("Test", Language::English), "test_en");
    }

    #[test]
    #[should_panic(expected = "Key 'äöü' is not registered for language 0")]
    fn keyNonEmpty_nonAscii_1Translations_notAvailable() {
        let dictionary = Dictionary::default();
        dictionary.register("äöü", Language::Japanese, "test_jp");
        dictionary.get("äöü", Language::English);
    }

    #[test]
    #[should_panic(expected = "Key 'äöü' is not registered for language 0")]
    fn keyNonEmpty_nonAscii_2Translations_notAvailable() {
        let dictionary = Dictionary::default();
        dictionary.register("äöü", Language::Japanese, "test_jp");
        dictionary.register("äöü", Language::German, "test_ge");
        dictionary.get("äöü", Language::English);
    }

    #[test]
    #[should_panic(expected = "Key '' is not registered for language 0")]
    fn keyEmpty_1Translations_notAvailable() {
        let dictionary = Dictionary::default();
        dictionary.register("", Language::Japanese, "test_jp");
        dictionary.get("", Language::English);
    }

    #[test]
    #[should_panic(expected = "Key '' is not registered for language 0")]
    fn keyEmpty_2Translations_notAvailable() {
        let dictionary = Dictionary::default();
        dictionary.register("", Language::Japanese, "test_jp");
        dictionary.register("", Language::German, "test_ge");
        dictionary.get("", Language::English);
    }

    #[test]
    #[should_panic(expected = "Value is empty for key 'äöü' and language 0")]
    fn keyNonEmpty_nonAscii_1Translations_available_invalid() {
        let dictionary = Dictionary::default();
        dictionary.register("äöü", Language::English, "");
        dictionary.get("äöü", Language::English);
    }

    #[test]
    fn keyEmpty_2Translations_available_valid() {
        let dictionary = Dictionary::default();
        dictionary.register("", Language::Japanese, "test_jp");
        dictionary.register("", Language::English, "test_en");
        dictionary.get("", Language::English);
    }

    #[test]
    fn keyEmpty_ascii_2Translations_available_valid() {
        let dictionary = Dictionary::default();
        dictionary.register("", Language::Japanese, "test_jp");
        dictionary.register("", Language::English, "test_en");
        assert_eq!(dictionary.get("", Language::English), "test_en");
    }
}

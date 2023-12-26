#![allow(warnings)]
use crate::mnemonics::original::languages::english::ENGLISHORIGINAL;

use super::languages::{dutch::DUTCHORIGINAL, chinese_simplified::CHINESESIMPLIFIEDORIGINAL, esperanto::ESPERANTOORIGINAL, french::FRENCHORIGINAL, german::GERMANORIGINAL, italian::ITALIANORIGINAL, japanese::JAPANESEORIGINAL, lojban::LOJBANORIGINAL, portuguese::PORTUGUESEORIGINAL, russian::RUSSIANORIGINAL, spanish::SPANISHORIGINAL};

// WordsetOriginal is a struct that contains the name of the wordset, the prefix length and the words
// Name is the ISO639 language code (https://en.wikipedia.org/wiki/List_of_ISO_639_language_codes)
// Prefix length is the number of letters to use to identify a word in the wordset
// Words is an array of 1626 words
pub(crate) struct WordsetOriginal {
    pub name: &'static str,
    pub prefix_len: usize,
    pub words: [&'static str; 1626],
}

// Wordsets of original-type (1626-word) mnemonics
pub(crate) static WORDSETSORIGINAL : [WordsetOriginal; 8] = [
    // TODO: Fix broken wordsets
    // TODO: Test all wordsets fully
    // CHINESESIMPLIFIEDORIGINAL, // Broken
    // DUTCHORIGINAL, // Broken
    ENGLISHORIGINAL,
    ESPERANTOORIGINAL,
    FRENCHORIGINAL,
    // GERMANORIGINAL, // Broken
    ITALIANORIGINAL,
    JAPANESEORIGINAL,
    LOJBANORIGINAL,
    PORTUGUESEORIGINAL,
    RUSSIANORIGINAL,
    // SPANISHORIGINAL, // Broken
];
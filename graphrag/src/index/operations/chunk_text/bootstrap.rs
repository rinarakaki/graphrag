//! Bootstrap definition.

static mut INITIALIZED_NLTK: bool = false;

/// Bootstrap definition.
pub fn bootstrap() {
    if !INITIALIZED_NLTK {
        use nltk;
        use nltk::corpus::wordnet as wn;

        nltk.download("punkt");
        nltk.download("punkt_tab");
        nltk.download("averaged_perceptron_tagger");
        nltk.download("averaged_perceptron_tagger_eng");
        nltk.download("maxent_ne_chunker");
        nltk.download("maxent_ne_chunker_tab");
        nltk.download("words");
        nltk.download("wordnet");
        wn.ensure_loaded();
        INITIALIZED_NLTK = true;
    }
}

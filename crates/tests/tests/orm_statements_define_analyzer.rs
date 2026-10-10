#[cfg(test)]
mod tests {
    use aparecium::{statements::*, *};

    #[test]
    fn test_define_analyzer_statement() {
        use AnalyzerFilter::*;
        use Tokenizer::*;

        let analyzer = define_analyzer("ascii").tokenizers([Class]).filters([
            Lowercase,
            Ascii,
            Edgengram(2, 15),
            Snowball(SnowballLanguage::English),
        ]);

        assert_eq!(
            analyzer.build(),
            "DEFINE ANALYZER ascii TOKENIZERS Class FILTERS lowercase,ascii,edgengram(2, 15),snowball(english);"
        );
    }
}

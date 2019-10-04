//! Ebuild and Eclass file parser

#[derive(Parser)]
#[grammar = "pwfr/core/ebuild/efile.pest"]
pub struct EFile;

#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn empty() {
        let efile = EFile::parse(Rule::efile, "").expect("unsuccessful parse");
        assert_eq!(
            efile.clone().map(|r| r.as_rule()).collect::<Vec<_>>(),
            vec![Rule::compound_list, Rule::EOI]
        );

        let compound_list = efile.clone().next().expect("unsuccessful getting");
        assert_eq!(
            compound_list
                .into_inner()
                .map(|r| r.as_rule())
                .collect::<Vec<_>>(),
            vec![Rule::empty_line]
        );
    }

    #[test]
    fn comments() {
        let efile = EFile::parse(
            Rule::efile,
            "#some comment\n#another comment\n#other comment",
        )
        .expect("unsuccessful parse");
        assert_eq!(
            efile.clone().map(|r| r.as_rule()).collect::<Vec<_>>(),
            vec![Rule::compound_list, Rule::EOI]
        );

        let compound_list = efile.clone().next().expect("unsuccessful getting");
        assert_eq!(
            compound_list
                .into_inner()
                .map(|r| r.as_rule())
                .collect::<Vec<_>>(),
            vec![
                Rule::empty_line,
                Rule::newline,
                Rule::newline,
                Rule::compound_list
            ]
        );
    }

    #[test]
    fn comments_and_variables() {
        let efile = EFile::parse(
            Rule::efile,
            "a = \"4\" #some comment\nb  =\"6\"#another comment\nc= \"7\"  #other comment\nd=\"8\"",
        )
        .expect("unsuccessful parse");
        assert_eq!(
            efile.clone().map(|r| r.as_rule()).collect::<Vec<_>>(),
            vec![Rule::compound_list, Rule::EOI]
        );

        let compound_list = efile.clone().next().expect("unsuccessful getting");
        assert_eq!(
            compound_list
                .into_inner()
                .map(|r| r.as_rule())
                .collect::<Vec<_>>(),
            vec![Rule::command, Rule::newline, Rule::compound_list]
        );
    }
}

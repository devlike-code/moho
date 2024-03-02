use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "moho.pest"]
pub struct MohoAST;

#[cfg(test)]
mod ast_tests {
    use crate::ast::{MohoAST, Rule};
    use pest::Parser;

    // property tests

    #[test]
    fn parse_properties() {
        let p = MohoAST::parse(Rule::properties, "[]");
        assert!(p.is_err());

        let p = MohoAST::parse(Rule::properties, "[A]");
        assert!(p.is_ok());

        let p = MohoAST::parse(Rule::properties, "[A=5]");
        assert!(p.is_ok());

        let p = MohoAST::parse(
            Rule::properties,
            "[A=5, B = \"hello world, jeremy!\", C='h', D =3.14, E=nullptr]",
        );
        assert!(p.is_ok());
    }

    // class tests

    #[test]
    fn parse_class_no_inherit() {
        let p = MohoAST::parse(Rule::class, "class Abc {}");
        assert!(p.is_ok());
    }

    #[test]
    fn parse_class_inherit() {
        let p = MohoAST::parse(Rule::class, "class Abc : Def {}");
        assert!(p.is_ok());
    }

    #[test]
    fn parse_class_properties() {
        let p = MohoAST::parse(Rule::class, "[Meta, VeryMeta, Count=5] class Abc {}");
        assert!(p.is_ok());
    }

    #[test]
    fn parse_declarations() {
        let p = MohoAST::parse(Rule::body, "int a; char b = 5; bool x = false;");
        assert!(p.is_ok());

        // at this point, we don't check types
        let p = MohoAST::parse(
            Rule::body,
            "int a = 3.14; char b = \"hey jude\"; bool x = 'c';",
        );
        assert!(p.is_ok());

        // we can do pointers
        let p = MohoAST::parse(Rule::body, "int* x = nullptr; Actor* a;");
        assert!(p.is_ok());

        // references too
        let p = MohoAST::parse(Rule::body, "int& x;");
        assert!(p.is_ok());

        // and arrays
        let p = MohoAST::parse(Rule::body, "SimpleController[] bindings;");
        assert!(p.is_ok());
    }

    #[test]
    fn parse_files() {
        let p = MohoAST::parse(Rule::class, include_str!("../assets/single_class.moho"));
        assert!(p.is_ok());

        let p = MohoAST::parse(Rule::class, include_str!("../assets/multi_classes.moho"));
        assert!(p.is_ok());
    }
}

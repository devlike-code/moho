use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "moho.pest"]
pub struct MohoGrammar;

#[cfg(test)]
mod ast_tests {
    use crate::grammar::{MohoGrammar, Rule};
    use pest::Parser;

    // property tests

    #[test]
    fn parse_properties() {
        let p = MohoGrammar::parse(Rule::properties, "[]");
        assert!(p.is_err());

        let p = MohoGrammar::parse(Rule::properties, "[A]");
        assert!(p.is_ok());

        let p = MohoGrammar::parse(Rule::properties, "[A=5]");
        assert!(p.is_ok());

        let p = MohoGrammar::parse(
            Rule::properties,
            "[A=5, B = \"hello world, jeremy!\", C='h', D =3.14, E=nullptr]",
        );
        assert!(p.is_ok());
    }

    // class tests

    #[test]
    fn parse_class_no_inherit() {
        let p = MohoGrammar::parse(Rule::class, "class Abc {}");
        assert!(p.is_ok());
    }

    #[test]
    fn parse_class_inherit() {
        let p = MohoGrammar::parse(Rule::class, "class Abc : Def {}");
        assert!(p.is_ok());
    }

    #[test]
    fn parse_class_properties() {
        let p = MohoGrammar::parse(Rule::class, "[Meta, VeryMeta, Count=5] class Abc {}");
        assert!(p.is_ok());
    }

    #[test]
    fn parse_declarations() {
        let p = MohoGrammar::parse(Rule::body, "int a; char b = 5; bool x = false;");
        assert!(p.is_ok());

        // at this point, we don't check types
        let p = MohoGrammar::parse(
            Rule::body,
            "int a = 3.14; char b = \"hey jude\"; bool x = 'c';",
        );
        assert!(p.is_ok());

        // we can do pointers
        let p = MohoGrammar::parse(Rule::body, "int* x = nullptr; Actor* a;");
        assert!(p.is_ok());

        // references too
        let p = MohoGrammar::parse(Rule::body, "int& x;");
        assert!(p.is_ok());

        // and arrays
        let p = MohoGrammar::parse(Rule::body, "SimpleController[] bindings;");
        assert!(p.is_ok());
    }

    #[test]
    fn parse_files() {
        let p = MohoGrammar::parse(Rule::class, include_str!("../assets/single_class.moho"));
        assert!(p.is_ok());

        let p = MohoGrammar::parse(Rule::class, include_str!("../assets/multi_classes.moho"));
        assert!(p.is_ok());
    }

    #[test]
    fn parse_method() {
        let p = MohoGrammar::parse(Rule::method_decl, "APawn* GetPlayerPawn(const UObject* WorldContextObject, 
            int32 PlayerIndex);");
        assert!(p.is_ok());
        println!("{:?}", p);
    }
}

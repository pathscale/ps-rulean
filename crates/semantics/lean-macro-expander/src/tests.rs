use lean_syn_expr::{Syntax, SyntaxAtom, SyntaxKind, SyntaxNode};

fn format_syntax(syntax: &Syntax) -> String {
    match syntax {
        Syntax::Missing => "<missing>".to_string(),
        Syntax::Atom(atom) => atom.value.to_string(),
        Syntax::Node(node) => {
            let kind = format!("{:?}", node.kind);
            if node.children.is_empty() {
                format!("({kind})")
            } else {
                let children: Vec<String> = node
                    .children
                    .iter()
                    .filter(|child| {
                        // Filter out whitespace and comments for cleaner output
                        match child {
                            Syntax::Node(n) => {
                                n.kind != SyntaxKind::Whitespace && n.kind != SyntaxKind::Comment
                            }
                            _ => true,
                        }
                    })
                    .map(format_syntax)
                    .collect();
                if children.is_empty() {
                    format!("({kind})")
                } else {
                    format!("({} {})", kind, children.join(" "))
                }
            }
        }
    }
}

#[test]
fn test_pattern_matching() {
    use eterned::BaseCoword;
    use lean_syn_expr::{SourcePos, SourceRange, SyntaxAtom, SyntaxNode};
    use smallvec::smallvec;

    use crate::pattern::PatternMatcher;

    let dummy_range = SourceRange {
        start: SourcePos::new(0, 0, 0),
        end: SourcePos::new(0, 0, 0),
    };

    // Pattern: $x
    let pattern = Syntax::Node(Box::new(SyntaxNode::new(
        SyntaxKind::SyntaxAntiquotation,
        dummy_range,
        smallvec![Syntax::Atom(SyntaxAtom::new(
            dummy_range,
            BaseCoword::new("x"),
        ))],
    )));

    // Syntax: 42
    let syntax = Syntax::Atom(SyntaxAtom::new(dummy_range, BaseCoword::new("42")));

    let result = PatternMatcher::match_pattern(&pattern, &syntax).unwrap();
    assert!(result.is_some());

    let bindings = result.unwrap().bindings;
    assert_eq!(bindings.len(), 1);
    assert!(bindings.contains_key(&BaseCoword::new("x")));
}

#[test]
fn test_template_substitution() {
    use eterned::BaseCoword;
    use im::HashMap;
    use lean_syn_expr::{SourcePos, SourceRange};
    use smallvec::smallvec;

    use crate::pattern::substitute_template;

    let dummy_range = SourceRange {
        start: SourcePos::new(0, 0, 0),
        end: SourcePos::new(0, 0, 0),
    };

    // Template: $x + $x
    let template = Syntax::Node(Box::new(SyntaxNode::new(
        SyntaxKind::BinOp,
        dummy_range,
        smallvec![
            Syntax::Atom(SyntaxAtom::new(dummy_range, BaseCoword::new("+"),)),
            Syntax::Node(Box::new(SyntaxNode::new(
                SyntaxKind::SyntaxAntiquotation,
                dummy_range,
                smallvec![Syntax::Atom(SyntaxAtom::new(
                    dummy_range,
                    BaseCoword::new("x"),
                ))],
            ))),
            Syntax::Node(Box::new(SyntaxNode::new(
                SyntaxKind::SyntaxAntiquotation,
                dummy_range,
                smallvec![Syntax::Atom(SyntaxAtom::new(
                    dummy_range,
                    BaseCoword::new("x"),
                ))],
            ))),
        ],
    )));

    // Bindings: x -> 42
    let mut bindings = HashMap::new();
    bindings.insert(
        BaseCoword::new("x"),
        Syntax::Atom(SyntaxAtom::new(dummy_range, BaseCoword::new("42"))),
    );

    let result = substitute_template(&template, &bindings).unwrap();
    let formatted = format_syntax(&result);
    assert_eq!(formatted, "(BinOp + 42 42)");
}

#[test]
fn test_hygiene() {
    use crate::hygiene::NameGenerator;

    let name_generator = NameGenerator::new();
    let name1 = name_generator.fresh_name("x");
    let name2 = name_generator.fresh_name("x");

    assert_ne!(name1, name2);
    assert!(name1.as_str().starts_with("x."));
    assert!(name2.as_str().starts_with("x."));
}

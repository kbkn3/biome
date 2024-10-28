use biome_analyze::{
    context::RuleContext, declare_lint_rule, Ast, Rule, RuleDiagnostic, RuleSource, RuleSourceKind,
};
use biome_console::markup;
use biome_js_syntax::JsIdentifierBinding;
use biome_rowan::AstNode;

declare_lint_rule! {
    /// Disallow using Object.assign with an object literal as the first argument and prefer the use of object spread instead.
    ///
    /// When `Object.assign` is called using an object literal as the first argument, this rule requires using the object spread syntax instead.
    /// This rule also warns on cases where an `Object.assign` call is made using a single argument that is an object literal, in this case, the `Object.assign` call is not needed.
    ///
    /// Introduced in ES2018, object spread is a declarative alternative which may perform better than the more dynamic, imperative `Object.assign`.
    ///
    /// Source: https://eslint.org/docs/latest/rules/prefer-object-spread
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// Object.assign({}, foo);
    ///
    // Object.assign({}, {foo: 'bar'});
    ///
    /// Object.assign({ foo: 'bar'}, baz);
    ///
    /// Object.assign({}, baz, { foo: 'bar' });
    ///
    /// Object.assign({}, { ...baz });
    ///
    /// // Object.assign with a single argument that is an object literal
    /// Object.assign({});
    ///
    /// Object.assign({ foo: bar });
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// ({ ...foo });
    ///
    /// ({ ...baz, foo: 'bar' });
    ///
    /// // Any Object.assign call without an object literal as the first argument
    /// Object.assign(foo, { bar: baz });
    ///
    /// Object.assign(foo, bar);
    ///
    /// Object.assign(foo, { bar, baz });
    ///
    /// Object.assign(foo, { ...baz });
    /// ```
    ///
    pub UseObjectSpread {
        version: "next",
        name: "useObjectSpread",
        language: "js",
        sources: &[
            RuleSource::Eslint("prefer-object-spread"),
        ],
        source_kind: RuleSourceKind::Inspired,
        recommended: false,
    }
}

impl Rule for UseObjectSpread {
    type Query = Ast<JsIdentifierBinding>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let _binding = ctx.query();
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        //
        // Read our guidelines to write great diagnostics:
        // https://docs.rs/biome_analyze/latest/biome_analyze/#what-a-rule-should-say-to-the-user
        //
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Variable is read here."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}

use crate::tests::string_to_stream;

use rustc_ast::token;
use rustc_ast::tokenstream::{Spacing, TokenStream, TokenStreamBuilder, TokenTree};
use rustc_span::create_default_session_globals_then;
use rustc_span::{BytePos, Span, Symbol};
use smallvec::smallvec;

fn string_to_ts(string: &str) -> TokenStream {
    string_to_stream(string.to_owned())
}

fn sp(a: u32, b: u32) -> Span {
    Span::with_root_ctxt(BytePos(a), BytePos(b))
}

fn joint(tree: TokenTree) -> TokenStream {
    TokenStream::new(vec![(tree, Spacing::Joint)])
}

#[test]
fn test_concat() {
    create_default_session_globals_then(|| {
        let test_res = string_to_ts("foo::bar::baz");
        let test_fst = string_to_ts("foo::bar");
        let test_snd = string_to_ts("::baz");
        let eq_res = TokenStream::from_streams(smallvec![test_fst, test_snd]);
        assert_eq!(test_res.trees().count(), 5);
        assert_eq!(eq_res.trees().count(), 5);
        assert_eq!(test_res.eq_unspanned(&eq_res), true);
    })
}

#[test]
fn test_to_from_bijection() {
    create_default_session_globals_then(|| {
        let test_start = string_to_ts("foo::bar(baz)");
        let test_end = test_start.trees().cloned().collect();
        assert_eq!(test_start, test_end)
    })
}

#[test]
fn test_eq_0() {
    create_default_session_globals_then(|| {
        let test_res = string_to_ts("foo");
        let test_eqs = string_to_ts("foo");
        assert_eq!(test_res, test_eqs)
    })
}

#[test]
fn test_eq_1() {
    create_default_session_globals_then(|| {
        let test_res = string_to_ts("::bar::baz");
        let test_eqs = string_to_ts("::bar::baz");
        assert_eq!(test_res, test_eqs)
    })
}

#[test]
fn test_eq_3() {
    create_default_session_globals_then(|| {
        let test_res = string_to_ts("");
        let test_eqs = string_to_ts("");
        assert_eq!(test_res, test_eqs)
    })
}

#[test]
fn test_diseq_0() {
    create_default_session_globals_then(|| {
        let test_res = string_to_ts("::bar::baz");
        let test_eqs = string_to_ts("bar::baz");
        assert_eq!(test_res == test_eqs, false)
    })
}

#[test]
fn test_diseq_1() {
    create_default_session_globals_then(|| {
        let test_res = string_to_ts("(bar,baz)");
        let test_eqs = string_to_ts("bar,baz");
        assert_eq!(test_res == test_eqs, false)
    })
}

#[test]
fn test_is_empty() {
    create_default_session_globals_then(|| {
        let test0: TokenStream = Vec::<TokenTree>::new().into_iter().collect();
        let test1: TokenStream =
            TokenTree::token(token::Ident(Symbol::intern("a"), false), sp(0, 1)).into();
        let test2 = string_to_ts("foo(bar::baz)");

        assert_eq!(test0.is_empty(), true);
        assert_eq!(test1.is_empty(), false);
        assert_eq!(test2.is_empty(), false);
    })
}

#[test]
fn test_dotdotdot() {
    create_default_session_globals_then(|| {
        let mut builder = TokenStreamBuilder::new();
        builder.push(joint(TokenTree::token(token::Dot, sp(0, 1))));
        builder.push(joint(TokenTree::token(token::Dot, sp(1, 2))));
        builder.push(TokenTree::token(token::Dot, sp(2, 3)));
        let stream = builder.build();
        assert!(stream.eq_unspanned(&string_to_ts("...")));
        assert_eq!(stream.trees().count(), 1);
    })
}

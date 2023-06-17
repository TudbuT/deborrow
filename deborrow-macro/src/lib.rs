use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

/// A safe way to borrow multiple fields of a struct mutably at once.
///
/// This is just as safe to having an impl on the struct which takes &mut self
/// and then accesses multiple of its own fields mutably, aka. completely safe.
#[proc_macro]
pub fn deborrow(ts: TokenStream) -> TokenStream {
    let mut ts = ts.into_iter();
    let item = ts
        .next()
        .expect("expected item in deborrow!($item, $($field )*)");
    if !matches!(
        ts.next(),
        Some(TokenTree::Punct(x))
        if x.as_char() == ','
    ) {
        panic!("expected , in deborrow!($item, $($field )*)");
    }
    let fields = ts.collect::<Vec<_>>();
    let mut result = Vec::new();
    result.append(
        &mut ("fn __deborrow_unify<'a, "
            .parse::<TokenStream>()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>()),
    );
    result.append(
        &mut fields
            .iter()
            .flat_map(|x| {
                vec![
                    x.to_owned(),
                    TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                ]
            })
            .collect::<Vec<_>>(),
    );
    result.push(TokenTree::Punct(Punct::new('>', Spacing::Alone)));
    result.push(TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        TokenStream::from_iter(fields.iter().flat_map(|x| {
            vec![
                x.to_owned(),
                TokenTree::Punct(Punct::new(':', Spacing::Alone)),
                TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
                TokenTree::Ident(Ident::new("a", Span::mixed_site())),
                TokenTree::Ident(Ident::new("mut", Span::mixed_site())),
                TokenTree::Ident(Ident::new(
                    &("T".to_owned() + &x.to_string()),
                    Span::mixed_site(),
                )),
                TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            ]
        })),
    )));
    result.append(
        &mut ("->"
            .parse::<TokenStream>()
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>()),
    );
    result.push(TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        TokenStream::from_iter(fields.iter().flat_map(|x| {
            vec![
                TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                TokenTree::Punct(Punct::new('\'', Spacing::Joint)),
                TokenTree::Ident(Ident::new("a", Span::mixed_site())),
                TokenTree::Ident(Ident::new("mut", Span::mixed_site())),
                TokenTree::Ident(Ident::new(
                    &("T".to_owned() + &x.to_string()),
                    Span::mixed_site(),
                )),
                TokenTree::Punct(Punct::new(',', Spacing::Alone)),
            ]
        })),
    )));
    result.push(TokenTree::Group(Group::new(
        Delimiter::Brace,
        TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter(fields.iter().flat_map(|x| {
                vec![
                    x.to_owned(),
                    TokenTree::Punct(Punct::new(',', Spacing::Alone)),
                ]
            })),
        ))
        .into(),
    )));
    let mut tuple = vec![];
    for field in &fields[0..fields.len() - 1] {
        tuple.push(TokenTree::Ident(Ident::new("unsafe", Span::mixed_site())));
        let mut e = ("::deborrow::deborrow".parse::<TokenStream>().unwrap())
            .into_iter()
            .collect::<Vec<_>>();
        e.push(TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter(vec![
                TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                TokenTree::Ident(Ident::new("mut", Span::mixed_site())),
                item.clone(),
                TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                field.clone(),
            ]),
        )));
        tuple.push(TokenTree::Group(Group::new(
            Delimiter::Brace,
            TokenStream::from_iter(e.into_iter()),
        )));
        tuple.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
    }
    if !fields.is_empty() {
        tuple.push(TokenTree::Group(Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter(vec![
                TokenTree::Punct(Punct::new('&', Spacing::Joint)),
                TokenTree::Ident(Ident::new("mut", Span::mixed_site())),
                item,
                TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                fields[fields.len() - 1].clone(),
            ]),
        )));
        tuple.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
    }
    result.push(TokenTree::Ident(Ident::new(
        "__deborrow_unify",
        Span::mixed_site(),
    )));
    result.push(TokenTree::Group(Group::new(
        Delimiter::Parenthesis,
        TokenStream::from_iter(tuple.into_iter()),
    )));
    TokenStream::from(TokenTree::Group(Group::new(
        Delimiter::Brace,
        TokenStream::from_iter(result.into_iter()),
    )))
}

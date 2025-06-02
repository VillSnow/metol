use nom::{
    branch::alt,
    bytes::{tag, take_while1},
    character::{
        char as nom_char,
        complete::{alphanumeric1, one_of},
        none_of,
    },
    combinator::{map, map_res, not, opt, peek, recognize, verify},
    error::{ErrorKind, ParseError},
    multi::{many0, many1, many_till, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult, Input as _, Parser,
};

mod input;
mod node;
mod utils;

use input::Input;
use utils::LINE_BREAK_PATTERNS;

pub use node::{Node, RawNode};

fn word_chars1(input: Input) -> IResult<Input, Input> {
    take_while1(|c: char| matches!(c, '0'..='9' | 'A'..='Z' | '_' | 'a'..='z'))
        .parse_complete(input)
}

fn eol<'a>(input: Input<'a>) -> IResult<Input<'a>, Input<'a>> {
    if input.s.is_empty() {
        return Ok(input.take_split(0));
    }

    for pat in LINE_BREAK_PATTERNS {
        if input.s.starts_with(pat) {
            dbg!("ok");
            return Ok(input.take_split(pat.len()));
        }
    }

    Err(nom::Err::Error(nom::error::Error::from_error_kind(
        input,
        ErrorKind::Verify,
    )))
}

fn parse_domain(input: Input) -> IResult<Input, Vec<&str>> {
    let domain_label = verify(
        take_while1::<_, Input, _>(|c: char| matches!(c, '-' | '0'..='9' | 'A'..='Z' | 'a'..='z' )),
        |s: &Input| !s.s.starts_with('_') && !s.s.ends_with('-'),
    );

    map(
        pair(
            separated_list1::<Input, _, _, _>(nom_char('.'), domain_label),
            opt(nom_char('.')),
        ),
        |(labels, root)| {
            labels
                .into_iter()
                .map(|x| x.s)
                .chain(root.into_iter().map(|_| ""))
                .collect()
        },
    )
    .parse_complete(input)
}

fn parse_char(input: Input) -> IResult<Input, RawNode> {
    match input.s.chars().next() {
        None => Err(nom::Err::Error(nom::error::Error::from_error_kind(
            input,
            ErrorKind::Eof,
        ))),
        Some(c) => Ok((input.take_from(c.len_utf8()), RawNode::Char(c))),
    }
}

fn parse_global_user(input: Input) -> IResult<Input, RawNode> {
    map(
        preceded(
            nom_char('@'),
            separated_pair(word_chars1, nom_char('@'), parse_domain),
        ),
        |(name, host)| RawNode::GlobalUser(name.s, host),
    )
    .parse_complete(input)
}

fn parse_local_user(input: Input) -> IResult<Input, RawNode> {
    map(preceded(nom_char('@'), word_chars1), |x| {
        RawNode::LocalUser(x.s)
    })
    .parse_complete(input)
}

fn parse_local_custom_emoji(input: Input) -> IResult<Input, RawNode> {
    map(
        pair(
            delimited(
                pair(many0(nom_char('\u{200B}')), nom_char(':')),
                word_chars1,
                pair(nom_char(':'), many0(nom_char('\u{200B}'))),
            ),
            peek(not(alphanumeric1)),
        ),
        |(x, _)| RawNode::LocalCustomEmoji(x.s),
    )
    .parse_complete(input)
}

fn hashtag_item(input: Input) -> IResult<Input, &str> {
    let prohibited_chars = concat!(" \u{3000}\t\r\n", r##".,!?'"#:/[\]【】()「」（）<>"##);

    alt((
        recognize(delimited(nom_char('('), many0(hashtag_item), nom_char(')'))),
        recognize(delimited(nom_char('['), many0(hashtag_item), nom_char(']'))),
        recognize(delimited(
            nom_char('「'),
            many0(hashtag_item),
            nom_char('」'),
        )),
        recognize(delimited(
            nom_char('（'),
            many0(hashtag_item),
            nom_char('）'),
        )),
        recognize(none_of(prohibited_chars)),
    ))
    .parse_complete(input)
    .map(|(i, r)| (i, r.s))
}

fn parse_hashtag(input: Input) -> IResult<Input, RawNode> {
    map(
        preceded(nom_char('#'), recognize(many1(hashtag_item))),
        |x| RawNode::HashTag(x.s),
    )
    .parse_complete(input)
}

fn parse_small(input: Input) -> IResult<Input, RawNode> {
    map_res(
        |input| parse_enclosed_text(input, "<small>", "</small>"),
        |children| {
            if !children.is_empty() {
                Ok(RawNode::Small(Box::new(RawNode::Span(children))))
            } else {
                Err(nom::error::Error::new(input, nom::error::ErrorKind::Many1))
            }
        },
    )
    .parse_complete(input)
}

fn parse_center<'a>(input: Input<'a>) -> IResult<Input<'a>, RawNode<'a>> {
    if !input.is_line_head {
        return Err(nom::Err::Error(nom::error::Error::from_error_kind(
            input,
            ErrorKind::Verify,
        )));
    }
    map_res(
        pair(
            |input| parse_enclosed_text(input, "<center>", "</center>"),
            peek(eol),
        ),
        |(children, _)| {
            if !children.is_empty() {
                Ok(RawNode::Center(Box::new(RawNode::Span(children))))
            } else {
                Err(nom::error::Error::new(input, nom::error::ErrorKind::Many1))
            }
        },
    )
    .parse_complete(input)
}

fn parse_plain_tag<'a>(input: Input<'a>) -> IResult<Input<'a>, RawNode<'a>> {
    let open = "<plain>";
    let close = "</plain>";
    if !input.s.starts_with(open) {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    }
    let input = input.take_from(open.len());

    let Some(n) = input.s.find(close) else {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Tag,
        )));
    };
    if n == 0 {
        return Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Many1,
        )));
    }

    let content = &input.s[..n];
    let input = input.take_from(n + close.len());

    Ok((input, RawNode::PlainTag(content)))
}

fn parse_quote(input: Input) -> IResult<Input, RawNode> {
    if !input.is_line_head {
        return Err(nom::Err::Error(nom::error::Error::from_error_kind(
            input,
            ErrorKind::Verify,
        )));
    }

    map(
        (
            many1(terminated(nom_char('>'), opt(one_of(" \u{3000}")))),
            map(
                many_till(parse_span_item, alt((eol, peek(eol)))),
                |(xs, _)| RawNode::Span(xs),
            ),
        ),
        |(n, x)| RawNode::Quote(n.len(), Box::new(x)),
    )
    .parse(input)
}

fn parse_span_item(input: Input) -> IResult<Input, RawNode> {
    alt((
        parse_global_user,
        parse_local_user,
        parse_local_custom_emoji,
        parse_hashtag,
        parse_small,
        parse_center,
        parse_plain_tag,
        parse_quote,
        parse_char,
    ))
    .parse_complete(input)
}

fn parse_text(input: Input) -> IResult<Input, RawNode> {
    map(many0(parse_span_item), RawNode::Span).parse_complete(input)
}

fn parse_enclosed_text<'i, 't: 'i>(
    input: Input<'i>,
    open: &'t str,
    close: &'t str,
) -> IResult<Input<'i>, Vec<RawNode<'i>>> {
    preceded(
        tag(open),
        map(many_till(parse_span_item, tag(close)), |(nodes, _)| nodes),
    )
    .parse_complete(input)
}

pub fn parse_mfm(input: &str) -> Node {
    parse_mfm_raw(input).into()
}

pub fn parse_mfm_raw(input: &str) -> RawNode {
    match parse_text(Input {
        s: input,
        is_line_head: true,
    }) {
        Ok((x, node)) if x.s.is_empty() => node,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests;

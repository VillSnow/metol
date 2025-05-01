use std::str::{CharIndices, Chars};

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::{
        char as nom_char,
        complete::{alphanumeric1, none_of},
    },
    combinator::{map, map_res, not, opt, peek, recognize, verify},
    error::{ErrorKind, ParseError},
    multi::{many0, many1, many_till, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair},
    IResult, Input as _, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RawNode<'a> {
    Span(Vec<RawNode<'a>>),
    GlobalUser(&'a str, Vec<&'a str>),
    LocalUser(&'a str),
    LocalCustomEmoji(&'a str),
    HashTag(&'a str),
    Small(Box<RawNode<'a>>),
    Center(Box<RawNode<'a>>),
    Char(char),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Empty,
    Span(Vec<Node>),
    GlobalUser(String, Vec<String>),
    LocalUser(String),
    LocalCustomEmoji(String),
    HashTag(String),
    Small(Box<Node>),
    Center(Box<Node>),
    Plain(String),
}

#[derive(Debug, Clone, Copy)]
struct Input<'a> {
    s: &'a str,
    is_line_head: bool,
}

impl<'a> nom::Input for Input<'a> {
    type Item = char;
    type Iter = Chars<'a>;
    type IterIndices = CharIndices<'a>;

    fn input_len(&self) -> usize {
        self.s.len()
    }

    fn take(&self, index: usize) -> Self {
        Self {
            s: &self.s[..index],
            is_line_head: self.is_line_head,
        }
    }

    fn take_from(&self, index: usize) -> Self {
        let is_line_head = self.s[..index]
            .chars()
            .next_back()
            .map(is_line_break_char)
            .unwrap_or(self.is_line_head);

        Self {
            s: &self.s[index..],
            is_line_head,
        }
    }

    fn take_split(&self, index: usize) -> (Self, Self) {
        let (prefix, suffix) = self.s.split_at(index);

        (
            Input {
                s: suffix,
                is_line_head: prefix
                    .chars()
                    .next_back()
                    .map(is_line_break_char)
                    .unwrap_or(self.is_line_head),
            },
            Input {
                s: prefix,
                is_line_head: self.is_line_head,
            },
        )
    }

    fn position<P>(&self, predicate: P) -> Option<usize>
    where
        P: Fn(Self::Item) -> bool,
    {
        self.s.find(predicate)
    }

    fn iter_elements(&self) -> Self::Iter {
        self.s.chars()
    }

    fn iter_indices(&self) -> Self::IterIndices {
        self.s.char_indices()
    }

    fn slice_index(&self, count: usize) -> Result<usize, nom::Needed> {
        nom::Input::slice_index(&self.s, count)
    }
}

impl nom::Offset for Input<'_> {
    fn offset(&self, second: &Self) -> usize {
        second.s.as_ptr() as usize - self.s.as_ptr() as usize
    }
}

impl<'a> nom::Compare<&'a str> for Input<'a> {
    fn compare(&self, t: &'a str) -> nom::CompareResult {
        nom::Compare::compare(&self.s, t)
    }

    fn compare_no_case(&self, t: &'a str) -> nom::CompareResult {
        nom::Compare::compare_no_case(&self.s, t)
    }
}

fn is_line_break_char(c: char) -> bool {
    matches!(
        c,
        '\r' | '\n' | '\u{000B}' | '\u{000C}' | '\u{0085}' | '\u{2028}' | '\u{2029}'
    )
}

fn word_chars1(input: Input) -> IResult<Input, Input> {
    take_while1(|c: char| matches!(c, '0'..='9' | 'A'..='Z' | '_' | 'a'..='z'))
        .parse_complete(input)
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
            peek(|input: Input<'a>| {
                if input
                    .s
                    .chars()
                    .next()
                    .map(is_line_break_char)
                    .unwrap_or(true)
                {
                    Ok((input, ()))
                } else {
                    Err(nom::Err::Error(nom::error::Error::from_error_kind(
                        input,
                        ErrorKind::Verify,
                    )))
                }
            }),
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

fn parse_span_item(input: Input) -> IResult<Input, RawNode> {
    alt((
        parse_global_user,
        parse_local_user,
        parse_local_custom_emoji,
        parse_hashtag,
        parse_small,
        parse_center,
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

impl RawNode<'_> {
    fn flatten(self) -> Self {
        match self {
            RawNode::Span(children) => {
                let mut result = Vec::new();

                let children = children.into_iter().map(RawNode::flatten);
                for child in children {
                    match child {
                        RawNode::Span(mut grandchildren) => result.append(&mut grandchildren),
                        leaf => result.push(leaf),
                    }
                }

                if result.len() == 1 {
                    result.pop().unwrap()
                } else {
                    RawNode::Span(result)
                }
            }
            RawNode::Small(child) => RawNode::Small(Box::new(child.flatten())),
            RawNode::Center(child) => RawNode::Center(Box::new(child.flatten())),
            RawNode::GlobalUser(_, _)
            | RawNode::LocalUser(_)
            | RawNode::LocalCustomEmoji(_)
            | RawNode::HashTag(_)
            | RawNode::Char(_) => self,
        }
    }

    fn into_node(self) -> Node {
        match self {
            RawNode::Span(children) => {
                let mut result = Vec::new();

                for child in children {
                    match (result.last_mut(), child) {
                        (Some(Node::Plain(s)), RawNode::Char(c)) => s.push(c),
                        (_, raw_item) => result.push(Node::from(raw_item)),
                    }
                }
                if result.is_empty() {
                    Node::Empty
                } else if result.len() == 1 {
                    result.pop().unwrap()
                } else {
                    Node::Span(result)
                }
            }
            RawNode::GlobalUser(name, host) => Node::GlobalUser(
                name.to_owned(),
                host.into_iter().map(str::to_owned).collect(),
            ),
            RawNode::LocalUser(name) => Node::LocalUser(name.to_owned()),
            RawNode::LocalCustomEmoji(name) => Node::LocalCustomEmoji(name.to_owned()),
            RawNode::HashTag(name) => Node::HashTag(name.to_owned()),
            RawNode::Small(child) => Node::Small(Box::new(child.into_node())),
            RawNode::Center(child) => Node::Center(Box::new(child.into_node())),
            RawNode::Char(c) => Node::Plain(c.to_string()),
        }
    }
}

impl Node {
    pub fn into_global(self, source_host: Vec<String>) -> Self {
        match self {
            Node::Span(nodes) => Node::Span(
                nodes
                    .into_iter()
                    .map(|x| x.into_global(source_host.clone()))
                    .collect(),
            ),
            Node::LocalUser(name) => Node::GlobalUser(name, source_host.clone()),
            Node::Small(child) => Node::Small(Box::new(child.into_global(source_host.clone()))),
            Node::Center(child) => Node::Center(Box::new(child.into_global(source_host.clone()))),
            Node::Empty
            | Node::GlobalUser(_, _)
            | Node::LocalCustomEmoji(_)
            | Node::HashTag(_)
            | Node::Plain(_) => self,
        }
    }
}

impl<'a> From<RawNode<'a>> for Node {
    fn from(raw: RawNode<'a>) -> Self {
        raw.flatten().into_node()
    }
}

#[cfg(test)]
mod tests;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{alphanumeric1, anychar},
    combinator::{all_consuming, map, not, opt, peek, verify},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair},
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RawNode<'a> {
    Span(Vec<RawNode<'a>>),
    GlobalUser(&'a str, Vec<&'a str>),
    LocalUser(&'a str),
    LocalCustomEmoji(&'a str),
    HashTag(&'a str),
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
    Plain(String),
}

fn word_chars1(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| match c {
        '0'..='9' | 'A'..='Z' | '_' | 'a'..='z' => true,
        _ => false,
    })
    .parse(input)
}

fn parse_domain(input: &str) -> IResult<&str, Vec<&str>> {
    let domain_label = verify(
        take_while1(|c: char| match c {
            '-' | '0'..='9' | 'A'..='Z' | 'a'..='z' => true,
            _ => false,
        }),
        |s: &str| s.chars().next().unwrap() != '-' && s.chars().last().unwrap() != '-',
    );

    map(
        pair(separated_list1(tag("."), domain_label), opt(tag("."))),
        |(mut labels, root)| {
            if root.is_some() {
                labels.push("")
            }
            labels
        },
    )
    .parse(input)
}

fn parse_char(input: &str) -> IResult<&str, RawNode> {
    map(anychar, |c| RawNode::Char(c)).parse(input)
}

fn parse_global_user(input: &str) -> IResult<&str, RawNode> {
    map(
        preceded(
            tag("@"),
            separated_pair(word_chars1, tag("@"), parse_domain),
        ),
        |(name, host)| RawNode::GlobalUser(name, host),
    )
    .parse(input)
}

fn parse_local_user(input: &str) -> IResult<&str, RawNode> {
    map(preceded(tag("@"), word_chars1), RawNode::LocalUser).parse(input)
}

fn parse_local_custom_emoji(input: &str) -> IResult<&str, RawNode> {
    map(
        pair(
            delimited(
                pair(many0(tag("\u{200B}")), tag(":")),
                word_chars1,
                pair(tag(":"), many0(tag("\u{200B}"))),
            ),
            peek(not(alphanumeric1)),
        ),
        |(s, _)| RawNode::LocalCustomEmoji(s),
    )
    .parse(input)
}

fn hashtag_item(input: &str) -> IResult<&str, &str> {
    let prohibited_chars = concat!(" \u{3000}\t\r\n", r##".,!?'"#:/[\]【】()「」（）<>"##);

    alt((
        recognize(delimited(tag("("), many0(hashtag_item), tag(")"))),
        recognize(delimited(tag("["), many0(hashtag_item), tag("]"))),
        recognize(delimited(tag("「"), many0(hashtag_item), tag("」"))),
        recognize(delimited(tag("（"), many0(hashtag_item), tag("）"))),
        recognize(none_of(prohibited_chars)),
    ))
    .parse(input)
}

fn parse_hashtag(input: &str) -> IResult<&str, RawNode> {
    map(
        preceded(tag("#"), recognize(many1(hashtag_item))),
        RawNode::HashTag,
    )
    .parse(input)
}

fn parse_text(input: &str) -> IResult<&str, RawNode> {
    map(
        all_consuming(many0(alt((
            parse_global_user,
            parse_local_user,
            parse_local_custom_emoji,
            parse_hashtag,
            parse_char,
        )))),
        RawNode::Span,
    )
    .parse(input)
}

pub fn parse_mfm(input: &str) -> Node {
    parse_mfm_raw(input).into()
}

pub fn parse_mfm_raw(input: &str) -> RawNode {
    match parse_text(input) {
        Ok(("", result)) => result,
        _ => unreachable!(),
    }
}

impl<'a> RawNode<'a> {
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
            leaf => leaf,
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
                if result.len() == 0 {
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
            _ => self,
        }
    }
}

impl<'a> From<RawNode<'a>> for Node {
    fn from(raw: RawNode<'a>) -> Self {
        raw.flatten().into_node()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn plain() {
        let raw_node = parse_mfm_raw("hoge");
        assert_eq!(
            raw_node,
            RawNode::Span(vec![
                RawNode::Char('h'),
                RawNode::Char('o'),
                RawNode::Char('g'),
                RawNode::Char('e'),
            ])
        );

        let node = Node::from(raw_node);
        assert_eq!(node, Node::Plain("hoge".to_owned()));
    }

    #[test]
    fn user() {
        let raw_node = parse_mfm_raw("@aliceさん");
        assert_eq!(
            raw_node,
            RawNode::Span(vec![
                RawNode::LocalUser("alice"),
                RawNode::Char('さ'),
                RawNode::Char('ん'),
            ])
        );

        let node = Node::from(raw_node);
        assert_eq!(
            node,
            Node::Span(vec![
                Node::LocalUser("alice".to_owned()),
                Node::Plain("さん".to_owned()),
            ])
        );

        let global_node = node.into_global(vec!["example".to_owned(), "com".to_owned()]);
        assert_eq!(
            global_node,
            Node::Span(vec![
                Node::GlobalUser(
                    "alice".to_owned(),
                    vec!["example".to_owned(), "com".to_owned()]
                ),
                Node::Plain("さん".to_owned()),
            ])
        );
    }

    #[test]
    fn user_global() {
        let raw_node = parse_mfm_raw("@alice@example.comさん");
        assert_eq!(
            raw_node,
            RawNode::Span(vec![
                RawNode::GlobalUser("alice", vec!["example", "com"]),
                RawNode::Char('さ'),
                RawNode::Char('ん'),
            ])
        );

        let node = Node::from(raw_node);
        assert_eq!(
            node,
            Node::Span(vec![
                Node::GlobalUser(
                    "alice".to_owned(),
                    vec!["example".to_owned(), "com".to_owned()]
                ),
                Node::Plain("さん".to_owned()),
            ])
        );

        let global_node = node.into_global(vec!["other".to_owned(), "com".to_owned()]);
        assert_eq!(
            global_node,
            Node::Span(vec![
                Node::GlobalUser(
                    "alice".to_owned(),
                    vec!["example".to_owned(), "com".to_owned()]
                ),
                Node::Plain("さん".to_owned()),
            ])
        );
    }

    #[test]
    fn domain_root() {
        let raw_node = parse_mfm_raw("@alice@example.com.さん");
        assert_eq!(
            raw_node,
            RawNode::Span(vec![
                RawNode::GlobalUser("alice", vec!["example", "com", ""]),
                RawNode::Char('さ'),
                RawNode::Char('ん'),
            ])
        );

        let node = Node::from(raw_node);
        assert_eq!(
            node,
            Node::Span(vec![
                Node::GlobalUser(
                    "alice".to_owned(),
                    vec!["example".to_owned(), "com".to_owned(), "".to_owned()]
                ),
                Node::Plain("さん".to_owned()),
            ])
        );

        let global_node = node.into_global(vec!["other".to_owned(), "com".to_owned()]);
        assert_eq!(
            global_node,
            Node::Span(vec![
                Node::GlobalUser(
                    "alice".to_owned(),
                    vec!["example".to_owned(), "com".to_owned(), "".to_owned()]
                ),
                Node::Plain("さん".to_owned()),
            ])
        );
    }

    #[test]
    fn emoji1() {
        let raw_node = parse_mfm_raw("hoge:smile:");
        assert_eq!(
            raw_node,
            RawNode::Span(vec![
                RawNode::Char('h'),
                RawNode::Char('o'),
                RawNode::Char('g'),
                RawNode::Char('e'),
                RawNode::LocalCustomEmoji("smile")
            ])
        );

        let node = Node::from(raw_node);
        assert_eq!(
            node,
            Node::Span(vec![
                Node::Plain("hoge".to_owned()),
                Node::LocalCustomEmoji("smile".to_owned()),
            ])
        );
    }

    #[test]
    fn emoji2() {
        let raw_node = parse_mfm_raw("hoge:smile:ふが");
        assert_eq!(
            raw_node,
            RawNode::Span(vec![
                RawNode::Char('h'),
                RawNode::Char('o'),
                RawNode::Char('g'),
                RawNode::Char('e'),
                RawNode::LocalCustomEmoji("smile"),
                RawNode::Char('ふ'),
                RawNode::Char('が'),
            ])
        );

        let node = Node::from(raw_node);
        assert_eq!(
            node,
            Node::Span(vec![
                Node::Plain("hoge".to_owned()),
                Node::LocalCustomEmoji("smile".to_owned()),
                Node::Plain("ふが".to_owned()),
            ])
        );
    }

    #[test]
    fn time() {
        let raw_node = parse_mfm_raw("12:34:56");
        assert_eq!(
            raw_node,
            RawNode::Span(vec![
                RawNode::Char('1'),
                RawNode::Char('2'),
                RawNode::Char(':'),
                RawNode::Char('3'),
                RawNode::Char('4'),
                RawNode::Char(':'),
                RawNode::Char('5'),
                RawNode::Char('6'),
            ])
        );

        let node = Node::from(raw_node);
        assert_eq!(node, Node::Plain("12:34:56".to_owned()));
    }

    #[test]
    fn hashtag1() {
        let raw_node = parse_mfm_raw("#");
        assert_eq!(raw_node, RawNode::Span(vec![RawNode::Char('#')]));

        let node = Node::from(raw_node);
        assert_eq!(node, Node::Plain("#".to_owned()));
    }

    #[test]
    fn hashtag2() {
        let raw_node = parse_mfm_raw("#tag");
        assert_eq!(raw_node, RawNode::Span(vec![RawNode::HashTag("tag")]));

        let node = Node::from(raw_node);
        assert_eq!(node, Node::HashTag("tag".to_owned()));
    }

    #[test]
    fn hashtag3() {
        let raw_node = parse_mfm_raw("#tag text");
        assert_eq!(
            raw_node,
            RawNode::Span(vec![
                RawNode::HashTag("tag"),
                RawNode::Char(' '),
                RawNode::Char('t'),
                RawNode::Char('e'),
                RawNode::Char('x'),
                RawNode::Char('t')
            ])
        );

        let node = Node::from(raw_node);
        assert_eq!(
            node,
            Node::Span(vec![
                Node::HashTag("tag".to_owned()),
                Node::Plain(" text".to_owned())
            ])
        );
    }

    #[test]
    fn hashtag4() {
        let raw_node = parse_mfm_raw("#p(a[r]e)n");
        assert_eq!(
            raw_node,
            RawNode::Span(vec![RawNode::HashTag("p(a[r]e)n"),])
        );

        let node = Node::from(raw_node);
        assert_eq!(node, Node::HashTag("p(a[r]e)n".to_owned()));
    }

    #[test]
    fn hashtag5() {
        let raw_node = parse_mfm_raw("#p(aren");
        assert_eq!(
            raw_node,
            RawNode::Span(vec![
                RawNode::HashTag("p"),
                RawNode::Char('('),
                RawNode::Char('a'),
                RawNode::Char('r'),
                RawNode::Char('e'),
                RawNode::Char('n'),
            ])
        );

        let node = Node::from(raw_node);
        assert_eq!(
            node,
            Node::Span(vec![
                Node::HashTag("p".to_owned()),
                Node::Plain("(aren".to_owned())
            ])
        );
    }
}

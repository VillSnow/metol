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

#[test]
fn small1() {
    let node = parse_mfm("aaa<small>bbb</small>ccc");
    assert_eq!(
        node,
        Node::Span(vec![
            Node::Plain("aaa".to_owned()),
            Node::Small(Box::new(Node::Plain("bbb".to_owned()))),
            Node::Plain("ccc".to_owned()),
        ])
    );
}

#[test]
fn small2() {
    let node = parse_mfm("aaa<small>bbb<small>ccc</small>ddd</small>eee");
    assert_eq!(
        node,
        Node::Span(vec![
            Node::Plain("aaa".to_owned()),
            Node::Small(Box::new(Node::Span(vec![
                Node::Plain("bbb".to_owned()),
                Node::Small(Box::new(Node::Plain("ccc".to_owned()))),
                Node::Plain("ddd".to_owned()),
            ]))),
            Node::Plain("eee".to_owned()),
        ])
    );
}

#[test]
fn small3() {
    let node = parse_mfm("aaa<small>bbb</small>ccc<small>ddd</small>eee");
    assert_eq!(
        node,
        Node::Span(vec![
            Node::Plain("aaa".to_owned()),
            Node::Small(Box::new(Node::Plain("bbb".to_owned()))),
            Node::Plain("ccc".to_owned()),
            Node::Small(Box::new(Node::Plain("ddd".to_owned()))),
            Node::Plain("eee".to_owned()),
        ])
    );
}

#[test]
fn center1() {
    let node = parse_mfm("<center>aaa</center>");
    assert_eq!(node, Node::Center(Box::new(Node::Plain("aaa".to_owned()))));
}

#[test]
fn center2() {
    let node = parse_mfm("aaa<center>bbb</center>");
    assert_eq!(node, Node::Plain("aaa<center>bbb</center>".to_owned()));
}

#[test]
fn center3() {
    let node = parse_mfm("<center>aaa</center>bbb");
    assert_eq!(node, Node::Plain("<center>aaa</center>bbb".to_owned()));
}

#[test]
fn center4() {
    let node = parse_mfm("aaa\n<center>bbb</center>");
    assert_eq!(
        node,
        Node::Span(vec![
            Node::Plain("aaa\n".to_owned()),
            Node::Center(Box::new(Node::Plain("bbb".to_owned()))),
        ])
    );
}

#[test]
fn center5() {
    let node = parse_mfm("<center>aaa</center>\nbbb");
    assert_eq!(
        node,
        Node::Span(vec![
            Node::Center(Box::new(Node::Plain("aaa".to_owned()))),
            Node::Plain("\nbbb".to_owned()),
        ])
    );
}

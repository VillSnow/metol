#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RawNode<'a> {
    Span(Vec<RawNode<'a>>),
    GlobalUser(&'a str, Vec<&'a str>),
    LocalUser(&'a str),
    LocalCustomEmoji(&'a str),
    HashTag(&'a str),
    Small(Box<RawNode<'a>>),
    Center(Box<RawNode<'a>>),
    PlainTag(&'a str),
    Quote(usize, Box<RawNode<'a>>),
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
    PlainTag(String),
    Quote(usize, Box<Node>),
    Plain(String),
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
            RawNode::Quote(n, child) => RawNode::Quote(n, Box::new(child.flatten())),
            RawNode::GlobalUser(_, _)
            | RawNode::LocalUser(_)
            | RawNode::LocalCustomEmoji(_)
            | RawNode::HashTag(_)
            | RawNode::PlainTag(_)
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
            RawNode::PlainTag(s) => Node::PlainTag(s.to_owned()),
            RawNode::Quote(n, child) => Node::Quote(n, Box::new(child.into_node())),
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
            Node::Quote(n, child) => {
                Node::Quote(n, Box::new(child.into_global(source_host.clone())))
            }
            Node::Empty
            | Node::GlobalUser(_, _)
            | Node::LocalCustomEmoji(_)
            | Node::HashTag(_)
            | Node::PlainTag(_)
            | Node::Plain(_) => self,
        }
    }
}

impl<'a> From<RawNode<'a>> for Node {
    fn from(raw: RawNode<'a>) -> Self {
        raw.flatten().into_node()
    }
}

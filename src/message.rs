const ADD_RE: &str = r#"\((.+)\)\+\+|(\S+)\+\+"#;
const SUB_RE: &str = r#"\((.+)\)--|(\S+)--"#;

pub enum Message<'a> {
    Ping,
    Incr(Vec<&'a str>),
    Decr(Vec<&'a str>),
    Karma(&'a str),
}

pub struct Parser {
    karma_add_re: regex::Regex,
    karma_sub_re: regex::Regex,
}

impl Parser {
    pub fn new() -> Self {
        let karma_add_re = regex::Regex::new(ADD_RE).unwrap();
        let karma_sub_re = regex::Regex::new(SUB_RE).unwrap();
        Parser {
            karma_add_re,
            karma_sub_re,
        }
    }

    pub fn parse<'a>(&self, msg: &'a str) -> Option<Message<'a>> {
        if msg == "!ping" {
            return Some(Message::Ping);
        }
        let add_re_matches = self.karma_add_re.captures_iter(&msg);
        let mut ret = Vec::new();
        for c in add_re_matches {
            if c.get(1).is_some() {
                ret.push(c.get(1).unwrap().as_str());
            } else if c.get(2).is_some() {
                ret.push(c.get(2).unwrap().as_str());
            } else {
                unreachable!();
            };
        }
        if ret.len() > 0 {
            return Some(Message::Incr(ret));
        }
        let sub_re_matches = self.karma_sub_re.captures_iter(&msg);
        for c in sub_re_matches {
            if c.get(1).is_some() {
                ret.push(c.get(1).unwrap().as_str());
            } else if c.get(2).is_some() {
                ret.push(c.get(2).unwrap().as_str());
            } else {
                unreachable!();
            };
        }
        if ret.len() > 0 {
            return Some(Message::Decr(ret));
        }
        if msg.starts_with("!karma ") {
            let name = msg.split_once(" ").unwrap().1;
            return Some(Message::Karma(name));
        }
        None
    }
}

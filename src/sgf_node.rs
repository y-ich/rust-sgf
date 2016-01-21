use std::collections::HashMap;
use std::fmt;
use regex::Regex;

pub type SgfPoint      = String;
pub type SgfColor      = char;
pub type SgfNumber     = i32;
pub type SgfReal       = f32;
pub type SgfDouble     = char;
pub type SgfText       = String;
pub type SgfSimpleText = String;

#[derive(Debug)]
pub struct SgfNode {
    properties: HashMap<String, Vec<String>>,
    pub children: Vec<SgfNode>,
}

impl fmt::Display for SgfNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = write!(f, "{{\n");
        for (key, value) in self.properties.iter() {
            let vstr = if value.len() == 1 {
                value[0].clone()
            } else {
                value.iter().map(|e| format!("[{}]", e)).fold("".to_string(), |acc, i| acc + &i)
            };
            result = result.and(write!(f, "    {}: {}\n", key, vstr));
        }
        result.and(write!(f, "}}"))
    }
}

#[test]
#[should_panic]
fn test_fmt() {
    let mut hash = HashMap::new();
    hash.insert("GC".to_string(), vec!["test".to_string()]);
    hash.insert("FF".to_string(), vec!["4".to_string()]);

    let n = SgfNode::new(hash);
    println!("{}", n);
    assert!(false)
}

impl SgfNode {
    pub fn new(properties: HashMap<String, Vec<String>>) -> SgfNode {
        SgfNode {
            properties: properties,
            children: Vec::new(),
        }
    }

    pub fn leaf_mut(&mut self) -> &mut SgfNode {
        if self.children.len() == 0 {
            self
        } else {
            self.children[0].leaf_mut()
        }
    }

    pub fn get_point(&self, id: &str) -> Option<SgfPoint> {
        self.properties.get(id).map(|v| v[0].clone())
    }

    pub fn set_point(&mut self, id: &str, value: SgfPoint) {
        self.set_property(id, vec![value.to_string()]); // to_string is redundant but looks like consistent.
    }

    pub fn get_number(&self, id: &str) -> Option<SgfNumber> {
        self.properties.get(id).and_then(|v| v[0].parse::<i32>().ok())
    }

    pub fn set_number(&mut self, id: &str, value: SgfNumber) {
        self.set_property(id, vec![value.to_string()]);
    }

    pub fn get_points(&self, id: &str) -> Option<Vec<SgfPoint>> {
        self.properties.get(id).cloned()
    }

    pub fn set_points(&mut self, id: &str, value: Vec<SgfPoint>) {
        self.set_property(id, value);
    }

    pub fn get_color(&self, id: &str) -> Option<SgfColor> {
        self.properties.get(id).and_then(|v| v[0].chars().next())
    }

    pub fn set_color(&mut self, id: &str, value: SgfColor) {
        self.set_property(id, vec![value.to_string()]);
    }

    pub fn get_double(&self, id: &str) -> Option<SgfDouble> {
        self.properties.get(id).and_then(|v| v[0].chars().next())
    }

    pub fn set_double(&mut self, id: &str, value: SgfDouble) {
        self.set_property(id, vec![value.to_string()]);
    }

    pub fn get_text(&self, id: &str) -> Option<SgfText> {
        self.properties.get(id).map(|v| decode_text(&v[0]))
    }

    pub fn set_text(&mut self, id: &str, value: String) {
        self.set_property(id, vec![encode_text(&value)]);
    }

    pub fn get_simple_text(&self, id: &str) -> Option<SgfSimpleText> {
        self.properties.get(id).map(|v| decode_simple_text(&v[0]))
    }

    pub fn set_simple_text(&mut self, id: &str, value: String) {
        self.set_property(id, vec![encode_text(&value)]);
    }

    pub fn get_real(&self, id: &str) -> Option<SgfReal> {
        self.properties.get(id).and_then(|v| v[0].parse::<f32>().ok())
    }

    pub fn set_real(&mut self, id: &str, value: SgfReal) {
        self.set_property(id, vec![value.to_string()]);
    }

    pub fn get_point_point(&self, id: &str) -> Option<(SgfPoint, SgfPoint)> {
        self.properties.get(id).and_then(|v| {
            let mut compose = v[0].splitn(2, ":");
            compose.next().and_then(|f|
                compose.next().map(|s| (f.to_string(), s.to_string())))
        })
    }

    pub fn set_point_point(&mut self, id: &str, value: (SgfPoint, SgfPoint)) {
        self.set_property(id, vec![format!("{}:{}", value.0, value.1)]);
    }

    pub fn get_point_simple_text(&self, id: &str) -> Option<(SgfPoint, SgfSimpleText)> {
        self.properties.get(id).and_then(|v| {
            let mut compose = v[0].splitn(2, ":");
            compose.next().and_then(|f|
                compose.next().map(|s| (f.to_string(), decode_simple_text(s))))
        })
    }

    pub fn set_point_simple_text(&mut self, id: &str, value: (SgfPoint, SgfSimpleText)) {
        self.set_property(id, vec![format!("{}:{}", value.0, encode_text(&value.1))]);
    }

    pub fn get_simple_text_simple_text(&self, id: &str) -> Option<(SgfSimpleText, SgfSimpleText)> {
        self.properties.get(id).and_then(|v| {
            let mut compose = v[0].splitn(2, ":");
            compose.next().and_then(|f|
                compose.next().map(|s| (decode_simple_text(f), decode_simple_text(s))))
        })
    }

    pub fn set_simple_text_simple_text(&mut self, id: &str, value: (SgfSimpleText, SgfSimpleText)) {
        self.set_property(id, vec![format!("{}:{}", encode_text(&value.0), encode_text(&value.1))]);
    }

    pub fn get_number_number(&self, id: &str) -> Option<(SgfNumber, SgfNumber)> {
        self.properties.get(id).and_then(|v| {
            let mut compose = v[0].splitn(2, ":");
            compose.next()
                .and_then(|f| f.parse::<i32>().ok())
                .and_then(|f| {
                    compose.next().and_then(|s|
                        match s.parse::<i32>() {
                            Ok(s) => Some((f, s)),
                            Err(_) => None,
                        }
                    )
                })
        })
    }

    pub fn set_number_number(&mut self, id: &str, value: (SgfNumber, SgfNumber)) {
        self.set_property(id, vec![format!("{}:{}", value.0, value.1)]);
    }

    pub fn get_number_simple_text(&self, id: &str) -> Option<(SgfNumber, SgfSimpleText)> {
        self.properties.get(id).and_then(|v| {
            let mut compose = v[0].splitn(2, ":");
            compose.next()
                .and_then(|f| f.parse::<i32>().ok())
                .and_then(|f| compose.next().map(|s| (f, decode_simple_text(s))))
        })
    }

    pub fn set_number_simple_text(&mut self, id: &str, value: (SgfNumber, SgfSimpleText)) {
        self.set_property(id, vec![format!("{}:{}", value.0, encode_text(&value.1))]);
    }

    fn set_property(&mut self, id: &str, value: Vec<String>) {
        self.properties.remove(id);
        self.properties.insert(id.to_string(), value);
    }
}

fn decode_text(s: &str) -> String {
    let s = Regex::new(r"\\(\r\n|\n\r|\n|\r)").unwrap().replace_all(s, ""); // soft line break
    let s = Regex::new(r"\\(.)").unwrap().replace_all(&s, "$1"); // escaping
    s
}

#[test]
fn test_decode_text() {
    assert_eq!(decode_text("[test\\\ntest\\:\\]"), "[testtest:]".to_string());
}

fn decode_simple_text(s: &str) -> String {
    let s = decode_text(s);
    let s = Regex::new(r"\r\n|\n\r|\n|\r").unwrap().replace_all(&s, " ");
    s
}

#[test]
fn test_decode_simple_text() {
    assert_eq!(decode_simple_text("test\ntest\r\ntest\n\rtest\rtest"), "test test test test test".to_string());
}

fn encode_text(s: &str) -> String {
    Regex::new(r"([\]\\:])").unwrap().replace_all(&s, "\\$1") // escaping
}

#[test]
fn test_encode_text() {
    assert_eq!(encode_text("]\\:"), "\\]\\\\\\:".to_string());
}

#[cfg(test)]
mod sgf_node_tests {
    use sgf_parse;

    #[test]
    fn test_parse() {
        let result = sgf_parse("(;CA[UTF-8]FF[4])");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_fail() {
        let result = sgf_parse("(;CA[UTF-8]FFF[4])");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_number() {
        let node = &sgf_parse("(;CA[UTF-8]FF[4])").unwrap()[0];
        assert_eq!(node.get_number("FF"), Some(4));
    }

    #[test]
    fn test_get_real() {
        let node = &sgf_parse("(;CA[UTF-8]FF[4]KM[6.5])").unwrap()[0];
        assert_eq!(node.get_real("KM"), Some(6.5));
    }

    #[test]
    fn test_get_points() {
        let node = &sgf_parse("(;CA[UTF-8]FF[4]KM[6.5]AB[ab])").unwrap()[0];
        assert_eq!(node.get_points("AB"), Some(vec!["ab".to_string()]));
    }

    #[test]
    fn test_get_point() {
        let node = &sgf_parse("(;CA[UTF-8]FF[4]KM[6.5];B[ab])").unwrap()[0];
        assert_eq!(node.children[0].get_point("B"), Some("ab".to_string()));
    }

    #[test]
    fn test_get_text() {
        let node = &sgf_parse("(;CA[UTF-8]FF[4]GC[text\ntext])").unwrap()[0];
        assert_eq!(node.get_text("GC"), Some("text\ntext".to_string()));
    }

    #[test]
    fn test_get_simple_text() {
        let node = &sgf_parse("(;CA[UTF-8]FF[4]N[simple\\\ntext\nsimple])").unwrap()[0];
        assert_eq!(node.get_simple_text("N"), Some("simpletext simple".to_string()));
    }

    #[test]
    fn test_get_simple_text_simple_text() {
        let node = &sgf_parse("(;CA[UTF-8]FF[4]AP[mimiaka:1.0])").unwrap()[0];
        assert_eq!(node.get_simple_text_simple_text("AP"), Some(("mimiaka".to_string(), "1.0".to_string())));
    }

    #[test]
    fn test_set_text() {
        let node = &mut sgf_parse("(;CA[UTF-8]FF[4]AP[mimiaka:1.0])").unwrap()[0];
        node.set_text("GC", "test:".to_string());
        assert_eq!(node.get_text("GC"), Some("test:".to_string()));
    }
}

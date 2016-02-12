use std::collections::HashMap;
use std::ops::{Index, IndexMut, Deref, DerefMut};
use std::fmt;
use regex::Regex;
use parser::*;

pub type SgfPoint      = String;
pub type SgfColor      = char;
pub type SgfNumber     = i32;
pub type SgfReal       = f32;
pub type SgfDouble     = char;
pub type SgfText       = String;
pub type SgfSimpleText = String;

#[derive(Debug)]
pub enum SgfError {
    NoProperties,
    EmptyProperty,
    ParseError,
}

/// SGF collection
#[derive(Debug)]
pub struct SgfCollection(Vec<SgfNode>);

impl SgfCollection {
    /// Parses a SGF string and returns a SgfCollection.
    ///
    /// # Example
    ///
    /// ```
    /// use sgf::*;
    ///
    /// let c = SgfCollection::from_sgf("(;CA[UTF-8]FF[4])");
    /// ```
    ///
    pub fn from_sgf(sgf_str: &str) -> ParseResult<SgfCollection> {
        collection(sgf_str)
    }

    pub fn new(games: Vec<SgfNode>) -> SgfCollection {
        SgfCollection(games)
    }
}

impl Deref for SgfCollection {
    type Target = [SgfNode];

    fn deref(&self) -> &[SgfNode] {
        &self.0
    }
}

impl DerefMut for SgfCollection {
    fn deref_mut(&mut self) -> &mut [SgfNode] {
        &mut self.0
    }
}

impl Index<usize> for SgfCollection {
    type Output = SgfNode;

    #[inline]
    fn index<'a>(&'a self, index: usize) -> &'a Self::Output {
        &(**self)[index]
    }
}

impl IndexMut<usize> for SgfCollection {
    #[inline]
    fn index_mut<'a>(&mut self, index: usize) -> &mut Self::Output {
        &mut (**self)[index]
    }
}

impl fmt::Display for SgfCollection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |acc, item|
            acc.and(write!(f, "({})", item))
        )
    }
}

#[cfg(test)]
mod test_sgf_collection {
    use sgf_node::*;
    #[test]
    fn test_from_sgf1() {
        let result = SgfCollection::from_sgf("(;CA[UTF-8]FF[4])");
        assert!(result.is_ok());
    }

    #[test]
    fn test_from_sgf2() {
        let node = &SgfCollection::from_sgf("(;FF[4]C[root](;C[a];C[b](;C[c])
            (;C[d];C[e]))
            (;C[f](;C[g];C[h];C[i])
            (;C[j])))").unwrap()[0];
        assert!(node.children.len() == 2 && node.children[0].children[0].children.len() == 2);
    }

    #[test]
    fn test_from_sgf_long_propid() {
        let result = SgfCollection::from_sgf("(;CA[UTF-8]LONGNAME[])");
        assert!(result.is_ok());
    }

    #[test]
    fn test_fmt() {
        let sgf = "(;FF[4];C[a];C[b](;C[c])(;C[d];C[e])(;C[f](;C[g];C[h];C[i])(;C[j])))";
        let collection = SgfCollection::from_sgf(sgf).unwrap();
        let string = format!("{}", collection);
        assert_eq!(&string, sgf);
    }

    #[test]
    fn test_index() {
        let sgf = "(;FF[4]GC[game1])(;FF[4]GC[game2])";
        let collection = SgfCollection::from_sgf(sgf).unwrap();
        assert_eq!(collection[1].get_simple_text("GC").unwrap(), "game2".to_string());
    }
}

/// SGF node with children. It means that a node also represents game tree.
/// Access the field 'children' directly to traverse in its tree.
/// To access SGF properties of the node, use various accessors below.
pub struct SgfNode {
    properties: HashMap<String, Vec<String>>,
    pub children: Vec<SgfNode>,
}

impl fmt::Debug for SgfNode {
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

impl fmt::Display for SgfNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = write!(f, ";");
        for (key, value) in self.properties.iter() {
            result = result.and(write!(f, "{}", key));
            for v in value {
                result = result.and(write!(f, "[{}]", v));
            }
        }
        if self.children.len() == 1 {
            result = result.and(write!(f, "{}", self.children[0]));
        } else {
            for child in self.children.iter() {
                result = result.and(write!(f, "({})", child));
            }
        }
        result
    }
}

#[test]
#[should_panic]
fn test_debug_fmt() {
    let mut hash = HashMap::new();
    hash.insert("GC".to_string(), vec!["test".to_string()]);
    hash.insert("FF".to_string(), vec!["4".to_string()]);

    let n = SgfNode::new(hash);
    println!("{:?}", n);
    assert!(false)
}

impl SgfNode {
    /// Constructor.
    /// Returns an SgfNode with given propertes.
    /// Properties should be stored in a HashMap in String name and a vector of String value.
    pub fn new(properties: HashMap<String, Vec<String>>) -> SgfNode {
        SgfNode {
            properties: properties,
            children: Vec::new(),
        }
    }

    /// Returns a mutable reference of a leaf node in main line
    /// This is for the parser.
    pub fn leaf_mut(&mut self) -> &mut SgfNode {
        if self.children.len() == 0 {
            self
        } else {
            self.children[0].leaf_mut()
        }
    }

    fn get_property(&self, id: &str) -> Result<&Vec<String>, SgfError> {
        self.properties.get(id).ok_or(SgfError::NoProperties)
    }

    fn set_property(&mut self, id: &str, value: Vec<String>) -> &mut Self {
        self.properties.remove(id);
        self.properties.insert(id.to_string(), value);
        self
    }

    /// Returns a Result of id's value as SgfPoint.
    pub fn get_point(&self, id: &str) -> Result<SgfPoint, SgfError> {
        self.get_property(id).map(|v| v[0].clone())
    }

    /// Sets an SgfpPoint value to property id.
    pub fn set_point(&mut self, id: &str, value: SgfPoint) -> &mut Self {
        self.set_property(id, vec![value.to_string()]) // to_string is redundant but looks like consistent.
    }

    /// Returns a Result of id's value as SgfNumber.
    pub fn get_number(&self, id: &str) -> Result<SgfNumber, SgfError> {
        self.get_property(id).and_then(|v| v[0].parse::<i32>().map_err(|_| SgfError::ParseError))
    }

    /// Sets an SgfpNumber value to property id.
    pub fn set_number(&mut self, id: &str, value: SgfNumber) -> &mut Self {
        self.set_property(id, vec![value.to_string()])
    }

    /// Returns a Result of id's value as a vector of SgfPoint.
    pub fn get_points(&self, id: &str) -> Result<Vec<SgfPoint>, SgfError> {
        self.properties.get(id).cloned().ok_or(SgfError::NoProperties)
    }

    /// Sets an SgfPoint vector to property id.
    pub fn set_points(&mut self, id: &str, value: Vec<SgfPoint>) -> &mut Self {
        self.set_property(id, value)
    }

    /// Returns a Result of id's value as SgfColor.
    pub fn get_color(&self, id: &str) -> Result<SgfColor, SgfError> {
        self.get_property(id).and_then(|v| v[0].chars().next().ok_or(SgfError::EmptyProperty))
    }

    /// Sets an SgfColor to property id.
    pub fn set_color(&mut self, id: &str, value: SgfColor) -> &mut Self {
        self.set_property(id, vec![value.to_string()])
    }

    /// Returns a Result of id's value as SgfDouble.
    pub fn get_double(&self, id: &str) -> Result<SgfDouble, SgfError> {
        self.get_property(id).and_then(|v| v[0].chars().next().ok_or(SgfError::EmptyProperty))
    }

    /// Sets an SgfDouble to property id.
    pub fn set_double(&mut self, id: &str, value: SgfDouble) -> &mut Self {
        self.set_property(id, vec![value.to_string()])
    }

    /// Returns a Result of id's value as SgfText.
    pub fn get_text(&self, id: &str) -> Result<SgfText, SgfError> {
        self.get_property(id).map(|v| decode_text(&v[0]))
    }

    /// Sets an SgfText to property id.
    pub fn set_text(&mut self, id: &str, value: String) -> &mut Self {
        self.set_property(id, vec![encode_text(&value)])
    }

    /// Returns a Result of id's value as SgfSimpleText.
    pub fn get_simple_text(&self, id: &str) -> Result<SgfSimpleText, SgfError> {
        self.get_property(id).map(|v| decode_simple_text(&v[0]))
    }

    /// Sets an SgfSimpleText to property id.
    pub fn set_simple_text(&mut self, id: &str, value: String) -> &mut Self {
        self.set_property(id, vec![encode_text(&value)])
    }

    /// Returns a Result of id's value as SgfReal.
    pub fn get_real(&self, id: &str) -> Result<SgfReal, SgfError> {
        self.get_property(id).and_then(|v| v[0].parse::<f32>().map_err(|_| SgfError::ParseError))
    }

    /// Sets an SgfReal to property id.
    pub fn set_real(&mut self, id: &str, value: SgfReal) -> &mut Self {
        self.set_property(id, vec![value.to_string()])
    }

    /// Returns a Result of id's value as Compose of SgfPoints.
    pub fn get_point_point(&self, id: &str) -> Result<(SgfPoint, SgfPoint), SgfError> {
        self.get_property(id).and_then(|v| {
            let mut compose = v[0].splitn(2, ":");
            compose.next().ok_or(SgfError::EmptyProperty).and_then(|f|
                compose.next().ok_or(SgfError::EmptyProperty).map(|s| (f.to_string(), s.to_string())))
        })
    }

    /// Sets a compose of SgfPoints to property id.
    pub fn set_point_point(&mut self, id: &str, value: (SgfPoint, SgfPoint)) -> &mut Self {
        self.set_property(id, vec![format!("{}:{}", value.0, value.1)])
    }

    /// Returns a Result of id's value as Compose of SgfPoint and SgfSimpleText.
    pub fn get_point_simple_text(&self, id: &str) -> Result<(SgfPoint, SgfSimpleText), SgfError> {
        self.get_property(id).and_then(|v| {
            let mut compose = v[0].splitn(2, ":");
            compose.next().ok_or(SgfError::EmptyProperty).and_then(|f|
                compose.next().ok_or(SgfError::EmptyProperty).map(|s| (f.to_string(), decode_simple_text(s))))
        })
    }

    /// Sets a compose of SgfPoint and SgfSimpleText to property id.
    pub fn set_point_simple_text(&mut self, id: &str, value: (SgfPoint, SgfSimpleText)) -> &mut Self {
        self.set_property(id, vec![format!("{}:{}", value.0, encode_text(&value.1))])
    }

    /// Returns a Result of id's value as Compose of SgfSimpleTexts.
    pub fn get_simple_text_simple_text(&self, id: &str) -> Result<(SgfSimpleText, SgfSimpleText), SgfError> {
        self.get_property(id).and_then(|v| {
            let mut compose = v[0].splitn(2, ":");
            compose.next().ok_or(SgfError::EmptyProperty).and_then(|f|
                compose.next().ok_or(SgfError::EmptyProperty).map(|s| (decode_simple_text(f), decode_simple_text(s))))
        })
    }

    /// Sets a compose of SgfSimpleTexts to property id.
    pub fn set_simple_text_simple_text(&mut self, id: &str, value: (SgfSimpleText, SgfSimpleText)) -> &mut Self {
        self.set_property(id, vec![format!("{}:{}", encode_text(&value.0), encode_text(&value.1))])
    }

    /// Returns a Result of id's value as Compose of SgfNumbers.
    pub fn get_number_number(&self, id: &str) -> Result<(SgfNumber, SgfNumber), SgfError> {
        self.get_property(id).and_then(|v| {
            let mut compose = v[0].splitn(2, ":");
            compose.next().ok_or(SgfError::EmptyProperty)
                .and_then(|f| f.parse::<i32>().map_err(|_| SgfError::ParseError))
                .and_then(|f| {
                    compose.next().ok_or(SgfError::EmptyProperty).and_then(|s|
                        match s.parse::<i32>() {
                            Ok(s) => Ok((f, s)),
                            Err(_) => Err(SgfError::ParseError),
                        }
                    )
                })
        })
    }

    /// Sets a compose of SgfNumbers to property id.
    pub fn set_number_number(&mut self, id: &str, value: (SgfNumber, SgfNumber)) -> &mut Self {
        self.set_property(id, vec![format!("{}:{}", value.0, value.1)])
    }

    /// Returns a Result of id's value as Compose of SgfNumber and SgfSimpleText.
    pub fn get_number_simple_text(&self, id: &str) -> Result<(SgfNumber, SgfSimpleText), SgfError> {
        self.get_property(id).and_then(|v| {
            let mut compose = v[0].splitn(2, ":");
            compose.next().ok_or(SgfError::EmptyProperty)
                .and_then(|f| f.parse::<i32>().map_err(|_| SgfError::ParseError))
                .and_then(|f| compose.next().ok_or(SgfError::EmptyProperty).map(|s| (f, decode_simple_text(s))))
        })
    }

    /// Sets a compose of SgfNumber and SgfSimpleText to property id.
    pub fn set_number_simple_text(&mut self, id: &str, value: (SgfNumber, SgfSimpleText)) -> &mut Self {
        self.set_property(id, vec![format!("{}:{}", value.0, encode_text(&value.1))])
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
    use sgf_node::*;
    #[test]
    fn test_get_number() {
        let node = &SgfCollection::from_sgf("(;CA[UTF-8]FF[4])").unwrap()[0];
        assert_eq!(node.get_number("FF").unwrap(), 4);
    }

    #[test]
    fn test_get_real() {
        let node = &SgfCollection::from_sgf("(;CA[UTF-8]FF[4]KM[6.5])").unwrap()[0];
        assert_eq!(node.get_real("KM").unwrap(), 6.5);
    }

    #[test]
    fn test_get_points() {
        let node = &SgfCollection::from_sgf("(;CA[UTF-8]FF[4]KM[6.5]AB[ab])").unwrap()[0];
        assert_eq!(node.get_points("AB").unwrap(), vec!["ab".to_string()]);
    }

    #[test]
    fn test_get_point() {
        let node = &SgfCollection::from_sgf("(;CA[UTF-8]FF[4]KM[6.5];B[ab])").unwrap()[0];
        assert_eq!(node.children[0].get_point("B").unwrap(), "ab".to_string());
    }

    #[test]
    fn test_get_text() {
        let node = &SgfCollection::from_sgf("(;CA[UTF-8]FF[4]GC[text\ntext])").unwrap()[0];
        assert_eq!(node.get_text("GC").unwrap(), "text\ntext".to_string());
    }

    #[test]
    fn test_get_simple_text() {
        let node = &SgfCollection::from_sgf("(;CA[UTF-8]FF[4]N[simple\\\ntext\nsimple])").unwrap()[0];
        assert_eq!(node.get_simple_text("N").unwrap(), "simpletext simple".to_string());
    }

    #[test]
    fn test_get_simple_text_simple_text() {
        let node = &SgfCollection::from_sgf("(;CA[UTF-8]FF[4]AP[mimiaka:1.0])").unwrap()[0];
        assert_eq!(node.get_simple_text_simple_text("AP").unwrap(), ("mimiaka".to_string(), "1.0".to_string()));
    }

    #[test]
    fn test_set_text() {
        let node = &mut SgfCollection::from_sgf("(;CA[UTF-8]FF[4]AP[mimiaka:1.0])").unwrap()[0];
        node.set_text("GC", "test:".to_string());
        assert_eq!(node.get_text("GC").unwrap(), "test:".to_string());
    }
}

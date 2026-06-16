//! Shared XML parsing utilities built on the workspace `quick-xml` reader.
//!
//! Rather than have each legal-XML format drive the streaming reader by hand,
//! this module parses a document once into a small, owned [`XmlNode`] tree. The
//! tree carries local element names (namespace prefixes stripped), attributes
//! and concatenated text content, which is exactly the surface the format
//! parsers need to reconstruct their typed models. Using `quick-xml` for the
//! tokenisation gives us correct entity decoding and attribute parsing for free.

use crate::DiffError;
use quick_xml::Reader;
use quick_xml::events::{BytesRef, BytesStart, Event};

use super::xml_error;

/// A parsed XML element: its (prefix-stripped) name, attributes, direct text and
/// child elements.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct XmlNode {
    /// Local element name, with any namespace prefix removed.
    pub(crate) name: String,
    /// Attributes as `(local_name, value)` pairs, prefix-stripped.
    pub(crate) attributes: Vec<(String, String)>,
    /// Concatenated, decoded text directly within this element.
    pub(crate) text: String,
    /// Child elements in document order.
    pub(crate) children: Vec<XmlNode>,
}

impl XmlNode {
    fn new(name: String, attributes: Vec<(String, String)>) -> Self {
        Self {
            name,
            attributes,
            text: String::new(),
            children: Vec::new(),
        }
    }

    /// Returns the value of the named attribute, if present.
    pub(crate) fn attr(&self, name: &str) -> Option<&str> {
        self.attributes
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }

    /// Returns the first direct child element with the given local name.
    pub(crate) fn child(&self, name: &str) -> Option<&XmlNode> {
        self.children.iter().find(|c| c.name == name)
    }

    /// Returns all direct child elements with the given local name.
    pub(crate) fn children_named<'a>(
        &'a self,
        name: &'a str,
    ) -> impl Iterator<Item = &'a XmlNode> + 'a {
        self.children.iter().filter(move |c| c.name == name)
    }

    /// Recursively finds the first descendant (or self) with the given name.
    pub(crate) fn find_descendant(&self, name: &str) -> Option<&XmlNode> {
        if self.name == name {
            return Some(self);
        }
        for child in &self.children {
            if let Some(found) = child.find_descendant(name) {
                return Some(found);
            }
        }
        None
    }

    /// The trimmed text content of this element.
    pub(crate) fn trimmed_text(&self) -> &str {
        self.text.trim()
    }
}

/// Strips an optional `prefix:` from a tag or attribute name, returning the
/// local part as an owned `String`.
fn local_name(raw: &[u8]) -> String {
    let s = String::from_utf8_lossy(raw);
    match s.rsplit_once(':') {
        Some((_, local)) => local.to_string(),
        None => s.into_owned(),
    }
}

/// Resolves an XML entity reference to its replacement text.
///
/// `quick-xml` 0.40 emits entity references (`&amp;`, `&#10;`, …) as standalone
/// [`Event::GeneralRef`] events instead of inlining them into the surrounding
/// text, so callers must turn them back into characters themselves. Numeric
/// character references are resolved directly; named references are looked up in
/// the set of predefined XML entities.
fn resolve_entity_ref(reference: &BytesRef<'_>) -> Result<String, DiffError> {
    if let Some(ch) = reference
        .resolve_char_ref()
        .map_err(|e| xml_error("xml character reference", e))?
    {
        return Ok(ch.to_string());
    }

    let name = reference
        .decode()
        .map_err(|e| xml_error("xml entity reference", e))?;
    quick_xml::escape::resolve_predefined_entity(&name)
        .map(str::to_string)
        .ok_or_else(|| {
            xml_error(
                "xml entity reference",
                format!("unknown entity '&{};'", name),
            )
        })
}

/// Builds an [`XmlNode`] from a start/empty tag, decoding its attributes.
fn node_from_start(start: &BytesStart<'_>) -> Result<XmlNode, DiffError> {
    let name = local_name(start.name().as_ref());
    let mut attributes = Vec::new();
    for attr in start.attributes() {
        let attr = attr.map_err(|e| xml_error("xml attribute", e))?;
        // Skip namespace declarations; they are not part of the data model.
        let key_raw = attr.key.as_ref();
        if key_raw == b"xmlns" || key_raw.starts_with(b"xmlns:") {
            continue;
        }
        let key = local_name(key_raw);
        // `Implicit1_0` reproduces the (now deprecated) `unescape_value()`
        // behaviour: decode UTF-8 and resolve the predefined XML entities.
        let value = attr
            .normalized_value(quick_xml::XmlVersion::Implicit1_0)
            .map_err(|e| xml_error("xml attribute value", e))?
            .into_owned();
        attributes.push((key, value));
    }
    Ok(XmlNode::new(name, attributes))
}

/// Parses an XML document into a single root [`XmlNode`].
///
/// # Errors
///
/// Returns [`DiffError::SerializationError`] if the document is malformed or has
/// no root element.
pub(crate) fn parse_document(xml: &str) -> Result<XmlNode, DiffError> {
    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(false);

    // Stack of elements currently open; the last is the innermost.
    let mut stack: Vec<XmlNode> = Vec::new();
    let mut root: Option<XmlNode> = None;

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref start)) => {
                stack.push(node_from_start(start)?);
            }
            Ok(Event::Empty(ref start)) => {
                let node = node_from_start(start)?;
                match stack.last_mut() {
                    Some(parent) => parent.children.push(node),
                    None => root = Some(node),
                }
            }
            Ok(Event::End(_)) => {
                let finished = stack
                    .pop()
                    .ok_or_else(|| xml_error("xml structure", "unbalanced closing tag"))?;
                match stack.last_mut() {
                    Some(parent) => parent.children.push(finished),
                    None => root = Some(finished),
                }
            }
            Ok(Event::Text(text)) => {
                let decoded = text
                    .decode()
                    .map_err(|e| xml_error("xml text", e))?
                    .into_owned();
                if let Some(current) = stack.last_mut() {
                    current.text.push_str(&decoded);
                }
            }
            Ok(Event::GeneralRef(reference)) => {
                let resolved = resolve_entity_ref(&reference)?;
                if let Some(current) = stack.last_mut() {
                    current.text.push_str(&resolved);
                }
            }
            Ok(Event::CData(cdata)) => {
                // CDATA is taken verbatim as text content.
                let decoded = String::from_utf8_lossy(cdata.as_ref()).into_owned();
                if let Some(current) = stack.last_mut() {
                    current.text.push_str(&decoded);
                }
            }
            Ok(Event::Eof) => break,
            Ok(_) => {}
            Err(e) => return Err(xml_error("xml parse", e)),
        }
    }

    if !stack.is_empty() {
        return Err(xml_error(
            "xml structure",
            "unclosed element(s) at end of input",
        ));
    }

    root.ok_or_else(|| xml_error("xml structure", "document has no root element"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple() {
        let xml = r#"<root id="1"><child>hello</child></root>"#;
        let node = parse_document(xml).expect("parse");
        assert_eq!(node.name, "root");
        assert_eq!(node.attr("id"), Some("1"));
        let child = node.child("child").expect("child");
        assert_eq!(child.trimmed_text(), "hello");
    }

    #[test]
    fn test_namespace_prefix_stripped() {
        let xml = r#"<an:act xmlns:an="urn:x"><an:body>b</an:body></an:act>"#;
        let node = parse_document(xml).expect("parse");
        assert_eq!(node.name, "act");
        assert!(node.child("body").is_some());
    }

    #[test]
    fn test_entity_decoding() {
        let xml = r#"<root>a &amp; b &lt; c</root>"#;
        let node = parse_document(xml).expect("parse");
        assert_eq!(node.trimmed_text(), "a & b < c");
    }

    #[test]
    fn test_attribute_entity_decoding() {
        let xml = r#"<root title="a &quot;b&quot;"/>"#;
        let node = parse_document(xml).expect("parse");
        assert_eq!(node.attr("title"), Some("a \"b\""));
    }

    #[test]
    fn test_empty_element() {
        let xml = r#"<root><leaf v="1"/></root>"#;
        let node = parse_document(xml).expect("parse");
        let leaf = node.child("leaf").expect("leaf");
        assert_eq!(leaf.attr("v"), Some("1"));
        assert!(leaf.children.is_empty());
    }

    #[test]
    fn test_children_named() {
        let xml = r#"<root><a>1</a><a>2</a><b>3</b></root>"#;
        let node = parse_document(xml).expect("parse");
        assert_eq!(node.children_named("a").count(), 2);
        assert_eq!(node.children_named("b").count(), 1);
    }

    #[test]
    fn test_find_descendant() {
        let xml = r#"<a><b><c>deep</c></b></a>"#;
        let node = parse_document(xml).expect("parse");
        let c = node.find_descendant("c").expect("c");
        assert_eq!(c.trimmed_text(), "deep");
    }

    #[test]
    fn test_unbalanced_errors() {
        assert!(parse_document("<a><b></a>").is_err());
    }

    #[test]
    fn test_no_root_errors() {
        assert!(parse_document("   ").is_err());
    }

    #[test]
    fn test_namespace_declaration_not_attribute() {
        let xml = r#"<root xmlns="urn:x" id="7"/>"#;
        let node = parse_document(xml).expect("parse");
        assert_eq!(node.attr("id"), Some("7"));
        // xmlns must not leak into the attribute set.
        assert!(node.attr("xmlns").is_none());
    }
}

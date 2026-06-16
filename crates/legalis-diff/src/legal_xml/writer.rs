//! A small, dependency-free indented XML writer.
//!
//! The legal-XML formats in this module emit human-readable, indented documents
//! with a stable element ordering. Rather than thread a streaming writer through
//! every format, this builder offers a tiny tree-free API: callers open and
//! close elements (or write self-contained leaf elements) and the builder tracks
//! indentation and performs correct XML escaping of both text content and
//! attribute values.
//!
//! Escaping follows the XML 1.0 rules: `&`, `<`, `>` in content and
//! additionally `"` in attribute values.

/// An incremental, indentation-aware XML document builder.
#[derive(Debug)]
pub(crate) struct XmlBuilder {
    buffer: String,
    depth: usize,
    indent: &'static str,
}

impl XmlBuilder {
    /// Creates a new builder emitting the standard XML declaration.
    pub(crate) fn new() -> Self {
        let mut buffer = String::new();
        buffer.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
        Self {
            buffer,
            depth: 0,
            indent: "  ",
        }
    }

    fn write_indent(&mut self) {
        for _ in 0..self.depth {
            self.buffer.push_str(self.indent);
        }
    }

    /// Opens an element with optional attributes, leaving it open for children.
    pub(crate) fn open(&mut self, name: &str, attrs: &[(&str, &str)]) {
        self.write_indent();
        self.buffer.push('<');
        self.buffer.push_str(name);
        self.write_attrs(attrs);
        self.buffer.push_str(">\n");
        self.depth += 1;
    }

    /// Closes the most recently opened element.
    pub(crate) fn close(&mut self, name: &str) {
        self.depth = self.depth.saturating_sub(1);
        self.write_indent();
        self.buffer.push_str("</");
        self.buffer.push_str(name);
        self.buffer.push_str(">\n");
    }

    /// Writes a leaf element with text content on a single line.
    pub(crate) fn leaf(&mut self, name: &str, attrs: &[(&str, &str)], text: &str) {
        self.write_indent();
        self.buffer.push('<');
        self.buffer.push_str(name);
        self.write_attrs(attrs);
        self.buffer.push('>');
        self.buffer.push_str(&escape_text(text));
        self.buffer.push_str("</");
        self.buffer.push_str(name);
        self.buffer.push_str(">\n");
    }

    /// Writes a self-closing empty element with optional attributes.
    pub(crate) fn empty(&mut self, name: &str, attrs: &[(&str, &str)]) {
        self.write_indent();
        self.buffer.push('<');
        self.buffer.push_str(name);
        self.write_attrs(attrs);
        self.buffer.push_str("/>\n");
    }

    fn write_attrs(&mut self, attrs: &[(&str, &str)]) {
        for (key, value) in attrs {
            self.buffer.push(' ');
            self.buffer.push_str(key);
            self.buffer.push_str("=\"");
            self.buffer.push_str(&escape_attr(value));
            self.buffer.push('"');
        }
    }

    /// Consumes the builder and returns the rendered document.
    pub(crate) fn finish(self) -> String {
        self.buffer
    }
}

/// Escapes a string for use as XML element text content.
pub(crate) fn escape_text(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            other => out.push(other),
        }
    }
    out
}

/// Escapes a string for use inside a double-quoted XML attribute value.
pub(crate) fn escape_attr(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            other => out.push(other),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_declaration_present() {
        let b = XmlBuilder::new();
        assert!(b.finish().starts_with("<?xml version=\"1.0\""));
    }

    #[test]
    fn test_nested_elements() {
        let mut b = XmlBuilder::new();
        b.open("root", &[("id", "1")]);
        b.leaf("child", &[], "value");
        b.close("root");
        let out = b.finish();
        assert!(out.contains("<root id=\"1\">"));
        assert!(out.contains("<child>value</child>"));
        assert!(out.contains("</root>"));
    }

    #[test]
    fn test_empty_element() {
        let mut b = XmlBuilder::new();
        b.empty("br", &[("kind", "x")]);
        assert!(b.finish().contains("<br kind=\"x\"/>"));
    }

    #[test]
    fn test_escape_text() {
        assert_eq!(escape_text("a & b < c > d"), "a &amp; b &lt; c &gt; d");
    }

    #[test]
    fn test_escape_attr_quotes() {
        assert_eq!(escape_attr("he said \"hi\""), "he said &quot;hi&quot;");
    }

    #[test]
    fn test_indentation_increases() {
        let mut b = XmlBuilder::new();
        b.open("a", &[]);
        b.open("b", &[]);
        b.leaf("c", &[], "x");
        b.close("b");
        b.close("a");
        let out = b.finish();
        // <c> sits two levels deep (inside <a> then <b>): four spaces of indent.
        assert!(out.contains("    <c>x</c>"));
    }
}

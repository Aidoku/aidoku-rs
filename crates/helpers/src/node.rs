use aidoku_imports::html::Node;
use alloc::string::String;

pub trait NodeHelpers {
    /// Get the text of the element and its children. It's different from
    /// [aidoku_imports::html::Node::text] in that `<p>` and `<br>` are considered
    /// and will insert linebreaks.
    fn text_with_newlines(&self) -> String;
}

impl NodeHelpers for Node {
    fn text_with_newlines(&self) -> String {
        if !self.select("p").array().is_empty() {
            Node::new_fragment(self.html().read().replace("<br>", "\\n<br>"))
                .map(|node| {
                    let mut ret = String::new();
                    for p in node.select("p").array() {
                        if let Ok(pnode) = p.as_node() {
                            ret.push_str(
                                pnode
                                    .text()
                                    .read()
                                    .replace("\\n", "\n")
                                    .replace("\n ", "\n")
                                    .trim(),
                            );
                            ret.push('\n');
                        }
                    }
                    ret
                })
                .unwrap_or_default()
        } else {
            Node::new_fragment(self.html().read().replace("<br>", "\n<br>"))
                .map(|v| v.select("body").untrimmed_text().read())
                .unwrap_or_default()
        }
    }
}

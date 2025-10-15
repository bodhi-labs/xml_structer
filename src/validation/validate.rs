use super::report::{Report, Severity};
use roxmltree::{Document, Node};

pub fn run(xml: &str) -> anyhow::Result<Report> {
    let mut rep = Report::new();

    let doc = match Document::parse(xml) {
        Ok(d) => d,
        Err(e) => {
            rep.push(
                e.pos().row as usize,
                e.pos().col as usize,
                format!("XML parsing error: {}", e),
                Severity::Error,
            );
            return Ok(rep);
        }
    };

    // well-formed extras
    if xml.starts_with('\u{FEFF}') {
        rep.push(
            1,
            1,
            "UTF-8 BOM detected (harmless but unnecessary)",
            Severity::Info,
        );
    }

    // TEI rules
    let root = doc.root_element();
    validate_root(root, &mut rep);
    walk(root, &mut rep);

    Ok(rep)
}

pub fn validate_root(root: Node, rep: &mut Report) {
    let tag_name = root.tag_name().name().to_lowercase();
    if tag_name.contains("tei") {
        // Valid TEI root element
    } else {
        rep.push(
            0,
            0,
            format!(
                "Root element should contain 'tei' (case-insensitive), found <{}>",
                root.tag_name().name()
            ),
            Severity::Warning,
        );
    }
}

pub fn walk(node: Node, rep: &mut Report) {
    match node.tag_name().name() {
        "pb" => {
            if node.attribute("ed").is_none() {
                let (line, column) = node_pos(node);
                rep.push(line, column, "<pb> missing @ed", Severity::Error);
            }
            if node.attribute("n").is_none() {
                let (line, column) = node_pos(node);
                rep.push(line, column, "<pb> missing @n", Severity::Error);
            }
        }
        "head" => {
            if !node.ancestors().any(|a| a.tag_name().name() == "div") {
                let (line, column) = node_pos(node);
                rep.push(
                    line,
                    column,
                    "<head> should be inside <div>",
                    Severity::Warning,
                );
            }
        }
        _ => {}
    }
    for child in node.children().filter(|n| n.is_element()) {
        walk(child, rep);
    }
}

fn node_pos(n: Node) -> (usize, usize) {
    let pos = n.document().text_pos_at(n.range().start);
    (pos.row as usize, pos.col as usize)
}

/* FastNote Rust/GTK4 Edition — Markdown renderer */

pub fn render_markdown(md: &str) -> String {
    let escaped = html_escape(md);
    let mut html = String::from(
        "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><title>FastNote</title>\
        <style>body{font-family:sans-serif;max-width:800px;margin:auto;padding:1em}\
        code{background:#f4f4f4;padding:2px 4px;border-radius:3px}\
        pre{background:#f4f4f4;padding:1em;overflow-x:auto;border-radius:4px}\
        </style></head><body>\n",
    );

    for line in escaped.lines() {
        if line.is_empty() {
            html.push_str("<br>\n");
        } else if let Some(s) = line.strip_prefix("# ") {
            html.push_str(&format!("<h1>{}</h1>\n", s));
        } else if let Some(s) = line.strip_prefix("## ") {
            html.push_str(&format!("<h2>{}</h2>\n", s));
        } else if let Some(s) = line.strip_prefix("### ") {
            html.push_str(&format!("<h3>{}</h3>\n", s));
        } else if line.starts_with("**") && line.ends_with("**") && line.len() > 4 {
            html.push_str(&format!("<strong>{}</strong>\n", &line[2..line.len()-2]));
        } else if line.starts_with("*") && line.ends_with("*") && line.len() > 3 {
            html.push_str(&format!("<em>{}</em>\n", &line[1..line.len()-1]));
        } else if line.starts_with("- ") {
            html.push_str(&format!("<li>{}</li>\n", &line[2..]));
        } else if line.starts_with("`") && line.ends_with("`") && line.len() > 2 {
            html.push_str(&format!("<code>{}</code>\n", &line[1..line.len()-1]));
        } else {
            html.push_str(&format!("<p>{}</p>\n", line));
        }
    }

    html.push_str("</body></html>\n");
    html
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

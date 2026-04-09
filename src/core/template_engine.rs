use std::collections::HashMap;

use crate::core::result::{ReasonCode, YaswitchError};

const BEGIN_MARKER: &str = "yaswitch:begin";
const END_MARKER: &str = "yaswitch:end";

pub fn render_template(
    template: &str,
    context: &HashMap<String, String>,
) -> Result<String, YaswitchError> {
    let mut rendered = template.to_owned();

    for (key, value) in context {
        let placeholder = format!("{{{{{key}}}}}");
        rendered = rendered.replace(&placeholder, value);
    }

    let mut unresolved_keys = Vec::new();
    let mut current = rendered.as_str();
    while let Some(open_idx) = current.find("{{") {
        let after_open = &current[open_idx + 2..];
        if let Some(close_idx) = after_open.find("}}") {
            let key = after_open[..close_idx].trim();
            if !key.is_empty() {
                unresolved_keys.push(key.to_string());
            }
            current = &after_open[close_idx + 2..];
        } else {
            break;
        }
    }

    if !unresolved_keys.is_empty() {
        unresolved_keys.sort();
        unresolved_keys.dedup();
        return Err(YaswitchError::new(
            ReasonCode::TemplateKeyMissing,
            format!("missing template keys: {}", unresolved_keys.join(", ")),
        ));
    }

    Ok(rendered)
}

pub fn inject_managed_block(
    original: &str,
    managed_content: &str,
    allow_append: bool,
) -> Result<String, YaswitchError> {
    let replacement = format!(
        "# {BEGIN_MARKER}\n{}\n# {END_MARKER}",
        managed_content.trim_end()
    );

    let begin = format!("# {BEGIN_MARKER}");
    let end = format!("# {END_MARKER}");

    if let Some(begin_idx) = original.find(&begin) {
        let remainder = &original[begin_idx + begin.len()..];
        if let Some(end_rel_idx) = remainder.find(&end) {
            let end_idx = begin_idx + begin.len() + end_rel_idx + end.len();
            let mut output = String::new();
            output.push_str(&original[..begin_idx]);
            output.push_str(&replacement);
            output.push_str(&original[end_idx..]);
            return Ok(output);
        }
    }

    if allow_append {
        if original.trim().is_empty() {
            Ok(format!("{replacement}\n"))
        } else {
            Ok(format!("{}\n\n{replacement}\n", original.trim_end()))
        }
    } else {
        Err(YaswitchError::new(
            ReasonCode::MarkerNotFoundAppendDisabled,
            "managed marker block not found and append is disabled",
        ))
    }
}

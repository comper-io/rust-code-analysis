use regex::Regex;

/// Represents a section extracted from a Vue SFC
#[derive(Debug, Clone)]
pub struct VueSection {
    pub content: Vec<u8>,
    pub start_line: usize,
    pub lang: Option<String>,
}

/// All sections extracted from a Vue SFC
#[derive(Debug, Default)]
pub struct VueSections {
    pub template: Option<VueSection>,
    pub script: Option<VueSection>,
    pub style: Option<VueSection>,
}

/// Extract all sections from a Vue Single File Component
pub fn extract_vue_sections(source: &[u8]) -> VueSections {
    let source_str = String::from_utf8_lossy(source);
    let mut sections = VueSections::default();

    // Extract template section
    if let Some(template) = extract_section(&source_str, "template") {
        sections.template = Some(template);
    }

    // Extract script section
    if let Some(script) = extract_section(&source_str, "script") {
        sections.script = Some(script);
    }

    // Extract style section
    if let Some(style) = extract_section(&source_str, "style") {
        sections.style = Some(style);
    }

    sections
}

/// Extract a single section from the Vue file
fn extract_section(source: &str, tag: &str) -> Option<VueSection> {
    // Match <tag> or <tag lang="..."> or <tag setup lang="...">
    // This pattern handles various attributes including lang, setup, scoped, etc.
    let pattern = format!(
        r#"(?s)<{tag}(?:\s+[^>]*)?>(.+?)</{tag}>"#,
        tag = regex::escape(tag)
    );

    let re = Regex::new(&pattern).ok()?;
    let captures = re.captures(source)?;

    let full_match = captures.get(0)?;
    let content_match = captures.get(1)?;

    // Count newlines before the content to get start line
    let start_line = source[..full_match.start()].matches('\n').count() + 1;

    // Extract lang attribute if present
    let opening_tag_pattern = format!(r#"<{tag}(?:\s+([^>]*))>"#, tag = regex::escape(tag));
    let opening_re = Regex::new(&opening_tag_pattern).ok()?;

    let lang = if let Some(opening_captures) =
        opening_re.captures(&source[full_match.start()..full_match.end()])
    {
        if let Some(attributes) = opening_captures.get(1) {
            extract_lang_attribute(attributes.as_str())
        } else {
            None
        }
    } else {
        None
    };

    Some(VueSection {
        content: content_match.as_str().trim().as_bytes().to_vec(),
        start_line,
        lang,
    })
}

/// Extract the lang attribute value from tag attributes
fn extract_lang_attribute(attributes: &str) -> Option<String> {
    let lang_re = Regex::new(r#"lang=["']?([^"'\s>]+)["']?"#).ok()?;
    lang_re
        .captures(attributes)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple_vue() {
        let vue_content = r#"
<template>
  <div>Hello {{ name }}</div>
</template>

<script>
export default {
  data() {
    return { name: 'World' }
  }
}
</script>

<style>
.container {
  color: blue;
}
</style>
"#;

        let sections = extract_vue_sections(vue_content.as_bytes());

        assert!(sections.template.is_some());
        assert!(sections.script.is_some());
        assert!(sections.style.is_some());

        let template = sections.template.unwrap();
        assert!(String::from_utf8_lossy(&template.content).contains("Hello"));
        assert_eq!(template.lang, None);

        let script = sections.script.unwrap();
        assert!(String::from_utf8_lossy(&script.content).contains("export default"));

        let style = sections.style.unwrap();
        assert!(String::from_utf8_lossy(&style.content).contains("color: blue"));
    }

    #[test]
    fn test_extract_vue_with_lang_attributes() {
        let vue_content = r#"
<template lang="pug">
  div Hello
</template>

<script lang="ts">
const greeting: string = "Hello";
</script>

<style lang="scss">
$color: blue;
</style>
"#;

        let sections = extract_vue_sections(vue_content.as_bytes());

        assert_eq!(
            sections.template.as_ref().and_then(|s| s.lang.as_deref()),
            Some("pug")
        );
        assert_eq!(
            sections.script.as_ref().and_then(|s| s.lang.as_deref()),
            Some("ts")
        );
        assert_eq!(
            sections.style.as_ref().and_then(|s| s.lang.as_deref()),
            Some("scss")
        );
    }

    #[test]
    fn test_extract_vue_with_setup() {
        let vue_content = r#"
<script setup lang="ts">
import { ref } from 'vue';
const count = ref(0);
</script>
"#;

        let sections = extract_vue_sections(vue_content.as_bytes());

        assert!(sections.script.is_some());
        let script = sections.script.unwrap();
        assert_eq!(script.lang.as_deref(), Some("ts"));
        assert!(String::from_utf8_lossy(&script.content).contains("ref"));
    }
}

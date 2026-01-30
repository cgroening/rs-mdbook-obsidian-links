use regex::Regex;
use serde_json::Value;
use std::io::{self, Read};
use anyhow::Result;


/// Convert an anchor string by lowercasing and replacing spaces/underscores with hyphens.
///
/// # Examples
/// `Test test` becomes `test-test`
///
/// # Arguments
/// - `anchor` - The anchor string to convert.
///
/// # Returns
/// A converted anchor string.
fn convert_anchor(anchor: &str) -> String {
    anchor
        .to_lowercase()
        .replace(' ', "-")
        .replace('_', "-")
}


/// Convert Obsidian-style links in the content to Markdown links.
///
/// The following variants are supported:
/// 1. `[[mdname#section|text]]` -> `[text](mdname.md#converted-section)`
/// 2. `[[mdname#section]]` -> `[mdname](mdname.md#converted-section)`
/// 3. `[[mdname|text]]` -> `[text](mdname.md)`
///
/// # Arguments
/// - `content` - The content string containing Obsidian links.
///
/// # Returns
/// A string with Obsidian links converted to Markdown links.
fn convert_obsidian_links(content: &str) -> String {
    let re = Regex::new(
        r"\[\[([^#\|\]]+)(?:#([^#\|\]]+))?(?:\|([^\]]+))?\]\]"
    ).unwrap();

    re.replace_all(content, |caps: &regex::Captures| {
        let mdname = caps[1].trim();
        let sektion = caps.get(2).map(|m| m.as_str().trim());
        let text = caps.get(3).map(|m| m.as_str().trim());

        // Display text is either the explicit text or the mdname
        let display = text.unwrap_or(mdname);

        // Anchor is optional
        let anchor = sektion
            .map(|s| format!("#{}", convert_anchor(s)))
            .unwrap_or_default();

        format!("[{}]({}.md{})", display, mdname, anchor)
    }).to_string()
}


/// Recursively processes an item, converting Obsidian links in its content and sub-items.
///
/// # Arguments
/// - `item` - The JSON value representing the item to process.
///
/// # Returns
/// A Result indicating success or failure.
fn process_item(item: &mut Value) -> Result<()> {
    if let Some(chapter) = item.get_mut("Chapter") {
        // Process chapter content
        if let Some(content) = chapter.get_mut("content").and_then(|c| c.as_str()) {
            let converted = convert_obsidian_links(content);
            chapter["content"] = Value::String(converted);
        }

        // Process sub-items recursively
        if let Some(sub_items) = chapter.get_mut("sub_items").and_then(|s| s.as_array_mut()) {
            for sub in sub_items {
                process_item(sub)?;
            }
        }
    }
    Ok(())
}

/// Processes the entire book, converting Obsidian links in all sections.
///
/// # Arguments
/// - `book` - The JSON value representing the book.
///
/// # Returns
/// A Result indicating success or failure.
fn process_book(book: &mut Value) -> Result<()> {
    if let Some(sections) = book.get_mut("sections").and_then(|s| s.as_array_mut()) {
        for section in sections {
            process_item(section)?;
        }
    }
    Ok(())
}


/// Main function to handle input/output and command-line arguments.
///
/// # Returns
/// A Result indicating success or failure.
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    // Handle "supports" command-line argument
    if args.len() == 3 && args[1] == "supports" {
        if args[2] != "not-supported" {
            std::process::exit(0);
        } else {
            std::process::exit(1);
        }
    }

    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let input_json: Value = serde_json::from_str(&input)?;

    // Determine the data structure based on input format
    let mut data = if input_json.is_array() {
        let arr = input_json.as_array().unwrap();
        if arr.len() != 2 {
            anyhow::bail!("Expected array of length 2, got {}", arr.len());
        }
        arr[1].clone()
    } else if input_json.is_object() && input_json.get("book").is_some() {
        input_json["book"].clone()
    } else {
        anyhow::bail!("Unexpected input format");
    };

    // Process the book to convert Obsidian links
    process_book(&mut data)?;
    serde_json::to_writer(io::stdout(), &data)?;
    Ok(())
}


/// Unit tests for the conversion functions.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_anchor() {
        assert_eq!(convert_anchor("Test test"), "test-test");
        assert_eq!(convert_anchor("Test Test"), "test-test");
        assert_eq!(convert_anchor("Hello World Example"), "hello-world-example");
        assert_eq!(convert_anchor("UPPERCASE"), "uppercase");
    }

    #[test]
    fn test_convert_obsidian_links_all_variants() {
        // Variant 1: [[mdname#section|text]]
        let input1 = "[[chapter_111#Test test|Test]]";
        let expected1 = "[Test](chapter_111.md#test-test)";
        assert_eq!(convert_obsidian_links(input1), expected1);

        // Variant 2: [[mdname#section]]
        let input2 = "[[chapter_111#Test test]]";
        let expected2 = "[chapter_111](chapter_111.md#test-test)";
        assert_eq!(convert_obsidian_links(input2), expected2);

        // Variant 3: [[mdname|text]]
        let input3 = "[[chapter_111|Test]]";
        let expected3 = "[Test](chapter_111.md)";
        assert_eq!(convert_obsidian_links(input3), expected3);

        // Variant 4: [[mdname]]
        let input4 = "[[chapter_111]]";
        let expected4 = "[chapter_111](chapter_111.md)";
        assert_eq!(convert_obsidian_links(input4), expected4);
    }

    #[test]
    fn test_multiple_links() {
        let input = "Text [[a#B C|X]] und [[d]] und [[e#F]] und [[g|H]].";
        let expected = "Text [X](a.md#b-c) und [d](d.md) und [e](e.md#f) und [H](g.md).";
        assert_eq!(convert_obsidian_links(input), expected);
    }

    #[test]
    fn test_no_conversion_needed() {
        let input = "Normal Text [normal](link.md)";
        assert_eq!(convert_obsidian_links(input), input);
    }
}

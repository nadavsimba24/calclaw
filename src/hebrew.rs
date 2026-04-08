// Hebrew text processing utilities for CalcLaw

/// Check if text contains Hebrew characters
pub fn is_hebrew(text: &str) -> bool {
    text.chars().any(|c| ('\u{0590}'..='\u{05FF}').contains(&c))
}

/// Wrap Hebrew text with RTL (right-to-left) Unicode markers
pub fn ensure_rtl(text: &str) -> String {
    if is_hebrew(text) {
        format!("\u{202B}{}\u{202C}", text) // Unicode RTL markers
    } else {
        text.to_string()
    }
}

/// Count Hebrew words in text
pub fn count_hebrew_words(text: &str) -> usize {
    text.split_whitespace()
        .filter(|word| is_hebrew(word))
        .count()
}

/// Get percentage of Hebrew content in text
pub fn hebrew_percentage(text: &str) -> f32 {
    if text.is_empty() {
        return 0.0;
    }
    
    let hebrew_chars = text.chars().filter(|c| is_hebrew(&c.to_string())).count();
    let total_chars = text.chars().count();
    
    (hebrew_chars as f32 / total_chars as f32) * 100.0
}

/// Detect if text is primarily Hebrew
pub fn is_primarily_hebrew(text: &str, threshold: f32) -> bool {
    hebrew_percentage(text) >= threshold
}

/// Default threshold for considering text as Hebrew
pub const DEFAULT_HEBREW_THRESHOLD: f32 = 30.0; // 30% Hebrew characters

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hebrew() {
        assert!(is_hebrew("שלום"));
        assert!(is_hebrew("שלום עולם"));
        assert!(!is_hebrew("hello"));
        assert!(!is_hebrew("123"));
        assert!(is_hebrew("שלום hello"));
    }

    #[test]
    fn test_ensure_rtl() {
        let hebrew_text = "שלום";
        let wrapped = ensure_rtl(hebrew_text);
        assert!(wrapped.contains('\u{202B}')); // RTL marker
        assert!(wrapped.contains('\u{202C}')); // Pop directional formatting
        
        let english_text = "hello";
        let not_wrapped = ensure_rtl(english_text);
        assert_eq!(not_wrapped, "hello");
    }

    #[test]
    fn test_count_hebrew_words() {
        assert_eq!(count_hebrew_words("שלום עולם"), 2);
        assert_eq!(count_hebrew_words("hello world שלום"), 1);
        assert_eq!(count_hebrew_words("hello world"), 0);
    }

    #[test]
    fn test_hebrew_percentage() {
        assert_eq!(hebrew_percentage("שלום"), 100.0);
        assert_eq!(hebrew_percentage("hello"), 0.0);
        assert!(hebrew_percentage("שלום hello") > 0.0);
        assert!(hebrew_percentage("שלום hello") < 100.0);
    }

    #[test]
    fn test_is_primarily_hebrew() {
        assert!(is_primarily_hebrew("שלום עולם זה טקסט בעברית", 30.0));
        assert!(!is_primarily_hebrew("this is english text עם קצת עברית", 30.0));
        assert!(is_primarily_hebrew("זה טקסט מעורב mixed text", 20.0));
    }
}
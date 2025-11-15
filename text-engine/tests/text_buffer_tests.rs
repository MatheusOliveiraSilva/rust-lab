use text_engine::text_buffer::{TextBuffer};

#[test]
fn test_creation() {
    let buffer = TextBuffer::new();
    assert_eq!(buffer.is_empty(), true);

    let buffer = TextBuffer::from_str("Salve cria");
    assert_eq!(buffer.as_str(), "Salve cria");
}

#[test]
fn test_inserts() {
    let mut buffer = TextBuffer::from_str("abdc");
    buffer.insert(2, "XYZ");
    assert_eq!(buffer.as_str(), "abXYZdc");

    buffer.insert(0, ">>");
    buffer.insert(buffer.len(), "<<");
    assert_eq!(buffer.as_str(), ">>abXYZdc<<");

    buffer.delete(0, 2);
    buffer.delete(buffer.len(), buffer.len()-2);
    assert_eq!(buffer.as_str(), "abXYZdc");

    buffer.replace_range(2, 5, "");
    assert_eq!(buffer.as_str(), "abdc");
}

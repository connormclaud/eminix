use eminix::GapBuffer;

#[test]
fn new_buffer_is_empty() {
    let buf = GapBuffer::new(16);
    assert!(buf.is_empty());
    assert_eq!(buf.len(), 0);
    assert_eq!(buf.contents(), vec![]);
}

#[test]
fn insert_single_char() {
    let mut buf = GapBuffer::new(16);
    buf.insert(b'a');
    assert_eq!(buf.contents(), b"a");
    assert_eq!(buf.len(), 1);
}

#[test]
fn insert_multiple_chars() {
    let mut buf = GapBuffer::new(16);
    for &ch in b"hello" {
        buf.insert(ch);
    }
    assert_eq!(buf.contents(), b"hello");
}

#[test]
fn delete_removes_char_before_cursor() {
    let mut buf = GapBuffer::new(16);
    for &ch in b"abc" {
        buf.insert(ch);
    }
    let deleted = buf.delete();
    assert_eq!(deleted, Some(b'c'));
    assert_eq!(buf.contents(), b"ab");
}

#[test]
fn delete_on_empty_returns_none() {
    let mut buf = GapBuffer::new(16);
    assert_eq!(buf.delete(), None);
}

#[test]
fn move_left_then_insert() {
    let mut buf = GapBuffer::new(16);
    for &ch in b"ac" {
        buf.insert(ch);
    }
    buf.move_left();
    buf.insert(b'b');
    assert_eq!(buf.contents(), b"abc");
}

#[test]
fn move_right_after_move_left() {
    let mut buf = GapBuffer::new(16);
    for &ch in b"abc" {
        buf.insert(ch);
    }
    buf.move_left();
    buf.move_left();
    buf.move_right();
    buf.insert(b'X');
    assert_eq!(buf.contents(), b"abXc");
}

#[test]
fn move_left_at_start_returns_false() {
    let mut buf = GapBuffer::new(16);
    assert!(!buf.move_left());
}

#[test]
fn move_right_at_end_returns_false() {
    let mut buf = GapBuffer::new(16);
    buf.insert(b'a');
    assert!(!buf.move_right());
}

#[test]
fn delete_in_the_middle() {
    let mut buf = GapBuffer::new(16);
    for &ch in b"abcd" {
        buf.insert(ch);
    }
    buf.move_left();
    buf.move_left();
    let deleted = buf.delete();
    assert_eq!(deleted, Some(b'b'));
    assert_eq!(buf.contents(), b"acd");
}

#[test]
fn insert_beyond_initial_capacity() {
    let mut buf = GapBuffer::new(4);
    for &ch in b"hello world" {
        buf.insert(ch);
    }
    assert_eq!(buf.contents(), b"hello world");
    assert_eq!(buf.len(), 11);
}

#[test]
#[should_panic(expected = "capacity must be non-zero")]
fn new_with_zero_capacity_panics() {
    GapBuffer::new(0);
}

/// A gap buffer for efficient text editing at the cursor position.
///
/// The buffer is a contiguous array with a "gap" where the cursor sits.
/// Insertions and deletions at the cursor are O(1). Moving the cursor
/// shifts elements across the gap.
///
/// Layout: [text before cursor ... GAP ... text after cursor]
pub struct GapBuffer {
    buffer: Vec<u8>,
    start: usize,
    end: usize,
}

impl GapBuffer {
    /// Create a new empty gap buffer with the given initial gap capacity.
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "capacity must be non-zero");
        Self {
            buffer: vec![0u8; capacity],
            start: 0,
            end: capacity,
        }
    }

    /// Insert a byte at the cursor position.
    pub fn insert(&mut self, byte: u8) {
        if self.start == self.end {
            let old_capacity = self.buffer.len();
            let new_capacity = old_capacity * 2;
            self.buffer.resize(new_capacity, 0);
            self.buffer.copy_within(
                self.end..old_capacity,
                new_capacity - old_capacity + self.end,
            );
            self.end += new_capacity - old_capacity;
        }
        self.buffer[self.start] = byte;
        self.start += 1;
    }

    /// Delete the byte immediately before the cursor (backspace).
    /// Returns the deleted byte, or None if the cursor is at the start.
    pub fn delete(&mut self) -> Option<u8> {
        if self.start == 0 {
            return None;
        }
        self.start -= 1;
        Some(self.buffer[self.start])
    }

    /// Move the cursor one position to the left.
    /// Returns false if already at the start.
    pub fn move_left(&mut self) -> bool {
        if self.start == 0 {
            return false;
        }
        self.buffer[self.end - 1] = self.buffer[self.start - 1];
        self.start -= 1;
        self.end -= 1;
        true
    }

    /// Move the cursor one position to the right.
    /// Returns false if already at the end.
    pub fn move_right(&mut self) -> bool {
        if self.end == self.buffer.len() {
            return false;
        }
        self.buffer[self.start] = self.buffer[self.end];
        self.start += 1;
        self.end += 1;
        true
    }

    /// Return the full buffer contents as a Vec<u8>, skipping the gap.
    pub fn contents(&self) -> Vec<u8> {
        let mut result = self.buffer[..self.start].to_vec();
        result.extend_from_slice(&self.buffer[self.end..]);
        result
    }

    /// The number of actual bytes in the buffer (excluding the gap).
    pub fn len(&self) -> usize {
        self.buffer.len() - self.end + self.start
    }

    /// Whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

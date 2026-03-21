# The Gap Buffer: A C Programmer's Mental Model

Think of it as a `char buf[N]` with a hole in the middle. The cursor *is* the hole.

```
Index:   0     1     2     3     4     5     6     7
Data:  [ a ] [ b ] [ ? ] [ ? ] [ ? ] [ c ] [ d ] [ e ]
                   ^start             ^end
                   |---- garbage -----|
```

**Two pointers, one rule:**
- `start` = first index *of the gap* (garbage begins here)
- `end` = first index *after the gap* (valid text resumes here)
- Text = `buf[0..start]` + `buf[end..len]`. The gap is invisible to the user.

**Invariant:** `start <= end`. When `start == end`, the gap is empty (needs grow).

## Operations

**Insert at cursor** — O(1). Drop the byte into the gap, advance `start`.
```c
buf[start] = byte;    /* gap's first slot is always free */
start += 1;           /* gap shrinks from the left       */
```

**Delete before cursor (backspace)** — O(1). Just widen the gap leftward.
```c
start -= 1;           /* the byte at buf[start] is now "in the gap" */
                      /* no memset needed -- gap is garbage anyway  */
```

**Move cursor left** — O(1). Slide one byte from before the gap to after the gap.
```
Before:  [ a ] [ b ] [ ??? ] [ c ] [ d ]
                ^s-1   ^s      ^e

end   -= 1;
buf[end] = buf[start - 1];   /* move 'b' across the gap */
start -= 1;

After:   [ a ] [ ???? ] [ b ] [ c ] [ d ]
                ^s        ^e
```

Key insight: `start - 1` is the last valid char before the gap.
`end - 1` is the last garbage slot in the gap. No swap — gap is garbage.

**Move cursor right** — O(1). Mirror of move left.
```c
buf[start] = buf[end];       /* move first char after gap to before it */
start += 1;
end   += 1;
```

## Why not just use an array with `memmove`?

You could. That is what most editors did in the 1970s. The problem: inserting at position *k* in an array of *n* bytes costs O(n - k) because you `memmove` everything after *k*. A gap buffer pays this cost *only when the cursor jumps* to a distant position. Sequential typing (the common case) is always O(1).

In C terms: the gap buffer trades `memmove` at every keystroke for `memmove` only on cursor jumps. For a typist, that is almost free.

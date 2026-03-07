//! Monitor: samples sensor models and maintains a bounded rolling window
//! of signal values for temporal property evaluation.
//!
//! The monitor is the "M" in MAPE-K. Each tick it:
//! 1. Samples all registered sensor models.
//! 2. Stores values in a fixed-capacity ring buffer per signal.
//! 3. Exposes the window to the analyzer for LTL checking.
//!
//! Window capacity is fixed at init time. No heap allocation in the tick loop.
//! All loops bounded by window size and sensor count.

#![forbid(unsafe_code)]

use std::collections::HashMap;

/// Maximum window size (bounded resource, NASA P10).
pub const MAX_WINDOW_SIZE: usize = 1024;

/// Maximum number of sensor channels a monitor can track.
pub const MAX_SENSORS: usize = 64;

// ---------------------------------------------------------------------------
// Ring buffer — fixed-capacity, stack-friendly
// ---------------------------------------------------------------------------

/// A fixed-capacity ring buffer of u64 values.
/// Overwrites oldest values when full.
#[derive(Debug, Clone)]
pub struct RingBuffer {
    buf: Vec<u64>,
    capacity: usize,
    write_pos: usize,
    len: usize,
}

impl RingBuffer {
    /// Create a new ring buffer with the given capacity.
    /// Capacity is clamped to MAX_WINDOW_SIZE.
    pub fn new(capacity: usize) -> Self {
        let cap = capacity.min(MAX_WINDOW_SIZE);
        Self { buf: vec![0u64; cap], capacity: cap, write_pos: 0, len: 0 }
    }

    /// Push a value into the ring buffer, overwriting the oldest if full.
    pub fn push(&mut self, value: u64) {
        if self.capacity == 0 {
            return;
        }
        self.buf[self.write_pos] = value;
        self.write_pos = (self.write_pos + 1) % self.capacity;
        if self.len < self.capacity {
            self.len += 1;
        }
    }

    /// Number of values currently stored.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the buffer is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Read the value at logical index `i` (0 = oldest).
    /// Returns `None` if `i >= len`.
    pub fn get(&self, i: usize) -> Option<u64> {
        if i >= self.len {
            return None;
        }
        // Oldest entry is at (write_pos - len + capacity) % capacity.
        let start = (self.write_pos + self.capacity - self.len) % self.capacity;
        let idx = (start + i) % self.capacity;
        Some(self.buf[idx])
    }

    /// Iterate over all stored values from oldest to newest.
    /// Returns an iterator that yields at most `len` items.
    pub fn iter(&self) -> RingBufferIter<'_> {
        RingBufferIter { buf: self, pos: 0 }
    }

    /// Clear all stored values.
    pub fn clear(&mut self) {
        self.write_pos = 0;
        self.len = 0;
    }
}

/// Iterator over a RingBuffer from oldest to newest.
pub struct RingBufferIter<'a> {
    buf: &'a RingBuffer,
    pos: usize,
}

impl Iterator for RingBufferIter<'_> {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        if self.pos >= self.buf.len() {
            return None;
        }
        let val = self.buf.get(self.pos);
        self.pos += 1;
        val
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.buf.len().saturating_sub(self.pos);
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for RingBufferIter<'_> {}

// ---------------------------------------------------------------------------
// Monitor
// ---------------------------------------------------------------------------

/// The Monitor component of the MAPE-K loop.
///
/// Maintains a rolling window of signal values per registered sensor channel.
/// Sensors are sampled externally and values pushed via `record_sample`.
#[derive(Debug, Clone)]
pub struct Monitor {
    /// Per-signal rolling window buffers, keyed by signal name.
    windows: HashMap<String, RingBuffer>,
    /// Window capacity (same for all channels).
    window_size: usize,
    /// Current tick counter.
    tick: u64,
}

impl Monitor {
    /// Create a new monitor with the given window size.
    /// `signal_names` registers the channels to track.
    pub fn new(window_size: usize, signal_names: &[&str]) -> Self {
        let ws = window_size.min(MAX_WINDOW_SIZE);
        let mut windows = HashMap::with_capacity(signal_names.len().min(MAX_SENSORS));
        for name in signal_names.iter().take(MAX_SENSORS) {
            windows.insert(name.to_string(), RingBuffer::new(ws));
        }
        Self { windows, window_size: ws, tick: 0 }
    }

    /// Record a sensor sample for the named signal at the current tick.
    /// Ignores unknown signal names (no panic, no allocation).
    pub fn record_sample(&mut self, signal_name: &str, value: u64) {
        if let Some(buf) = self.windows.get_mut(signal_name) {
            buf.push(value);
        }
    }

    /// Advance the tick counter after all samples for this tick are recorded.
    pub fn advance_tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }

    /// Current tick number.
    pub fn tick(&self) -> u64 {
        self.tick
    }

    /// Get the rolling window for a signal. Returns `None` if unregistered.
    pub fn window(&self, signal_name: &str) -> Option<&RingBuffer> {
        self.windows.get(signal_name)
    }

    /// Configured window size.
    pub fn window_size(&self) -> usize {
        self.window_size
    }

    /// Reset all windows and tick counter.
    pub fn reset(&mut self) {
        for buf in self.windows.values_mut() {
            buf.clear();
        }
        self.tick = 0;
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ring_buffer_basic_push_get() {
        let mut rb = RingBuffer::new(4);
        rb.push(10);
        rb.push(20);
        rb.push(30);
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.get(0), Some(10));
        assert_eq!(rb.get(1), Some(20));
        assert_eq!(rb.get(2), Some(30));
        assert_eq!(rb.get(3), None);
    }

    #[test]
    fn ring_buffer_wraps_around() {
        let mut rb = RingBuffer::new(3);
        rb.push(1);
        rb.push(2);
        rb.push(3);
        rb.push(4); // overwrites 1
        assert_eq!(rb.len(), 3);
        assert_eq!(rb.get(0), Some(2));
        assert_eq!(rb.get(1), Some(3));
        assert_eq!(rb.get(2), Some(4));
    }

    #[test]
    fn ring_buffer_iter() {
        let mut rb = RingBuffer::new(3);
        rb.push(10);
        rb.push(20);
        rb.push(30);
        rb.push(40);
        let vals: Vec<u64> = rb.iter().collect();
        assert_eq!(vals, vec![20, 30, 40]);
    }

    #[test]
    fn ring_buffer_clear() {
        let mut rb = RingBuffer::new(4);
        rb.push(1);
        rb.push(2);
        rb.clear();
        assert!(rb.is_empty());
        assert_eq!(rb.get(0), None);
    }

    #[test]
    fn monitor_records_and_windows() {
        let mut mon = Monitor::new(8, &["pressure", "rate"]);
        mon.record_sample("pressure", 120);
        mon.record_sample("rate", 72);
        mon.advance_tick();
        mon.record_sample("pressure", 118);
        mon.record_sample("rate", 74);
        mon.advance_tick();

        let pw = mon.window("pressure").unwrap();
        assert_eq!(pw.len(), 2);
        assert_eq!(pw.get(0), Some(120));
        assert_eq!(pw.get(1), Some(118));
    }

    #[test]
    fn monitor_ignores_unknown_signal() {
        let mut mon = Monitor::new(4, &["known"]);
        mon.record_sample("unknown", 999); // should not panic
        assert!(mon.window("unknown").is_none());
    }

    #[test]
    fn monitor_reset() {
        let mut mon = Monitor::new(4, &["s"]);
        mon.record_sample("s", 1);
        mon.advance_tick();
        mon.reset();
        assert_eq!(mon.tick(), 0);
        assert!(mon.window("s").unwrap().is_empty());
    }
}

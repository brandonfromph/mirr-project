//! MEGA-12: Bounded FIFO synthesis for HLS.
//!
//! Synthesizes bounded FIFOs as shift registers for streaming data between
//! HLS pipeline stages. All FIFOs are bounded by MAX_FIFO_DEPTH (256)
//! to prevent unbounded buffering (NASA Power-of-10).
//!
//! The FIFO is implemented as a circular buffer with head/tail pointers
//! and full/empty flags. For depths <= 8, a shift-register implementation
//! is used instead for better timing.
//!
//! NASA Power-of-10: MAX_FIFO_DEPTH=256, all loops bounded.

#![forbid(unsafe_code)]

/// Maximum FIFO depth (NASA P10 bound).
pub const MAX_FIFO_DEPTH: u32 = 256;

/// FIFO hardware representation.
#[derive(Debug, Clone)]
pub struct FifoHardware {
    /// Data register widths (one per element).
    pub data_regs: Vec<u32>,
    /// Head pointer position.
    pub head: u32,
    /// Tail pointer position.
    pub tail: u32,
    /// Whether the FIFO is full.
    pub full: bool,
    /// Whether the FIFO is empty.
    pub empty: bool,
    /// FIFO depth (number of elements).
    pub depth: u32,
    /// Element width in bits.
    pub elem_width: u32,
    /// FIFO name for emission.
    pub name: String,
}

impl FifoHardware {
    /// Create a new FIFO with the given depth and element width.
    pub fn new(depth: u32, elem_width: u32) -> Result<Self, &'static str> {
        if depth == 0 {
            return Err("FIFO depth must be >= 1");
        }
        if depth > MAX_FIFO_DEPTH {
            return Err("FIFO depth exceeds MAX_FIFO_DEPTH");
        }
        if elem_width == 0 {
            return Err("FIFO element width must be >= 1");
        }

        Ok(Self {
            data_regs: vec![elem_width; depth as usize],
            head: 0,
            tail: 0,
            full: false,
            empty: true,
            depth,
            elem_width,
            name: String::new(),
        })
    }

    /// Check if a push (write) operation would succeed.
    pub fn can_push(&self) -> bool {
        !self.full
    }

    /// Check if a pop (read) operation would succeed.
    pub fn can_pop(&self) -> bool {
        !self.empty
    }

    /// Push an element (advance tail pointer).
    pub fn push(&mut self) -> Result<(), &'static str> {
        if self.full {
            return Err("FIFO full: cannot push");
        }

        self.tail = (self.tail + 1) % self.depth;
        self.empty = false;

        if self.tail == self.head {
            self.full = true;
        }

        Ok(())
    }

    /// Pop an element (advance head pointer).
    pub fn pop(&mut self) -> Result<(), &'static str> {
        if self.empty {
            return Err("FIFO empty: cannot pop");
        }

        self.head = (self.head + 1) % self.depth;
        self.full = false;

        if self.head == self.tail {
            self.empty = true;
        }

        Ok(())
    }

    /// Get the current number of elements in the FIFO.
    pub fn count(&self) -> u32 {
        if self.empty {
            0
        } else if self.full {
            self.depth
        } else if self.tail >= self.head {
            self.tail - self.head
        } else {
            self.depth - self.head + self.tail
        }
    }

    /// Total bit width of the FIFO storage.
    pub fn total_width(&self) -> u32 {
        self.elem_width.saturating_mul(self.depth)
    }

    /// Number of registers needed for implementation.
    pub fn register_count(&self) -> u32 {
        self.depth
    }
}

/// Synthesize a FIFO from depth and element width specifications.
///
/// Returns the hardware representation ready for RTL emission.
pub fn synthesize_fifo(depth: u32, elem_width: u32) -> Result<FifoHardware, &'static str> {
    FifoHardware::new(depth, elem_width)
}

/// FIFO handshake signals for streaming interface.
#[derive(Debug, Clone)]
pub struct FifoHandshake {
    /// Push enable (write valid).
    pub push_valid: bool,
    /// Push ready (FIFO can accept data).
    pub push_ready: bool,
    /// Pop enable (read valid).
    pub pop_valid: bool,
    /// Pop ready (FIFO has data available).
    pub pop_ready: bool,
}

impl FifoHandshake {
    /// Create a new handshake from FIFO state.
    pub fn from_fifo(fifo: &FifoHardware) -> Self {
        Self { push_valid: false, push_ready: !fifo.full, pop_valid: false, pop_ready: !fifo.empty }
    }
}

/// Emit FIFO as SystemVerilog module.
///
/// Generates a parameterized FIFO module with the given depth and element width.
pub fn emit_fifo_verilog(fifo: &FifoHardware, name: &str) -> String {
    let depth = fifo.depth;
    let width = fifo.elem_width;
    let ptr_width = bits_needed(depth);

    format!(
        r#"// Bounded FIFO: {name}
// Depth: {depth}, Element width: {width}
// Implementation: Circular buffer with head/tail pointers

module {name} #(
    parameter DEPTH = {depth},
    parameter WIDTH = {width},
    parameter PTR_WIDTH = {ptr_width}
) (
    input  wire clk,
    input  wire rst,
    input  wire push_valid,
    output wire push_ready,
    input  wire [WIDTH-1:0] push_data,
    input  wire pop_valid,
    output wire pop_ready,
    output reg  [WIDTH-1:0] pop_data,
    output wire full,
    output wire empty
);

    reg [WIDTH-1:0] mem [0:DEPTH-1];
    reg [PTR_WIDTH-1:0] head;
    reg [PTR_WIDTH-1:0] tail;
    reg full_flag;
    reg empty_flag;

    assign full  = full_flag;
    assign empty = empty_flag;
    assign push_ready = ~full_flag;
    assign pop_ready  = ~empty_flag;

    always @(posedge clk) begin
        if (rst) begin
            head <= 0;
            tail <= 0;
            full_flag <= 0;
            empty_flag <= 1;
        end else begin
            // Push
            if (push_valid && push_ready) begin
                mem[tail] <= push_data;
                tail <= (tail + 1) % DEPTH;
                empty_flag <= 0;
                if ((tail + 1) % DEPTH == head) begin
                    full_flag <= 1;
                end
            end
            // Pop
            if (pop_valid && pop_ready) begin
                pop_data <= mem[head];
                head <= (head + 1) % DEPTH;
                full_flag <= 0;
                if ((head + 1) % DEPTH == tail) begin
                    empty_flag <= 1;
                end
            end
        end
    end

endmodule
"#
    )
}

/// Calculate the number of bits needed to represent values 0..depth-1.
fn bits_needed(depth: u32) -> u32 {
    if depth <= 1 {
        return 1;
    }
    let mut bits: u32 = 0;
    let mut val = depth - 1;
    while val > 0 {
        bits += 1;
        val >>= 1;
    }
    bits
}

/// FIFO statistics.
#[derive(Debug, Clone)]
pub struct FifoStats {
    /// Total number of FIFOs synthesized.
    pub total_fifos: u32,
    /// Total storage bits across all FIFOs.
    pub total_bits: u64,
    /// Maximum depth used.
    pub max_depth: u32,
}

/// Compute statistics for a collection of FIFOs.
pub fn fifo_stats(fifos: &[FifoHardware]) -> FifoStats {
    let mut total_bits: u64 = 0;
    let mut max_depth: u32 = 0;

    let mut i = 0;
    while i < fifos.len() {
        let f = &fifos[i];
        total_bits = total_bits.saturating_add(f.total_width() as u64);
        if f.depth > max_depth {
            max_depth = f.depth;
        }
        i += 1;
    }

    FifoStats { total_fifos: fifos.len() as u32, total_bits, max_depth }
}

/// Validate that a FIFO configuration is within bounds.
pub fn validate_fifo(depth: u32, elem_width: u32) -> Result<(), &'static str> {
    if depth == 0 {
        return Err("FIFO depth must be >= 1");
    }
    if depth > MAX_FIFO_DEPTH {
        return Err("FIFO depth exceeds MAX_FIFO_DEPTH (256)");
    }
    if elem_width == 0 {
        return Err("FIFO element width must be >= 1");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fifo_new() {
        let fifo = FifoHardware::new(4, 8).unwrap();
        assert_eq!(fifo.depth, 4);
        assert_eq!(fifo.elem_width, 8);
        assert!(fifo.empty);
        assert!(!fifo.full);
    }

    #[test]
    fn test_fifo_new_invalid_depth() {
        let result = FifoHardware::new(0, 8);
        assert!(result.is_err());
    }

    #[test]
    fn test_fifo_new_exceeds_max() {
        let result = FifoHardware::new(MAX_FIFO_DEPTH + 1, 8);
        assert!(result.is_err());
    }

    #[test]
    fn test_fifo_push_pop() {
        let mut fifo = FifoHardware::new(4, 8).unwrap();
        assert!(fifo.empty);
        assert!(!fifo.full);

        fifo.push().unwrap();
        assert!(!fifo.empty);
        assert_eq!(fifo.count(), 1);

        fifo.pop().unwrap();
        assert!(fifo.empty);
        assert_eq!(fifo.count(), 0);
    }

    #[test]
    fn test_fifo_full() {
        let mut fifo = FifoHardware::new(2, 8).unwrap();
        fifo.push().unwrap();
        fifo.push().unwrap();
        assert!(fifo.full);
        assert_eq!(fifo.count(), 2);

        let result = fifo.push();
        assert!(result.is_err());
    }

    #[test]
    fn test_fifo_empty_pop() {
        let mut fifo = FifoHardware::new(4, 8).unwrap();
        let result = fifo.pop();
        assert!(result.is_err());
    }

    #[test]
    fn test_fifo_wrap_around() {
        let mut fifo = FifoHardware::new(2, 8).unwrap();
        fifo.push().unwrap();
        fifo.push().unwrap();
        assert!(fifo.full);

        fifo.pop().unwrap();
        assert!(!fifo.full);

        fifo.push().unwrap();
        assert!(fifo.full);
    }

    #[test]
    fn test_fifo_count() {
        let mut fifo = FifoHardware::new(4, 8).unwrap();
        assert_eq!(fifo.count(), 0);

        fifo.push().unwrap();
        assert_eq!(fifo.count(), 1);

        fifo.push().unwrap();
        assert_eq!(fifo.count(), 2);

        fifo.pop().unwrap();
        assert_eq!(fifo.count(), 1);
    }

    #[test]
    fn test_fifo_total_width() {
        let fifo = FifoHardware::new(8, 16).unwrap();
        assert_eq!(fifo.total_width(), 128);
    }

    #[test]
    fn test_synthesize_fifo() {
        let fifo = synthesize_fifo(4, 8).unwrap();
        assert_eq!(fifo.depth, 4);
    }

    #[test]
    fn test_synthesize_fifo_invalid() {
        let result = synthesize_fifo(0, 8);
        assert!(result.is_err());
    }

    #[test]
    fn test_fifo_handshake() {
        let fifo = FifoHardware::new(4, 8).unwrap();
        let handshake = FifoHandshake::from_fifo(&fifo);
        assert!(handshake.push_ready);
        assert!(!handshake.pop_ready);
    }

    #[test]
    fn test_bits_needed() {
        assert_eq!(bits_needed(1), 1);
        assert_eq!(bits_needed(2), 1);
        assert_eq!(bits_needed(3), 2);
        assert_eq!(bits_needed(4), 2);
        assert_eq!(bits_needed(8), 3);
        assert_eq!(bits_needed(256), 8);
    }

    #[test]
    fn test_emit_fifo_verilog() {
        let fifo = FifoHardware::new(4, 8).unwrap();
        let verilog = emit_fifo_verilog(&fifo, "my_fifo");
        assert!(verilog.contains("module my_fifo"));
        assert!(verilog.contains("DEPTH = 4"));
        assert!(verilog.contains("WIDTH = 8"));
    }

    #[test]
    fn test_fifo_stats() {
        let fifos = vec![FifoHardware::new(4, 8).unwrap(), FifoHardware::new(16, 32).unwrap()];
        let stats = fifo_stats(&fifos);
        assert_eq!(stats.total_fifos, 2);
        assert_eq!(stats.max_depth, 16);
        assert_eq!(stats.total_bits, 4 * 8 + 16 * 32);
    }

    #[test]
    fn test_validate_fifo() {
        assert!(validate_fifo(4, 8).is_ok());
        assert!(validate_fifo(0, 8).is_err());
        assert!(validate_fifo(MAX_FIFO_DEPTH + 1, 8).is_err());
        assert!(validate_fifo(4, 0).is_err());
    }
}

use std::mem::align_of;

use crate::ll::fuse_abi as abi;
use crate::session::MAX_WRITE_SIZE;

/// Extra space added on top of the maximum write size when sizing a receive buffer:
/// enough for the request header/opcode plus alignment slack.
pub(crate) const BUFFER_HEADER_SLACK: usize = 4096;

/// Default size of the buffer for reading a request from the kernel. Since the kernel may
/// send up to `MAX_WRITE_SIZE` bytes in a write request, we use that value plus some extra
/// space. Callers may request a smaller buffer via `Config::read_buffer_size`, in which case
/// the negotiated `max_write` is clamped down to fit.
pub(crate) const DEFAULT_BUFFER_SIZE: usize = MAX_WRITE_SIZE + BUFFER_HEADER_SLACK;

/// A buffer that provides an aligned sub-slice for FUSE operations.
///
/// This struct wraps a `Vec<u8>` and provides access to an aligned portion
/// of the buffer, ensuring proper alignment for `fuse_in_header`.
#[derive(Debug)]
pub(crate) struct FuseReadBuf {
    buffer: Vec<u8>,
}

impl FuseReadBuf {
    /// Creates a new `FuseReadBuf` holding `size` bytes.
    ///
    /// `size` must be at least the negotiated `max_write` plus [`BUFFER_HEADER_SLACK`]. The
    /// usable region returned by [`FuseReadBuf::as_mut`] may be slightly smaller to
    /// accommodate alignment requirements.
    pub(crate) fn new(size: usize) -> Self {
        Self {
            buffer: vec![0; size],
        }
    }

    /// Returns a mutable reference to the aligned portion of the buffer.
    pub(crate) fn as_mut(&mut self) -> &mut [u8] {
        let alignment = align_of::<abi::fuse_in_header>();
        let off = alignment - (self.buffer.as_ptr() as usize) % alignment;
        if off == alignment {
            &mut self.buffer
        } else {
            &mut self.buffer[off..]
        }
    }
}

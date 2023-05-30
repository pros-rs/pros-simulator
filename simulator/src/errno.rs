// All vexos errno messages, starting at 0:
// Success
// Not owner
// No such file or directory
// No such process
// Interrupted system call
// I/O error
// No such device or address
// Arg list too long
// Exec format error
// Bad file number
// No children
// No more processes
// Not enough space
// Permission denied
// Bad address
// Device or resource busy
// File exists
// Cross-device link
// No such device
// Not a directory
// Is a directory
// Invalid argument
// Too many open files in system
// File descriptor value too large
// Not a character device
// Text file busy
// File too large
// No space left on device
// Illegal seek
// Read-only file system
// Too many links
// Broken pipe
// Mathematics argument out of domain of function
// Result too large
// No message of desired type
// Identifier removed
// Deadlock
// No lock
// Not a stream
// No data
// Stream ioctl timeout
// No stream resources
// Virtual circuit is gone
// Protocol error
// Multihop attempted
// Bad message
// Function not implemented
// Directory not empty
// File or path name too long
// Too many symbolic links
// Operation not supported on socket
// Connection reset by peer
// No buffer space available
// Address family not supported by protocol family
// Protocol wrong type for socket
// Socket operation on non-socket
// Protocol not available
// Connection refused
// Address already in use
// Software caused connection abort
// Network is unreachable
// Network interface is not configured
// Connection timed out
// Host is down
// Host is unreachable
// Connection already in progress
// Socket already connected
// Destination address required
// Message too long
// Unknown protocol
// Address not available
// Connection aborted by network
// Socket is already connected
// Socket is not connected
// Not supported
// Illegal byte sequence
// Value too large for defined data type
// Operation canceled
// State not recoverable
// Previous owner died

use crate::*;
use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::ToPrimitive;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, FromPrimitive, ToPrimitive)]
pub enum Errno {
    Success = 0,
    ENXIO = 6,
    EINVAL = 23,
}

impl Errno {
    /// Set the provided state's errno to this value.
    pub fn update_state(&self, state: &mut impl AsState<RobotState>) {
        state.with_state(|state, _| {
            state.errno = self.as_errno();
        });
    }
    pub fn is_success(&self) -> bool {
        self == &Errno::Success
    }
    pub fn is_error(&self) -> bool {
        !self.is_success()
    }
}

pub trait AsErrno {
    fn as_errno(&self) -> Errno;

    fn as_errno_u8(&self) -> u8 {
        self.as_errno().to_u8().unwrap()
    }
    fn as_errno_i32(&self) -> i32 {
        self.as_errno_u8().into()
    }
}

impl<T> AsErrno for Result<T, Errno> {
    fn as_errno(&self) -> Errno {
        match self.as_ref().err() {
            Some(errno) => *errno,
            None => Errno::Success,
        }
    }
}

impl AsErrno for Errno {
    fn as_errno(&self) -> Errno {
        *self
    }
}

//! An interface to `libmagic`, a tool for file type recognition.
//!
//! # Examples
//!
//! You'll typically interact with `libmagic` by creating a `Magic` object and
//! using that to query a file's type.
//!
//! ```rust
//! # extern crate magic;
//! # use magic::Magic;
//! # use std::path::Path;
//!
//! let cargo_toml = Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
//!
//! let mut m = Magic::new().unwrap();
//! let mimetype = m.query_mimetype(&cargo_toml).unwrap();
//!
//! assert_eq!(mimetype, "text/plain; charset=us-ascii");
//! ```
extern crate magic_sys;

use magic_sys::{magic_t, MAGIC_MIME};
use std::ffi::{CStr, CString};
use std::os::unix::ffi::OsStringExt;
use std::path::Path;
use std::fmt::{self, Formatter, Display};
use std::error::Error;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering, ATOMIC_BOOL_INIT};

#[derive(Debug)]
pub struct Magic {
    cookie: magic_t,
}

/// A flag indicating that an instance of `Magic` is alive.
static MAGIC_CREATED: AtomicBool = ATOMIC_BOOL_INIT;

impl Magic {
    pub fn new() -> Result<Magic, CreationError> {
        // Try to set MAGIC_CREATED to true if it's currently false
        match MAGIC_CREATED.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => {
                // We were successful. Now we can create a magic cookie
                let cookie = unsafe { magic_sys::magic_open(MAGIC_MIME) };

                if magic.cookie.is_null() {
                    // creation failed. Release MAGIC_CREATED and return an error
                    MAGIC_CREATED.store(false, Ordering::Relaxed);
                    Err(CreationError::CreationFailed)
                } else {
                    magic.load_default_database()?;
                    Ok(Magic { cookie })
                }
            }
            // MAGIC_CREATED was already true, return an error because a Magic
            // instance already exists
            Err(_) => Err(CreationError::DuplicateInstances),
        }
    }

    fn load_default_database(&mut self) -> Result<(), MagicError> {
        let outcome = unsafe { magic_sys::magic_load(self.cookie, ptr::null()) };

        if outcome == 0 {
            Ok(())
        } else {
            Err(self.last_error().unwrap_or_default())
        }
    }

    pub fn query_mimetype<P: AsRef<Path>>(&mut self, filename: P) -> Result<String, MagicError> {
        // Getting a CString from a Path is actually non-trivial due to
        // OS-specific details. Libmagic is only really available for *nix, so
        // we can get away with extracting the underlying bytes from an OS
        // string (a *nix path is just a bunch of non-null bytes) and passing
        // that in.
        let filename = CString::new(filename.as_ref().as_os_str().to_owned().into_vec())
            .expect("Paths never have null bytes");

        let mimetype = unsafe { magic_sys::magic_file(self.cookie, filename.as_ptr()) };

        if mimetype.is_null() {
            Err(self.last_error().unwrap_or_default())
        } else {
            let mimetype = unsafe { CStr::from_ptr(mimetype) };
            Ok(mimetype.to_string_lossy().to_string())
        }
    }

    /// Get the most recent `libmagic` error message, if there is one.
    fn last_error(&self) -> Option<MagicError> {
        // Get the last error message, if there was one
        let err = unsafe { magic_sys::magic_error(self.cookie) };

        if err.is_null() {
            None
        } else {
            let msg = unsafe { CStr::from_ptr(err) };
            Some(MagicError {
                msg: msg.to_string_lossy().to_string(),
            })
        }
    }
}

impl Drop for Magic {
    fn drop(&mut self) {
        unsafe {
            magic_sys::magic_close(self.cookie);
        }

        // Unset the MAGIC_CREATED flag because we are no longer using the
        // library
        MAGIC_CREATED.store(false, Ordering::Relaxed);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MagicError {
    pub msg: String,
}

impl Default for MagicError {
    fn default() -> MagicError {
        MagicError {
            msg: String::from("An error occurred"),
        }
    }
}

impl Display for MagicError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CreationError {
    CreationFailed,
    DuplicateInstances,
    MagicError(MagicError),
}

impl From<MagicError> for CreationError {
    fn from(other: MagicError) -> CreationError {
        CreationError::MagicError(other)
    }
}

impl Display for CreationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            CreationError::CreationFailed => write!(f, "Creation Failed"),
            CreationError::DuplicateInstances write!(f, "Only one instance of Magic may exist at a time"),
            CreationError::MagicError(ref inner) => write!(f, "{}", inner.msg),
        }
    }
}

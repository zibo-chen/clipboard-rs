use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClipboardError {
	#[error("Platform error: {0}")]
	Platform(String),

	#[error("Failed to open clipboard")]
	OpenFailed,

	#[error("Failed to clear clipboard")]
	ClearFailed,

	#[error("Failed to read clipboard data")]
	ReadFailed,

	#[error("Failed to write clipboard data")]
	WriteFailed,

	#[error("Invalid format: {0}")]
	InvalidFormat(String),

	#[error("Format not available")]
	FormatNotAvailable,

	#[error("Timeout while waiting for clipboard data")]
	Timeout,

	#[error("Image processing error: {0}")]
	ImageError(String),

	#[error("Invalid image data")]
	InvalidImageData,

	#[error("String encoding error: {0}")]
	EncodingError(#[from] std::str::Utf8Error),

	#[error("String conversion error: {0}")]
	StringConversionError(#[from] std::string::FromUtf8Error),

	#[error("Image library error: {0}")]
	ImageLibraryError(#[from] image::ImageError),

	#[error("IO error: {0}")]
	IoError(#[from] std::io::Error),

	#[error("Content is empty")]
	EmptyContent,

	#[error("No image data found")]
	NoImageData,

	#[error("No file data found")]
	NoFileData,

	#[error("Invalid HTML offsets")]
	InvalidHtmlOffsets,

	#[error("Invalid bitmap data")]
	InvalidBitmapData,

	#[error("Failed to create DIB")]
	CreateDibFailed,

	#[error("Failed to get device context")]
	DeviceContextFailed,

	#[error("X11 connection error: {0}")]
	X11ConnectionError(String),

	#[error("X11 atom error: {0}")]
	X11AtomError(String),

	#[error("X11 property error: {0}")]
	X11PropertyError(String),

	#[error("Windows clipboard error: {code}")]
	WindowsClipboardError { code: i32 },

	#[error("Thread has no message queue (error 1418) - clipboard access may require a window message loop")]
	ThreadNoMessageQueue,

	#[error("macOS pasteboard error: {0}")]
	MacOsPasteboardError(String),

	#[error("Monitor creation failed")]
	MonitorCreationFailed,

	#[error("Handler registration failed")]
	HandlerRegistrationFailed,

	#[error("Clipboard data type mismatch")]
	DataTypeMismatch,

	#[error("Failed to take ownership of clipboard")]
	OwnershipFailed,

	#[error("Unsupported operation")]
	UnsupportedOperation,

	#[error("Internal error: {0}")]
	Internal(String),
}

impl From<Box<dyn std::error::Error + Send + Sync + 'static>> for ClipboardError {
	fn from(err: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
		ClipboardError::Internal(err.to_string())
	}
}

impl From<String> for ClipboardError {
	fn from(err: String) -> Self {
		ClipboardError::Internal(err)
	}
}

#[cfg(target_os = "windows")]
impl From<clipboard_win::ErrorCode> for ClipboardError {
	fn from(err: clipboard_win::ErrorCode) -> Self {
		match err.raw_code() {
			1418 => ClipboardError::ThreadNoMessageQueue,
			code => ClipboardError::WindowsClipboardError { code },
		}
	}
}

#[cfg(all(
	unix,
	not(any(
		target_os = "macos",
		target_os = "ios",
		target_os = "android",
		target_os = "emscripten"
	))
))]
impl From<x11rb::errors::ReplyError> for ClipboardError {
	fn from(err: x11rb::errors::ReplyError) -> Self {
		ClipboardError::X11ConnectionError(err.to_string())
	}
}

#[cfg(all(
	unix,
	not(any(
		target_os = "macos",
		target_os = "ios",
		target_os = "android",
		target_os = "emscripten"
	))
))]
impl From<x11rb::errors::ConnectionError> for ClipboardError {
	fn from(err: x11rb::errors::ConnectionError) -> Self {
		ClipboardError::X11ConnectionError(err.to_string())
	}
}

#[cfg(all(
	unix,
	not(any(
		target_os = "macos",
		target_os = "ios",
		target_os = "android",
		target_os = "emscripten"
	))
))]
impl From<x11rb::errors::ReplyOrIdError> for ClipboardError {
	fn from(err: x11rb::errors::ReplyOrIdError) -> Self {
		ClipboardError::X11ConnectionError(err.to_string())
	}
}

#[cfg(all(
	unix,
	not(any(
		target_os = "macos",
		target_os = "ios",
		target_os = "android",
		target_os = "emscripten"
	))
))]
impl From<x11rb::errors::ConnectError> for ClipboardError {
	fn from(err: x11rb::errors::ConnectError) -> Self {
		ClipboardError::X11ConnectionError(err.to_string())
	}
}

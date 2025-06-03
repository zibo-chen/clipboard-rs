use clipboard_rs::{
	common::{RustImage, RustImageData},
	Clipboard, ClipboardContext, ContentFormat, ClipboardError,
};

#[test]
fn test_image() {
	let ctx = ClipboardContext::new().unwrap();

	let rust_img = RustImageData::from_path("tests/test.png").unwrap();

	let binding = RustImageData::from_path("tests/test.png").unwrap();

	let rust_img_bytes = binding.to_png().unwrap();

	// Set image with better error handling
	match ctx.set_image(rust_img) {
		Ok(_) => println!("Successfully set image to clipboard"),
		Err(ClipboardError::ThreadNoMessageQueue) => {
			println!("Skipping test - no message queue available in test environment");
			return;
		}
		Err(e) => panic!("Failed to set image: {:?}", e),
	}

	assert!(ctx.has(ContentFormat::Image));


	std::thread::sleep(std::time::Duration::from_millis(1000));
	let clipboard_img = match ctx.get_image() {
		Ok(img) => img,
		Err(ClipboardError::ThreadNoMessageQueue) => {
			println!("Skipping verification - no message queue available in test environment");
			return;
		}
		Err(e) => panic!("Failed to get image: {:?}", e),
	};

	assert_eq!(
		clipboard_img.to_png().unwrap().get_bytes().len(),
		rust_img_bytes.get_bytes().len()
	);
}

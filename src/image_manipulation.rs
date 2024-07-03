// Crate import
use crate::core::*;
use image::DynamicImage;
use std::path::Path;

// Class definition
pub struct Image {}
impl Image {
	pub fn combine(
		path_bottom: String,
		path_top: String,
		path_result: String,
		pos_x: int,
		pos_y: int, 
		force: bool
	) -> Result<bool, String> {
		if !Path::new(path_bottom.as_str()).exists() {
			return Err("Image file not found.".to_string());
		}
		if !Path::new(path_top.as_str()).exists(){
			return Err("Image file not found.".to_string());
		}
		if Path::new(path_result.as_str()).exists() && !force {
			return Err("The output file can't be overwritten.".to_string());
		}
		if path_bottom.contains("..") ||
			 path_top.contains("..")    ||
			 path_result.contains("..") {
			return Err("The image path can't contain \"..\".".to_string());
		}
		
		let mut bottom_image: DynamicImage = match image::open(path_bottom.to_string()) {
			Ok(image) => image,
			Err(_) => return Err("An error occurred while opening the bottom image.".to_string())
		};
		let top_image: DynamicImage = match image::open(path_top.to_string()) {
			Ok(image) => image,
			Err(_) => return Err("An error occurred while opening the top image.".to_string())
		};
		image::imageops::overlay(&mut bottom_image, &top_image, pos_x, pos_y);
		match bottom_image.save(path_result.to_string()) {
			Ok(_) => (),
			Err(_) => return Err("An error occurred while saving the image.".to_string())
		};
		return Ok(true);
	}
}

// DLL functions
pub mod export {
	use super::*;

	#[no_mangle]
	// [String, String , String, int, int, bool]
	pub extern "system" fn image_combine(_ptr: raw_pointer) -> raw_pointer {
		// Initialize data
		let data: Value = match Value::from_raw_pointer(_ptr)
		{ Ok(val) => val, Err(_) => return dll_error("Invalid argument.") };

		// Check data
		if !data.is_array() { return dll_error("Array data was expected."); }
		if data.len() != 6  { return dll_error("Wrong number of parameters."); }
		if !data[0].is_string()  { return dll_error("Argument error."); }
		if !data[1].is_string()  { return dll_error("Argument error."); }
		if !data[2].is_string()  { return dll_error("Argument error."); }
		if !data[3].is_number()  { return dll_error("Argument error."); }
		if !data[4].is_number()  { return dll_error("Argument error."); }
		if !data[5].is_boolean() { return dll_error("Argument error."); }

		// Process data
		let result: bool = match Image::combine(
			data[0].to_string(),
			data[1].to_string(),
			data[2].to_string(),
			data[3].as_i64().unwrap(),
			data[4].as_i64().unwrap(),
			data[5].as_bool().unwrap()
		) { Ok(val) => val, Err(msg) => return dll_error(&msg) };

		// Return result
		return Value::Boolean(result).to_raw_pointer();
	}
}

// Tests
#[cfg(test)]
mod tests {
	use super::*;
	use std::fs::remove_file;
	use json::array;

	use crate::image_combine;
	#[test]
	fn test_image_combine() {
		let path_result: &str = "tests/result_image.png";
		if Path::new(path_result).exists() {let _ = remove_file(path_result);}

		let image: (&str, &str) = ("tests/image0.png", "tests/image1.png");

		// Create new image without overwriting
		let input: Value = array!(image.0, image.1, path_result, 30, 40, false);
		let result: Value = Value::from_raw_pointer(image_combine(input.to_raw_pointer())).unwrap();
		assert!(result.is_boolean() && result.as_bool().unwrap() == true);

		// Recreate the image without overwriting
		let input: Value = array!(image.0, image.1, path_result, 30, 40, false);
		let result: Value = Value::from_raw_pointer( image_combine(input.to_raw_pointer()) ).unwrap();
		assert!(result.has_key("error"));

		// Create new image with overwriting
		let _ = remove_file(path_result);
		let input: Value = array!(image.0, image.1, path_result, 30, 40, true);
		let result: Value = Value::from_raw_pointer(image_combine(input.to_raw_pointer())).unwrap();
		assert!(result.is_boolean() && result.as_bool().unwrap() == true);

		// Recreate the image with overwriting
		let input: Value = array!(image.0, image.1, path_result, 30, 40, true);
		let result: Value = Value::from_raw_pointer(image_combine(input.to_raw_pointer())).unwrap();
		assert!(result.is_boolean() && result.as_bool().unwrap() == true);
	}
}

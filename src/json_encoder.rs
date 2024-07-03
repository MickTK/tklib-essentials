#[allow(unused_imports)]
use crate::core::*;

pub struct Json;
impl Json {

  // Encode a rocket hash string to json string
  pub fn encode(hash: &str) -> String {
    let mut formatted_string: String = String::new();
    let mut buffer: String = String::new();
    let mut is_inside_a_string: bool = false;
    // Symbols
    let quotation: String = String::from(r#"\""#);
    let nil: String = String::from("nil");
    let rocket: String = String::from("=>");

    for c in hash.chars() {
      buffer.push(c);

      // Keep \" inside the strings
      if quotation.starts_with(&buffer) {
        if quotation.len() == buffer.len() {
          formatted_string += &buffer;
          buffer = String::new();
        }
        continue;
      }

      // Check if it is reading a string value
      if c == '\"' { is_inside_a_string = !is_inside_a_string; }

      if !is_inside_a_string {

        // Remove all the spaces
        if buffer == " " || buffer == "\t" || buffer == "\n" {
          buffer = String::new();
        }

        // Convert key to string
        if buffer.starts_with(":") {
          if buffer.ends_with("=") ||
             buffer.ends_with("]") ||
             buffer.ends_with(",") ||
             buffer.ends_with("}") {
            buffer = buffer.replace(" ", "");
            let mut chars = buffer.chars();
            chars.next();
            let next_char: char = chars.next_back().unwrap();
            formatted_string += "\"";
            formatted_string += chars.as_str();
            formatted_string += "\"";
            buffer = String::from(next_char.to_string());
          }
          // Keep the key in the buffer
          else { continue; }
        }
      }

      // Convert nil to null
      if nil.starts_with(&buffer) {
        if nil.len() == buffer.len() {
          formatted_string += "null";
          buffer = String::new();
        }
        continue;
      }
      // Convert => to :
      if rocket.starts_with(&buffer) {
        if rocket.len() == buffer.len() {
          formatted_string += ":";
          buffer = String::new();
        }
        continue;
      }

      formatted_string += &buffer;
      buffer = String::new();
    }

    // Convert key to string
    if buffer.starts_with(":") {
      buffer = buffer.replace(" ", "");
      let mut chars = buffer.chars();
      chars.next();
      formatted_string += "\"";
      formatted_string += chars.as_str();
      formatted_string += "\"";
    }
    else {
      formatted_string += &buffer;
    }

    return match json::parse(formatted_string.as_str()) {
      Ok(x) => x.dump().to_string(),
      Err(_) => "".to_string()
    }
  }

  // Decode a json string to a rocket hash string
  pub fn decode(json: &str) -> String {
    let mut formatted_string: String = String::new();
    let mut buffer: String = String::new();
    let mut is_inside_a_string: bool = false;
    // Symbols
    let quotation: String = String::from(r#"\""#);
    let null: String = String::from("null");

    match json::parse(json) {
      Ok(_) => (),
      Err(_) => return "".to_string()
    };

    for c in json.chars() {
      buffer.push(c);

      // Keep \" inside the strings
      if quotation.starts_with(&buffer) {
        if quotation.len() == buffer.len() {
          formatted_string += &buffer;
          buffer = String::new();
        }
        continue;
      }

      // Check if it is reading a string value
      if c == '\"' { is_inside_a_string = !is_inside_a_string; }

      // Convert nil to null
      if null.starts_with(&buffer) {
        if null.len() == buffer.len() {
          formatted_string += "nil";
          buffer = String::new();
        }
        continue;
      }

      formatted_string += &buffer;
      buffer = String::new();
    }
    formatted_string += &buffer;
    return formatted_string;
  }
}

pub mod export {
  use super::*;

  #[no_mangle]
  // ruby hash formatted string -> json formatted string / nil
  pub extern "system" fn json_encode(_ptr: raw_pointer) -> raw_pointer {
    let data: &str = unsafe{std::ffi::CStr::from_ptr(_ptr)}.to_str().unwrap();
    return std::ffi::CString::new(Json::encode(data)).expect("CString::new failed!").into_raw();
  }

  #[no_mangle]
  // json formatted string -> ruby hash formatted string / nil
  pub extern "system" fn json_decode(_ptr: raw_pointer) -> raw_pointer {
    let data: &str = unsafe{std::ffi::CStr::from_ptr(_ptr)}.to_str().unwrap();
    return std::ffi::CString::new(Json::decode(data)).expect("CString::new failed!").into_raw();
  }
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::json_encode;
	#[test]
	fn test_json_encode() {

    // Integer
    let strings: (&str, &str) = (
      r#"123456"#,
      r#"123456"#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_encode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);

    // Float
    let strings: (&str, &str) = (
      r#"123.456"#,
      r#"123.456"#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_encode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);

    // String
    let strings: (&str, &str) = (
      r#""this is a string""#,
      r#""this is a string""#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_encode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);
    let strings: (&str, &str) = (
      r#"this is not a string""#,
      r#""#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_encode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);
    let strings: (&str, &str) = (
      r#""neither this is a string"#,
      r#""#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_encode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);

    // Boolean
    let strings: (&str, &str) = (
      r#"true"#,
      r#"true"#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_encode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);
    let strings: (&str, &str) = (
      r#"false"#,
      r#"false"#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_encode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);
    let strings: (&str, &str) = (
      r#"falseù"#,
      r#""#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_encode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);

    // Symbol
    let strings: (&str, &str) = (
      r#":this is a symbol"#,
      r#""thisisasymbol""#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_encode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);

    // Nil
    let strings: (&str, &str) = (
      r#"nil"#,
      r#"null"#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_encode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);

    // Array
    let strings: (&str, &str) = (
      r#"[[[[[[:key]]], 12.69, "hi", nil, {}], true, false], 10]"#,
      r#"[[[[[["key"]]],12.69,"hi",null,{}],true,false],10]"#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_encode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);

    // Hash
    let strings: (&str, &str) = (
      r#"{ :key => ["value\"", nil, 10, 11.11, false, :k,[[],{:hello=>true}]] }"#,
      r#"{"key":["value\"",null,10,11.11,false,"k",[[],{"hello":true}]]}"#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_encode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);
    let strings: (&str, &str) = (
      r#"{ :key => }"#,
      r#""#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_encode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);
  }

  use crate::json_decode;
  #[test]
  fn test_json_decode() {
    let strings: (&str, &str) = (
      r#"null"#,
      r#"nil"#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_decode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);

    let strings: (&str, &str) = (
      r#"{"key" : "value"}"#,
      r#"{"key" : "value"}"#
    );
    let input: String = String::from(strings.0);
    let input: raw_pointer = std::ffi::CString::new(input).expect("CString::new failed!").into_raw();
    let result: String = unsafe{std::ffi::CStr::from_ptr(json_decode(input))}.to_str().unwrap().to_string();
    assert!(result == strings.1);
  }
}

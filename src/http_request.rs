#[allow(unused_imports)]
use crate::core::*;

use std::collections::HashMap;
use url::Url;

pub struct Request;
impl Request {
  pub fn connection_status() -> bool {
    return match ureq::head("https://www.google.com").call() {
      Ok(_) => true,
      Err(_) => false
    };
  }

  pub fn get(_url: String, _params: HashMap<String, String>) -> Result<String, String> {
    let url: String = match Url::parse_with_params(_url.as_str(), _params) {
      Ok(u) => u.to_string(),
      Err(_) => return Err("Invalid url.".to_string())
    };
    let response: ureq::Response = match ureq::get(url.as_str()).call() {
      Ok(text) => text,
      Err(_) => return Err("Response error".to_string())
    };
    return match response.into_string() {
      Ok(ret) => Ok(ret),
      Err(_) => return Err("Response error".to_string())
    };
  }

  pub fn post(_url: String, _params: HashMap<String, String>, _form: &Vec<(&str, &str)>) -> Result<String, String> {
    let url: String = match Url::parse_with_params(_url.as_str(), _params) {
      Ok(u) => u.to_string(),
      Err(_) => return Err("Invalid url.".to_string())
    };
    let response: ureq::Response = match ureq::post(url.as_str()).send_form(_form) {
      Ok(text) => text,
      Err(_) => return Err("Response error".to_string())
    };
    return match response.into_string() {
      Ok(ret) => Ok(ret),
      Err(_) => return Err("Response error".to_string())
    };
  }
}

pub mod export {
  use super::*;

  #[no_mangle]
  pub extern "system" fn request_connection_status() -> raw_pointer {
    return Value::Boolean(Request::connection_status()).to_raw_pointer();
  }

  #[no_mangle]
  // [String, [String]] => [Url, [Param1, Value1, ...]]
  pub extern "system" fn request_get(_ptr: raw_pointer) -> raw_pointer {
		// Initialize data
		let data: Value = match Value::from_raw_pointer(_ptr)
		{ Ok(val) => val, Err(_) => return dll_error("Invalid argument.") };

    // Check data
    if !data.is_array() { return dll_error("Array data was expected."); }
		if data.len() != 2  { return dll_error("Wrong number of parameters."); }
    if !data[0].is_string()   { return dll_error("Argument error."); }
    if !data[1].is_array()    { return dll_error("Argument error."); }
    if data[1].len() % 2 == 1 { return dll_error("Argument error."); }

    let mut params: HashMap<String, String> = HashMap::new();
    for i in (0..data[1].len()).step_by(2) {
      if !data[1][i].is_string()   { return dll_error("Argument error."); }
      if !data[1][i+1].is_string() { return dll_error("Argument error."); }
      params.insert(data[1][i].to_string(), data[1][i+1].to_string());
    }
    let result: String = match Request::get(data[0].to_string(), params) {
      Ok(ret) => ret,
      Err(msg) => return dll_error(&msg)
    };
    return Value::String(result).to_raw_pointer();
  }

  #[no_mangle]
  // [String, [String], [String]] => [Url, [Param1, Value1, ...], [FormKey1, FormValue1, ...]]
  pub extern "system" fn request_post(_ptr: raw_pointer) -> raw_pointer {
		// Initialize data
		let data: Value = match Value::from_raw_pointer(_ptr)
		{ Ok(val) => val, Err(_) => return dll_error("Invalid argument.") };

    // Check data
    if !data.is_array() { return dll_error("Array data was expected."); }
		if data.len() != 3  { return dll_error("Wrong number of parameters."); }
    if !data[0].is_string()   { return dll_error("Argument error."); }
    if !data[1].is_array()    { return dll_error("Argument error."); }
    if data[1].len() % 2 == 1 { return dll_error("Argument error."); }
    if !data[2].is_array()    { return dll_error("Argument error."); }
    if data[2].len() % 2 == 1 { return dll_error("Argument error."); }

    let mut params: HashMap<String, String> = HashMap::new();
    for i in (0..data[1].len()).step_by(2) {
      if !data[1][i].is_string()   { return dll_error("Argument error."); }
      if !data[1][i+1].is_string() { return dll_error("Argument error."); }
      params.insert(data[1][i].to_string(), data[1][i+1].to_string());
    }

    let mut form_data: Vec<(&str, &str)> = vec![];
    for i in (0..data[2].len()).step_by(2) {
      if !data[2][i].is_string()   { return dll_error("Argument error."); }
      if !data[2][i+1].is_string() { return dll_error("Argument error."); }
      form_data.push((data[2][i].as_str().unwrap(), data[2][i+1].as_str().unwrap()));
    }

    let result: String = match Request::post(data[0].to_string(), params, &form_data) {
      Ok(ret) => ret,
      Err(msg) => return dll_error(&msg)
    };
    return Value::String(result).to_raw_pointer();
  }
}

#[cfg(test)]
mod tests {
	use super::*;
  use json::array;

	use crate::request_connection_status;
  #[test]
	fn test_request_connection_status() {
    // Skip test if there is no connection
    if !Request::connection_status() { println!("No internet connection. Skipping request tests."); return }
    
    // Internet connection is on
    let result: Value = Value::from_raw_pointer(request_connection_status()).unwrap();
    assert!(result.as_bool().unwrap());
  }

	use crate::request_get;
	#[test]
	fn test_request_get() {
    // Skip test if there is no connection
    if !Request::connection_status() { return }
    
    // Request with get method without parameters
    let input: Value = array!("https://echo.free.beeceptor.com", []);
    let result: Value = Value::from_raw_pointer(request_get(input.to_raw_pointer())).unwrap();
    let result: Value = json::parse(result.as_str().unwrap()).unwrap();
    assert!(result["parsedQueryParams"].len() == 0);

    // Request with get method with parameters
    let input: Value = array!("https://echo.free.beeceptor.com", ["ciao","mondo"]);
    let result: Value = Value::from_raw_pointer(request_get(input.to_raw_pointer())).unwrap();
    let result: Value = json::parse(result.as_str().unwrap()).unwrap();
    assert!(result["parsedQueryParams"].len() == 1);
    assert!(result["parsedQueryParams"]["ciao"] == "mondo");
	}

  use crate::request_post;
	#[test]
	fn test_request_post() {
    // Skip test if there is no connection
    if !Request::connection_status() { return }

    // Request with get method without parameters
    let input: Value = array!("https://echo.free.beeceptor.com", [], []);
    let result: Value = Value::from_raw_pointer(request_post(input.to_raw_pointer())).unwrap();
    let result: Value = json::parse(result.as_str().unwrap()).unwrap();
    assert!(result["parsedQueryParams"].len() == 0);
    assert!(result["parsedBody"].len() == 0);

    // Request with get method with parameters
    let input: Value = array!("https://echo.free.beeceptor.com", ["ciao","mondo"], []);
    let result: Value = Value::from_raw_pointer(request_post(input.to_raw_pointer())).unwrap();
    let result: Value = json::parse(result.as_str().unwrap()).unwrap();
    assert!(result["parsedQueryParams"].len() == 1);
    assert!(result["parsedQueryParams"]["ciao"] == "mondo");
    assert!(result["parsedBody"].len() == 0);

    // Request with get method with parameters
    let input: Value = array!("https://echo.free.beeceptor.com", [], ["hello","world"]);
    let result: Value = Value::from_raw_pointer(request_post(input.to_raw_pointer())).unwrap();
    let result: Value = json::parse(result.as_str().unwrap()).unwrap();
    assert!(result["parsedQueryParams"].len() == 0);
    assert!(result["parsedBody"].len() == 1);
    assert!(result["parsedBody"]["hello"] == "world");

    // Request with get method with parameters
    let input: Value = array!("https://echo.free.beeceptor.com", ["ciao","mondo"], ["hello","world","food","pizza"]);
    let result: Value = Value::from_raw_pointer(request_post(input.to_raw_pointer())).unwrap();
    let result: Value = json::parse(result.as_str().unwrap()).unwrap();
    assert!(result["parsedQueryParams"].len() == 1);
    assert!(result["parsedQueryParams"]["ciao"] == "mondo");
    assert!(result["parsedBody"].len() == 2);
    assert!(result["parsedBody"]["hello"] == "world");
    assert!(result["parsedBody"]["food"] == "pizza");
	}
}

#[allow(unused_imports)]
use crate::core::*;

pub struct Csv;
impl Csv {
  pub fn parse(data: String, delimiter: String) -> Result<Vec<Value>, String> {
    // Delimiter
    let del: u8 = if delimiter.len() > 0 {
      match delimiter.chars().next() {
        None => return Err("Invalid delimiter.".to_string()),
        Some(x) => x as u8
      }
    } else { b',' };
    // Return value
    let mut ret: Vec<Value> = Vec::new();
    // Csv reader
    let mut reader = csv::ReaderBuilder::new()
    .delimiter(del).from_reader(data.as_str().as_bytes());
    // Header
    let header: &csv::StringRecord = match reader.headers() {
      Ok(x) => &x.clone(),
      Err(_) => return Err("Invalid csv header.".to_string())
    };
    // Return value builder
    for result in reader.records() {
      let mut row: Value = Value::new_object();
      let record = match result {
        Ok(x) => x,
        Err(_) => return Err("Invalid csv row.".to_string())
      };
      for i in 0..header.len() {
        let key: &str = match header.get(i) {
          Some(x) => x,
          None => return Err("Invalid csv header.".to_string())
        };
        let value: &str = match record.get(i) {
          Some(x) => x,
          None => return Err("Invalid csv row.".to_string())
        };
        let _ = row.insert(key, value.to_string());
      }
      ret.push(row);
    }

    return Ok(ret);
  }
}

pub mod export {
  use super::*;

  #[no_mangle]
  // [String, String] -> [text, delimiter]
  pub extern "system" fn csv_parse(_ptr: raw_pointer) -> raw_pointer {
    // Initialize data
		let data: Value = match Value::from_raw_pointer(_ptr)
		{ Ok(val) => val, Err(_) => return dll_error("Invalid argument.") };

    // Check data
		if !data.is_array() { return dll_error("Array data was expected."); }
		if data.len() != 2  { return dll_error("Wrong number of parameters."); }
		if !data[0].is_string()  { return dll_error("Argument error."); }
		if !data[1].is_string()  { return dll_error("Argument error."); }

    // Process data
		let result: Vec<Value> = match Csv::parse(
			data[0].to_string(),
			data[1].to_string()
		) { Ok(val) => val, Err(msg) => return dll_error(&msg) };

    // Return result
		return Value::Array(result).to_raw_pointer();
  }
}

// Tests
#[cfg(test)]
mod tests {
	use super::*;
	use json::array;
	use json::JsonValue::{Array, Object};
  use json::object::Object as Obj;

  use crate::csv_parse;
	#[test]
	fn test_csv_parse() {
		let input: Value = array!("a-b-c-d\nhello-world-ciao-mondo", "-");
		let result: Value = Value::from_raw_pointer(csv_parse(input.to_raw_pointer())).unwrap();
		assert!(result.is_array());
    let arr: Vec<Value> = match result {
      Array(v) => v,
      _ => Vec::<Value>::new()
    };
    let hash: Obj = match arr.get(0).unwrap() {
      Object(v) => v.clone(),
      _ => Obj::new()
    };
		assert!(hash.get("b").unwrap().as_str().unwrap() == "world");
  }
}

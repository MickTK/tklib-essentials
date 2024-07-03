/* Type definition and alias */
// String c formatted. Input and output of the dll.
#[allow(non_camel_case_types)] pub type raw_pointer = *const std::ffi::c_char;
#[allow(non_camel_case_types)] pub type int = i64;
#[allow(non_camel_case_types)] pub type float = f64;
pub type Value = json::JsonValue;

// Traits
pub trait Pointer {
  fn from_raw_pointer(_ptr:raw_pointer) -> json::Result<Value>;
  fn to_raw_pointer(&self) -> raw_pointer;
}
impl Pointer for Value {
  fn from_raw_pointer(_ptr:raw_pointer) -> json::Result<Value> {
    return json::parse(unsafe{std::ffi::CStr::from_ptr(_ptr)}.to_str().unwrap());
  }
  fn to_raw_pointer(&self) -> raw_pointer {
    return std::ffi::CString::new(self.dump().to_string())
    .expect("CString::new failed!")
    .into_raw()
  }
}

#[allow(dead_code)]
// {"error": "Error message."}
pub fn dll_error(message: &str) -> raw_pointer {
  return json::object!("error": message).to_raw_pointer();
}

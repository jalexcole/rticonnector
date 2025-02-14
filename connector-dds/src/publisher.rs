use std::{ffi::{c_char, c_int, c_void, CStr}, marker::PhantomData, ptr, time::Duration};

use rticonnector::Connector;
use rticonnector_sys::{RTI_Connector_get_matched_publications, RTI_Connector_wait_for_acknowledgments, RTI_Connector_wait_for_matched_subscription};

use crate::topic::TopicType;



pub struct DataWriter<'a, T> where T : TopicType<'a> {
    pub(crate) connector: &'a Connector,
    pub(crate) data_writer: *mut c_void,
    _marker: PhantomData<T>,
}

impl <'a, T> DataWriter<'a, T> where T : TopicType<'a> {

    pub(crate) fn  new(connector: &'a Connector, data_writer: *mut c_void) -> DataWriter<'a, T>  {
        

        Self {
            connector,
            data_writer,
            _marker: PhantomData,
        }
    }

    /// Safe wrapper for `RTI_Connector_wait_for_acknowledgments`
    pub fn wait_for_acknowledgments(&self, timeout: Duration) -> Result<i32, &'static str> {
        // Call the unsafe FFI function
        let new_count = unsafe {
            RTI_Connector_wait_for_acknowledgments(self.data_writer, timeout.as_millis() as i32)
        };

        // Check if the function call was successful
        if new_count < 0 {
            return Err("Error waiting for acknowledgments.");
        }

        Ok(new_count)
    }
    /// Safe wrapper for `RTI_Connector_wait_for_matched_subscription`
    pub fn wait_for_matched_subscription(&self, ms_timeout: i32) -> Result<i32, &'static str> {
        // Ensure the writer pointer is not null
        if self.data_writer.is_null() {
            return Err("Null pointer: RTI data writer instance does not exist.");
        }

        // Variable to hold the new count of matched subscriptions
        let mut current_count_change: c_int = 0;

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_wait_for_matched_subscription(
                self.data_writer,
                ms_timeout,
                &mut current_count_change,
            )
        };

        // Check if the function call was successful
        if result != 0 {
            return Err("Error waiting for matched subscriptions or timeout occurred.");
        }

        Ok(current_count_change)
    }
    /// Safe Rust wrapper for `RTI_Connector_get_matched_publications`
    pub fn get_matched_publications(&self) -> Result<T, String> {
        let mut json_ptr: *mut c_char = ptr::null_mut();

        // Call the unsafe FFI function
        let result =
            unsafe { RTI_Connector_get_matched_publications(self.data_writer, &mut json_ptr) };

        // Check if the function call was successful
        if result != 0 {
            return Err(format!("Data writer error code: {}", result));
        }

        // Ensure the pointer is not null
        if json_ptr.is_null() {
            return Err("ConnectorError::NullPointer".to_string());
        }

        // Convert the C string to a Rust String
        unsafe {
            let c_str = CStr::from_ptr(json_ptr);
            let json_str  = c_str.to_str().expect("Unable to convert c string int &str");

            // Free the memory if needed (depending on how the memory is managed in the FFI)
            // For example: libc::free(json_ptr) or a corresponding RTI free function.

            let result: T = serde_json::from_str(json_str).unwrap();
            Ok(result)
        }
    }
}
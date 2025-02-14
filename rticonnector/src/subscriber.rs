use std::{ffi::{c_char, c_int, c_void, CStr}, ptr, time::Duration};

use rticonnector_sys::{RTI_Connector_get_matched_publications, RTI_Connector_wait_for_matched_publication};

use crate::{topic::TopicType, Connector};



pub struct DynamicDataReader<'a> {
    pub(crate) connector: &'a Connector,
    pub(crate) data_reader: *mut c_void,
}

impl DynamicDataReader<'_> {
    pub fn wait_for_data_on_reader(&self, timeout: Duration) -> Result<i32, String> {
        todo!()
    }

    /// Safe wrapper for `RTI_Connector_wait_for_matched_publication`
    pub fn wait_for_matched_publication(&self, ms_timeout: i32) -> Result<i32, &'static str> {
        // Ensure the reader pointer is not null
        if self.data_reader.is_null() {
            return Err("Null pointer: RTI data reader instance does not exist.");
        }

        // Variable to hold the new count of matched publications
        let mut current_count_change: c_int = 0;

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_wait_for_matched_publication(
                self.data_reader,
                ms_timeout,
                &mut current_count_change,
            )
        };

        // Check if the function call was successful
        if result != 0 {
            return Err("Error waiting for matched publications or timeout occurred.");
        }

        Ok(current_count_change)
    }

    /// Safe Rust wrapper for `RTI_Connector_get_matched_publications`
    pub fn get_matched_publications(&self) -> Result<String, String> {
        let mut json_ptr: *mut c_char = ptr::null_mut();

        // Call the unsafe FFI function
        let result =
            unsafe { RTI_Connector_get_matched_publications(self.data_reader, &mut json_ptr) };

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
            let json_str = c_str.to_string_lossy().into_owned();

            // Free the memory if needed (depending on how the memory is managed in the FFI)
            // For example: libc::free(json_ptr) or a corresponding RTI free function.

            Ok(json_str)
        }
    }
}

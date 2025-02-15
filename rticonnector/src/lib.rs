//! This is a home made binding for the [rticonnextdds-connector](https://github.com/rticommunity/rticonnextdds-connector)
//! The library is incomplete and will not catch all edge cases, and should
//! be used at your own risk.
//! This library was not written by RTI and is not endorsed by RTI.
//!
//! The ideals of this library is to provide a Rust interface to the C library
//! without exposing any unsafe code,and nothing more.

use core::panic;
use std::ffi::CString;
use std::ffi::{c_char, c_double, c_int, c_void, CStr};
use std::ptr;
use std::time::Duration;

use rticonnector_sys::*;

use thiserror::Error;

#[derive(Clone)]
pub struct RTIOptions {
    options: RTI_Connector_Options,
}

impl RTIOptions {
    pub fn new(enable_on_data_event: bool, one_based_sequence_indexing: bool) -> Self {
        Self {
            options: RTI_Connector_Options {
                enable_on_data_event: enable_on_data_event as i32,
                one_based_sequence_indexing: one_based_sequence_indexing as i32,
            },
        }
    }
}

impl Default for RTIOptions {
    fn default() -> Self {
        Self {
            options: RTI_Connector_Options {
                enable_on_data_event: 1,
                one_based_sequence_indexing: 1,
            },
        }
    }
}

pub struct Connector {
    connector: *mut RTI_Connector,
}

impl Connector {
    /// Creates a new [`Connector`].
    ///
    /// Panics if the configuration file is not valid or `&[RTIOptions]`
    /// exceeds a length of 128.
    pub fn new(config_name: &str, config_file: &str, options: &[RTIOptions]) -> Self {
        let rti_options: [RTI_Connector_Options; 128] = match options
            .iter()
            .map(|o| o.options)
            .collect::<Vec<_>>()
            .try_into()
        {
            Ok(options) => options,
            Err(_) => panic!("Failed to convert options to array"),
        };
        let connector = unsafe {
            RTI_Connector_new(
                CString::new(config_name).unwrap().as_ptr(),
                CString::new(config_file).unwrap().as_ptr(),
                rti_options.as_ptr(),
            )
        };
        Self { connector }
    }

    /// Safe wrapper for `RTI_Connector_get_sample_count`
    pub fn get_sample_count(&self, entity_name: &str) -> Result<f64, &'static str> {
        // Ensure the connector pointer is not null
        if self.connector.is_null() {
            return Err("Null pointer: RTI Connector instance does not exist.");
        }

        // Convert the entity name to a C string
        let c_entity_name = CString::new(entity_name).map_err(|_| "Failed to create CString")?;

        // Variable to hold the sample count
        let mut sample_count: c_double = 0.0;

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_get_sample_count(
                self.connector as *mut c_void,
                c_entity_name.as_ptr(),
                &mut sample_count,
            )
        };

        // Check if the function call was successful
        if result != 0 {
            return Err("Error getting sample count or data reader does not exist.");
        }

        Ok(sample_count)
    }

    pub fn get_boolean_from_infos(
        &self,
        entity_name: &str,
        index: usize,
        field_name: &str,
    ) -> Result<bool, String> {
        let mut return_value: c_int = 0;

        // Convert Rust strings to C strings
        let c_entity_name = CString::new(entity_name).map_err(|_| "ConnectorError::NullPointer")?;
        let c_field_name = CString::new(field_name).map_err(|_| "ConnectorError::NullPointer")?;

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_get_boolean_from_infos(
                self.connector as *mut c_void,
                &mut return_value,
                c_entity_name.as_ptr(),
                index as c_int,
                c_field_name.as_ptr(),
            )
        };

        // Check the result and handle errors
        if result != 0 {
            return Err("ConnectorError::FfiError".to_string());
        }

        // Return the value (1 for true, 0 for false)
        match return_value {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err("ConnectorError::FieldNotFound".to_string()),
        }
    }

    pub fn set_json_instance(&mut self, entity_name: &str, json: &str) -> Result<(), String> {
        if self.connector.is_null() {
            return Err("Connector has been closed".to_string());
        }

        let response = unsafe {
            RTI_Connector_set_json_instance(
                self.connector as *mut c_void,
                CString::new(entity_name).unwrap().as_ptr(),
                CString::new(json).unwrap().as_ptr(),
            )
        };

        if response == 0 {
            Ok(())
        } else {
            Err("Failed to set json instance".to_string())
        }
    }

    /// Safe wrapper for `RTI_Connector_set_boolean_into_samples`
    pub fn set_boolean_into_samples(
        &self,
        entity_name: &str,
        field_name: &str,
        value: bool,
    ) -> Result<(), ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        // Convert Rust strings to C strings
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;
        let c_field_name = CString::new(field_name).map_err(|_| ConnectorError::NullPointer)?;

        // Convert the Rust boolean to an integer (non-zero = true, zero = false)
        let c_value = if value { 1 } else { 0 };

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_set_boolean_into_samples(
                connector_ptr as *mut c_void,
                c_entity_name.as_ptr(),
                c_field_name.as_ptr(),
                c_value,
            )
        };

        // Check the result for success or error
        if result == 0 {
            Ok(())
        } else {
            Err(ConnectorError::FfiError)
        }
    }

    /// Safe wrapper for `RTI_Connector_set_number_into_samples`
    pub fn set_number_into_samples(
        &self,
        entity_name: &str,
        field_name: &str,
        value: f64,
    ) -> Result<(), ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        // Convert Rust strings to C strings
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;
        let c_field_name = CString::new(field_name).map_err(|_| ConnectorError::NullPointer)?;

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_set_number_into_samples(
                connector_ptr as *mut c_void,
                c_entity_name.as_ptr(),
                c_field_name.as_ptr(),
                value as c_double, // Convert Rust f64 to C double
            )
        };

        // Check the result for success or error
        if result == 0 {
            Ok(())
        } else {
            Err(ConnectorError::FfiError)
        }
    }

    /// Safe wrapper for `RTI_Connector_set_string_into_samples`
    pub fn set_string_into_samples(
        &self,
        entity_name: &str,
        field_name: &str,
        value: &str,
    ) -> Result<(), ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }
        let connector_ptr = self.connector;

        // Convert Rust strings to C strings
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;
        let c_field_name = CString::new(field_name).map_err(|_| ConnectorError::NullPointer)?;
        let c_value = CString::new(value).map_err(|_| ConnectorError::NullPointer)?;

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_set_string_into_samples(
                connector_ptr as *mut c_void,
                c_entity_name.as_ptr(),
                c_field_name.as_ptr(),
                c_value.as_ptr(),
            )
        };

        // Check the result for success or error
        if result == 0 {
            Ok(())
        } else {
            Err(ConnectorError::FfiError)
        }
    }

    /// Safe wrapper for `RTI_Connector_get_json_from_infos`
    pub fn get_json_from_infos(
        &self,
        entity_name: &str,
        index: usize,
        field_name: &str,
    ) -> Result<String, ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        // Convert Rust strings to C strings
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;
        let c_field_name = CString::new(field_name).map_err(|_| ConnectorError::NullPointer)?;

        // Prepare a pointer to hold the JSON string
        let mut value: *mut c_char = ptr::null_mut();

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_get_json_from_infos(
                connector_ptr as *mut c_void,
                c_entity_name.as_ptr(),
                index as c_int,
                c_field_name.as_ptr(),
                &mut value as *mut *mut c_char,
            )
        };

        // Check if the call was successful
        if result != 0 {
            return Err(ConnectorError::FfiError);
        }

        // Convert the returned C string into a Rust String
        if value.is_null() {
            return Err(ConnectorError::FieldNotFound);
        }

        let json_str = unsafe { CStr::from_ptr(value).to_string_lossy().into_owned() };

        // Free the C string memory if necessary (depending on the C function's contract)

        Ok(json_str)
    }

    /// Safe wrapper for `RTI_Connector_get_json_sample`
    pub fn get_json_sample(
        &self,
        entity_name: &str,
        index: usize,
    ) -> Result<String, ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        // Convert the entity name to a C string
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;

        // Prepare a pointer to hold the JSON string
        let mut json_str: *mut c_char = ptr::null_mut();

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_get_json_sample(
                connector_ptr as *mut c_void,
                c_entity_name.as_ptr(),
                index as c_int,
                &mut json_str as *mut *mut c_char,
            )
        };

        // Check if the function call was successful
        if result != 0 {
            return Err(ConnectorError::FfiError);
        }

        // Convert the returned C string into a Rust String
        if json_str.is_null() {
            return Err(ConnectorError::SampleNotFound);
        }

        let json_str_rust = unsafe { CStr::from_ptr(json_str).to_string_lossy().into_owned() };

        // Free the C string memory if necessary (depending on the C function's contract)

        Ok(json_str_rust)
    }

    /// Safe wrapper for `RTI_Connector_get_json_member`
    pub fn get_json_member(
        &self,
        entity_name: &str,
        index: usize,
        member_name: &str,
    ) -> Result<String, ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        // Convert Rust strings to C strings
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;
        let c_member_name = CString::new(member_name).map_err(|_| ConnectorError::NullPointer)?;

        // Prepare a pointer to hold the JSON string
        let mut json_str: *mut c_char = ptr::null_mut();

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_get_json_member(
                connector_ptr as *mut c_void,
                c_entity_name.as_ptr(),
                index as c_int,
                c_member_name.as_ptr(),
                &mut json_str as *mut *mut c_char,
            )
        };

        // Check if the function call was successful
        if result != 0 {
            return Err(ConnectorError::FfiError);
        }

        // Convert the returned C string into a Rust String
        if json_str.is_null() {
            return Err(ConnectorError::MemberNotFound);
        }

        let json_str_rust = unsafe { CStr::from_ptr(json_str).to_string_lossy().into_owned() };

        // Free the C string memory if necessary (depending on the C function's contract)

        Ok(json_str_rust)
    }

    /// Safe wrapper for `RTI_Connector_clear`
    pub fn clear(&self, entity_name: &str) -> Result<(), ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        // Convert the entity name to a C string
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;

        // Call the unsafe FFI function
        let result =
            unsafe { RTI_Connector_clear(connector_ptr as *mut c_void, c_entity_name.as_ptr()) };

        // Check if the function call was successful
        if result != 0 {
            return Err(ConnectorError::FfiError);
        }

        Ok(())
    }

    /// Safe wrapper for `RTI_Connector_read`
    pub fn read(&self, entity_name: &str) -> Result<(), ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        // Convert the entity name to a C string
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;

        // Call the unsafe FFI function
        let result =
            unsafe { RTI_Connector_read(connector_ptr as *mut c_void, c_entity_name.as_ptr()) };

        // Check if the function call was successful
        if result != 0 {
            return Err(ConnectorError::FfiError);
        }

        Ok(())
    }

    /// Safe wrapper for `RTI_Connector_take`
    pub fn take(&self, entity_name: &str) -> Result<(), ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        // Convert the entity name to a C string
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;

        // Call the unsafe FFI function
        let result =
            unsafe { RTI_Connector_take(connector_ptr as *mut c_void, c_entity_name.as_ptr()) };

        // Check if the function call was successful
        if result != 0 {
            return Err(ConnectorError::FfiError);
        }

        Ok(())
    }

    /// Safe wrapper for `RTI_Connector_write`
    pub fn write(&self, entity_name: &str, params_json: &str) -> Result<(), ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        // Convert the entity name and JSON string to C strings
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;
        let c_params_json = CString::new(params_json).map_err(|_| ConnectorError::NullPointer)?;

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_write(
                connector_ptr as *mut c_void,
                c_entity_name.as_ptr(),
                c_params_json.as_ptr(),
            )
        };

        // Check if the function call was successful
        if result != 0 {
            return Err(ConnectorError::FfiError);
        }

        Ok(())
    }

    fn delete(&mut self) {
        if self.connector.is_null() {
            panic!("Attempted to delete an already invalid connector");
        }

        unsafe { RTI_Connector_delete(self.connector) }
    }

    /// Safe wrapper for `RTI_Connector_get_number_from_sample`
    pub fn get_number_from_sample(
        &mut self,
        entity_name: &str,
        index: i32,
        field_name: &str,
    ) -> Result<f64, ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        // Convert the entity name and field name to C strings
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;
        let c_field_name = CString::new(field_name).map_err(|_| ConnectorError::NullPointer)?;

        // Prepare a variable to hold the return value
        let mut return_value: f64 = 0.0;

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_get_number_from_sample(
                connector_ptr as *mut c_void,
                &mut return_value,
                c_entity_name.as_ptr(),
                index,
                c_field_name.as_ptr(),
            )
        };

        // Check if the function call was successful
        if result != 0 {
            return Err(ConnectorError::FfiError);
        }

        Ok(return_value)
    }

    /// Safe wrapper for `RTI_Connector_get_boolean_from_sample`
    pub fn get_boolean_from_sample(
        &mut self,
        entity_name: &str,
        index: i32,
        field_name: &str,
    ) -> Result<bool, ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        // Convert the entity name and field name to C strings
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;
        let c_field_name = CString::new(field_name).map_err(|_| ConnectorError::NullPointer)?;

        // Prepare a variable to hold the return value
        let mut return_value: c_int = 0;

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_get_boolean_from_sample(
                connector_ptr as *mut c_void,
                &mut return_value,
                c_entity_name.as_ptr(),
                index,
                c_field_name.as_ptr(),
            )
        };

        // Check if the function call was successful
        if result != 0 {
            return Err(ConnectorError::FfiError);
        }

        // Convert the return value to a boolean
        Ok(return_value != 0)
    }

    /// Safe wrapper for `RTI_Connector_get_string_from_sample`
    pub fn get_string_from_sample(
        &mut self,
        entity_name: &str,
        index: i32,
        field_name: &str,
    ) -> Result<String, ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        // Convert the entity name and field name to C strings
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;
        let c_field_name = CString::new(field_name).map_err(|_| ConnectorError::NullPointer)?;

        // Prepare a variable to hold the pointer to the string return value
        let mut return_value: *mut c_char = ptr::null_mut();

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_get_string_from_sample(
                connector_ptr as *mut c_void,
                &mut return_value,
                c_entity_name.as_ptr(),
                index,
                c_field_name.as_ptr(),
            )
        };

        // Check if the function call was successful
        if result != 0 {
            return Err(ConnectorError::FfiError);
        }

        // Convert the returned C string to a Rust String
        if return_value.is_null() {
            return Err(ConnectorError::InvalidString);
        }

        let c_str = unsafe { CStr::from_ptr(return_value) };
        let string_value = c_str.to_string_lossy().into_owned();

        // Free the string memory if needed (depends on FFI implementation)
        // unsafe { some_free_function(return_value); }

        Ok(string_value)
    }

    /// Safe wrapper for `RTI_Connector_clear_member`
    pub fn clear_member(&self, entity_name: &str, member_name: &str) -> Result<(), ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        // Convert the entity name and member name to C strings
        let c_entity_name = CString::new(entity_name).map_err(|_| ConnectorError::NullPointer)?;
        let c_member_name = CString::new(member_name).map_err(|_| ConnectorError::NullPointer)?;

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_clear_member(
                connector_ptr as *mut c_void,
                c_entity_name.as_ptr(),
                c_member_name.as_ptr(),
            )
        };

        // Check if the function call was successful
        if result != 0 {
            return Err(ConnectorError::FfiError);
        }

        Ok(())
    }

    pub fn get_dynamic_datareader(
        &self,
        entity_name: &str,
    ) -> Result<DynamicDataReader, ConnectorError> {
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        let connector_ptr = self.connector;

        let data_reader = unsafe {
            RTI_Connector_get_datareader(
                self.connector as *mut c_void,
                CString::new(entity_name).unwrap().as_ptr(),
            )
        };
        if data_reader.is_null() {
            return Err(ConnectorError::NullPointer);
        } else {
            Ok(DynamicDataReader {
                connector: &self,
                data_reader,
            })
        }
    }

    /// Safe wrapper for `RTI_Connector_get_datawriter`
    pub fn get_dynamic_datawriter(
        &mut self,
        entity_name: &str,
    ) -> Result<DynamicDataWriter, ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        // let connector_ptr = self
        //     .connector
        //     .ok_or("Null pointer: RTI Connector instance does not exist.")?;

        // Convert the entity name to a C string
        let c_entity_name = CString::new(entity_name).map_err(|_| "Failed to create C string.");

        // Call the unsafe FFI function
        let datawriter_ptr = unsafe {
            RTI_Connector_get_datawriter(self.connector as *mut c_void, c_entity_name.unwrap().as_ptr())
        };

        // Check if the returned pointer is null
        if datawriter_ptr.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        Ok(DynamicDataWriter {
            connector: self,
            data_writer: datawriter_ptr,
        })
    }

    /// Safe wrapper for `RTI_Connector_get_native_sample`
    pub fn get_native_sample(
        &self,
        entity_name: &str,
        index: usize,
    ) -> Result<*const c_void, ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }

        // let connector_ptr = self
        //     .connector
        //     .ok_or("Null pointer: RTI Connector instance does not exist.")?;

        // Convert the entity name to a C string
        let c_entity_name = CString::new(entity_name).map_err(|_| "Failed to create C string.");

        // Call the unsafe FFI function
        let sample_ptr = unsafe {
            RTI_Connector_get_native_sample(
                self.connector as *mut c_void,
                c_entity_name.unwrap().as_ptr(),
                index as c_int,
            )
        };

        // Check if the returned pointer is null
        if sample_ptr.is_null() {
            return Err(ConnectorError::SampleNotFound);
        }

        Ok(sample_ptr)
    }

    pub fn wait_fo_data(&mut self, timeout: Duration) -> Result<u32, ConnectorError> {
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }
        let result = unsafe {
            RTI_Connector_wait_for_data_on_reader(self.connector as *mut c_void, timeout.as_millis() as c_int)
        };
        if result < 0 {
            Err(ConnectorError::FfiError)
        } else {
            Ok(result as u32)
        }
    }

    /// Safe wrapper for `RTI_Connector_wait_for_data_on_reader`
    pub fn wait_for_data_on_reader(&self, ms_timeout: Duration) -> Result<(), ConnectorError> {
        // Ensure the connector is not None (null pointer check)
        if self.connector.is_null() {
            return Err(ConnectorError::NullPointer);
        }
        // let connector_ptr = self
        //     .connector
        //     .ok_or("Null pointer: RTI Connector instance does not exist.")?;

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_wait_for_data_on_reader(
                self.connector as *mut c_void,
                ms_timeout.as_millis() as c_int,
            )
        };

        // Check if the function call was successful
        if result != 0 {
            return Err(ConnectorError::SampleNotFound);
        }

        Ok(())
    }

    pub fn get_last_error() -> String {
        unsafe {
            let result = RTI_Connector_get_last_error_message();
            String::from(CString::from_raw(result).to_str().unwrap())
        }
    }

    pub fn set_max_objects_per_thread(value: usize) -> Result<(), &'static str> {
        let result = unsafe { RTI_Connector_set_max_objects_per_thread(value as c_int) };

        if result != 0 {
            Err("Error setting max objects per thread")
        } else {
            Ok(())
        }
    }
}

pub struct DynamicDataReader<'a> {
    pub(crate) connector: &'a Connector,
    pub(crate) data_reader: *mut c_void,
}

impl DynamicDataReader<'_> {
    pub fn wait_for_data_on_reader(&self, timeout: Duration) -> Result<i32, ConnectorError> {
        // Here we assume a similar pattern as in Connector.
        if self.data_reader.is_null() {
            return Err(ConnectorError::NullPointer);
        }
        let result = unsafe {
            RTI_Connector_wait_for_data_on_reader(self.data_reader, timeout.as_millis() as c_int)
        };
        if result != 0 {
            Err(ConnectorError::FfiError)
        } else {
            Ok(result)
        }
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

pub struct DynamicDataWriter<'a> {
    pub(crate) connector: &'a Connector,
    pub(crate) data_writer: *mut c_void,
}

impl DynamicDataWriter<'_> {
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
    pub fn wait_for_matched_subscription(&self, ms_timeout: Duration) -> Result<i32, String> {
        // Ensure the writer pointer is not null
        if self.data_writer.is_null() {
            return Err("Null pointer: RTI data writer instance does not exist.".to_string());
        }

        // Variable to hold the new count of matched subscriptions
        let mut current_count_change: c_int = 0;

        // Call the unsafe FFI function
        let result = unsafe {
            RTI_Connector_wait_for_matched_subscription(
                self.data_writer,
                ms_timeout.as_millis() as i32,
                &mut current_count_change,
            )
        };

        // Check if the function call was successful
        if result != 0 {
            return Err("Error waiting for matched subscriptions or timeout occurred.".to_string());
        }

        Ok(current_count_change)
    }
    /// Safe Rust wrapper for `RTI_Connector_get_matched_publications`
    pub fn get_matched_publications(&self) -> Result<String, String> {
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
            let json_str = c_str.to_string_lossy().into_owned();

            // Free the memory if needed (depending on how the memory is managed in the FFI)
            // For example: libc::free(json_ptr) or a corresponding RTI free function.

            Ok(json_str)
        }
    }
}

impl Drop for Connector {
    fn drop(&mut self) {
        self.delete();
    }
}

/// A sample of any complex data type, which can be inspected and manipulated reflectively.

#[derive(Debug, Clone, Copy)]
pub enum DDSError {
    Timeout = 10,
    NoData = 11,
    Unknown = -1,
}

// impl Error for DDSError {}

// Error type for handling possible errors
#[derive(Debug, Error)]
pub enum ConnectorError {
    #[error("Error: Null pointer")]
    NullPointer,
    #[error("Error: FFI Error")]
    FfiError,
    #[error("Error: Field Not Boolean")]
    FieldNotBoolean,
    #[error("Error: Field Not Found")]
    FieldNotFound,
    #[error("Error: Sample Not Found")]
    SampleNotFound,
    #[error("Error: Member Not Found")]
    MemberNotFound,
    #[error("Error: Invalid String")]
    InvalidString,
}

use std::error::Error;
use arrow::datatypes::Schema;

mod device_type;
mod render;

pub struct Resource {
    resource_name: String,
    resource_type: String,
}

pub struct Profile {
    resources: Vec<Resource>,
}

pub struct Device {
    profile: Profile,
}

fn mock_device() -> Option<Device> {
    None
}

fn device_profile_to_arrow_schema() -> Result<Schema, Box<dyn Error>> {
    OK(())
}
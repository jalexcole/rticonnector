pub trait Entity {
    fn enable(&mut self);

}


pub struct InstanceHandle {

}

struct DynamicData;

impl DynamicData {
    pub fn to_cdr_buffer(sample: &Self) -> Vec<char> {
        todo!()
    }

    
}

struct DynamicDataInfo {}

struct DynamicDataMemberInfo;
struct DynamicDataProperty;

struct DynamicDataSeq {}
struct DynamicDataSerializationSupport {}
struct DynamicDataTypeProperty {}
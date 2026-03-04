use crate::keyvalue::KeyValue;

struct Achievement {
    name: String,
    description: String
}

impl From<KeyValue> for Achievement {
    fn from(value: KeyValue) -> Self {
        todo!()
    }
}

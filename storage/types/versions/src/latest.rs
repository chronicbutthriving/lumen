pub mod object {
    pub use crate::v1::object::GetObjectParams;
    pub use crate::v1::object::StorageObject;
    pub use crate::v1::object::StorageProviderKind;
}

pub mod system {
    pub use crate::v1::system::Ping;
    pub use crate::v1::system::PingStatus;
}

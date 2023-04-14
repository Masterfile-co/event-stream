// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistryEvent {
    #[prost(string, tag="100")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag="101")]
    pub metadata: ::core::option::Option<super::super::common::v1::TransactionMetadata>,
    #[prost(uint64, tag="200")]
    pub ordinal: u64,
    #[prost(oneof="registry_event::Event", tags="1, 2, 3, 4, 5")]
    pub event: ::core::option::Option<registry_event::Event>,
}
/// Nested message and enum types in `RegistryEvent`.
pub mod registry_event {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeploymentAdded {
        #[prost(string, tag="1")]
        pub deployment: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FactoryAdded {
        #[prost(string, tag="1")]
        pub factory: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub name: ::prost::alloc::string::String,
        #[prost(uint64, tag="3")]
        pub version: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoleAdminChanged {
        #[prost(string, tag="1")]
        pub role: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub previous_admin_role: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub new_admin_role: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoleGranted {
        #[prost(string, tag="1")]
        pub role: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub account: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub sender: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RoleRevoked {
        #[prost(string, tag="1")]
        pub role: ::prost::alloc::string::String,
        #[prost(string, tag="2")]
        pub account: ::prost::alloc::string::String,
        #[prost(string, tag="3")]
        pub sender: ::prost::alloc::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        DeploymentAdded(DeploymentAdded),
        #[prost(message, tag="2")]
        FactoryAdded(FactoryAdded),
        #[prost(message, tag="3")]
        RoleAdminChanged(RoleAdminChanged),
        #[prost(message, tag="4")]
        RoleGranted(RoleGranted),
        #[prost(message, tag="5")]
        RoleRevoked(RoleRevoked),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegistryEvents {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<RegistryEvent>,
}
/// Encoded file descriptor set for the `masterfile.registry.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xab, 0x16, 0x0a, 0x0e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x16, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x66, 0x69, 0x6c, 0x65, 0x2e,
    0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x1a, 0x0c, 0x63, 0x6f, 0x6d,
    0x6d, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa8, 0x08, 0x0a, 0x0d, 0x52, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x61, 0x0a, 0x0f, 0x64,
    0x65, 0x70, 0x6c, 0x6f, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x41, 0x64, 0x64, 0x65, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x35, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x66, 0x69, 0x6c,
    0x65, 0x2e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x44, 0x65, 0x70, 0x6c,
    0x6f, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x41, 0x64, 0x64, 0x65, 0x64, 0x48, 0x00, 0x52, 0x0f, 0x64,
    0x65, 0x70, 0x6c, 0x6f, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x41, 0x64, 0x64, 0x65, 0x64, 0x12, 0x58,
    0x0a, 0x0c, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x41, 0x64, 0x64, 0x65, 0x64, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x66, 0x69, 0x6c,
    0x65, 0x2e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x46, 0x61, 0x63, 0x74,
    0x6f, 0x72, 0x79, 0x41, 0x64, 0x64, 0x65, 0x64, 0x48, 0x00, 0x52, 0x0c, 0x66, 0x61, 0x63, 0x74,
    0x6f, 0x72, 0x79, 0x41, 0x64, 0x64, 0x65, 0x64, 0x12, 0x64, 0x0a, 0x10, 0x72, 0x6f, 0x6c, 0x65,
    0x41, 0x64, 0x6d, 0x69, 0x6e, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x36, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x66, 0x69, 0x6c, 0x65, 0x2e,
    0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x67, 0x69,
    0x73, 0x74, 0x72, 0x79, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x52, 0x6f, 0x6c, 0x65, 0x41, 0x64,
    0x6d, 0x69, 0x6e, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x48, 0x00, 0x52, 0x10, 0x72, 0x6f,
    0x6c, 0x65, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x12, 0x55,
    0x0a, 0x0b, 0x72, 0x6f, 0x6c, 0x65, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x64, 0x18, 0x04, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x31, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x66, 0x69, 0x6c, 0x65,
    0x2e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x67,
    0x69, 0x73, 0x74, 0x72, 0x79, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x2e, 0x52, 0x6f, 0x6c, 0x65, 0x47,
    0x72, 0x61, 0x6e, 0x74, 0x65, 0x64, 0x48, 0x00, 0x52, 0x0b, 0x72, 0x6f, 0x6c, 0x65, 0x47, 0x72,
    0x61, 0x6e, 0x74, 0x65, 0x64, 0x12, 0x55, 0x0a, 0x0b, 0x72, 0x6f, 0x6c, 0x65, 0x52, 0x65, 0x76,
    0x6f, 0x6b, 0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x31, 0x2e, 0x6d, 0x61, 0x73,
    0x74, 0x65, 0x72, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79,
    0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x2e, 0x52, 0x6f, 0x6c, 0x65, 0x52, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x64, 0x48, 0x00, 0x52,
    0x0b, 0x72, 0x6f, 0x6c, 0x65, 0x52, 0x65, 0x76, 0x6f, 0x6b, 0x65, 0x64, 0x12, 0x18, 0x0a, 0x07,
    0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x18, 0x64, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x12, 0x45, 0x0a, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61,
    0x74, 0x61, 0x18, 0x65, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x29, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e, 0x76, 0x31, 0x2e,
    0x54, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x4d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0x52, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x19, 0x0a,
    0x07, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c, 0x18, 0xc8, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52,
    0x07, 0x6f, 0x72, 0x64, 0x69, 0x6e, 0x61, 0x6c, 0x1a, 0x45, 0x0a, 0x0f, 0x44, 0x65, 0x70, 0x6c,
    0x6f, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x41, 0x64, 0x64, 0x65, 0x64, 0x12, 0x1e, 0x0a, 0x0a, 0x64,
    0x65, 0x70, 0x6c, 0x6f, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0a, 0x64, 0x65, 0x70, 0x6c, 0x6f, 0x79, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x1a,
    0x56, 0x0a, 0x0c, 0x46, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x41, 0x64, 0x64, 0x65, 0x64, 0x12,
    0x18, 0x0a, 0x07, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x07, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x18, 0x0a,
    0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x1a, 0x78, 0x0a, 0x10, 0x52, 0x6f, 0x6c, 0x65, 0x41,
    0x64, 0x6d, 0x69, 0x6e, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x72,
    0x6f, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x12,
    0x2c, 0x0a, 0x11, 0x70, 0x72, 0x65, 0x76, 0x69, 0x6f, 0x75, 0x73, 0x41, 0x64, 0x6d, 0x69, 0x6e,
    0x52, 0x6f, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x11, 0x70, 0x72, 0x65, 0x76,
    0x69, 0x6f, 0x75, 0x73, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x6f, 0x6c, 0x65, 0x12, 0x22, 0x0a,
    0x0c, 0x6e, 0x65, 0x77, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x6f, 0x6c, 0x65, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x0c, 0x6e, 0x65, 0x77, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x6f, 0x6c,
    0x65, 0x1a, 0x53, 0x0a, 0x0b, 0x52, 0x6f, 0x6c, 0x65, 0x47, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x64,
    0x12, 0x12, 0x0a, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04,
    0x72, 0x6f, 0x6c, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x16,
    0x0a, 0x06, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06,
    0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x1a, 0x53, 0x0a, 0x0b, 0x52, 0x6f, 0x6c, 0x65, 0x52, 0x65,
    0x76, 0x6f, 0x6b, 0x65, 0x64, 0x12, 0x12, 0x0a, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x04, 0x72, 0x6f, 0x6c, 0x65, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x63, 0x63,
    0x6f, 0x75, 0x6e, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x63, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x65, 0x6e, 0x64, 0x65, 0x72, 0x42, 0x07, 0x0a, 0x05, 0x65,
    0x76, 0x65, 0x6e, 0x74, 0x22, 0x4f, 0x0a, 0x0e, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79,
    0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x3d, 0x0a, 0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x66,
    0x69, 0x6c, 0x65, 0x2e, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x2e,
    0x52, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x06, 0x65,
    0x76, 0x65, 0x6e, 0x74, 0x73, 0x4a, 0xee, 0x0c, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x32, 0x01,
    0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x02, 0x00, 0x16, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x04, 0x00, 0x1f, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x08, 0x00, 0x12,
    0x04, 0x07, 0x08, 0x0d, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03,
    0x07, 0x0e, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x10, 0x34,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x08, 0x10, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x20, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x09, 0x10, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x09, 0x10, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x09, 0x1d, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x09, 0x2c,
    0x2d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x10, 0x36, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0a, 0x10, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x21, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03,
    0x12, 0x03, 0x0b, 0x10, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03,
    0x0b, 0x10, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0b, 0x1c,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0b, 0x2a, 0x2b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0c, 0x10, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x0c, 0x10, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x04, 0x01, 0x12, 0x03, 0x0c, 0x1c, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04,
    0x03, 0x12, 0x03, 0x0c, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x05, 0x12, 0x03,
    0x0e, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0e, 0x08,
    0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0e, 0x0f, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x0e, 0x19, 0x1c, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x0f, 0x08, 0x40, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x06, 0x12, 0x03, 0x0f, 0x08, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x0f, 0x31, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x03, 0x12,
    0x03, 0x0f, 0x3c, 0x3f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12, 0x03, 0x10, 0x08,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x10, 0x08, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x10, 0x0f, 0x16, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x10, 0x19, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x00, 0x03, 0x00, 0x12, 0x04, 0x12, 0x08, 0x15, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03,
    0x00, 0x01, 0x12, 0x03, 0x12, 0x10, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x13, 0x10, 0x26, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x13, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x13, 0x17, 0x21, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x13, 0x24, 0x25, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x14, 0x10, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x14, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x14, 0x17, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x14, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x01, 0x12, 0x04, 0x17,
    0x08, 0x1b, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x01, 0x01, 0x12, 0x03, 0x17, 0x10,
    0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x12, 0x03, 0x18, 0x10, 0x23,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x18, 0x10, 0x16,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x17, 0x1e,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x21, 0x22,
    0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x12, 0x03, 0x19, 0x10, 0x20, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x19, 0x10, 0x16, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x19, 0x17, 0x1b, 0x0a,
    0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x19, 0x1e, 0x1f, 0x0a,
    0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x10, 0x23, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1a, 0x10, 0x16, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x17, 0x1e, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x00, 0x03, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1a, 0x21, 0x22, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x00, 0x03, 0x02, 0x12, 0x04, 0x1d, 0x08, 0x21, 0x09, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x03, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x10, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00,
    0x03, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1e, 0x10, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1e, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1e, 0x17, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1e, 0x1e, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x10, 0x2d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x1f, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x1f, 0x17, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x1f, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x20, 0x10, 0x28, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x20, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x20, 0x17, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x02, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x20, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x03, 0x12,
    0x04, 0x23, 0x08, 0x27, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x03, 0x03, 0x01, 0x12, 0x03,
    0x23, 0x10, 0x1b, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x12, 0x03, 0x24,
    0x10, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x24,
    0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x24,
    0x17, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x24,
    0x1e, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x03, 0x02, 0x01, 0x12, 0x03, 0x25, 0x10,
    0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x25, 0x10,
    0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x25, 0x17,
    0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x25, 0x21,
    0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x03, 0x03, 0x02, 0x02, 0x12, 0x03, 0x26, 0x10, 0x22,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x26, 0x10, 0x16,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x17, 0x1d,
    0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03, 0x03, 0x02, 0x02, 0x03, 0x12, 0x03, 0x26, 0x20, 0x21,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x00, 0x03, 0x04, 0x12, 0x04, 0x29, 0x08, 0x2d, 0x09, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x03, 0x04, 0x01, 0x12, 0x03, 0x29, 0x10, 0x1b, 0x0a, 0x0d, 0x0a, 0x06,
    0x04, 0x00, 0x03, 0x04, 0x02, 0x00, 0x12, 0x03, 0x2a, 0x10, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2a, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2a, 0x17, 0x1b, 0x0a, 0x0e, 0x0a, 0x07, 0x04,
    0x00, 0x03, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2a, 0x1e, 0x1f, 0x0a, 0x0d, 0x0a, 0x06, 0x04,
    0x00, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x10, 0x23, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2b, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2b, 0x17, 0x1e, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00,
    0x03, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2b, 0x21, 0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00,
    0x03, 0x04, 0x02, 0x02, 0x12, 0x03, 0x2c, 0x10, 0x22, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2c, 0x10, 0x16, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x17, 0x1d, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x03,
    0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2c, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x30, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x30, 0x08,
    0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x31, 0x08, 0x2a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x31, 0x11, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x1f, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x31, 0x28, 0x29, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
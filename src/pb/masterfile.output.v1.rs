// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterfileEvent {
    #[prost(oneof="masterfile_event::Event", tags="1, 2, 3, 4, 5")]
    pub event: ::core::option::Option<masterfile_event::Event>,
}
/// Nested message and enum types in `MasterfileEvent`.
pub mod masterfile_event {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Event {
        #[prost(message, tag="1")]
        Registry(super::super::super::registry::v1::RegistryEvent),
        #[prost(message, tag="2")]
        Factory(super::super::super::factory::v1::FactoryEvent),
        #[prost(message, tag="3")]
        Drop(super::super::super::drop::v1::DropEvent),
        #[prost(message, tag="4")]
        Safe(super::super::super::safe::v1::SafeEvent),
        #[prost(message, tag="5")]
        Splits(super::super::super::splits::v1::SplitEvent),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MasterfileEvents {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<MasterfileEvent>,
}
/// Encoded file descriptor set for the `masterfile.output.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xa0, 0x08, 0x0a, 0x0c, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x14, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x6f, 0x75,
    0x74, 0x70, 0x75, 0x74, 0x2e, 0x76, 0x31, 0x1a, 0x0c, 0x63, 0x6f, 0x6d, 0x6d, 0x6f, 0x6e, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0a, 0x64, 0x72, 0x6f, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x1a, 0x0a, 0x73, 0x61, 0x66, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x73,
    0x70, 0x6c, 0x69, 0x74, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0d, 0x66, 0x61, 0x63,
    0x74, 0x6f, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0e, 0x72, 0x65, 0x67, 0x69,
    0x73, 0x74, 0x72, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc6, 0x02, 0x0a, 0x0f, 0x4d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x66, 0x69, 0x6c, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x12, 0x43,
    0x0a, 0x08, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x25, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x72, 0x65,
    0x67, 0x69, 0x73, 0x74, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x73, 0x74,
    0x72, 0x79, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x48, 0x00, 0x52, 0x08, 0x72, 0x65, 0x67, 0x69, 0x73,
    0x74, 0x72, 0x79, 0x12, 0x3f, 0x0a, 0x07, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x66, 0x69, 0x6c,
    0x65, 0x2e, 0x66, 0x61, 0x63, 0x74, 0x6f, 0x72, 0x79, 0x2e, 0x76, 0x31, 0x2e, 0x46, 0x61, 0x63,
    0x74, 0x6f, 0x72, 0x79, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x48, 0x00, 0x52, 0x07, 0x66, 0x61, 0x63,
    0x74, 0x6f, 0x72, 0x79, 0x12, 0x33, 0x0a, 0x04, 0x64, 0x72, 0x6f, 0x70, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x66, 0x69, 0x6c, 0x65, 0x2e,
    0x64, 0x72, 0x6f, 0x70, 0x2e, 0x76, 0x31, 0x2e, 0x44, 0x72, 0x6f, 0x70, 0x45, 0x76, 0x65, 0x6e,
    0x74, 0x48, 0x00, 0x52, 0x04, 0x64, 0x72, 0x6f, 0x70, 0x12, 0x33, 0x0a, 0x04, 0x73, 0x61, 0x66,
    0x65, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x66, 0x69, 0x6c, 0x65, 0x2e, 0x73, 0x61, 0x66, 0x65, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x61, 0x66,
    0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x48, 0x00, 0x52, 0x04, 0x73, 0x61, 0x66, 0x65, 0x12, 0x3a,
    0x0a, 0x06, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20,
    0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x66, 0x69, 0x6c, 0x65, 0x2e, 0x73, 0x70, 0x6c, 0x69,
    0x74, 0x73, 0x2e, 0x76, 0x31, 0x2e, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x45, 0x76, 0x65, 0x6e, 0x74,
    0x48, 0x00, 0x52, 0x06, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x73, 0x42, 0x07, 0x0a, 0x05, 0x65, 0x76,
    0x65, 0x6e, 0x74, 0x22, 0x51, 0x0a, 0x10, 0x4d, 0x61, 0x73, 0x74, 0x65, 0x72, 0x66, 0x69, 0x6c,
    0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x3d, 0x0a, 0x06, 0x65, 0x76, 0x65, 0x6e, 0x74,
    0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x25, 0x2e, 0x6d, 0x61, 0x73, 0x74, 0x65, 0x72,
    0x66, 0x69, 0x6c, 0x65, 0x2e, 0x6f, 0x75, 0x74, 0x70, 0x75, 0x74, 0x2e, 0x76, 0x31, 0x2e, 0x4d,
    0x61, 0x73, 0x74, 0x65, 0x72, 0x66, 0x69, 0x6c, 0x65, 0x45, 0x76, 0x65, 0x6e, 0x74, 0x52, 0x06,
    0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x4a, 0x82, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x17,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x00, 0x12, 0x03, 0x02, 0x00, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x03, 0x00,
    0x14, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x04, 0x00, 0x14, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x05, 0x00, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x04, 0x12, 0x03, 0x06,
    0x00, 0x17, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x05, 0x12, 0x03, 0x07, 0x00, 0x18, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x09, 0x00, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0b,
    0x00, 0x13, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x08, 0x17, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x0c, 0x08, 0x12, 0x09, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x0c, 0x0e, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x10, 0x42, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x0d, 0x10, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0d, 0x35, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d,
    0x40, 0x41, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x10, 0x3f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0e, 0x10, 0x32, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x33, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x3d, 0x3e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x02, 0x12, 0x03, 0x0f, 0x10, 0x36, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x0f, 0x10, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f,
    0x2d, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x34, 0x35,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x10, 0x10, 0x36, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x10, 0x10, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x10, 0x2d, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x10, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12,
    0x03, 0x11, 0x10, 0x3b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x11,
    0x10, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x11, 0x30, 0x36,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x11, 0x39, 0x3a, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x15, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x15, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x16, 0x08, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x16, 0x08,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x16, 0x11, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x21, 0x27, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x2a, 0x2b, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)
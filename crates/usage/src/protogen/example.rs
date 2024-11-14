// This file is @generated by prost-build.
#[derive(::grpc_build_core::NamedMessage)]
#[name = "example.Person"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Person {
    #[prost(string, tag = "1")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub last_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub gender: ::core::option::Option<Gender>,
}
#[derive(::grpc_build_core::NamedMessage)]
#[name = "example.Gender"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Gender {
    #[prost(oneof = "gender::Gender", tags = "1, 2")]
    pub gender: ::core::option::Option<gender::Gender>,
}
/// Nested message and enum types in `Gender`.
pub mod gender {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Gender {
        #[prost(message, tag = "1")]
        Male(()),
        #[prost(message, tag = "2")]
        Female(()),
    }
}
#[derive(::grpc_build_core::NamedMessage)]
#[name = "example.HomoSepiens"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomoSepiens {
    #[prost(string, tag = "1")]
    pub first_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub last_name: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub gender: ::core::option::Option<HomoSepiensGender>,
}
#[derive(::grpc_build_core::NamedMessage)]
#[name = "example.HomoSepiensGender"]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HomoSepiensGender {
    #[prost(oneof = "homo_sepiens_gender::Gender", tags = "1, 2")]
    pub gender: ::core::option::Option<homo_sepiens_gender::Gender>,
}
/// Nested message and enum types in `HomoSepiensGender`.
pub mod homo_sepiens_gender {
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Gender {
        #[prost(message, tag = "1")]
        Male(()),
        #[prost(message, tag = "2")]
        Female(()),
    }
}

#![recursion_limit = "1024"]
//! libsip has three basic components a parser and managers.
//!
//! Managers are utility struct's meant to ease one specifc asspect of the sip protocol,
//! the only manager currently implemented is for endpoint registration.
//!
//! ### Parsing
//! libsip exposes many parsing function though only one `parse_message` is needed.
//! ```rust
//! extern crate libsip;
//! extern crate nom;
//! 
//! use libsip::parse_message;
//! use nom::error::VerboseError;
//!
//! let packet = "SIP/2.0 200 OK\r\n\r\n";
//! let output = libsip::parse_message::<VerboseError<&[u8]>>(packet.as_ref()).unwrap();
//! ```
//!
//! ### Creating Messages
//! This crate provides 2 abstraction's to aid in building sip messages.
//! The `ResponseGenerator` is used to create sip response's and the
//! `RequestGenerator` is used to generate sip requests.
//!  ```rust
//!     extern crate libsip;
//!     extern crate nom;
//!
//!     use nom::error::VerboseError;
//!     use libsip::ResponseGenerator;
//!     use libsip::RequestGenerator;
//!     use libsip::Method;
//!     use libsip::uri::parse_uri;
//!
//!     let _res = ResponseGenerator::new()
//!                         .code(200)
//!                         .build()
//!                         .unwrap();
//!
//!     let uri = parse_uri::<VerboseError<&[u8]>>("sip:1@0.0.0.0:5060;transport=UDP".as_ref()).unwrap().1;
//!     let _req = RequestGenerator::new()
//!                         .method(Method::Invite)
//!                         .uri(uri)
//!                         .build()
//!                         .unwrap();
//!  ```
//!
//! ### Registration
//! The registration manager is used to generate REGISTER requests. Once
//! that is sent to the server you must wait for the Challange response pass
//! it to the `set_challenge` method of the RegistrationManager.
//! reqpeatedly calling the `get_request` method will cause the c_nonce
//! counter to be incremented and a new hash computed.

#[macro_use]
extern crate nom;
extern crate serde;

#[macro_use]
mod macros;

mod client;
mod core;
pub mod headers;
mod parse;
mod request;
mod response;
pub mod uri;

pub use crate::{
    client::{
        SoftPhone, MessageHelper, MessageWriter,
        InviteHelper, RegistrationManager,
        HeaderWriteConfig
    },
    request::RequestGenerator,
    response::ResponseGenerator,
    core::{
        Transport, Method, Version,
        SipMessage, parse_message, parse_version,
        parse_response, parse_request
    },
    headers::{
        ContentType,
        Language,
        Header, Headers, NamedHeader,
        AuthHeader, AuthContext, parse_header,
        AuthSchema, via::ViaHeader
    },
    uri::{Domain, UriParam, Uri, UriAuth, UriSchema, parse_uri}
};

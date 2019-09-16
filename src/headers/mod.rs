mod write;
pub mod parse;
pub use self::parse::parse_header;

mod content;
pub use self::content::ContentType;

mod language;
pub use self::language::Language;

mod named;
pub use self::named::NamedHeader;

use crate::core::Method;

#[derive(Debug, PartialEq, Clone)]
pub enum Header {
    To(NamedHeader),
    Contact(NamedHeader),
    From(NamedHeader),
    ReplyTo(NamedHeader),
    CSeq(u32, Method),
    MaxForwards(u32),
    Expires(u32),
    Accept(Vec<Method>),
    ContentLength(u32),
    Allow(Vec<Method>),
    UserAgent(String),
    CallId(String),
    ContentType(ContentType),
    ContentLanguage(Language),
    ContentEncoding(ContentType),
    AcceptLanguage(Language),
    AcceptEncoding(ContentType),
    AlertInfo(String),
    ErrorInfo(String),
    AuthenticationInfo(String),
    Authorization(String),
    CallInfo(String),
    InReplyTo(String),
    ContentDisposition(String),
    Date(String),
    MinExpires(u32),
    MimeVersion(f32),
    Organization(String),
    ProxyAuthenticate(String),
    ProxyAuthorization(String),
    ProxyRequire(String),
    Require(String),
    RetryAfter(String),
    Route(String),
    Subject(String),
    RecordRoute(String),
    Server(String),
    Supported(Vec<String>),
    Timestamp(u32),
    Unsupported(String),
    Warning(String),
    Via(String),
    Priority(String),
    WwwAuthenticate(String)
}

//! A Collection of Header implementations for common HTTP Headers.
//!
//! ## Mime
//!
//! Several header fields use MIME values for their contents. Keeping with the
//! strongly-typed theme, the [mime](https://docs.rs/mime) crate
//! is used, such as `ContentType(pub Mime)`.

pub use self::accept_charset::AcceptCharset;
pub use self::accept_encoding::AcceptEncoding;
pub use self::accept_language::AcceptLanguage;
pub use self::accept_ranges::{AcceptRanges, RangeUnit};
pub use self::accept::Accept;
pub use self::access_control_allow_credentials::AccessControlAllowCredentials;
pub use self::access_control_allow_headers::AccessControlAllowHeaders;
pub use self::access_control_allow_methods::AccessControlAllowMethods;
pub use self::access_control_allow_origin::AccessControlAllowOrigin;
pub use self::access_control_expose_headers::AccessControlExposeHeaders;
pub use self::access_control_max_age::AccessControlMaxAge;
pub use self::access_control_request_headers::AccessControlRequestHeaders;
pub use self::access_control_request_method::AccessControlRequestMethod;
pub use self::allow::Allow;
pub use self::authorization::{Authorization, Scheme, Basic, Bearer};
pub use self::cache_control::{CacheControl, CacheDirective};
pub use self::connection::{Connection, ConnectionOption};
pub use self::content_disposition::{ContentDisposition, DispositionType, DispositionParam};
pub use self::content_encoding::ContentEncoding;
pub use self::content_language::ContentLanguage;
pub use self::content_length::ContentLength;
pub use self::content_location::ContentLocation;
pub use self::content_range::{ContentRange, ContentRangeSpec};
pub use self::content_type::ContentType;
pub use self::cookie::{Cookie, CookieIter};
pub use self::date::Date;
pub use self::etag::ETag;
pub use self::expect::Expect;
pub use self::expires::Expires;
pub use self::from::From;
pub use self::host::Host;
pub use self::if_match::IfMatch;
pub use self::if_modified_since::IfModifiedSince;
pub use self::if_none_match::IfNoneMatch;
pub use self::if_range::IfRange;
pub use self::if_unmodified_since::IfUnmodifiedSince;
pub use self::last_event_id::LastEventId;
pub use self::last_modified::LastModified;
pub use self::link::{Link, LinkValue, RelationType, MediaDesc};
pub use self::location::Location;
pub use self::origin::Origin;
pub use self::pragma::Pragma;
pub use self::prefer::{Prefer, Preference};
pub use self::preference_applied::PreferenceApplied;
pub use self::proxy_authorization::ProxyAuthorization;
pub use self::range::{Range, ByteRangeSpec};
pub use self::referer::Referer;
pub use self::referrer_policy::ReferrerPolicy;
pub use self::retry_after::RetryAfter;
pub use self::server::Server;
pub use self::set_cookie::SetCookie;
pub use self::strict_transport_security::StrictTransportSecurity;
pub use self::te::Te;
pub use self::transfer_encoding::TransferEncoding;
pub use self::upgrade::{Upgrade, Protocol, ProtocolName};
pub use self::user_agent::UserAgent;
pub use self::vary::Vary;
pub use self::warning::Warning;

#[doc(hidden)]
#[macro_export]
macro_rules! bench_header(
    ($name:ident, $ty:ty, $value:expr) => {
        #[cfg(test)]
        #[cfg(feature = "nightly")]
        mod $name {
            use test::Bencher;
            use super::*;

            use header::{Header};

            #[bench]
            fn bench_parse(b: &mut Bencher) {
                let val = $value.into();
                b.iter(|| {
                    let _: $ty = Header::parse_header(&val).unwrap();
                });
            }

            #[bench]
            fn bench_format(b: &mut Bencher) {
                let raw = $value.into();
                let val: $ty = Header::parse_header(&raw).unwrap();
                b.iter(|| {
                    format!("{}", val);
                });
            }
        }
    }
);

#[doc(hidden)]
#[macro_export]
macro_rules! __hyper__deref {
    ($from:ty => $to:ty) => {
        impl ::std::ops::Deref for $from {
            type Target = $to;

            #[inline]
            fn deref(&self) -> &$to {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $from {
            #[inline]
            fn deref_mut(&mut self) -> &mut $to {
                &mut self.0
            }
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __hyper__tm {
    ($id:ident, $tm:ident{$($tf:item)*}) => {
        #[allow(unused_imports)]
        #[cfg(test)]
        mod $tm{
            use std::str;
            use $crate::header::*;
            use $crate::mime::*;
            use $crate::method::Method;
            use super::$id as HeaderField;
            $($tf)*
        }

    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! test_header {
    ($id:ident, $raw:expr) => {
        #[test]
        fn $id() {
            #[allow(unused, deprecated)]
            use std::ascii::AsciiExt;
            let raw = $raw;
            let a: Vec<Vec<u8>> = raw.iter().map(|x| x.to_vec()).collect();
            let a = a.into();
            let value = HeaderField::parse_header(&a);
            let result = format!("{}", value.unwrap());
            let expected = String::from_utf8(raw[0].to_vec()).unwrap();
            let result_cmp: Vec<String> = result
                .to_ascii_lowercase()
                .split(' ')
                .map(|x| x.to_owned())
                .collect();
            let expected_cmp: Vec<String> = expected
                .to_ascii_lowercase()
                .split(' ')
                .map(|x| x.to_owned())
                .collect();
            assert_eq!(result_cmp.concat(), expected_cmp.concat());
        }
    };
    ($id:ident, $raw:expr, $typed:expr) => {
        #[test]
        fn $id() {
            let a: Vec<Vec<u8>> = $raw.iter().map(|x| x.to_vec()).collect();
            let a = a.into();
            let val = HeaderField::parse_header(&a);
            let typed: Option<HeaderField> = $typed;
            // Test parsing
            assert_eq!(val.ok(), typed);
            // Test formatting
            if typed.is_some() {
                let raw = &($raw)[..];
                let mut iter = raw.iter().map(|b|str::from_utf8(&b[..]).unwrap());
                let mut joined = String::new();
                joined.push_str(iter.next().unwrap());
                for s in iter {
                    joined.push_str(", ");
                    joined.push_str(s);
                }
                assert_eq!(format!("{}", typed.unwrap()), joined);
            }
        }
    }
}

#[macro_export]
#[allow(missing_docs)]
macro_rules! header {
    // $a:meta: Attributes associated with the header item (usually docs)
    // $id:ident: Identifier of the header
    // $n:expr: Lowercase name of the header
    // $nn:expr: Nice name of the header

    // List header, zero or more items
    ($(#[$a:meta])*($id:ident, $n:expr) => ($item:ty)*) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(pub Vec<$item>);
        __hyper__deref!($id => Vec<$item>);
        impl $crate::header::Header for $id {
            fn header_name() -> &'static str {
                static NAME: &'static str = $n;
                NAME
            }
            #[inline]
            fn parse_header(raw: &$crate::header::Raw) -> $crate::Result<Self> {
                $crate::header::parsing::from_comma_delimited(raw).map($id)
            }
            #[inline]
            fn fmt_header(&self, f: &mut $crate::header::Formatter) -> ::std::fmt::Result {
                f.fmt_line(self)
            }
        }
        impl ::std::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                $crate::header::parsing::fmt_comma_delimited(f, &self.0[..])
            }
        }
    };
    // List header, one or more items
    ($(#[$a:meta])*($id:ident, $n:expr) => ($item:ty)+) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(pub Vec<$item>);
        __hyper__deref!($id => Vec<$item>);
        impl $crate::header::Header for $id {
            #[inline]
            fn header_name() -> &'static str {
                static NAME: &'static str = $n;
                NAME
            }
            #[inline]
            fn parse_header(raw: &$crate::header::Raw) -> $crate::Result<Self> {
                $crate::header::parsing::from_comma_delimited(raw).map($id)
            }
            #[inline]
            fn fmt_header(&self, f: &mut $crate::header::Formatter) -> ::std::fmt::Result {
                f.fmt_line(self)
            }
        }
        impl ::std::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                $crate::header::parsing::fmt_comma_delimited(f, &self.0[..])
            }
        }
    };
    // Single value header
    ($(#[$a:meta])*($id:ident, $n:expr) => [$value:ty]) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(pub $value);
        __hyper__deref!($id => $value);
        impl $crate::header::Header for $id {
            #[inline]
            fn header_name() -> &'static str {
                static NAME: &'static str = $n;
                NAME
            }
            #[inline]
            fn parse_header(raw: &$crate::header::Raw) -> $crate::Result<Self> {
                $crate::header::parsing::from_one_raw_str(raw).map($id)
            }
            #[inline]
            fn fmt_header(&self, f: &mut $crate::header::Formatter) -> ::std::fmt::Result {
                f.fmt_line(self)
            }
        }
        impl ::std::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
    };
    // Single value header (internal)
    ($(#[$a:meta])*($id:ident, $n:expr) => danger [$value:ty]) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(pub $value);
        __hyper__deref!($id => $value);
        impl $crate::header::Header for $id {
            #[inline]
            fn header_name() -> &'static str {
                static NAME: &'static str = $n;
                NAME
            }
            #[inline]
            fn parse_header(raw: &$crate::header::Raw) -> $crate::Result<Self> {
                $crate::header::parsing::from_one_raw_str(raw).map($id)
            }
            #[inline]
            fn fmt_header(&self, f: &mut $crate::header::Formatter) -> ::std::fmt::Result {
                f.danger_fmt_line_without_newline_replacer(self)
            }
        }
        impl ::std::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
    };
    // Single value cow header
    ($(#[$a:meta])*($id:ident, $n:expr) => Cow[$value:ty]) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub struct $id(::std::borrow::Cow<'static,$value>);
        impl $id {
            /// Creates a new $id
            pub fn new<I: Into<::std::borrow::Cow<'static,$value>>>(value: I) -> Self {
                $id(value.into())
            }
        }
        impl ::std::ops::Deref for $id {
            type Target = $value;
            #[inline]
            fn deref(&self) -> &Self::Target {
                &(self.0)
            }
        }
        impl $crate::header::Header for $id {
            #[inline]
            fn header_name() -> &'static str {
                static NAME: &'static str = $n;
                NAME
            }
            #[inline]
            fn parse_header(raw: &$crate::header::Raw) -> $crate::Result<Self> {
                $crate::header::parsing::from_one_raw_str::<<$value as ::std::borrow::ToOwned>::Owned>(raw).map($id::new)
            }
            #[inline]
            fn fmt_header(&self, f: &mut $crate::header::Formatter) -> ::std::fmt::Result {
                f.fmt_line(self)
            }
        }
        impl ::std::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                ::std::fmt::Display::fmt(&self.0, f)
            }
        }
    };
    // List header, one or more items with "*" option
    ($(#[$a:meta])*($id:ident, $n:expr) => {Any / ($item:ty)+}) => {
        $(#[$a])*
        #[derive(Clone, Debug, PartialEq)]
        pub enum $id {
            /// Any value is a match
            Any,
            /// Only the listed items are a match
            Items(Vec<$item>),
        }
        impl $crate::header::Header for $id {
            #[inline]
            fn header_name() -> &'static str {
                static NAME: &'static str = $n;
                NAME
            }
            #[inline]
            fn parse_header(raw: &$crate::header::Raw) -> $crate::Result<Self> {
                // FIXME: Return None if no item is in $id::Only
                if raw.len() == 1 {
                    if &raw[0] == b"*" {
                        return Ok($id::Any)
                    }
                }
                $crate::header::parsing::from_comma_delimited(raw).map($id::Items)
            }
            #[inline]
            fn fmt_header(&self, f: &mut $crate::header::Formatter) -> ::std::fmt::Result {
                f.fmt_line(self)
            }
        }
        impl ::std::fmt::Display for $id {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $id::Any => f.write_str("*"),
                    $id::Items(ref fields) => $crate::header::parsing::fmt_comma_delimited(
                        f, &fields[..])
                }
            }
        }
    };

    // optional test module
    ($(#[$a:meta])*($id:ident, $n:expr) => ($item:ty)* $tm:ident{$($tf:item)*}) => {
        header! {
            $(#[$a])*
            ($id, $n) => ($item)*
        }

        __hyper__tm! { $id, $tm { $($tf)* }}
    };
    ($(#[$a:meta])*($id:ident, $n:expr) => ($item:ty)+ $tm:ident{$($tf:item)*}) => {
        header! {
            $(#[$a])*
            ($id, $n) => ($item)+
        }

        __hyper__tm! { $id, $tm { $($tf)* }}
    };
    ($(#[$a:meta])*($id:ident, $n:expr) => [$item:ty] $tm:ident{$($tf:item)*}) => {
        header! {
            $(#[$a])*
            ($id, $n) => [$item]
        }

        __hyper__tm! { $id, $tm { $($tf)* }}
    };
    ($(#[$a:meta])*($id:ident, $n:expr) => danger [$item:ty] $tm:ident{$($tf:item)*}) => {
        header! {
            $(#[$a])*
            ($id, $n) => danger [$item]
        }

        __hyper__tm! { $id, $tm { $($tf)* }}
    };
    ($(#[$a:meta])*($id:ident, $n:expr) => Cow[$item:ty] $tm:ident{$($tf:item)*}) => {
        header! {
            $(#[$a])*
            ($id, $n) => Cow[$item]
        }

        __hyper__tm! { $id, $tm { $($tf)* }}
    };
    ($(#[$a:meta])*($id:ident, $n:expr) => {Any / ($item:ty)+} $tm:ident{$($tf:item)*}) => {
        header! {
            $(#[$a])*
            ($id, $n) => {Any / ($item)+}
        }

        __hyper__tm! { $id, $tm { $($tf)* }}
    };
}


mod accept_charset;
mod accept_encoding;
mod accept_language;
mod accept_ranges;
mod accept;
mod access_control_allow_credentials;
mod access_control_allow_headers;
mod access_control_allow_methods;
mod access_control_allow_origin;
mod access_control_expose_headers;
mod access_control_max_age;
mod access_control_request_headers;
mod access_control_request_method;
mod allow;
mod authorization;
mod cache_control;
mod connection;
mod content_disposition;
mod content_encoding;
mod content_language;
mod content_length;
mod content_location;
mod content_range;
mod content_type;
mod cookie;
mod date;
mod etag;
mod expect;
mod expires;
mod from;
mod host;
mod if_match;
mod if_modified_since;
mod if_none_match;
mod if_range;
mod if_unmodified_since;
mod last_event_id;
mod last_modified;
mod link;
mod location;
mod origin;
mod pragma;
mod prefer;
mod preference_applied;
mod proxy_authorization;
mod range;
mod referer;
mod referrer_policy;
mod retry_after;
mod server;
mod set_cookie;
mod strict_transport_security;
mod te;
mod transfer_encoding;
mod upgrade;
mod user_agent;
mod vary;
mod warning;

/// Define a string-valued enum that tolerates values this crate doesn't know.
///
/// Auth0 adds values to many string fields over time. A closed serde enum
/// would fail to deserialize the whole response when that happens, so these
/// enums keep unrecognized values in an `Other(String)` variant and serialize
/// them back unchanged.
///
/// Build values from strings with `From` so known values map to their named
/// variant rather than `Other`.
// Only feature-gated modules use this macro.
#[cfg_attr(not(any(feature = "clients", feature = "jobs")), allow(unused_macros))]
macro_rules! string_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident => $value:literal,
            )+
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
        #[serde(from = "String", into = "String")]
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )+
            /// A value this crate does not recognize. The original string is preserved.
            Other(String),
        }

        impl $name {
            /// Return the wire value for this variant.
            pub fn as_str(&self) -> &str {
                match self {
                    $( Self::$variant => $value, )+
                    Self::Other(value) => value,
                }
            }
        }

        impl From<String> for $name {
            fn from(value: String) -> Self {
                match value.as_str() {
                    $( $value => Self::$variant, )+
                    _ => Self::Other(value),
                }
            }
        }

        impl From<&str> for $name {
            fn from(value: &str) -> Self {
                match value {
                    $( $value => Self::$variant, )+
                    _ => Self::Other(value.to_owned()),
                }
            }
        }

        impl From<$name> for String {
            fn from(value: $name) -> Self {
                match value {
                    $name::Other(value) => value,
                    known => known.as_str().to_owned(),
                }
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

#[cfg(test)]
mod tests {
    string_enum! {
        pub enum Colour {
            Red => "red",
            DarkBlue => "dark_blue",
        }
    }

    #[test]
    fn known_values_round_trip() {
        let colour: Colour = serde_json::from_str("\"dark_blue\"").unwrap();
        assert_eq!(colour, Colour::DarkBlue);
        assert_eq!(serde_json::to_string(&colour).unwrap(), "\"dark_blue\"");
    }

    #[test]
    fn unknown_values_are_preserved() {
        let colour: Colour = serde_json::from_str("\"green\"").unwrap();
        assert_eq!(colour, Colour::Other("green".into()));
        assert_eq!(serde_json::to_string(&colour).unwrap(), "\"green\"");
        assert_eq!(colour.to_string(), "green");
    }

    #[test]
    fn from_str_maps_known_values_to_named_variants() {
        assert_eq!(Colour::from("red"), Colour::Red);
        assert_eq!(Colour::from("red".to_string()), Colour::Red);
    }
}

use std::str::FromStr;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Network {
    MainNet,
    TestNet,
}

impl<'de> serde::Deserialize<'de> for Network {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt::{self, Formatter};

        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Network;

            fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
                formatter.write_str("string value for bitcoin network.")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Network::from_str(v).map_err(E::custom)
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_str(&v)
            }
        }

        deserializer.deserialize_any(Visitor)
    }
}

impl FromStr for Network {
    type Err = String;

    fn from_str(s: &str) -> Result<Network, Self::Err> {
        match s.as_ref() {
            "testnet" => Ok(Network::TestNet),
            "mainnet" => Ok(Network::MainNet),
            _ => Err(String::from("Invalid value for bitcoin network.")),
        }
    }
}

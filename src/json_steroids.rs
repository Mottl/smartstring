// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{SmartString, SmartStringMode};

use json_steroids::{JsonDeserialize, JsonSerialize};

impl<T: SmartStringMode> JsonSerialize for SmartString<T> {
    fn json_serialize<W: json_steroids::writer::Writer>(
        &self,
        writer: &mut json_steroids::JsonWriter<W>,
    ) {
        writer.write_string(self);
    }
}

impl<'de, T: SmartStringMode> JsonDeserialize<'de> for SmartString<T> {
    fn json_deserialize(
        parser: &mut json_steroids::JsonParser<'de>,
    ) -> json_steroids::Result<Self> {
        Ok(parser.parse_string()?.into_owned().into())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Compact;

    #[test]
    fn test_ser_de() {
        let checks = [
            ("", r#""""#),
            ("small test", r#""small test""#),
            (
                "longer than inline string for json_steroids testing",
                r#""longer than inline string for json_steroids testing""#,
            ),
            (r#"some "quotes"#, r#""some \"quotes""#),
        ];

        for (s, ser_s) in checks {
            let value = SmartString::<Compact>::from(s);
            let serialized = json_steroids::to_string(&value);
            assert_eq!(serialized, ser_s);

            let deserialized: crate::SmartString<Compact> =
                json_steroids::from_str(&serialized).unwrap();
            assert_eq!(&*deserialized, s);
        }
    }
}

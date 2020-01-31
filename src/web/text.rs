use crate::{Colors, Error, Serde, Text, TextStyle, Tree, Unit};
use std::collections::HashMap;

impl Serde<Text, String> for Text {
    fn de(s: String) -> Result<Text, Error> {
        let t = Tree::de(s)?;
        if t.children.len() != 1 {
            return Err(Error::DeserializeHtmlError(
                "deserialize Text failed, children's length should be 1".into(),
            ));
        }

        let text = t.children[0].borrow();
        Ok(Text::new(
            text.attrs.get("text").unwrap_or(&"").to_string(),
            TextStyle::de(t.attrs.get("style").unwrap_or(&"").to_string())?.into(),
        ))
    }

    fn ser(self) -> String {
        let mut m = HashMap::<&'static str, &'static str>::new();
        let mut cm = HashMap::<&'static str, &'static str>::new();
        m.insert("style", Box::leak(box self.style.ser()));
        cm.insert("text", Box::leak(box self.text));

        Tree::new(m, vec![Tree::new(cm, vec![], None, "plain")], None, "p")
            .borrow()
            .to_owned()
            .ser()
    }
}

impl Serde<TextStyle, String> for TextStyle {
    fn de(s: String) -> Result<TextStyle, Error> {
        let mut ts = TextStyle::default();
        s.split(";").collect::<Vec<&str>>().iter().for_each(|x| {
            let v = x[(x.find(":").unwrap_or(0) + 1)..].trim();
            match x {
                k if k.contains("color") => {
                    ts.color = Colors::de(v.into()).unwrap_or(Colors::Black)
                }
                k if k.contains("font-weight") => {
                    ts.weight = Unit::de(v.into()).unwrap_or(Unit::None(400.0));
                    ts.bold = match ts.weight {
                        Unit::None(x) => x == 700.0,
                        _ => false,
                    }
                }
                k if k.contains("font-style") => {
                    ts.italic = match v {
                        "italic" => true,
                        _ => false,
                    };
                }
                k if k.contains("font-size") => {
                    ts.size = Unit::de(v.into()).unwrap_or(Unit::Rem(1.0))
                }
                k if k.contains("height") => {
                    ts.height = Unit::de(v.into()).unwrap_or(Unit::Rem(1.0))
                }
                k if k.contains("font-stretch") => {
                    ts.stretch = Unit::de(v.into()).unwrap_or(Unit::Percent(100.0))
                }
                _ => {}
            }
        });

        Ok(ts)
    }

    fn ser(self) -> String {
        format!(
            "color: {}; font-weight: {}; font-style: {}; font-size: {}; height: {}; font-stretch: {};",
            self.color.ser(), match self.bold {
                true => "700".into(),
                false => self.weight.ser(),
            },
            match self.italic {
                true => "italic",
                false => "normal"
            },
            self.size.ser(),
            self.height.ser(),
            self.stretch.ser(),
        )
    }
}

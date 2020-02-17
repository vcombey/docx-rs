//! Style Definitions
//!
//! The corresponding ZIP item is `/word/styles.xml`.

mod default_style;
mod style;

pub use self::{default_style::*, style::*};

use docx_codegen::{IntoOwned, XmlRead, XmlWrite};
use std::io::Write;

use crate::{
    error::{Error, Result},
    schema::SCHEMA_MAIN,
};

/// Styles of the document
///
/// Styles are predefined sets of properties which can be applied to text.
///
/// ```rust
/// use docx::styles::*;
///
/// let style = Styles::new()
///     .default(DefaultStyle::default())
///     .push(Style::paragraph("style_id"));
/// ```
#[derive(Debug, Default, XmlRead, XmlWrite, IntoOwned)]
#[cfg_attr(test, derive(PartialEq))]
#[xml(tag = "w:styles")]
#[xml(extend_attrs = "styles_extend_attrs")]
pub struct Styles<'a> {
    /// Specifies the default set of properties.
    #[xml(child = "w:docDefaults")]
    pub default: Option<DefaultStyle<'a>>,
    /// Specifies a set of properties.
    #[xml(child = "w:style")]
    pub styles: Vec<Style<'a>>,
}

#[inline]
fn styles_extend_attrs<W: Write>(_: &Styles, mut w: W) -> Result<()> {
    write!(w, " xmlns:w=\"{}\"", SCHEMA_MAIN)?;
    Ok(())
}

impl<'a> Styles<'a> {
    pub fn new() -> Self {
        <Styles as Default>::default()
    }

    pub fn default(&mut self, style: DefaultStyle<'a>) -> &mut Self {
        self.default = Some(style);
        self
    }

    pub fn push(&mut self, style: Style<'a>) -> &mut Self {
        self.styles.push(style);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::__test_read_write;

    __test_read_write!(
        Styles,
        Styles::new(),
        format!(r#"<w:styles xmlns:w="{}"></w:styles>"#, SCHEMA_MAIN).as_str(),
        Styles {
            default: Some(DefaultStyle::default()),
            styles: vec![]
        },
        format!(
            r#"<w:styles xmlns:w="{}"><w:docDefaults></w:docDefaults></w:styles>"#,
            SCHEMA_MAIN
        )
        .as_str(),
        Styles {
            default: None,
            styles: vec![Style::paragraph("")]
        },
        format!(
            r#"<w:styles xmlns:w="{}"><w:style w:type="paragraph" w:styleId=""></w:style></w:styles>"#,
            SCHEMA_MAIN
        )
        .as_str(),
    );
}
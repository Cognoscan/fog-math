use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};

#[inline]
fn is_false(b: &bool) -> bool {
    !b
}

#[inline]
fn u32_is_one(v: &u32) -> bool {
    *v == 1
}

#[inline]
fn u32_one() -> u32 {
    1
}

/// Character Variant types. In general, prefer using Normal and including the
/// actual Unicode character.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum Variant {
    #[default]
    Normal,
    Bold,
    Italic,
    BoldItalic,
    DoubleStruck,
    BoldFraktur,
    Script,
    BoldScript,
    Fraktur,
    SansSerif,
    BoldSansSerif,
    SansSerifItalic,
    SansSerifBoldItalic,
    Monospace,
    Initial,
    Tailed,
    Looped,
    Stretched,
}

/// Adjust the script level of an element, either by setting it to a specific
/// value or changing the value by some amount.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScriptLevel {
    /// Increment/decrement the script level.
    Add(i32),
    /// Set the script level to a specific value.
    Set(u32),
}

/// A Math element, including any global attributes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Element {
    /// The actual element.
    e: MathElement,
    /// Optional attributes for the element.
    a: Option<Box<Attributes>>,
}

/// A Math element. Mirrors the elements in MathML.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum MathElement {
    /// A single-character operator with default properties.
    Op(char),
    /// Full operator. Some additional properties may have been overridden.
    Oper(Operator),
    /// Resolved operator. All properties are completely defined.
    ResolvedOper(ResolvedOperator),
    /// Raw text
    Text(String),
    /// An identifier, like a function name, variable, or symbolic constant.
    Id {
        t: String,
        /// Override the default italics that get used when the text is a single character.
        #[serde(default, skip_serializing_if = "is_false")]
        normal: bool,
    },
    /// A numeric value.
    Num(String),
    /// An error message. Meant to help converters display an error when parsing completes.
    Err(String),
    /// A blank space.
    Space(Space),
    /// A string literal, meant to be interpretted by programming languages and
    /// computer algebra systems.
    Str(String),
    /// Phantom elements. Rendered invisibly, but still affects layout.
    Phantom(Vec<Element>),
    /// A row of elements, used to group sub-expressions together.
    Row(Vec<Element>),
    /// Padding around elements.
    Padding(Padding),
    /// A fraction with a numerator and denominator.
    Frac {
        /// Line thickness, as a fraction of standard line thickness.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        line_thickness: Option<f32>,
        /// Numerator
        num: Box<Element>,
        /// Denominator
        den: Box<Element>,
    },
    /// Square root
    Sqrt(Box<Element>),
    /// Root with an explicit index.
    Root {
        /// Base of the root, also known as the radicand.
        base: Box<Element>,
        /// Index of the root, sometimes called the degree.
        index: Box<Element>,
    },
    /// Superscript
    Sup {
        base: Box<Element>,
        sup: Box<Element>,
    },
    /// Subscript
    Sub {
        base: Box<Element>,
        sub: Box<Element>,
    },
    /// Both superscript and subscript
    SubSup {
        base: Box<Element>,
        sub: Box<Element>,
        sup: Box<Element>,
    },
    /// Overscript
    Over {
        base: Box<Element>,
        over: Box<Element>,
        #[serde(default, skip_serializing_if = "is_false")]
        accent: bool,
    },
    /// Underscript
    Under {
        base: Box<Element>,
        under: Box<Element>,
        #[serde(default, skip_serializing_if = "is_false")]
        accent_under: bool,
    },
    /// Both overscript and underscript
    UnderOver {
        base: Box<Element>,
        under: Box<Element>,
        over: Box<Element>,
        #[serde(default, skip_serializing_if = "is_false")]
        accent: bool,
        #[serde(default, skip_serializing_if = "is_false")]
        accent_under: bool,
    },
    /// Multiscript, used to attach an arbitrary number of superscripts and
    /// subscripts both before and after the base element.
    MultiScript {
        base: Box<Element>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        post: Vec<Pair>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pre: Vec<Pair>,
    },
    /// A table
    Table {
        rows: Vec<TableRow>,
    },
}

/// A row in a table.
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct TableRow {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cells: Vec<TableCell>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub a: Option<Box<Attributes>>,
}

/// A cell in a table.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TableCell {
    #[serde(default = "u32_one", skip_serializing_if = "u32_is_one")]
    pub col_span: u32,
    #[serde(default = "u32_one", skip_serializing_if = "u32_is_one")]
    pub row_span: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub elems: Vec<Element>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub a: Option<Box<Attributes>>,
}

/// A pair of superscript and subscript, used by the Multiscript element.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Pair {
    pub sup: Box<Element>,
    pub sub: Box<Element>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Padding {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub elems: Vec<Element>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Length>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Length>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<Length>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lspace: Option<Length>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voffset: Option<Length>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Space {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<Length>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<Length>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<Length>,
}

/// Form of the operation. Normally derived from the operator's base character.
#[derive(Clone, Debug, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpForm {
    Prefix,
    Postfix,
    Infix,
}

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Operator {
    /// The operator's text, which should be a single character.
    /// MathML permits multi-character Operators but treats them as regular
    /// Text. A MathML transformer should likewise convert such MathML to a
    /// plain [`MathElement::Text`] instead, dropping any other
    /// operator-specific attributes in the process.
    pub t: char,
    /// Operator form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<OpForm>,
    /// Maximum size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_size: Option<LengthOrFraction>,
    /// Minimum size
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_size: Option<LengthOrFraction>,
    /// Left padding space
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lspace: Option<LengthOrFraction>,
    /// Right padding space
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rspace: Option<LengthOrFraction>,
    /// If the operator should stretch
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stretchy: Option<bool>,
    /// If the operator should stretch symmetrically
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symmetric: Option<bool>,
    /// If the operator is "large"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_op: Option<bool>,
    /// Turns underscript/overscript into superscript/subscript
    #[serde(skip_serializing_if = "Option::is_none")]
    pub movable_limits: Option<bool>,
    // Semantic: indicates the operator is a separator
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separator: Option<bool>,
    // Semantic: indicates the operator is a fence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fence: Option<bool>,
}

/// An operator whose properties have been completely resolved.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResolvedOperator {
    pub t: char,
    pub form: OpForm,
    pub max_size: Length,
    pub min_size: Length,
    pub lspace: Length,
    pub rspace: Length,
    pub stretchy: bool,
    pub symmetric: bool,
    pub large_op: bool,
    pub movable_limits: bool,
    pub separator: bool,
    pub fence: bool,
}

/// A font-relative length or a specified fraction of another length.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum LengthOrFraction {
    /// Font-relative unit, usually used for widths
    Em(f32),
    /// Font-relative unit, usually used for heights
    Ex(f32),
    /// Fraction, with 1 being 100%. Negative values are allowed but don't
    /// always have meaning.
    Frac(f32),
}

/// A font-relative length.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub enum Length {
    /// Font-relative unit, usually used for widths
    Em(f32),
    /// Font-relative unit, usually used for heights
    Ex(f32),
}

/// Global Element attributes. Mostly contains styling information, but also
/// includes the option to contain arbitrary additional data.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Attributes {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub class: Vec<String>,
    #[serde(skip_serializing_if = "is_false")]
    pub rtl: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_style: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<Variant>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script_level: Option<ScriptLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<BTreeMap<String, fog_pack::types::Value>>,
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn sizes() {
        let elem_size = std::mem::size_of::<Element>();
        let math_size = std::mem::size_of::<MathElement>();
        let op_size = std::mem::size_of::<Operator>();
        let rop_size = std::mem::size_of::<ResolvedOperator>();
        dbg!(elem_size);
        dbg!(math_size);
        dbg!(op_size);
        dbg!(rop_size);
        panic!();
    }
}
// Code generated; DO NOT EDIT.

use num_derive::FromPrimitive;

#[derive(Clone, Debug, PartialEq, Eq, FromPrimitive)]
pub enum Html {
    End = 0,
    LTBANG = 1,
    DoctypeToken1 = 2,
    GT = 3,
    Doctype2 = 4,
    LT = 5,
    SLASHGT = 6,
    LTSLASH = 7,
    EQ = 8,
    AttributeName = 9,
    AttributeValue = 10,
    Entity = 11,
    SQUOTE = 12,
    AttributeValue2 = 13,
    DQUOTE = 14,
    AttributeValue3 = 15,
    Text = 16,
    TagName = 17,
    TagName2 = 18,
    TagName3 = 19,
    TagName4 = 20,
    ErroneousEndTagName = 21,
    ImplicitEndTag = 22,
    RawText = 23,
    Comment = 24,
    Document = 25,
    Doctype = 26,
    Node = 27,
    Element = 28,
    ScriptElement = 29,
    StyleElement = 30,
    StartTag = 31,
    StartTag2 = 32,
    StartTag3 = 33,
    SelfClosingTag = 34,
    EndTag = 35,
    ErroneousEndTag = 36,
    Attribute = 37,
    QuotedAttributeValue = 38,
    DocumentRepeat1 = 39,
    StartTagRepeat1 = 40,
    Error = 41,
}

impl From<Html> for &'static str {
    #[inline(always)]
    fn from(tok: Html) -> Self {
        match tok {
            Html::End => "end",
            Html::LTBANG => "<!",
            Html::DoctypeToken1 => "doctype_token1",
            Html::GT => ">",
            Html::Doctype2 => "doctype",
            Html::LT => "<",
            Html::SLASHGT => "/>",
            Html::LTSLASH => "</",
            Html::EQ => "=",
            Html::AttributeName => "attribute_name",
            Html::AttributeValue => "attribute_value",
            Html::Entity => "entity",
            Html::SQUOTE => "'",
            Html::AttributeValue2 => "attribute_value",
            Html::DQUOTE => "\"",
            Html::AttributeValue3 => "attribute_value",
            Html::Text => "text",
            Html::TagName => "tag_name",
            Html::TagName2 => "tag_name",
            Html::TagName3 => "tag_name",
            Html::TagName4 => "tag_name",
            Html::ErroneousEndTagName => "erroneous_end_tag_name",
            Html::ImplicitEndTag => "_implicit_end_tag",
            Html::RawText => "raw_text",
            Html::Comment => "comment",
            Html::Document => "document",
            Html::Doctype => "doctype",
            Html::Node => "_node",
            Html::Element => "element",
            Html::ScriptElement => "script_element",
            Html::StyleElement => "style_element",
            Html::StartTag => "start_tag",
            Html::StartTag2 => "start_tag",
            Html::StartTag3 => "start_tag",
            Html::SelfClosingTag => "self_closing_tag",
            Html::EndTag => "end_tag",
            Html::ErroneousEndTag => "erroneous_end_tag",
            Html::Attribute => "attribute",
            Html::QuotedAttributeValue => "quoted_attribute_value",
            Html::DocumentRepeat1 => "document_repeat1",
            Html::StartTagRepeat1 => "start_tag_repeat1",
            Html::Error => "ERROR",
        }
    }
}

impl From<u16> for Html {
    #[inline(always)]
    fn from(x: u16) -> Self {
        num::FromPrimitive::from_u16(x).unwrap_or(Self::Error)
    }
}

// Html == u16
impl PartialEq<u16> for Html {
    #[inline(always)]
    fn eq(&self, x: &u16) -> bool {
        *self == Into::<Self>::into(*x)
    }
}

// u16 == Html
impl PartialEq<Html> for u16 {
    #[inline(always)]
    fn eq(&self, x: &Html) -> bool {
        *x == *self
    }
}

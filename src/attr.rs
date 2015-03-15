use std::iter::IntoIterator;

use syntax::ast;
use syntax::attr;
use syntax::codemap::{DUMMY_SP, Span, respan};
use syntax::parse::token;
use syntax::ptr::P;

use invoke::Invoke;
use lit::LitBuilder;
use str::ToInternedString;

//////////////////////////////////////////////////////////////////////////////

pub struct AttrBuilder<F> {
    callback: F,
    span: Span,
}

impl<F> AttrBuilder<F>
    where F: Invoke<ast::Attribute>,
{
    pub fn new_with_callback(callback: F) -> Self {
        AttrBuilder {
            callback: callback,
            span: DUMMY_SP,
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn build_meta_item(self, item: P<ast::MetaItem>) -> F::Result {
        let attr = respan(self.span, ast::Attribute_ {
            id: attr::mk_attr_id(),
            style: ast::AttrOuter,
            value: item,
            is_sugared_doc: false,
        });
        self.callback.invoke(attr)
    }

    pub fn build_meta_item_(self, item: ast::MetaItem_) -> F::Result {
        let item = P(respan(self.span, item));
        self.build_meta_item(item)
    }

    pub fn word<T>(self, word: T) -> F::Result
        where T: ToInternedString
    {
        self.build_meta_item_(ast::MetaWord(word.to_interned_string()))
    }

    pub fn list<T>(self, word: T) -> AttrListBuilder<Self>
        where T: ToInternedString
    {
        AttrListBuilder::new_with_callback(word, self)
    }

    pub fn name_value<T>(self, name: T) -> LitBuilder<AttrNameValueBuilder<Self>>
        where T: ToInternedString,
    {
        LitBuilder::new_with_callback(AttrNameValueBuilder {
            callback: self,
            name: name.to_interned_string(),
        })
    }

    pub fn inline(self) -> F::Result {
        self.word("inline")
    }

    pub fn test(self) -> F::Result {
        self.word("test")
    }

    pub fn build_allow<I, T>(self, iter: I) -> F::Result
        where I: IntoIterator<Item=T>,
              T: ToInternedString,
    {
        self.list("allow").words(iter).build()
    }

    pub fn build_warn<I, T>(self, iter: I) -> F::Result
        where I: IntoIterator<Item=T>,
              T: ToInternedString,
    {
        self.list("warn").words(iter).build()
    }

    pub fn build_deny<I, T>(self, iter: I) -> F::Result
        where I: IntoIterator<Item=T>,
              T: ToInternedString,
    {
        self.list("deny").words(iter).build()
    }

    pub fn build_features<I, T>(self, iter: I) -> F::Result
        where I: IntoIterator<Item=T>,
              T: ToInternedString,
    {
        self.list("feature").words(iter).build()
    }

    pub fn build_plugins<I, T>(self, iter: I) -> F::Result
        where I: IntoIterator<Item=T>,
              T: ToInternedString,
    {
        self.list("plugin").words(iter).build()
    }
}

impl<F> Invoke<P<ast::MetaItem>> for AttrBuilder<F>
    where F: Invoke<ast::Attribute>,
{
    type Result = F::Result;

    fn invoke(self, item: P<ast::MetaItem>) -> F::Result {
        self.build_meta_item(item)
    }
}

impl<F> Invoke<ast::MetaItem_> for AttrBuilder<F>
    where F: Invoke<ast::Attribute>,
{
    type Result = F::Result;

    fn invoke(self, item: ast::MetaItem_) -> F::Result {
        self.build_meta_item_(item)
    }
}

//////////////////////////////////////////////////////////////////////////////

pub struct AttrListBuilder<F> {
    callback: F,
    span: Span,
    name: token::InternedString,
    items: Vec<P<ast::MetaItem>>,
}

impl<F> AttrListBuilder<F>
    where F: Invoke<P<ast::MetaItem>>,
{
    pub fn new_with_callback<T>(name: T, callback: F) -> Self
        where T: ToInternedString,
    {
        AttrListBuilder {
            callback: callback,
            span: DUMMY_SP,
            name: name.to_interned_string(),
            items: vec![],
        }
    }

    pub fn span(mut self, span: Span) -> Self {
        self.span = span;
        self
    }

    pub fn with_meta_items<I>(mut self, iter: I) -> Self
        where I: IntoIterator<Item=P<ast::MetaItem>>,
    {
        self.items.extend(iter);
        self
    }

    pub fn with_meta_items_<I>(self, iter: I) -> Self
        where I: IntoIterator<Item=ast::MetaItem_>,
    {
        let iter = iter.into_iter();
        let span = self.span;
        self.with_meta_items(iter.map(|item| P(respan(span, item))))
    }

    pub fn with_meta_item(mut self, item: P<ast::MetaItem>) -> Self {
        self.items.push(item);
        self
    }

    pub fn with_meta_item_(self, item: ast::MetaItem_) -> Self {
        let span = self.span;
        self.with_meta_item(P(respan(span, item)))
    }

    pub fn words<I, T>(self, iter: I) -> Self
        where I: IntoIterator<Item=T>,
              T: ToInternedString,
    {
        let iter = iter.into_iter();
        self.with_meta_items_(iter.map(|word| ast::MetaWord(word.to_interned_string())))
    }

    pub fn word<T>(self, word: T) -> Self
        where T: ToInternedString,
    {
        self.with_meta_item_(ast::MetaWord(word.to_interned_string()))
    }

    pub fn list<T>(self, name: T) -> AttrListBuilder<Self>
        where T: ToInternedString,
    {
        AttrListBuilder::new_with_callback(name, self)
    }

    pub fn name_value<T>(self, name: T) -> LitBuilder<AttrNameValueBuilder<Self>>
        where T: ToInternedString,
    {
        LitBuilder::new_with_callback(AttrNameValueBuilder {
            callback: self,
            name: name.to_interned_string(),
        })
    }

    pub fn build(self) -> F::Result {
        let item = respan(self.span, ast::MetaList(self.name, self.items));
        self.callback.invoke(P(item))
    }
}

impl<F> Invoke<P<ast::MetaItem>> for AttrListBuilder<F>
    where F: Invoke<P<ast::MetaItem>>,
{
    type Result = Self;

    fn invoke(self, item: P<ast::MetaItem>) -> Self {
        self.with_meta_item(item)
    }
}

impl<F> Invoke<ast::MetaItem_> for AttrListBuilder<F>
    where F: Invoke<P<ast::MetaItem>>,
{
    type Result = Self;

    fn invoke(self, item: ast::MetaItem_) -> Self {
        self.with_meta_item_(item)
    }
}

//////////////////////////////////////////////////////////////////////////////

pub struct AttrNameValueBuilder<F> {
    callback: F,
    name: token::InternedString,
}

impl<F: Invoke<ast::MetaItem_>> Invoke<P<ast::Lit>> for AttrNameValueBuilder<F> {
    type Result = F::Result;

    fn invoke(self, value: P<ast::Lit>) -> F::Result {
        let item = ast::MetaNameValue(self.name, (*value).clone());
        self.callback.invoke(item)
    }
}


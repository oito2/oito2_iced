// Copyright (C) 2026  OITO2
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Declarations of items and members without bodies, attributes or private
//! fields, pretty-printed by `prettyplease`.

use syn::{
    Block, Expr, Fields, FnArg, ImplItem, Item, ItemConst, ItemFn, ItemStatic, Pat, Signature,
    TraitItem, Type, Visibility,
};

use crate::attrs;
use crate::model::{Kind, Member};

/// The placeholder that stands for an elided expression or for private
/// fields while an item is printed.
const PLACEHOLDER: &str = "__oito2_elided__";

/// The longest declaration of a constant or static kept with its value.
const MAX_VALUE_WIDTH: usize = 100;

/// Prints `item` as a file of its own, without the trailing newline.
fn unparse(item: Item) -> String {
    let file = syn::File {
        shebang: None,
        attrs: Vec::new(),
        items: vec![item],
    };
    prettyplease::unparse(&file).trim_end().to_owned()
}

/// Whether `vis` is plain `pub`.
pub fn is_public(vis: &Visibility) -> bool {
    matches!(vis, Visibility::Public(_))
}

/// The kind and declaration of `item`, or `None` for items the
/// reference does not list.
pub fn item(item: &Item) -> Option<(Kind, String)> {
    let item = item.clone();
    Some(match item {
        Item::Fn(item) => (Kind::Function, function(item.vis, item.sig)),
        Item::Const(item) => (Kind::Constant, constant(item)),
        Item::Static(item) => (Kind::Static, statik(item)),
        Item::Type(mut item) => {
            item.attrs.clear();
            (Kind::Type, unparse(Item::Type(item)))
        }
        Item::Struct(mut item) => {
            item.attrs.retain(kept);
            let private = only_public_fields(&mut item.fields);
            (
                Kind::Struct,
                private_fields(unparse(Item::Struct(item)), private),
            )
        }
        Item::Union(mut item) => {
            item.attrs.retain(kept);
            let mut fields = Fields::Named(item.fields);
            let private = only_public_fields(&mut fields);
            let Fields::Named(fields) = fields else {
                return None;
            };
            item.fields = fields;
            (
                Kind::Struct,
                private_fields(unparse(Item::Union(item)), private),
            )
        }
        Item::Enum(mut item) => {
            item.attrs.retain(kept);
            item.variants = item
                .variants
                .into_iter()
                .filter(|variant| attrs::visible(&variant.attrs))
                .map(|mut variant| {
                    variant.attrs.clear();
                    variant
                        .fields
                        .iter_mut()
                        .for_each(|field| field.attrs.clear());
                    variant
                })
                .collect();
            (Kind::Enum, unparse(Item::Enum(item)))
        }
        Item::Trait(mut item) => {
            item.attrs.clear();
            item.items = item
                .items
                .into_iter()
                .filter_map(|item| trait_item(item, false))
                .collect();
            (Kind::Trait, unparse(Item::Trait(item)))
        }
        Item::Macro(item) => {
            let name = item.ident.map(|ident| ident.to_string())?;
            (Kind::Macro, format!("macro_rules! {name} {{ ... }}"))
        }
        _ => return None,
    })
}

/// Whether an attribute of a type is shown in its declaration.
fn kept(attr: &syn::Attribute) -> bool {
    attr.path().is_ident("non_exhaustive")
}

/// The signature of a function, without its body and without the `mut`
/// of its by-value parameters.
fn function(vis: Visibility, mut sig: Signature) -> String {
    for input in &mut sig.inputs {
        match input {
            FnArg::Receiver(receiver) if receiver.reference.is_none() => {
                receiver.mutability = None;
            }
            FnArg::Typed(typed) => {
                if let Pat::Ident(binding) = &mut *typed.pat {
                    binding.mutability = None;
                }
            }
            FnArg::Receiver(_) => {}
        }
    }
    let text = unparse(Item::Fn(ItemFn {
        attrs: Vec::new(),
        vis,
        sig,
        block: Box::new(Block {
            brace_token: Default::default(),
            stmts: Vec::new(),
        }),
    }));
    text.strip_suffix("{}")
        .unwrap_or(&text)
        .trim_end()
        .to_owned()
}

/// An expression printed as `...`.
fn elided() -> Option<Expr> {
    syn::parse_str(PLACEHOLDER).ok()
}

/// A constant with its value, or with `...` when the declaration is long.
fn constant(mut item: ItemConst) -> String {
    item.attrs.clear();
    let text = unparse(Item::Const(item.clone()));
    if text.len() <= MAX_VALUE_WIDTH && !text.contains('\n') {
        return text;
    }
    let Some(expr) = elided() else { return text };
    *item.expr = expr;
    unparse(Item::Const(item)).replace(PLACEHOLDER, "...")
}

/// A static with its value, or with `...` when the declaration is long.
fn statik(mut item: ItemStatic) -> String {
    item.attrs.clear();
    let text = unparse(Item::Static(item.clone()));
    if text.len() <= MAX_VALUE_WIDTH && !text.contains('\n') {
        return text;
    }
    let Some(expr) = elided() else { return text };
    *item.expr = expr;
    unparse(Item::Static(item)).replace(PLACEHOLDER, "...")
}

/// Keeps the public fields of `fields` without their attributes: private
/// named fields are removed and stood for by a placeholder field, private
/// tuple fields become `_`. Returns whether a field was private.
fn only_public_fields(fields: &mut Fields) -> bool {
    let mut private = false;
    match fields {
        Fields::Named(named) => {
            let all = std::mem::take(&mut named.named);
            for mut field in all {
                if !attrs::enabled(&field.attrs) {
                    continue;
                }
                if is_public(&field.vis) && !attrs::hidden(&field.attrs) {
                    field.attrs.clear();
                    named.named.push(field);
                } else {
                    private = true;
                }
            }
            if private
                && let Ok(placeholder) =
                    syn::parse_str::<syn::FieldsNamed>(&format!("{{ {PLACEHOLDER}: () }}"))
            {
                named.named.extend(placeholder.named);
            }
        }
        Fields::Unnamed(unnamed) => {
            for field in &mut unnamed.unnamed {
                field.attrs.clear();
                if !is_public(&field.vis) {
                    private = true;
                    field.vis = Visibility::Inherited;
                    field.ty = Type::Infer(syn::TypeInfer {
                        underscore_token: Default::default(),
                    });
                }
            }
        }
        Fields::Unit => {}
    }
    private
}

/// Replaces the placeholder field of `text` with `..`, on the same line as
/// the braces when it is the only field.
fn private_fields(text: String, private: bool) -> String {
    if !private {
        return text;
    }
    let lines: Vec<String> = text
        .lines()
        .map(|line| {
            if line.contains(PLACEHOLDER) {
                "    ..".to_owned()
            } else {
                line.to_owned()
            }
        })
        .collect();
    let text = lines.join("\n");
    match text.strip_suffix(" {\n    ..\n}") {
        Some(head) if !head.contains('{') => format!("{head} {{ .. }}"),
        _ => text,
    }
}

/// `item` without attributes and default bodies, or `None` when it is
/// not shown. With `with_defaults` the defaults of constants are kept.
fn trait_item(item: TraitItem, with_defaults: bool) -> Option<TraitItem> {
    Some(match item {
        TraitItem::Fn(mut item) if attrs::visible(&item.attrs) => {
            item.attrs.clear();
            item.default = None;
            item.semi_token = Some(Default::default());
            TraitItem::Fn(item)
        }
        TraitItem::Const(mut item) if attrs::visible(&item.attrs) => {
            item.attrs.clear();
            if !with_defaults {
                item.default = None;
            }
            TraitItem::Const(item)
        }
        TraitItem::Type(mut item) if attrs::visible(&item.attrs) => {
            item.attrs.clear();
            TraitItem::Type(item)
        }
        _ => return None,
    })
}

/// The lines between the first and the last line of `text`, unindented
/// by one level and without a trailing `,` or `;`.
fn inner(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let Some(body) = lines.get(1..lines.len().saturating_sub(1)) else {
        return text.to_owned();
    };
    body.iter()
        .map(|line| line.strip_prefix("    ").unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n")
        .trim_end_matches([',', ';'])
        .to_owned()
}

/// Prints `ty` on its own.
fn type_text(ty: &Type) -> String {
    let Ok(mut alias) = syn::parse_str::<syn::ItemType>("type T = ();") else {
        return String::new();
    };
    *alias.ty = ty.clone();
    let text = unparse(Item::Type(alias));
    text.strip_prefix("type T = ")
        .unwrap_or(&text)
        .trim_end_matches(';')
        .to_owned()
}

/// The variants, public fields or trait items of `item`.
pub fn members(item: &Item) -> Vec<Member> {
    match item {
        Item::Struct(item) => fields(&item.fields),
        Item::Union(item) => fields(&Fields::Named(item.fields.clone())),
        Item::Enum(item) => item
            .variants
            .iter()
            .filter(|variant| attrs::visible(&variant.attrs))
            .filter_map(|variant| {
                let mut variant = variant.clone();
                let summary = attrs::summary(&attrs::docs(&variant.attrs));
                variant.attrs.clear();
                variant
                    .fields
                    .iter_mut()
                    .for_each(|field| field.attrs.clear());
                let Ok(mut wrapper) = syn::parse_str::<syn::ItemEnum>("enum E {}") else {
                    return None;
                };
                let name = variant.ident.to_string();
                wrapper.variants.push(variant);
                Some(Member {
                    name,
                    kind: Kind::Variant,
                    signature: inner(&unparse(Item::Enum(wrapper))),
                    summary,
                })
            })
            .collect(),
        Item::Trait(item) => item
            .items
            .iter()
            .filter_map(|member| {
                let (name, kind, docs) = match member {
                    TraitItem::Fn(member) => {
                        (member.sig.ident.to_string(), Kind::Method, &member.attrs)
                    }
                    TraitItem::Const(member) => {
                        (member.ident.to_string(), Kind::Constant, &member.attrs)
                    }
                    TraitItem::Type(member) => {
                        (member.ident.to_string(), Kind::Type, &member.attrs)
                    }
                    _ => return None,
                };
                let summary = attrs::summary(&attrs::docs(docs));
                let member = trait_item(member.clone(), true)?;
                let mut wrapper: syn::ItemTrait = syn::parse_str("trait T {}").ok()?;
                wrapper.items.push(member);
                Some(Member {
                    name,
                    kind,
                    signature: inner(&unparse(Item::Trait(wrapper))),
                    summary,
                })
            })
            .collect(),
        _ => Vec::new(),
    }
}

/// The public fields of `fields`.
fn fields(fields: &Fields) -> Vec<Member> {
    fields
        .iter()
        .enumerate()
        .filter(|(_, field)| is_public(&field.vis) && attrs::visible(&field.attrs))
        .map(|(index, field)| {
            let name = field
                .ident
                .as_ref()
                .map_or_else(|| index.to_string(), ToString::to_string);
            Member {
                signature: format!("pub {name}: {}", type_text(&field.ty)),
                name,
                kind: Kind::Field,
                summary: attrs::summary(&attrs::docs(&field.attrs)),
            }
        })
        .collect()
}

/// The public methods and associated constants of an inherent `impl`.
pub fn impl_members(items: &[ImplItem]) -> Vec<Member> {
    items
        .iter()
        .filter_map(|member| match member {
            ImplItem::Fn(member) if is_public(&member.vis) && attrs::visible(&member.attrs) => {
                Some(Member {
                    name: member.sig.ident.to_string(),
                    kind: Kind::Method,
                    signature: function(member.vis.clone(), member.sig.clone()),
                    summary: attrs::summary(&attrs::docs(&member.attrs)),
                })
            }
            ImplItem::Const(member) if is_public(&member.vis) && attrs::visible(&member.attrs) => {
                Some(Member {
                    name: member.ident.to_string(),
                    kind: Kind::Constant,
                    signature: constant(ItemConst {
                        attrs: Vec::new(),
                        vis: member.vis.clone(),
                        const_token: member.const_token,
                        ident: member.ident.clone(),
                        generics: member.generics.clone(),
                        colon_token: member.colon_token,
                        ty: Box::new(member.ty.clone()),
                        eq_token: member.eq_token,
                        expr: Box::new(member.expr.clone()),
                        semi_token: member.semi_token,
                    }),
                    summary: attrs::summary(&attrs::docs(&member.attrs)),
                })
            }
            _ => None,
        })
        .collect()
}

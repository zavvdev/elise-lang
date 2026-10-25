#![allow(non_snake_case)]

pub mod builtins;
pub mod matchers;

use std::mem::discriminant;

use elise_aast::{
    AAstNode,
    aast_node_call::{
        AAstNodeCall,
        call_get::AAstNodeCallGet,
        call_typedef::AAstNodeCallTypedef,
        type_annot::{
            AAstNodeTypeAnnot, AAstNodeTypeAnnotCustom, AAstNodeTypeAnnotList,
            AAstNodeTypeAnnotPrim, AAstNodeTypeAnnotRecord, AAstNodeTypeAnnotRecordEntries,
        },
    },
    aast_node_data::{
        AAstNodeData, AAstNodeDataDict, AAstNodeDataDictKey, AAstNodeDataList, AAstNodeDataPrim,
    },
    aast_node_slot::AAstNodeSlot,
};
use elise_ast::{
    AstNode, AstNodeExpr, AstNodeExprCall, AstNodeExprDict, AstNodeExprList, AstNodeExprPrim,
    AstNodeTypeAnnot, AstNodeTypeAnnotGeneric,
};
use elise_binder::type_binder::TypeBinder;
use elise_bindings::{
    binding_path::{BindingPath, BindingPathSegment},
    type_bindings::{AliasedTypeBindings, TypeBinding},
};
use elise_shared_errors::{errors_semanalyzer::SemanalyzerErr, errors_type_binder::TypeBinderErr};
use elise_shared_types::ArityMismatchKind;

use crate::{
    builtins::{FnGet, FnTypedef, SlotData, TypeAnnotLexeme},
    matchers::{expect_identifier_ast_node, expect_type_annot_ast_node},
};

pub struct Harmony<'a> {
    pub ast: &'a Vec<AstNode>,

    // Local resolved type bindings. Flattened AAst type
    // definition nodes for access simplification.
    // Also returned from Harmony so it can accept it back
    // as globals from a different context. For example,
    // we receive type bindings from schema semantic
    // analysis and then pass it as globals into a source
    // code semantic analysis stage, so we can access
    // type definitions from schema file inside our source
    // code file.
    pub type_bindings: &'a mut AliasedTypeBindings,

    // Instruct semanalyzer to limit analysis to
    // type definitions only. Set to `true` for
    // the case of .elt files analysis.
    pub typedef_mode: bool,
}

impl<'a> Harmony<'a> {
    pub fn new(
        ast: &'a Vec<AstNode>,
        global_type_bindings: &'a mut AliasedTypeBindings,
        typedef_mode: bool,
    ) -> Self {
        Self {
            ast,
            type_bindings: global_type_bindings,
            typedef_mode,
        }
    }

    /// Walks through all AST list and produces a vector of AAST nodes.
    pub fn annotate(&mut self) -> Result<Vec<AAstNode>, SemanalyzerErr> {
        let mut aast: Vec<AAstNode> = vec![];

        for ast_node in self.ast {
            let aast_node = self.annotate_node(ast_node)?;
            self.verify_aast_node(&aast_node)?;
            aast.push(aast_node);
        }

        Ok(aast)
    }

    /// Annotates only one AST node.
    /// Doesn't annotate AST nodes for type annotations
    /// since they do not bring any logical value on itself
    /// and being captured separately depending on the context.
    fn annotate_node(&mut self, ast_node: &AstNode) -> Result<AAstNode, SemanalyzerErr> {
        match ast_node {
            AstNode::Expr(expr) => self.annotate_node_expr(expr),
            _ => Err(SemanalyzerErr::UnexpNode {
                span: ast_node.span(),
            }),
        }
    }

    fn verify_aast_node(&self, aast_node: &AAstNode) -> Result<(), SemanalyzerErr> {
        match aast_node {
            AAstNode::Call(AAstNodeCall::Typedef { .. }) => Ok(()),
            node => {
                if self.typedef_mode {
                    return Err(SemanalyzerErr::TypedefMode { span: node.span() });
                }
                Ok(())
            }
        }
    }

    fn annotate_node_expr(&mut self, ast_node: &AstNodeExpr) -> Result<AAstNode, SemanalyzerErr> {
        match ast_node {
            AstNodeExpr::Call(call) => self.call(call),
            AstNodeExpr::Slot(prim) => self.slot(prim),
            AstNodeExpr::Int(prim) => self.prim(prim, AAstNodeData::Int),
            AstNodeExpr::Str(prim) => self.prim(prim, AAstNodeData::Str),
            AstNodeExpr::List(list) => self.list(list),
            AstNodeExpr::Dict(dict) => self.dict(dict),
            _ => Err(SemanalyzerErr::UnexpNode {
                span: ast_node.span(),
            }),
        }
    }

    // ==================================================================
    // TYPE INFERENCE START
    // ==================================================================

    /// Takes AAstNode, filters out everything we cannot use for
    /// type inference (any entity that does not resolve into a data)
    /// and returns back TypeBinding.
    fn infer_type_binding(&self, aast_node: &AAstNode) -> Result<TypeBinding, SemanalyzerErr> {
        match aast_node {
            AAstNode::SlotData(AAstNodeSlot { span }) => {
                let Some(slot_bindings) = self
                    .type_bindings
                    .get(&SlotData::TYPEDEF_LEXEME.to_string())
                else {
                    return Err(SemanalyzerErr::NoTypeDefinition { span: *span });
                };
                Ok(slot_bindings.clone())
            }
            AAstNode::Call(AAstNodeCall::Get(AAstNodeCallGet {
                return_type_binding,
                ..
            })) => Ok(return_type_binding.clone()),
            unsupported_node => Err(SemanalyzerErr::CannotInferType {
                span: unsupported_node.span(),
            }),
        }
    }

    // ==================================================================
    // TYPE INFERENCE END
    // ==================================================================

    // ==================================================================
    // SLOT START
    // ==================================================================

    fn slot(&mut self, prim: &AstNodeExprPrim) -> Result<AAstNode, SemanalyzerErr> {
        match prim.lexeme.as_str() {
            SlotData::LEXEME => Ok(AAstNode::SlotData(AAstNodeSlot { span: prim.span() })),
            _ => Err(SemanalyzerErr::UnknownSlot { span: prim.span() }),
        }
    }

    // ==================================================================
    // SLOT END
    // ==================================================================

    // ==================================================================
    // PRIM START
    //
    // Primitive Ast nodes map 1:1 to AAst primitive nodes
    // so we one a single function for them.
    // ==================================================================

    fn prim(
        &mut self,
        prim: &AstNodeExprPrim,
        variant: fn(AAstNodeDataPrim) -> AAstNodeData,
    ) -> Result<AAstNode, SemanalyzerErr> {
        Ok(AAstNode::Data(variant(AAstNodeDataPrim {
            span: prim.span(),
            lexeme: prim.lexeme(),
        })))
    }

    // ==================================================================
    // PRIM END
    // ==================================================================

    // ==================================================================
    // LIST START
    // ==================================================================

    fn list(&mut self, list: &AstNodeExprList) -> Result<AAstNode, SemanalyzerErr> {
        // Each list item must be of the same type.
        // This validation guarantees only a shallow type comparison,
        // so technically, this code won't fail: [[1, 2], ["3", "4"]]
        // which is fine, because on the type validation level this code
        // will be rejected since we have no possibility to express
        // list item as a union type.
        let mut item_type: Option<&Box<AstNodeExpr>> = None;
        for item in list.items.iter() {
            if let Some(ty) = item_type
                && discriminant(&**item) != discriminant(ty)
            {
                return Err(SemanalyzerErr::ListInconsistentItemTypes { span: item.span() });
            } else {
                item_type = Some(item);
            }
        }

        // We allow only data nodes inside list items for now in order
        // to avoid complexity related to expression evaluation at runtime.
        // Grammatically, this code can be accepted:
        // [.add(1, 2)] because .add call is an expression that produced a
        // value, but this will require us to emit an additional type of
        // AAst nodes that must instruct the Compiler or VM to perform
        // evaluation of this expression first in order to derive its value
        // into the list. This will be covered in the future.
        let mut aast_items = vec![];
        for item in list.items.iter() {
            let AAstNode::Data(data_node) = self.annotate_node_expr(item)? else {
                return Err(SemanalyzerErr::UnsupportedListItem { span: item.span() });
            };
            aast_items.push(Box::new(data_node));
        }

        Ok(AAstNode::Data(AAstNodeData::List(AAstNodeDataList {
            span: list.span(),
            items: aast_items,
        })))
    }

    // ==================================================================
    // LIST END
    // ==================================================================

    // ==================================================================
    // DICT START
    // ==================================================================

    fn dict(&mut self, dict: &AstNodeExprDict) -> Result<AAstNode, SemanalyzerErr> {
        let mut entries = vec![];

        for entry in dict.entries.iter() {
            let AAstNode::Data(data_node) = self.annotate_node_expr(&entry.1)? else {
                return Err(SemanalyzerErr::UnsupportedDictItem {
                    span: entry.1.span(),
                });
            };
            entries.push((
                AAstNodeDataDictKey {
                    span: entry.0.span(),
                    lexeme: entry.0.lexeme(),
                },
                Box::new(data_node),
            ));
        }

        Ok(AAstNode::Data(AAstNodeData::Dict(AAstNodeDataDict {
            span: dict.span(),
            entries,
        })))
    }

    // ==================================================================
    // DICT END
    // ==================================================================

    // ==================================================================
    // CALL START
    // ==================================================================

    fn call(&mut self, call: &AstNodeExprCall) -> Result<AAstNode, SemanalyzerErr> {
        match call.lexeme.as_str() {
            FnTypedef::LEXEME => self.call_typedef(call),
            FnGet::LEXEME => self.call_get(call),
            _ => Err(SemanalyzerErr::UnknownFunction { span: call.span() }),
        }
    }

    // ==================================================================
    // CALL END
    // ==================================================================

    // ==================================================================
    // TYPEDEF CALL START
    // ==================================================================

    fn call_typedef(&mut self, call: &AstNodeExprCall) -> Result<AAstNode, SemanalyzerErr> {
        if call.body.len() != FnTypedef::ARGS_LEN {
            return Err(SemanalyzerErr::ArityMismatch {
                fn_name: FnTypedef::LEXEME,
                found: call.body.len(),
                span: call.span(),
                kind: ArityMismatchKind::Eq(FnTypedef::ARGS_LEN),
            });
        }

        let first_arg = &**call.body.first().unwrap();
        let second_arg = &**call.body.last().unwrap();

        let identifier = expect_identifier_ast_node(first_arg)?;

        // Ensure that the second argument is a type definition
        // and extract it.
        let ast_type_annot = expect_type_annot_ast_node(second_arg)?;

        // Analyze the whole type definition.
        let aast_type_annot = self.call_typedef__type_annot(ast_type_annot)?;

        // Save newly resolved type into a global scope.
        self.call_typedef__type_annot_capture(&identifier.lexeme, &aast_type_annot)?;

        Ok(AAstNode::Call(AAstNodeCall::Typedef(AAstNodeCallTypedef {
            alias: identifier.lexeme(),
            annot: aast_type_annot,
            span: call.span(),
        })))
    }

    fn call_typedef__type_annot(
        &mut self,
        annot: &AstNodeTypeAnnot,
    ) -> Result<AAstNodeTypeAnnot, SemanalyzerErr> {
        match annot.lexeme.as_str() {
            TypeAnnotLexeme::INT => Self::call_typedef__type_annot_int(annot),
            TypeAnnotLexeme::LIST => self.call_typedef__type_annot_list(annot),
            TypeAnnotLexeme::RECORD => self.call_typedef__type_annot_record(annot),
            _ => self.call_typedef__type_annot_custom(annot),
        }
    }

    fn call_typedef__type_annot_capture(
        &mut self,
        alias: &str,
        aast_typedef: &AAstNodeTypeAnnot,
    ) -> Result<(), SemanalyzerErr> {
        match TypeBinder::new(aast_typedef, self.type_bindings).bind() {
            Ok(bindings) => self.type_bindings.insert(alias.to_string(), bindings),
            Err(bind_err) => match bind_err {
                TypeBinderErr::Unresolvable { span } => {
                    return Err(SemanalyzerErr::UnknownTypedef { span });
                }
                _ => {
                    return Err(SemanalyzerErr::UnresolvableTypedef {
                        span: aast_typedef.span(),
                    });
                }
            },
        };
        Ok(())
    }

    fn call_typedef__type_annot_int(
        annot: &AstNodeTypeAnnot,
    ) -> Result<AAstNodeTypeAnnot, SemanalyzerErr> {
        if annot.generic.is_some() {
            return Err(SemanalyzerErr::UnexpectedGeneric { span: annot.span() });
        }
        Ok(AAstNodeTypeAnnot::Int(AAstNodeTypeAnnotPrim {
            span: annot.span(),
        }))
    }

    fn call_typedef__type_annot_list(
        &mut self,
        annot: &AstNodeTypeAnnot,
    ) -> Result<AAstNodeTypeAnnot, SemanalyzerErr> {
        let Some(generic) = annot.generic.as_ref() else {
            return Err(SemanalyzerErr::ExpectedGeneric { span: annot.span() });
        };

        let AstNodeTypeAnnotGeneric::Single(single_generic) = generic else {
            return Err(SemanalyzerErr::InvalidGeneric { span: annot.span() });
        };

        Ok(AAstNodeTypeAnnot::List(AAstNodeTypeAnnotList {
            span: annot.span(),
            item_type: Box::new(self.call_typedef__type_annot(single_generic)?),
        }))
    }

    fn call_typedef__type_annot_record(
        &mut self,
        annot: &AstNodeTypeAnnot,
    ) -> Result<AAstNodeTypeAnnot, SemanalyzerErr> {
        let Some(generic) = annot.generic.as_ref() else {
            return Err(SemanalyzerErr::ExpectedGeneric { span: annot.span() });
        };

        let AstNodeTypeAnnotGeneric::Record(ast_entries) = generic else {
            return Err(SemanalyzerErr::InvalidGeneric { span: annot.span() });
        };

        let entries = ast_entries
            .iter()
            .map(|(key, annot)| {
                Ok((
                    key.lexeme(),
                    Box::new(self.call_typedef__type_annot(annot)?),
                ))
            })
            .collect::<Result<AAstNodeTypeAnnotRecordEntries, SemanalyzerErr>>()?;

        Ok(AAstNodeTypeAnnot::Record(AAstNodeTypeAnnotRecord {
            span: annot.span(),
            entries,
        }))
    }

    fn call_typedef__type_annot_custom(
        &self,
        annot: &AstNodeTypeAnnot,
    ) -> Result<AAstNodeTypeAnnot, SemanalyzerErr> {
        if annot.generic.is_some() {
            return Err(SemanalyzerErr::UnexpectedGeneric { span: annot.span() });
        }

        // If we encounter a type definition with an unknown
        // type alias, return an error. It covers the case
        // with referencing itself, because at this point
        // we don't have a record of a type definition
        // being resolved, so resolving alias to itself will
        // fail here.
        if !self.type_bindings.contains_key(&annot.lexeme) {
            return Err(SemanalyzerErr::UnknownTypedef { span: annot.span() });
        }

        Ok(AAstNodeTypeAnnot::Custom(AAstNodeTypeAnnotCustom {
            alias: annot.lexeme(),
            span: annot.span(),
        }))
    }

    // ==================================================================
    // TYPEDEF CALL END
    // ==================================================================

    // ==================================================================
    // GET CALL START
    // ==================================================================

    fn call_get(&mut self, call: &AstNodeExprCall) -> Result<AAstNode, SemanalyzerErr> {
        if call.body.len() < FnGet::ARGS_LEN_MIN {
            return Err(SemanalyzerErr::ArityMismatch {
                fn_name: FnGet::LEXEME,
                found: call.body.len(),
                span: call.span(),
                kind: ArityMismatchKind::MoreEq(FnGet::ARGS_LEN_MIN),
            });
        }

        let first_arg = self.annotate_node(call.body.first().unwrap())?;
        let type_binding = self.infer_type_binding(&first_arg)?;

        // Skip the first element because it's a data being accessed.
        let access_args = call.body.iter().skip(1).map(|n| &**n).collect();
        let type_access_path = Self::call_get__build_binding_path(&access_args, true)?;

        if !type_binding.map.contains_key(&type_access_path) {
            return Err(SemanalyzerErr::UnresolvablePath { span: call.span() });
        };

        let data_access_path = Self::call_get__build_binding_path(&access_args, false)?;

        Ok(AAstNode::Call(AAstNodeCall::Get(AAstNodeCallGet {
            span: call.span(),
            type_binding_path: type_access_path.clone(),
            data_binding_path: data_access_path.clone(),
            return_type_binding: type_binding.strip_binding_paths_by_prefix(&type_access_path),
        })))
    }

    fn call_get__build_binding_path(
        ast_nodes: &Vec<&AstNode>,
        // Whether to resolve ast_nodes as type access
        // with AbstractIndex that can be used for accessing type
        // definition from type binding maps, or resolve integers as
        // Index segments for accessing data itself.
        // We need both because compiler needs data access path
        // but here in semanalyzer we also need to have type access
        // path for type resolutions.
        type_access_only: bool,
    ) -> Result<BindingPath, SemanalyzerErr> {
        let mut binding_path = BindingPath::new();

        for ast_node in ast_nodes.iter() {
            let AstNode::Expr(expr) = ast_node else {
                return Err(SemanalyzerErr::InvalDataAccessArg {
                    span: ast_node.span(),
                });
            };

            match expr {
                AstNodeExpr::Int(prim) => {
                    if type_access_only {
                        binding_path.push(BindingPathSegment::AbstractIndex);
                    } else {
                        binding_path.push(BindingPathSegment::Index(
                            // Unwrap is safe here since our AST node guarantees
                            // that integer is parsed correctly.
                            prim.lexeme.parse::<usize>().unwrap(),
                        ));
                    }
                }
                AstNodeExpr::Str(prim) => {
                    binding_path.push(BindingPathSegment::Field(prim.lexeme()));
                }
                _ => {
                    return Err(SemanalyzerErr::InvalDataAccessArg {
                        span: ast_node.span(),
                    });
                }
            }
        }

        Ok(binding_path)
    }

    // ==================================================================
    // GET CALL END
    // ==================================================================
}

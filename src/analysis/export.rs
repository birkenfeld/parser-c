// Original file: "Export.hs"
// File auto-generated using Corollary.

#[macro_use]
use corollary_support::*;

// NOTE: These imports are advisory. You probably need to change them to support Rust.
// use Language::C::Data::Ident;
// use Language::C::Data::Name;
// use nameId;
// use Language::C::Data::Node;
// use Language::C::Syntax::AST;
// use Language::C::Analysis::SemRep;
// use Data::Maybe;

use analysis::sem_rep::*;
use syntax::ast::*;
use data::ident::*;

pub fn exportDeclr(other_specs: Vec<CDeclSpec>,
                   ty: Type,
                   attrs: Attributes,
                   name: VarName)
                   -> (Vec<CDeclSpec>, CDeclr) {

    let (specs, derived) = exportType(ty);
    let (ident, asmname) = match name {
        VarName(vident, asmname_opt) => (Some(vident), asmname_opt),
        _ => (None, None),
    };

    (__op_addadd(other_specs, specs),
     CDeclarator(ident, derived, asmname, (exportAttrs(attrs)), ni))
}

pub fn exportTypeDecl(ty: Type) -> CDecl {
    let (declspecs, derived) = exportType(ty);
    let declrs = if null(derived) {
        vec![]
    } else {
        [Some(CDeclr(None, derived, None, vec![], ni, None, None))]
    };

    CDecl(declspecs, declrs, ni)
}

pub fn exportTypeDef(TypeDef(ident, ty, attrs, node_info): TypeDef) -> CDecl {

    let declr =
        (Some(CDeclarator((Some(ident)), derived, None, (exportAttrs(attrs)), ni)), None, None);

    CDecl((__op_concat(CStorageSpec((CTypedef(ni))), declspecs)),
          vec![declr],
          node_info)
}

pub fn exportType(ty: Type) -> (Vec<CDeclSpec>, Vec<CDerivedDeclr>) {

    fn exportTy (_0: Vec<CDeclSpec>, _1: Type) {
        match (_0, _1) {
            (dd, PtrType(ity, tyquals, attrs)) => {
                let ptr_declr = CPtrDeclr((exportTypeQualsAttrs(tyquals, attrs)), ni);

                exportTy((__op_concat(ptr_declr, dd)), ity)
            }
            (dd, ArrayType(ity, array_sz, tyquals, attrs)) => {
                let arr_declr = CArrDeclr((exportTypeQualsAttrs(tyquals, attrs)),
                                        (exportArraySize(array_sz)),
                                        ni);

                exportTy((__op_concat(arr_declr, dd)), ity)
            }
            (dd, FunctionType(FunType(ity, params, variadic), attrs)) => {
                let fun_declr = CFunDeclr((Right((__map!(exportParamDecl, params), variadic))),
                                        (exportAttrs(attrs)),
                                        ni);

                exportTy((__op_concat(fun_declr, dd)), ity)
            }
            (dd, FunctionType(FunTypeIncomplete(ity), attrs)) => {
                let fun_declr = CFunDeclr((Right((vec![], false))), (exportAttrs(attrs)), ni);

                exportTy((__op_concat(fun_declr, dd)), ity)
            }
            (dd, TypeDefType(TypeDefRef(ty_ident, _, node), quals, attrs)) => {
                let declspecs = __op_concat(CTypeSpec((CTypeDef(ty_ident, node))),
                                            __map!(CTypeQual, (exportTypeQualsAttrs(quals, attrs))));

                (declspecs, reverse(dd))
            }
            (dd, DirectType(ity, quals, attrs)) => {
                let declspecs = __op_addadd(__map!(CTypeQual, (exportTypeQualsAttrs(quals, attrs))),
                                            __map!(CTypeSpec, (exportTypeSpec(ity))));

                (declspecs, reverse(dd))
            }
        }
    }

    exportTy(vec![], ty)
}

pub fn exportTypeQuals(quals: TypeQuals) -> Vec<CTypeQual> {

    mapMaybe(select,
             vec![(constant, CConstQual(ni)),
                  (volatile, CVolatQual(ni)),
                  (restrict, CRestrQual(ni))])
}

pub fn exportTypeQualsAttrs(tyqs: TypeQuals, attrs: Attributes) -> Vec<CTypeQual> {
    (__op_addadd(exportTypeQuals(tyqs),
                 __map!(CAttrQual, (exportAttrs(attrs)))))
}

pub fn exportArraySize(_0: ArraySize) -> CArrSize {
    match (_0) {
        ArraySize(__static, e) => CArrSize(__static, e),
        UnknownArraySize(complete) => CNoArrSize(complete),
    }
}

pub fn exportTypeSpec(tyname: TypeName) -> Vec<CTypeSpec> {
    match tyname {
        TyVoid => vec![CVoidType(ni)],
        TyIntegral(ity) => exportIntType(ity),
        TyFloating(fty) => exportFloatType(fty),
        TyComplex(fty) => exportComplexType(fty),
        TyComp(comp) => exportCompTypeDecl(comp),
        TyEnum(__enum) => exportEnumTypeDecl(__enum),
        TyBuiltin(TyVaList) => vec![CTypeDef((internalIdent("va_list".to_string())), ni)],
        TyBuiltin(TyAny) => vec![CTypeDef((internalIdent("__ty_any".to_string())), ni)],
    }
}

pub fn exportIntType(ty: IntType) -> Vec<CTypeSpec> {
    match ty {
        TyBool => vec![CBoolType(ni)],
        TyChar => vec![CCharType(ni)],
        TySChar => vec![CSignedType(ni), CCharType(ni)],
        TyUChar => vec![CUnsigType(ni), CCharType(ni)],
        TyShort => vec![CShortType(ni)],
        TyUShort => vec![CUnsigType(ni), CShortType(ni)],
        TyInt => vec![CIntType(ni)],
        TyUInt => vec![CUnsigType(ni), CIntType(ni)],
        TyInt128 => vec![CInt128Type(ni)],
        TyUInt128 => vec![CUnsigType(ni), CInt128Type(ni)],
        TyLong => vec![CLongType(ni)],
        TyULong => vec![CUnsigType(ni), CLongType(ni)],
        TyLLong => vec![CLongType(ni), CLongType(ni)],
        TyULLong => vec![CUnsigType(ni), CLongType(ni), CLongType(ni)],
    }
}

pub fn exportFloatType(ty: FloatType) -> Vec<CTypeSpec> {
    match ty {
        TyFloat => vec![CFloatType(ni)],
        TyDouble => vec![CDoubleType(ni)],
        TyLDouble => vec![CLongType(ni), CDoubleType(ni)],
    }
}

pub fn exportComplexType(ty: FloatType) -> Vec<CTypeSpec> {
    __op_concat((CComplexType(ni)), exportFloatType(ty))
}

pub fn exportCompTypeDecl(ty: CompTypeRef) -> Vec<CTypeSpec> {

    let exportComp = |CompTypeRef(sue_ref, comp_tag, _n)| {
        CStruct(((if comp_tag == StructTag {
                      CStructTag
                  } else {
                      CUnionTag
                  })),
                (exportSUERef(sue_ref)),
                None,
                vec![],
                ni)
    };

    vec![CSUType((exportComp(ty)), ni)]
}

pub fn exportEnumTypeDecl(ty: EnumTypeRef) -> Vec<CTypeSpec> {

    let exportEnum = |EnumTypeRef(sue_ref, _n)| CEnum((exportSUERef(sue_ref)), None, vec![], ni);

    vec![CEnumType((exportEnum(ty)), ni)]
}

pub fn exportCompType(CompType(sue_ref, comp_tag, members, attrs, node_info): CompType)
                      -> Vec<CTypeSpec> {

    let comp = CStruct(((if comp_tag = StructTag {
                             CStructTag
                         } else {
                             CUnionTag
                         })),
                       (exportSUERef(sue_ref)),
                       (Some((__map!(exportMemberDecl, members)))),
                       (exportAttrs(attrs)),
                       node_info);

    vec![CSUType(comp, ni)]
}

pub fn exportCompTypeRef(CompType(sue_ref, com_tag, _, _, node_info): CompType) -> Vec<CTypeSpec> {
    exportCompTypeDecl((CompTypeRef(sue_ref, com_tag, node_info)))
}

pub fn exportEnumType(EnumType(sue_ref, enumerators, attrs, node_info): EnumType)
                      -> Vec<CTypeSpec> {

    let __enum = CEnum((exportSUERef(sue_ref)),
                       (Some((__map!(exportEnumerator, enumerators)))),
                       (exportAttrs(attrs)),
                       node_info);

    let exportEnumerator = |Enumerator(ident, val, _ty, _)| (ident, Some(val));

    vec![CEnumType(__enum, ni)]
}

pub fn exportEnumTypeRef(EnumType(sue_ref, _, _, node_info): EnumType) -> Vec<CTypeSpec> {
    exportEnumTypeDecl((EnumTypeRef(sue_ref, node_info)))
}

pub fn exportSUERef(_0: SUERef) -> Option<Ident> {
    match (_0) {
        AnonymousRef(name) => {
            Some((internalIdent(__op_addadd("$".to_string(), show((nameId(name)))))))
        }
        NamedRef(ident) => Some(ident),
    }
}

pub fn exportMemberDecl(_0: MemberDecl) -> CDecl {
    match (_0) {
        AnonBitField(ty, expr, node_info) => {
            CDecl((__map!(CTypeSpec, exportTypeSpec(fromDirectType(ty)))),
                  vec![(None, None, Some(expr))],
                  node_info)
        }
        MemberDecl(vardecl, bitfieldsz, node_info) => {
            let (specs, declarator) = exportVarDecl(vardecl);

            CDecl(specs, vec![(Some(declarator), None, bitfieldsz)], node_info)
        }
    }
}

pub fn exportVarDecl(VarDecl(name, attrs, ty): VarDecl) -> (Vec<CDeclSpec>, CDeclr) {
    exportDeclr((exportDeclAttrs(attrs)), ty, vec![], name)
}

pub fn exportParamDecl(paramdecl: ParamDecl) -> CDecl {
    {
        let (specs, declr) = exportVarDecl((getVarDecl(paramdecl)));

        CDecl(specs,
              vec![(Some(declr), None, None)],
              (nodeInfo(paramdecl)))
    }
}

pub fn exportDeclAttrs(DeclAttrs(fun_attrs, storage, attrs): DeclAttrs) -> Vec<CDeclSpec> {
    __op_addadd(__map!(CFunSpec, (exportFunAttrs(fun_attrs))),
                __op_addadd(__map!(CStorageSpec, (exportStorage(storage))),
                            __map!((CTypeQual(CAttrQual)), (exportAttrs(attrs)))))
}

pub fn exportFunAttrs(fattrs: FunctionAttrs) -> Vec<CFunSpec> {

    let inlQual = if isInline(fattrs) {
        Some((CInlineQual(ni)))
    } else {
        None
    };

    let noretQual = if isNoreturn(fattrs) {
        Some((CNoreturnQual(ni)))
    } else {
        None
    };

    catMaybes(vec![inlQual, noretQual])
}

pub fn exportStorage(_0: Storage) -> Vec<CStorageSpec> {
    match (_0) {
        NoStorage => vec![],
        Auto(reg) => if reg { vec![CRegister(ni)] } else { vec![] },
        Static(InternalLinkage, thread_local) => threadLocal(thread_local, vec![CStatic(ni)]),
        Static(ExternalLinkage, thread_local) => threadLocal(thread_local, vec![CExtern(ni)]),
        Static(NoLinkage, _) => panic!("impossible storage: static without linkage"),
        FunLinkage(InternalLinkage) => vec![CStatic(ni)],
        FunLinkage(ExternalLinkage) => vec![],
        FunLinkage(NoLinkage) => {
            panic!("impossible storage: function without linkage")
        }
    }
}

pub fn threadLocal(_0: bool) -> Vec<CStorageSpec> {
    match (_0) {
        false => id,
        true => ((CThread(ni))(__op_concat)),
    }
}

pub fn exportAttrs(input: Vec<Attr>) -> Vec<CAttr> {

    let exportAttr = |Attr(ident, es, n)| CAttr(ident, es, n);

    __map!(exportAttr, input)
}

pub fn fromDirectType(_0: Type) -> TypeName {
    match (_0) {
        DirectType(ty, _, _) => ty,
        TypeDefType(TypeDefRef(_, ty, _), _, _) => fromDirectType(ty),
        _ => panic!("fromDirectType"),
    }
}

pub fn ni() -> NodeInfo {
    undefNode
}

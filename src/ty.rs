use crate::*;

pub fn translate_ty(ty: &mir::Ty) -> mini::Type {
    match ty.kind() {
        mir::TyKind::Bool => mini::Type::Bool,
        mir::TyKind::Int(int_ty) => mini::Type::Int(translate_int_ty(int_ty)),
        mir::TyKind::Uint(uint_ty) => mini::Type::Int(translate_uint_ty(uint_ty)),
        _ => todo!(),
    }
}

fn translate_int_ty(int_ty: &mir::IntTy) -> mini::IntType {
    use mir::IntTy::*;

    let size = match int_ty {
        Isize => 8, // TODO this is not generally 8.
        I8 => 1,
        I16 => 2,
        I32 => 4,
        I64 => 8,
        I128 => 16,
    };

    let signed = mini::Signedness::Signed;
    let size = mini::Size::from_bytes(size);
    mini::IntType { signed, size }
}

fn translate_uint_ty(uint_ty: &mir::UintTy) -> mini::IntType {
    use mir::UintTy::*;

    let size = match uint_ty {
        Usize => 8, // TODO this is not generally 8.
        U8 => 1,
        U16 => 2,
        U32 => 4,
        U64 => 8,
        U128 => 16,
    };

    let signed = mini::Signedness::Unsigned;
    let size = mini::Size::from_bytes(size);
    mini::IntType { signed, size }
}

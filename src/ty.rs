use crate::*;

pub fn translate_ty<'tcx>(ty: &mir::Ty<'tcx>, tcx: mir::TyCtxt<'tcx>) -> mini::Type {
    match ty.kind() {
        mir::TyKind::Bool => mini::Type::Bool,
        mir::TyKind::Int(int_ty) => mini::Type::Int(translate_int_ty(int_ty)),
        mir::TyKind::Uint(uint_ty) => mini::Type::Int(translate_uint_ty(uint_ty)),
        mir::TyKind::Tuple(ts) => {
            // TODO the ParamEnv might need to be an argument to `translate_ty` in the future.
            let a = mir::ParamEnv::empty().and(*ty);
            let layout = tcx.layout_of(a).unwrap().layout;
            let size = translate_size(layout.size());

            let fields = ts.iter()
                           .enumerate()
                           .map(|(i, t)| {
                                let t = translate_ty(&t, tcx);
                                let offset = layout.fields().offset(i);
                                let offset = translate_size(offset);

                                (offset, t)
                           }).collect();

            mini::Type::Tuple {
                fields,
                size,
            }
        },
        _ => todo!(),
    }
}

fn translate_int_ty(int_ty: &mir::IntTy) -> mini::IntType {
    use mir::IntTy::*;

    let size = match int_ty {
        // TODO look at TargetDataLayout for Isize.
        Isize => 8,
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

fn translate_size(size: mir::Size) -> mini::Size {
    mini::Size::from_bytes(size.bytes())
}

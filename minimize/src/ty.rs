use crate::*;

pub fn translate_ty<'tcx>(ty: &rs::Ty<'tcx>, tcx: rs::TyCtxt<'tcx>) -> mini::Type {
    match ty.kind() {
        rs::TyKind::Bool => mini::Type::Bool,
        rs::TyKind::Int(int_ty) => mini::Type::Int(translate_int_ty(int_ty)),
        rs::TyKind::Uint(uint_ty) => mini::Type::Int(translate_uint_ty(uint_ty)),
        rs::TyKind::Tuple(ts) => {
            // TODO the ParamEnv might need to be an argument to `translate_ty` in the future.
            let a = rs::ParamEnv::empty().and(*ty);
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
        x => {
            dbg!(x);
            todo!()
        }
    }
}

fn translate_int_ty(int_ty: &rs::IntTy) -> mini::IntType {
    use rs::IntTy::*;

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
    let size = specr::Size::from_bytes(size);
    mini::IntType { signed, size }
}

fn translate_uint_ty(uint_ty: &rs::UintTy) -> mini::IntType {
    use rs::UintTy::*;

    let size = match uint_ty {
        Usize => 8, // TODO this is not generally 8.
        U8 => 1,
        U16 => 2,
        U32 => 4,
        U64 => 8,
        U128 => 16,
    };

    let signed = mini::Signedness::Unsigned;
    let size = specr::Size::from_bytes(size);
    mini::IntType { signed, size }
}

fn translate_size(size: rs::Size) -> specr::Size {
    specr::Size::from_bytes(size.bytes())
}

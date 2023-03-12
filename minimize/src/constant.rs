use crate::*;

pub fn translate_const<'cx, 'tcx>(c: &rs::Constant<'tcx>, fcx: &mut FnCtxt<'cx, 'tcx>) -> ValueExpr {
    let kind = c.literal.eval(fcx.cx.tcx, rs::ParamEnv::empty());
    let rs::ConstantKind::Val(val, rs_ty) = kind else { panic!("unsupported ConstantKind!") };

    let ty = translate_ty(rs_ty, fcx.cx.tcx);

    let constant = match ty {
        Type::Int(int_ty) => {
            let val = val.try_to_scalar_int().unwrap();
            let int: Int = match int_ty.signed {
                Signed => val.try_to_int(val.size()).unwrap().into(),
                Unsigned => val.try_to_uint(val.size()).unwrap().into(),
            };
            Constant::Int(int)
        },
        Type::Bool => {
            Constant::Bool(val.try_to_bool().unwrap())
        }
        // A `static`
        Type::Ptr(_) => {
            let (alloc_id, offset) = val.try_to_scalar()
                         .unwrap()
                         .to_pointer(&fcx.cx.tcx)
                         .unwrap()
                         .into_parts();
            let alloc_id = alloc_id.expect("no alloc id?");
            let rel = translate_relocation(alloc_id, offset, fcx);
            Constant::GlobalPointer(rel)
        },
        // fallback: every type not yet covered will create a global instead.
        _ => return translate_const_as_global(val, rs_ty, fcx),
    };
    ValueExpr::Constant(constant, ty)
}

// allocates a global for this ConstValue, and returns the ValueExpr of the constant that references it.
fn translate_const_as_global<'cx, 'tcx>(val: rs::ConstValue<'tcx>, ty: rs::Ty<'tcx>, fcx: &mut FnCtxt<'cx, 'tcx>) -> ValueExpr {
    let pty = place_type_of(ty, fcx);

    let rel = match val {
        rs::ConstValue::ByRef { alloc, offset } => {
            let name = translate_const_allocation(alloc, fcx);
            let offset = translate_size(offset);

            Relocation { name, offset }
        },
        rs::ConstValue::ZeroSized => {
            let global = Global {
                bytes: List::new(),
                relocations: List::new(),
                align: Align::ONE,
            };
            let name = alloc_global(global, fcx);
            let offset = Size::ZERO;

            Relocation { name, offset }
        },
        _ => panic!("unsupported ConstValue!"),
    };

    let expr = Constant::GlobalPointer(rel);

    let ptr_ty = Type::Ptr(PtrType::Raw { pointee: pty.layout::<BasicMemory>() });

    let expr = ValueExpr::Constant(expr, ptr_ty);
    let expr = PlaceExpr::Deref {
        operand: GcCow::new(expr),
        ptype: pty,
    };
    ValueExpr::Load {
        source: GcCow::new(expr),
        destructive: false
    }
}

fn translate_relocation<'cx, 'tcx>(alloc_id: rs::AllocId, offset: rs::Size, fcx: &mut FnCtxt<'cx, 'tcx>) -> Relocation {
    let name = translate_alloc_id(alloc_id, fcx);
    let offset = translate_size(offset);
    Relocation { name, offset }
}

// calls `translate_const_allocation` with the allocation of alloc_id,
// and adds the alloc_id and it's newly-created global to alloc_map.
fn translate_alloc_id<'cx, 'tcx>(alloc_id: rs::AllocId, fcx: &mut FnCtxt<'cx, 'tcx>) -> GlobalName {
    if let Some(x) = fcx.cx.alloc_map.get(&alloc_id) {
        return *x;
    }

    let alloc = match fcx.cx.tcx.global_alloc(alloc_id) {
        rs::GlobalAlloc::Memory(alloc) => alloc,
        rs::GlobalAlloc::Static(def_id) => fcx.cx.tcx.eval_static_initializer(def_id).unwrap(),
        _ => panic!("unsupported!"),
    };
    let name = translate_const_allocation(alloc, fcx);
    fcx.cx.alloc_map.insert(alloc_id, name);
    name
}

// adds a Global representing this ConstAllocation, and returns the corresponding GlobalName.
fn translate_const_allocation<'cx, 'tcx>(allocation: rs::ConstAllocation<'tcx>, fcx: &mut FnCtxt<'cx, 'tcx>) -> GlobalName {
    let allocation = allocation.inner();
    let size = allocation.size();
    let alloc_range = rs::AllocRange { start: rs::Size::ZERO, size };
    let mut bytes: Vec<Option<u8>> = allocation.get_bytes_unchecked(alloc_range).iter().copied().map(Some).collect();
    for (i, b) in bytes.iter_mut().enumerate() {
        if !allocation.init_mask().get(rs::Size::from_bytes(i)) {
            *b = None;
        }
    }
    let relocations = allocation.provenance().ptrs().iter()
        .map(|&(offset, alloc_id)| {
            // "Note that the bytes of a pointer represent the offset of the pointer.", see https://doc.rust-lang.org/nightly/nightly-rustc/rustc_middle/mir/interpret/struct.Allocation.html
            // Hence we have to decode them.
            let inner_offset_bytes: &[Option<u8>] = &bytes[offset.bytes() as usize..][..BasicMemory::PTR_SIZE.bytes().try_to_usize().unwrap()];
            let inner_offset_bytes: List<u8> = inner_offset_bytes.iter().map(|x| x.unwrap()).collect();
            let inner_offset: Int = BasicMemory::ENDIANNESS.decode(Unsigned, inner_offset_bytes);
            let inner_offset = rs::Size::from_bytes(inner_offset.try_to_usize().unwrap());
            let relo = translate_relocation(alloc_id, inner_offset, fcx);

            let offset = translate_size(offset);
            (offset, relo)
        }).collect();
    let align = translate_align(allocation.align);
    let global = Global {
        bytes: bytes.into_iter().collect(),
        relocations,
        align,
    };

    alloc_global(global, fcx)
}

fn alloc_global<'cx, 'tcx>(global: Global, fcx: &mut FnCtxt<'cx, 'tcx>) -> GlobalName {
    // choose a fresh name
    let name = GlobalName(Name::new(fcx.cx.globals.iter().count() as _)); // TODO use .len() here, if supported

    fcx.cx.globals.insert(name, global);

    name
}

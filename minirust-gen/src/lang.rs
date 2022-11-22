use crate::prelude::*;
use crate::specr;
use crate::specr::prelude::*;
use crate::{lang, mem, prelude};
#[doc = " Some opaque type of function names."]
#[doc = " The details of this this is represented to not matter."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FnName(pub specr::Name);
impl specr::hidden::GcCompat for FnName {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        self.0.points_to(s);
    }
}
#[doc = " A closed MiniRust program."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Program {
    #[doc = " Associate a function with each declared function name."]
    pub functions: Map<FnName, Function>,
    #[doc = " The function where execution starts."]
    pub start: FnName,
}
impl specr::hidden::GcCompat for Program {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        self.functions.points_to(s);
        self.start.points_to(s);
    }
}
#[doc = " Opaque types of names for local variables and basic blocks."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LocalName(pub specr::Name);
impl specr::hidden::GcCompat for LocalName {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        self.0.points_to(s);
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BbName(pub specr::Name);
impl specr::hidden::GcCompat for BbName {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        self.0.points_to(s);
    }
}
#[doc = " A MiniRust function."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Function {
    #[doc = " The locals of this function, and their type."]
    pub locals: Map<LocalName, PlaceType>,
    #[doc = " A list of locals that are initially filled with the function arguments."]
    #[doc = " Also determines the call ABI for each argument."]
    pub args: List<(LocalName, ArgAbi)>,
    #[doc = " The name of a local that holds the return value when the function returns"]
    #[doc = " Also determines the return ABI."]
    pub ret: (LocalName, ArgAbi),
    #[doc = " Associate each basic block name with the associated block."]
    pub blocks: Map<BbName, BasicBlock>,
    #[doc = " The basic block where execution starts."]
    pub start: BbName,
}
impl specr::hidden::GcCompat for Function {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        self.locals.points_to(s);
        self.args.points_to(s);
        self.ret.points_to(s);
        self.blocks.points_to(s);
        self.start.points_to(s);
    }
}
#[doc = " A basic block is a sequence of statements followed by a terminator."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BasicBlock {
    pub statements: List<Statement>,
    pub terminator: Terminator,
}
impl specr::hidden::GcCompat for BasicBlock {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        self.statements.points_to(s);
        self.terminator.points_to(s);
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Statement {
    #[doc = " Copy value from `source` to `target`."]
    Assign {
        destination: PlaceExpr,
        source: ValueExpr,
    },
    #[doc = " Ensure that `place` contains a valid value of its type (else UB)."]
    #[doc = " Also perform retagging."]
    Finalize {
        place: PlaceExpr,
        #[doc = " Indicates whether this operation occurs as part of the prelude"]
        #[doc = " that we have at the top of each function (which affects retagging)."]
        fn_entry: bool,
    },
    #[doc = " Allocate the backing store for this local."]
    StorageLive(LocalName),
    #[doc = " Deallocate the backing store for this local."]
    StorageDead(LocalName),
}
impl specr::hidden::GcCompat for Statement {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Assign {
                destination,
                source,
            } => {
                destination.points_to(s);
                source.points_to(s);
            }
            Self::Finalize { place, fn_entry } => {
                place.points_to(s);
                fn_entry.points_to(s);
            }
            Self::StorageLive(a0) => {
                a0.points_to(s);
            }
            Self::StorageDead(a0) => {
                a0.points_to(s);
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Terminator {
    #[doc = " Just jump to the next block."]
    Goto(BbName),
    #[doc = " `condition` must evaluate to a `Value::Bool`."]
    #[doc = " If it is `true`, jump to `then_block`; else jump to `else_block`."]
    If {
        condition: ValueExpr,
        then_block: BbName,
        else_block: BbName,
    },
    #[doc = " If this is ever executed, we have UB."]
    Unreachable,
    #[doc = " Call the given function with the given arguments."]
    Call {
        callee: FnName,
        #[doc = " The arguments to pass, and which ABIs to use for that."]
        arguments: List<(ValueExpr, ArgAbi)>,
        #[doc = " The place to put the return value into, and which ABI to use for that."]
        ret: (PlaceExpr, ArgAbi),
        #[doc = " The block to jump to when this call returns."]
        next_block: BbName,
    },
    #[doc = " Return from the current function."]
    Return,
}
impl specr::hidden::GcCompat for Terminator {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Goto(a0) => {
                a0.points_to(s);
            }
            Self::If {
                condition,
                then_block,
                else_block,
            } => {
                condition.points_to(s);
                then_block.points_to(s);
                else_block.points_to(s);
            }
            Self::Unreachable => {}
            Self::Call {
                callee,
                arguments,
                ret,
                next_block,
            } => {
                callee.points_to(s);
                arguments.points_to(s);
                ret.points_to(s);
                next_block.points_to(s);
            }
            Self::Return => {}
        }
    }
}
#[doc = " Constants are Values, but cannot have provenance."]
#[doc = " Currently we do not support Ptr and Union constants."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Constant {
    #[doc = " A mathematical integer, used for `i*`/`u*` types."]
    Int(BigInt),
    #[doc = " A Boolean value, used for `bool`."]
    Bool(bool),
    #[doc = " An n-tuple, used for arrays, structs, tuples (including unit)."]
    Tuple(List<Constant>),
    #[doc = " A variant of a sum type, used for enums."]
    Variant {
        idx: BigInt,
        data: specr::hidden::GcCow<Constant>,
    },
}
impl specr::hidden::GcCompat for Constant {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Int(a0) => {
                a0.points_to(s);
            }
            Self::Bool(a0) => {
                a0.points_to(s);
            }
            Self::Tuple(a0) => {
                a0.points_to(s);
            }
            Self::Variant { idx, data } => {
                idx.points_to(s);
                data.points_to(s);
            }
        }
    }
}
#[doc = " A \"value expression\" evaluates to a `Value`."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ValueExpr {
    #[doc = " Just return a constant value."]
    Constant(Constant, Type),
    #[doc = " Load a value from memory."]
    Load {
        #[doc = " Whether this load de-initializes the source it is loaded from (\"move\")."]
        destructive: bool,
        #[doc = " The place to load from."]
        source: specr::hidden::GcCow<PlaceExpr>,
    },
    #[doc = " Create a pointer to a place."]
    AddrOf {
        #[doc = " The place to create a pointer to."]
        target: specr::hidden::GcCow<PlaceExpr>,
        #[doc = " The type of the created pointer."]
        ptr_ty: PtrType,
    },
    #[doc = " Unary operators."]
    UnOp {
        operator: UnOp,
        operand: specr::hidden::GcCow<ValueExpr>,
    },
    #[doc = " Binary operators."]
    BinOp {
        operator: BinOp,
        left: specr::hidden::GcCow<ValueExpr>,
        right: specr::hidden::GcCow<ValueExpr>,
    },
}
impl specr::hidden::GcCompat for ValueExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Constant(a0, a1) => {
                a0.points_to(s);
                a1.points_to(s);
            }
            Self::Load {
                destructive,
                source,
            } => {
                destructive.points_to(s);
                source.points_to(s);
            }
            Self::AddrOf { target, ptr_ty } => {
                target.points_to(s);
                ptr_ty.points_to(s);
            }
            Self::UnOp { operator, operand } => {
                operator.points_to(s);
                operand.points_to(s);
            }
            Self::BinOp {
                operator,
                left,
                right,
            } => {
                operator.points_to(s);
                left.points_to(s);
                right.points_to(s);
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnOpInt {
    #[doc = " Negate an integer value."]
    Neg,
    #[doc = " Cast an integer to another."]
    Cast,
}
impl specr::hidden::GcCompat for UnOpInt {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Neg => {}
            Self::Cast => {}
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnOp {
    #[doc = " An operation on integers, with the given output type."]
    Int(UnOpInt, IntType),
    #[doc = " Pointer-to-integer cast"]
    Ptr2Int,
    #[doc = " Integer-to-pointer cast"]
    Int2Ptr(PtrType),
}
impl specr::hidden::GcCompat for UnOp {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Int(a0, a1) => {
                a0.points_to(s);
                a1.points_to(s);
            }
            Self::Ptr2Int => {}
            Self::Int2Ptr(a0) => {
                a0.points_to(s);
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinOpInt {
    #[doc = " Add two integer values."]
    Add,
    #[doc = " Subtract two integer values."]
    Sub,
    #[doc = " Multiply two integer values."]
    Mul,
    #[doc = " Divide two integer values."]
    #[doc = " Division by zero is UB."]
    Div,
}
impl specr::hidden::GcCompat for BinOpInt {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Add => {}
            Self::Sub => {}
            Self::Mul => {}
            Self::Div => {}
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinOp {
    #[doc = " An operation on integers, with the given output type."]
    Int(BinOpInt, IntType),
    #[doc = " Pointer arithmetic (with or without inbounds requirement)."]
    PtrOffset { inbounds: bool },
}
impl specr::hidden::GcCompat for BinOp {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Int(a0, a1) => {
                a0.points_to(s);
                a1.points_to(s);
            }
            Self::PtrOffset { inbounds } => {
                inbounds.points_to(s);
            }
        }
    }
}
#[doc = " A \"place expression\" evaluates to a `Place`."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PlaceExpr {
    #[doc = " Denotes a local variable."]
    Local(LocalName),
    #[doc = " Dereference a value (of pointer/reference type)."]
    Deref {
        operand: specr::hidden::GcCow<ValueExpr>,
        ptype: PlaceType,
    },
    #[doc = " Project to a field."]
    Field {
        #[doc = " The place to base the projection on."]
        root: specr::hidden::GcCow<PlaceExpr>,
        #[doc = " The field to project to."]
        field: BigInt,
    },
    #[doc = " Index to an array element."]
    Index {
        #[doc = " The array to index into."]
        root: specr::hidden::GcCow<PlaceExpr>,
        #[doc = " The index to project to."]
        index: specr::hidden::GcCow<ValueExpr>,
    },
}
impl specr::hidden::GcCompat for PlaceExpr {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Local(a0) => {
                a0.points_to(s);
            }
            Self::Deref { operand, ptype } => {
                operand.points_to(s);
                ptype.points_to(s);
            }
            Self::Field { root, field } => {
                root.points_to(s);
                field.points_to(s);
            }
            Self::Index { root, index } => {
                root.points_to(s);
                index.points_to(s);
            }
        }
    }
}
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ArgAbi {
    Register,
    Stack(Size, Align),
}
impl specr::hidden::GcCompat for ArgAbi {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Register => {}
            Self::Stack(a0, a1) => {
                a0.points_to(s);
                a1.points_to(s);
            }
        }
    }
}
#[doc = " A \"layout\" describes the shape of data in memory."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Layout {
    pub size: Size,
    pub align: Align,
    pub inhabited: bool,
}
impl specr::hidden::GcCompat for Layout {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        self.size.points_to(s);
        self.align.points_to(s);
        self.inhabited.points_to(s);
    }
}
#[doc = " \"Value\" types -- these have a size, but not an alignment."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Int(IntType),
    Bool,
    Ptr(PtrType),
    #[doc = " \"Tuple\" is used for all heterogeneous types, i.e., both Rust tuples and structs."]
    Tuple {
        #[doc = " Fields must not overlap."]
        fields: Fields,
        #[doc = " The total size of the type can indicate trailing padding."]
        #[doc = " Must be large enough to contain all fields."]
        size: Size,
    },
    Array {
        elem: specr::hidden::GcCow<Type>,
        count: BigInt,
    },
    Union {
        #[doc = " Fields *may* overlap. Fields only exist for field access place projections,"]
        #[doc = " they are irrelevant for the representation relation."]
        fields: Fields,
        #[doc = " A union can be split into multiple \"chunks\", where only the data inside those chunks is"]
        #[doc = " preserved, and data between chunks is lost (like padding in a struct)."]
        #[doc = " This is necessary to model the behavior of some `repr(C)` unions, see"]
        #[doc = " <https://github.com/rust-lang/unsafe-code-guidelines/issues/156> for details."]
        chunks: List<(Size, Size)>,
        #[doc = " The total size of the union, can indicate padding after the last chunk."]
        size: Size,
    },
    Enum {
        #[doc = " Each variant is given by a type. All types are thought to \"start at offset 0\";"]
        #[doc = " if the discriminant is encoded as an explicit tag, then that will be put"]
        #[doc = " into the padding of the active variant. (This means it is *not* safe to hand"]
        #[doc = " out mutable references to a variant at that type, as then the tag might be"]
        #[doc = " overwritten!)"]
        #[doc = " The Rust type `!` is encoded as an `Enum` with an empty list of variants."]
        variants: List<Type>,
        #[doc = " This contains all the tricky details of how to encode the active variant"]
        #[doc = " at runtime."]
        tag_encoding: TagEncoding,
        #[doc = " The total size of the type can indicate trailing padding."]
        #[doc = " Must be large enough to contain all variants."]
        size: Size,
    },
}
impl specr::hidden::GcCompat for Type {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Int(a0) => {
                a0.points_to(s);
            }
            Self::Bool => {}
            Self::Ptr(a0) => {
                a0.points_to(s);
            }
            Self::Tuple { fields, size } => {
                fields.points_to(s);
                size.points_to(s);
            }
            Self::Array { elem, count } => {
                elem.points_to(s);
                count.points_to(s);
            }
            Self::Union {
                fields,
                chunks,
                size,
            } => {
                fields.points_to(s);
                chunks.points_to(s);
                size.points_to(s);
            }
            Self::Enum {
                variants,
                tag_encoding,
                size,
            } => {
                variants.points_to(s);
                tag_encoding.points_to(s);
                size.points_to(s);
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PtrType {
    Ref {
        #[doc = " Indicates a shared vs mutable reference."]
        #[doc = " FIXME: also indicate presence of `UnsafeCell`."]
        mutbl: Mutability,
        #[doc = " We only need to know the layout of the pointee."]
        #[doc = " (This also means we have a finite representation even when the Rust type is recursive.)"]
        pointee: Layout,
    },
    Box {
        pointee: Layout,
    },
    Raw {
        #[doc = " Raw pointer layout is relevant for Stacked Borrows retagging."]
        #[doc = " TODO: I hope we can remove this in the future."]
        pointee: Layout,
    },
}
impl specr::hidden::GcCompat for PtrType {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        match self {
            Self::Ref { mutbl, pointee } => {
                mutbl.points_to(s);
                pointee.points_to(s);
            }
            Self::Box { pointee } => {
                pointee.points_to(s);
            }
            Self::Raw { pointee } => {
                pointee.points_to(s);
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IntType {
    pub signed: Signedness,
    pub size: Size,
}
impl specr::hidden::GcCompat for IntType {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        self.signed.points_to(s);
        self.size.points_to(s);
    }
}
type Fields = List<(Size, Type)>;
#[doc = " We leave the details of enum tags to the future."]
#[doc = " (We might want to extend the \"variants\" field of `Enum` to also have a"]
#[doc = " discriminant for each variant. We will see.)"]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TagEncoding {}
impl specr::hidden::GcCompat for TagEncoding {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
    }
}
#[doc = " \"Place\" types are laid out in memory and thus also have an alignment requirement."]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PlaceType {
    pub ty: Type,
    pub align: Align,
}
impl specr::hidden::GcCompat for PlaceType {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn points_to(&self, s: &mut std::collections::HashSet<usize>) {
        self.ty.points_to(s);
        self.align.points_to(s);
    }
}

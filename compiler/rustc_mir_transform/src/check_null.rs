use rustc_index::IndexVec;
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::mir::*;
use rustc_middle::ty::{Ty, TyCtxt};
use rustc_session::Session;

use crate::check_pointers::{BorrowCheckMode, PointerCheck, check_pointers};

pub(super) struct CheckNull;

impl<'tcx> crate::MirPass<'tcx> for CheckNull {
    fn is_enabled(&self, sess: &Session) -> bool {
        sess.ub_checks()
    }

    fn run_pass(&self, tcx: TyCtxt<'tcx>, body: &mut Body<'tcx>) {
        check_pointers(tcx, body, &[], insert_null_check, BorrowCheckMode::IncludeBorrows);
    }

    fn is_required(&self) -> bool {
        true
    }
}

fn insert_null_check<'tcx>(
    tcx: TyCtxt<'tcx>,
    pointer: Place<'tcx>,
    pointee_ty: Ty<'tcx>,
    local_decls: &mut IndexVec<Local, LocalDecl<'tcx>>,
    stmts: &mut Vec<Statement<'tcx>>,
    source_info: SourceInfo,
) -> PointerCheck<'tcx> {
    // Cast the pointer to a *const ().
    let const_raw_ptr = Ty::new_imm_ptr(tcx, tcx.types.unit);
    let rvalue = Rvalue::Cast(CastKind::PtrToPtr, Operand::Copy(pointer), const_raw_ptr);
    let thin_ptr = local_decls.push(LocalDecl::with_source_info(const_raw_ptr, source_info)).into();
    stmts
        .push(Statement { source_info, kind: StatementKind::Assign(Box::new((thin_ptr, rvalue))) });

    // Transmute the pointer to a usize (equivalent to `ptr.addr()`).
    let rvalue = Rvalue::Cast(CastKind::Transmute, Operand::Copy(thin_ptr), tcx.types.usize);
    let addr = local_decls.push(LocalDecl::with_source_info(tcx.types.usize, source_info)).into();
    stmts.push(Statement { source_info, kind: StatementKind::Assign(Box::new((addr, rvalue))) });

    // Get size of the pointee (zero-sized reads and writes are allowed).
    let rvalue = Rvalue::NullaryOp(NullOp::SizeOf, pointee_ty);
    let sizeof_pointee =
        local_decls.push(LocalDecl::with_source_info(tcx.types.usize, source_info)).into();
    stmts.push(Statement {
        source_info,
        kind: StatementKind::Assign(Box::new((sizeof_pointee, rvalue))),
    });

    // Check that the pointee is not a ZST.
    let zero = Operand::Constant(Box::new(ConstOperand {
        span: source_info.span,
        user_ty: None,
        const_: Const::Val(ConstValue::Scalar(Scalar::from_target_usize(0, &tcx)), tcx.types.usize),
    }));
    let is_pointee_no_zst =
        local_decls.push(LocalDecl::with_source_info(tcx.types.bool, source_info)).into();
    stmts.push(Statement {
        source_info,
        kind: StatementKind::Assign(Box::new((
            is_pointee_no_zst,
            Rvalue::BinaryOp(BinOp::Ne, Box::new((Operand::Copy(sizeof_pointee), zero.clone()))),
        ))),
    });

    // Check whether the pointer is null.
    let is_null = local_decls.push(LocalDecl::with_source_info(tcx.types.bool, source_info)).into();
    stmts.push(Statement {
        source_info,
        kind: StatementKind::Assign(Box::new((
            is_null,
            Rvalue::BinaryOp(BinOp::Eq, Box::new((Operand::Copy(addr), zero))),
        ))),
    });

    // We want to throw an exception if the pointer is null and doesn't point to a ZST.
    let should_throw_exception =
        local_decls.push(LocalDecl::with_source_info(tcx.types.bool, source_info)).into();
    stmts.push(Statement {
        source_info,
        kind: StatementKind::Assign(Box::new((
            should_throw_exception,
            Rvalue::BinaryOp(
                BinOp::BitAnd,
                Box::new((Operand::Copy(is_null), Operand::Copy(is_pointee_no_zst))),
            ),
        ))),
    });

    // The final condition whether this pointer usage is ok or not.
    let is_ok = local_decls.push(LocalDecl::with_source_info(tcx.types.bool, source_info)).into();
    stmts.push(Statement {
        source_info,
        kind: StatementKind::Assign(Box::new((
            is_ok,
            Rvalue::UnaryOp(UnOp::Not, Operand::Copy(should_throw_exception)),
        ))),
    });

    // Emit a PointerCheck that asserts on the condition and otherwise triggers
    // a AssertKind::NullPointerDereference.
    PointerCheck {
        cond: Operand::Copy(is_ok),
        assert_kind: Box::new(AssertKind::NullPointerDereference),
    }
}

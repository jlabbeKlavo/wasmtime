// GENERATED BY ISLE. DO NOT EDIT!
//
// Generated automatically from the instruction-selection DSL code in:
// - src/clif.isle
// - src/prelude.isle
// - src/isa/aarch64/inst.isle
// - src/isa/aarch64/lower.isle

#![allow(dead_code, unreachable_code, unreachable_patterns)]
#![allow(unused_imports, unused_variables, non_snake_case)]
#![allow(irrefutable_let_patterns)]

use super::*; // Pulls in all external types.

/// Context during lowering: an implementation of this trait
/// must be provided with all external constructors and extractors.
/// A mutable borrow is passed along through all lowering logic.
pub trait Context {
    fn unpack_value_array_2(&mut self, arg0: &ValueArray2) -> (Value, Value);
    fn pack_value_array_2(&mut self, arg0: Value, arg1: Value) -> ValueArray2;
    fn unpack_value_array_3(&mut self, arg0: &ValueArray3) -> (Value, Value, Value);
    fn pack_value_array_3(&mut self, arg0: Value, arg1: Value, arg2: Value) -> ValueArray3;
    fn value_reg(&mut self, arg0: Reg) -> ValueRegs;
    fn value_regs(&mut self, arg0: Reg, arg1: Reg) -> ValueRegs;
    fn temp_writable_reg(&mut self, arg0: Type) -> WritableReg;
    fn invalid_reg(&mut self) -> Reg;
    fn put_in_reg(&mut self, arg0: Value) -> Reg;
    fn put_in_regs(&mut self, arg0: Value) -> ValueRegs;
    fn value_regs_get(&mut self, arg0: ValueRegs, arg1: usize) -> Reg;
    fn u8_as_u64(&mut self, arg0: u8) -> u64;
    fn u16_as_u64(&mut self, arg0: u16) -> u64;
    fn u32_as_u64(&mut self, arg0: u32) -> u64;
    fn ty_bits(&mut self, arg0: Type) -> u16;
    fn fits_in_64(&mut self, arg0: Type) -> Option<Type>;
    fn value_list_slice(&mut self, arg0: ValueList) -> ValueSlice;
    fn unwrap_head_value_list_1(&mut self, arg0: ValueList) -> (Value, ValueSlice);
    fn unwrap_head_value_list_2(&mut self, arg0: ValueList) -> (Value, Value, ValueSlice);
    fn writable_reg_to_reg(&mut self, arg0: WritableReg) -> Reg;
    fn u64_from_imm64(&mut self, arg0: Imm64) -> u64;
    fn u64_from_ieee32(&mut self, arg0: Ieee32) -> u64;
    fn u64_from_ieee64(&mut self, arg0: Ieee64) -> u64;
    fn inst_results(&mut self, arg0: Inst) -> ValueSlice;
    fn first_result(&mut self, arg0: Inst) -> Option<Value>;
    fn inst_data(&mut self, arg0: Inst) -> InstructionData;
    fn value_type(&mut self, arg0: Value) -> Type;
    fn multi_lane(&mut self, arg0: Type) -> Option<(u8, u16)>;
    fn def_inst(&mut self, arg0: Value) -> Option<Inst>;
    fn move_wide_const_from_u64(&mut self, arg0: u64) -> Option<MoveWideConst>;
    fn move_wide_const_from_negated_u64(&mut self, arg0: u64) -> Option<MoveWideConst>;
    fn imm_logic_from_u64(&mut self, arg0: u64) -> Option<ImmLogic>;
    fn integral_ty(&mut self, arg0: Type) -> Option<Type>;
    fn emit(&mut self, arg0: &MInst) -> Unit;
    fn zero_reg(&mut self) -> Reg;
    fn load_constant64_full(&mut self, arg0: u64) -> Reg;
}

/// Internal type MInst: defined at src/isa/aarch64/inst.isle line 2.
#[derive(Clone, Debug)]
pub enum MInst {
    Nop0,
    Nop4,
    AluRRR {
        alu_op: ALUOp,
        rd: WritableReg,
        rn: Reg,
        rm: Reg,
    },
    AluRRRR {
        alu_op: ALUOp3,
        rd: WritableReg,
        rn: Reg,
        rm: Reg,
        ra: Reg,
    },
    AluRRImm12 {
        alu_op: ALUOp,
        rd: WritableReg,
        rn: Reg,
        imm12: Imm12,
    },
    AluRRImmLogic {
        alu_op: ALUOp,
        rd: WritableReg,
        rn: Reg,
        imml: ImmLogic,
    },
    AluRRImmShift {
        alu_op: ALUOp,
        rd: WritableReg,
        rn: Reg,
        immshift: ImmShift,
    },
    AluRRRShift {
        alu_op: ALUOp,
        rd: WritableReg,
        rn: Reg,
        rm: Reg,
        shiftop: ShiftOpAndAmt,
    },
    AluRRRExtend {
        alu_op: ALUOp,
        rd: WritableReg,
        rn: Reg,
        rm: Reg,
        extendop: ExtendOp,
    },
    BitRR {
        op: BitOp,
        rd: WritableReg,
        rn: Reg,
    },
    ULoad8 {
        rd: WritableReg,
        mem: AMode,
        flags: MemFlags,
    },
    SLoad8 {
        rd: WritableReg,
        mem: AMode,
        flags: MemFlags,
    },
    ULoad16 {
        rd: WritableReg,
        mem: AMode,
        flags: MemFlags,
    },
    SLoad16 {
        rd: WritableReg,
        mem: AMode,
        flags: MemFlags,
    },
    ULoad32 {
        rd: WritableReg,
        mem: AMode,
        flags: MemFlags,
    },
    SLoad32 {
        rd: WritableReg,
        mem: AMode,
        flags: MemFlags,
    },
    ULoad64 {
        rd: WritableReg,
        mem: AMode,
        flags: MemFlags,
    },
    Store8 {
        rd: Reg,
        mem: AMode,
        flags: MemFlags,
    },
    Store16 {
        rd: Reg,
        mem: AMode,
        flags: MemFlags,
    },
    Store32 {
        rd: Reg,
        mem: AMode,
        flags: MemFlags,
    },
    Store64 {
        rd: Reg,
        mem: AMode,
        flags: MemFlags,
    },
    StoreP64 {
        rt: Reg,
        rt2: Reg,
        mem: PairAMode,
        flags: MemFlags,
    },
    LoadP64 {
        rt: WritableReg,
        rt2: WritableReg,
        mem: PairAMode,
        flags: MemFlags,
    },
    Mov64 {
        rd: WritableReg,
        rm: Reg,
    },
    Mov32 {
        rd: WritableReg,
        rm: Reg,
    },
    MovZ {
        rd: WritableReg,
        imm: MoveWideConst,
        size: OperandSize,
    },
    MovN {
        rd: WritableReg,
        imm: MoveWideConst,
        size: OperandSize,
    },
    MovK {
        rd: WritableReg,
        imm: MoveWideConst,
        size: OperandSize,
    },
    Extend {
        rd: WritableReg,
        rn: Reg,
        signed: bool,
        from_bits: u8,
        to_bits: u8,
    },
    CSel {
        rd: WritableReg,
        cond: Cond,
        rn: Reg,
        rm: Reg,
    },
    CSet {
        rd: WritableReg,
        cond: Cond,
    },
    CSetm {
        rd: WritableReg,
        cond: Cond,
    },
    CCmpImm {
        size: OperandSize,
        rn: Reg,
        imm: UImm5,
        nzcv: NZCV,
        cond: Cond,
    },
    AtomicRMWLoop {
        ty: Type,
        op: AtomicRmwOp,
    },
    AtomicRMW {
        op: AtomicRMWOp,
        rs: Reg,
        rt: WritableReg,
        rn: Reg,
        ty: Type,
    },
    AtomicCAS {
        rs: WritableReg,
        rt: Reg,
        rn: Reg,
        ty: Type,
    },
    AtomicCASLoop {
        ty: Type,
    },
    LoadAcquire {
        access_ty: Type,
        rt: WritableReg,
        rn: Reg,
    },
    StoreRelease {
        access_ty: Type,
        rt: Reg,
        rn: Reg,
    },
    Fence,
    FpuMove64 {
        rd: WritableReg,
        rn: Reg,
    },
    FpuMove128 {
        rd: WritableReg,
        rn: Reg,
    },
    FpuMoveFromVec {
        rd: WritableReg,
        rn: Reg,
        idx: u8,
        size: VectorSize,
    },
    FpuExtend {
        rd: WritableReg,
        rn: Reg,
        size: ScalarSize,
    },
    FpuRR {
        fpu_op: FPUOp1,
        rd: WritableReg,
        rn: Reg,
    },
    FpuRRR {
        fpu_op: FPUOp2,
        rd: WritableReg,
        rn: Reg,
        rm: Reg,
    },
    FpuRRI {
        fpu_op: FPUOpRI,
        rd: WritableReg,
        rn: Reg,
    },
    FpuRRRR {
        fpu_op: FPUOp3,
        rd: WritableReg,
        rn: Reg,
        rm: Reg,
        ra: Reg,
    },
    FpuCmp32 {
        rn: Reg,
        rm: Reg,
    },
    FpuCmp64 {
        rn: Reg,
        rm: Reg,
    },
    FpuLoad32 {
        rd: WritableReg,
        mem: AMode,
        flags: MemFlags,
    },
    FpuStore32 {
        rd: Reg,
        mem: AMode,
        flags: MemFlags,
    },
    FpuLoad64 {
        rd: WritableReg,
        mem: AMode,
        flags: MemFlags,
    },
    FpuStore64 {
        rd: Reg,
        mem: AMode,
        flags: MemFlags,
    },
    FpuLoad128 {
        rd: WritableReg,
        mem: AMode,
        flags: MemFlags,
    },
    FpuStore128 {
        rd: Reg,
        mem: AMode,
        flags: MemFlags,
    },
    FpuLoadP64 {
        rt: WritableReg,
        rt2: WritableReg,
        mem: PairAMode,
        flags: MemFlags,
    },
    FpuStoreP64 {
        rt: Reg,
        rt2: Reg,
        mem: PairAMode,
        flags: MemFlags,
    },
    FpuLoadP128 {
        rt: WritableReg,
        rt2: WritableReg,
        mem: PairAMode,
        flags: MemFlags,
    },
    FpuStoreP128 {
        rt: Reg,
        rt2: Reg,
        mem: PairAMode,
        flags: MemFlags,
    },
    LoadFpuConst64 {
        rd: WritableReg,
        const_data: u64,
    },
    LoadFpuConst128 {
        rd: WritableReg,
        const_data: u128,
    },
    FpuToInt {
        op: FpuToIntOp,
        rd: WritableReg,
        rn: Reg,
    },
    IntToFpu {
        op: IntToFpuOp,
        rd: WritableReg,
        rn: Reg,
    },
    FpuCSel32 {
        rd: WritableReg,
        rn: Reg,
        rm: Reg,
        cond: Cond,
    },
    FpuCSel64 {
        rd: WritableReg,
        rn: Reg,
        rm: Reg,
        cond: Cond,
    },
    FpuRound {
        op: FpuRoundMode,
        rd: WritableReg,
        rn: Reg,
    },
    MovToFpu {
        rd: WritableReg,
        rn: Reg,
        size: ScalarSize,
    },
    MovToVec {
        rd: WritableReg,
        rn: Reg,
        idx: u8,
        size: VectorSize,
    },
    MovFromVec {
        rd: WritableReg,
        rn: Reg,
        idx: u8,
        size: VectorSize,
    },
    MovFromVecSigned {
        rd: WritableReg,
        rn: Reg,
        idx: u8,
        size: VectorSize,
        scalar_size: OperandSize,
    },
    VecDup {
        rd: WritableReg,
        rn: Reg,
        size: VectorSize,
    },
    VecDupFromFpu {
        rd: WritableReg,
        rn: Reg,
        size: VectorSize,
    },
    VecDupFPImm {
        rd: WritableReg,
        imm: ASIMDFPModImm,
        size: VectorSize,
    },
    VecDupImm {
        rd: WritableReg,
        imm: ASIMDMovModImm,
        invert: bool,
        size: VectorSize,
    },
    VecExtend {
        t: VecExtendOp,
        rd: WritableReg,
        rn: Reg,
        high_half: bool,
    },
    VecMovElement {
        rd: WritableReg,
        rn: Reg,
        dest_idx: u8,
        src_idx: u8,
        size: VectorSize,
    },
    VecRRLong {
        op: VecRRLongOp,
        rd: WritableReg,
        rn: Reg,
        high_half: bool,
    },
    VecRRNarrow {
        op: VecRRNarrowOp,
        rd: WritableReg,
        rn: Reg,
        high_half: bool,
    },
    VecRRPair {
        op: VecPairOp,
        rd: WritableReg,
        rn: Reg,
    },
    VecRRRLong {
        alu_op: VecRRRLongOp,
        rd: WritableReg,
        rn: Reg,
        rm: Reg,
        high_half: bool,
    },
    VecRRPairLong {
        op: VecRRPairLongOp,
        rd: WritableReg,
        rn: Reg,
    },
    VecRRR {
        alu_op: VecALUOp,
        rd: WritableReg,
        rn: Reg,
        rm: Reg,
        size: VectorSize,
    },
    VecMisc {
        op: VecMisc2,
        rd: WritableReg,
        rn: Reg,
        size: VectorSize,
    },
    VecLanes {
        op: VecLanesOp,
        rd: WritableReg,
        rn: Reg,
        size: VectorSize,
    },
    VecShiftImm {
        op: VecShiftImmOp,
        rd: WritableReg,
        rn: Reg,
        size: VectorSize,
        imm: u8,
    },
    VecExtract {
        rd: WritableReg,
        rn: Reg,
        rm: Reg,
        imm4: u8,
    },
    VecTbl {
        rd: WritableReg,
        rn: Reg,
        rm: Reg,
        is_extension: bool,
    },
    VecTbl2 {
        rd: WritableReg,
        rn: Reg,
        rn2: Reg,
        rm: Reg,
        is_extension: bool,
    },
    VecLoadReplicate {
        rd: WritableReg,
        rn: Reg,
        size: VectorSize,
    },
    VecCSel {
        rd: WritableReg,
        rn: Reg,
        rm: Reg,
        cond: Cond,
    },
    MovToNZCV {
        rn: Reg,
    },
    MovFromNZCV {
        rd: WritableReg,
    },
    Call {
        info: BoxCallInfo,
    },
    CallInd {
        info: BoxCallIndInfo,
    },
    Ret,
    EpiloguePlaceholder,
    Jump {
        dest: BranchTarget,
    },
    CondBr {
        taken: BranchTarget,
        not_taken: BranchTarget,
        kind: CondBrKind,
    },
    TrapIf {
        kind: CondBrKind,
        trap_code: TrapCode,
    },
    IndirectBr {
        rn: Reg,
        targets: VecMachLabel,
    },
    Brk,
    Udf {
        trap_code: TrapCode,
    },
    Adr {
        rd: WritableReg,
        off: i32,
    },
    Word4 {
        data: u32,
    },
    Word8 {
        data: u64,
    },
    JTSequence {
        info: BoxJTSequenceInfo,
        ridx: Reg,
        rtmp1: WritableReg,
        rtmp2: WritableReg,
    },
    LoadExtName {
        rd: WritableReg,
        name: BoxExternalName,
        offset: i64,
    },
    LoadAddr {
        rd: WritableReg,
        mem: AMode,
    },
    VirtualSPOffsetAdj {
        offset: i64,
    },
    EmitIsland {
        needed_space: CodeOffset,
    },
    ElfTlsGetAddr {
        symbol: ExternalName,
    },
    ValueLabelMarker {
        reg: Reg,
        label: ValueLabel,
    },
    Unwind {
        inst: UnwindInst,
    },
}

/// Internal type ALUOp: defined at src/isa/aarch64/inst.isle line 783.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ALUOp {
    Add32,
    Add64,
    Sub32,
    Sub64,
    Orr32,
    Orr64,
    OrrNot32,
    OrrNot64,
    And32,
    And64,
    AndS32,
    AndS64,
    AndNot32,
    AndNot64,
    Eor32,
    Eor64,
    EorNot32,
    EorNot64,
    AddS32,
    AddS64,
    SubS32,
    SubS64,
    SMulH,
    UMulH,
    SDiv64,
    UDiv64,
    RotR32,
    RotR64,
    Lsr32,
    Lsr64,
    Asr32,
    Asr64,
    Lsl32,
    Lsl64,
    Adc32,
    Adc64,
    AdcS32,
    AdcS64,
    Sbc32,
    Sbc64,
    SbcS32,
    SbcS64,
}

/// Internal type ALUOp3: defined at src/isa/aarch64/inst.isle line 844.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ALUOp3 {
    MAdd32,
    MAdd64,
    MSub32,
    MSub64,
}

/// Internal type BitOp: defined at src/isa/aarch64/inst.isle line 892.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BitOp {
    RBit32,
    RBit64,
    Clz32,
    Clz64,
    Cls32,
    Cls64,
}

/// Internal type FPUOp1: defined at src/isa/aarch64/inst.isle line 951.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FPUOp1 {
    Abs32,
    Abs64,
    Neg32,
    Neg64,
    Sqrt32,
    Sqrt64,
    Cvt32To64,
    Cvt64To32,
}

/// Internal type FPUOp2: defined at src/isa/aarch64/inst.isle line 964.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FPUOp2 {
    Add32,
    Add64,
    Sub32,
    Sub64,
    Mul32,
    Mul64,
    Div32,
    Div64,
    Max32,
    Max64,
    Min32,
    Min64,
    Sqadd64,
    Uqadd64,
    Sqsub64,
    Uqsub64,
}

/// Internal type FPUOp3: defined at src/isa/aarch64/inst.isle line 989.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FPUOp3 {
    MAdd32,
    MAdd64,
}

/// Internal type FpuToIntOp: defined at src/isa/aarch64/inst.isle line 996.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FpuToIntOp {
    F32ToU32,
    F32ToI32,
    F32ToU64,
    F32ToI64,
    F64ToU32,
    F64ToI32,
    F64ToU64,
    F64ToI64,
}

/// Internal type IntToFpuOp: defined at src/isa/aarch64/inst.isle line 1009.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntToFpuOp {
    U32ToF32,
    I32ToF32,
    U32ToF64,
    I32ToF64,
    U64ToF32,
    I64ToF32,
    U64ToF64,
    I64ToF64,
}

/// Internal type FpuRoundMode: defined at src/isa/aarch64/inst.isle line 1023.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FpuRoundMode {
    Minus32,
    Minus64,
    Plus32,
    Plus64,
    Zero32,
    Zero64,
    Nearest32,
    Nearest64,
}

/// Internal type VecExtendOp: defined at src/isa/aarch64/inst.isle line 1036.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VecExtendOp {
    Sxtl8,
    Sxtl16,
    Sxtl32,
    Uxtl8,
    Uxtl16,
    Uxtl32,
}

/// Internal type VecALUOp: defined at src/isa/aarch64/inst.isle line 1053.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VecALUOp {
    Sqadd,
    Uqadd,
    Sqsub,
    Uqsub,
    Cmeq,
    Cmge,
    Cmgt,
    Cmhs,
    Cmhi,
    Fcmeq,
    Fcmgt,
    Fcmge,
    And,
    Bic,
    Orr,
    Eor,
    Bsl,
    Umaxp,
    Add,
    Sub,
    Mul,
    Sshl,
    Ushl,
    Umin,
    Smin,
    Umax,
    Smax,
    Urhadd,
    Fadd,
    Fsub,
    Fdiv,
    Fmax,
    Fmin,
    Fmul,
    Addp,
    Zip1,
    Sqrdmulh,
}

/// Internal type VecMisc2: defined at src/isa/aarch64/inst.isle line 1132.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VecMisc2 {
    Not,
    Neg,
    Abs,
    Fabs,
    Fneg,
    Fsqrt,
    Rev64,
    Fcvtzs,
    Fcvtzu,
    Scvtf,
    Ucvtf,
    Frintn,
    Frintz,
    Frintm,
    Frintp,
    Cnt,
    Cmeq0,
}

/// Internal type VecRRLongOp: defined at src/isa/aarch64/inst.isle line 1171.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VecRRLongOp {
    Fcvtl16,
    Fcvtl32,
    Shll8,
    Shll16,
    Shll32,
}

/// Internal type VecRRNarrowOp: defined at src/isa/aarch64/inst.isle line 1186.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VecRRNarrowOp {
    Xtn16,
    Xtn32,
    Xtn64,
    Sqxtn16,
    Sqxtn32,
    Sqxtn64,
    Sqxtun16,
    Sqxtun32,
    Sqxtun64,
    Uqxtn16,
    Uqxtn32,
    Uqxtn64,
    Fcvtn32,
    Fcvtn64,
}

/// Internal type VecRRRLongOp: defined at src/isa/aarch64/inst.isle line 1218.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VecRRRLongOp {
    Smull8,
    Smull16,
    Smull32,
    Umull8,
    Umull16,
    Umull32,
    Umlal8,
    Umlal16,
    Umlal32,
}

/// Internal type VecPairOp: defined at src/isa/aarch64/inst.isle line 1235.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VecPairOp {
    Addp,
}

/// Internal type VecRRPairLongOp: defined at src/isa/aarch64/inst.isle line 1243.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VecRRPairLongOp {
    Saddlp8,
    Saddlp16,
    Uaddlp8,
    Uaddlp16,
}

/// Internal type VecLanesOp: defined at src/isa/aarch64/inst.isle line 1254.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VecLanesOp {
    Addv,
    Uminv,
}

/// Internal type VecShiftImmOp: defined at src/isa/aarch64/inst.isle line 1263.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VecShiftImmOp {
    Shl,
    Ushr,
    Sshr,
}

/// Internal type AtomicRMWOp: defined at src/isa/aarch64/inst.isle line 1274.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AtomicRMWOp {
    Add,
    Clr,
    Eor,
    Set,
    Smax,
    Smin,
    Umax,
    Umin,
}

// Generated as internal constructor for term temp_reg.
pub fn constructor_temp_reg<C: Context>(ctx: &mut C, arg0: Type) -> Option<Reg> {
    let pattern0_0 = arg0;
    // Rule at src/prelude.isle line 60.
    let expr0_0 = C::temp_writable_reg(ctx, pattern0_0);
    let expr1_0 = C::writable_reg_to_reg(ctx, expr0_0);
    return Some(expr1_0);
}

// Generated as internal constructor for term lo_reg.
pub fn constructor_lo_reg<C: Context>(ctx: &mut C, arg0: Value) -> Option<Reg> {
    let pattern0_0 = arg0;
    // Rule at src/prelude.isle line 95.
    let expr0_0 = C::put_in_regs(ctx, pattern0_0);
    let expr1_0: usize = 0;
    let expr2_0 = C::value_regs_get(ctx, expr0_0, expr1_0);
    return Some(expr2_0);
}

// Generated as internal constructor for term movz.
pub fn constructor_movz<C: Context>(
    ctx: &mut C,
    arg0: MoveWideConst,
    arg1: &OperandSize,
) -> Option<Reg> {
    let pattern0_0 = arg0;
    let pattern1_0 = arg1;
    // Rule at src/isa/aarch64/inst.isle line 1315.
    let expr0_0: Type = I64;
    let expr1_0 = C::temp_writable_reg(ctx, expr0_0);
    let expr2_0 = MInst::MovZ {
        rd: expr1_0,
        imm: pattern0_0,
        size: pattern1_0.clone(),
    };
    let expr3_0 = C::emit(ctx, &expr2_0);
    let expr4_0 = C::writable_reg_to_reg(ctx, expr1_0);
    return Some(expr4_0);
}

// Generated as internal constructor for term movn.
pub fn constructor_movn<C: Context>(
    ctx: &mut C,
    arg0: MoveWideConst,
    arg1: &OperandSize,
) -> Option<Reg> {
    let pattern0_0 = arg0;
    let pattern1_0 = arg1;
    // Rule at src/isa/aarch64/inst.isle line 1322.
    let expr0_0: Type = I64;
    let expr1_0 = C::temp_writable_reg(ctx, expr0_0);
    let expr2_0 = MInst::MovN {
        rd: expr1_0,
        imm: pattern0_0,
        size: pattern1_0.clone(),
    };
    let expr3_0 = C::emit(ctx, &expr2_0);
    let expr4_0 = C::writable_reg_to_reg(ctx, expr1_0);
    return Some(expr4_0);
}

// Generated as internal constructor for term alu_rr_imm_logic.
pub fn constructor_alu_rr_imm_logic<C: Context>(
    ctx: &mut C,
    arg0: &ALUOp,
    arg1: Reg,
    arg2: ImmLogic,
) -> Option<Reg> {
    let pattern0_0 = arg0;
    let pattern1_0 = arg1;
    let pattern2_0 = arg2;
    // Rule at src/isa/aarch64/inst.isle line 1329.
    let expr0_0: Type = I64;
    let expr1_0 = C::temp_writable_reg(ctx, expr0_0);
    let expr2_0 = MInst::AluRRImmLogic {
        alu_op: pattern0_0.clone(),
        rd: expr1_0,
        rn: pattern1_0,
        imml: pattern2_0,
    };
    let expr3_0 = C::emit(ctx, &expr2_0);
    let expr4_0 = C::writable_reg_to_reg(ctx, expr1_0);
    return Some(expr4_0);
}

// Generated as internal constructor for term orr64.
pub fn constructor_orr64<C: Context>(ctx: &mut C, arg0: Reg, arg1: ImmLogic) -> Option<Reg> {
    let pattern0_0 = arg0;
    let pattern1_0 = arg1;
    // Rule at src/isa/aarch64/inst.isle line 1336.
    let expr0_0 = ALUOp::Orr64;
    let expr1_0 = constructor_alu_rr_imm_logic(ctx, &expr0_0, pattern0_0, pattern1_0)?;
    return Some(expr1_0);
}

// Generated as internal constructor for term imm.
pub fn constructor_imm<C: Context>(ctx: &mut C, arg0: Type, arg1: u64) -> Option<Reg> {
    let pattern0_0 = arg0;
    if let Some(pattern1_0) = C::integral_ty(ctx, pattern0_0) {
        let pattern2_0 = arg1;
        if let Some(pattern3_0) = C::imm_logic_from_u64(ctx, pattern2_0) {
            // Rule at src/isa/aarch64/inst.isle line 1351.
            let expr0_0 = C::zero_reg(ctx);
            let expr1_0 = constructor_orr64(ctx, expr0_0, pattern3_0)?;
            return Some(expr1_0);
        }
        if let Some(pattern3_0) = C::move_wide_const_from_u64(ctx, pattern2_0) {
            // Rule at src/isa/aarch64/inst.isle line 1343.
            let expr0_0 = OperandSize::Size64;
            let expr1_0 = constructor_movz(ctx, pattern3_0, &expr0_0)?;
            return Some(expr1_0);
        }
        if let Some(pattern3_0) = C::move_wide_const_from_negated_u64(ctx, pattern2_0) {
            // Rule at src/isa/aarch64/inst.isle line 1347.
            let expr0_0 = OperandSize::Size64;
            let expr1_0 = constructor_movn(ctx, pattern3_0, &expr0_0)?;
            return Some(expr1_0);
        }
        // Rule at src/isa/aarch64/inst.isle line 1358.
        let expr0_0 = C::load_constant64_full(ctx, pattern2_0);
        return Some(expr0_0);
    }
    return None;
}

// Generated as internal constructor for term lower.
pub fn constructor_lower<C: Context>(ctx: &mut C, arg0: Inst) -> Option<ValueRegs> {
    let pattern0_0 = arg0;
    if let Some(pattern1_0) = C::first_result(ctx, pattern0_0) {
        let pattern2_0 = C::value_type(ctx, pattern1_0);
        let pattern3_0 = C::inst_data(ctx, pattern0_0);
        match &pattern3_0 {
            &InstructionData::NullAry {
                opcode: ref pattern4_0,
            } => {
                if let &Opcode::Null = &pattern4_0 {
                    // Rule at src/isa/aarch64/lower.isle line 22.
                    let expr0_0: u64 = 0;
                    let expr1_0 = constructor_imm(ctx, pattern2_0, expr0_0)?;
                    let expr2_0 = C::value_reg(ctx, expr1_0);
                    return Some(expr2_0);
                }
            }
            &InstructionData::UnaryImm {
                opcode: ref pattern4_0,
                imm: pattern4_1,
            } => {
                if let &Opcode::Iconst = &pattern4_0 {
                    let pattern6_0 = C::u64_from_imm64(ctx, pattern4_1);
                    // Rule at src/isa/aarch64/lower.isle line 9.
                    let expr0_0 = constructor_imm(ctx, pattern2_0, pattern6_0)?;
                    let expr1_0 = C::value_reg(ctx, expr0_0);
                    return Some(expr1_0);
                }
            }
            &InstructionData::UnaryBool {
                opcode: ref pattern4_0,
                imm: pattern4_1,
            } => {
                if let &Opcode::Bconst = &pattern4_0 {
                    if pattern4_1 == true {
                        // Rule at src/isa/aarch64/lower.isle line 17.
                        let expr0_0: u64 = 1;
                        let expr1_0 = constructor_imm(ctx, pattern2_0, expr0_0)?;
                        let expr2_0 = C::value_reg(ctx, expr1_0);
                        return Some(expr2_0);
                    }
                    if pattern4_1 == false {
                        // Rule at src/isa/aarch64/lower.isle line 14.
                        let expr0_0: u64 = 0;
                        let expr1_0 = constructor_imm(ctx, pattern2_0, expr0_0)?;
                        let expr2_0 = C::value_reg(ctx, expr1_0);
                        return Some(expr2_0);
                    }
                }
            }
            _ => {}
        }
    }
    return None;
}
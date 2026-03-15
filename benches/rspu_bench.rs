#![forbid(unsafe_code)]
//! Criterion benchmarks for the R-SPU subsystem — encode, decode, emit_binary,
//! simulate, and tagged-register operations. All loops bounded (NASA Power-of-10).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use nasa_rust_project::emit::rspu_isa::{AluOp, AluUnaryOp, MAX_SIM_CYCLES};
use nasa_rust_project::emit::rspu_tagged::check_alu_tags;
use nasa_rust_project::{
    decode, emit_binary, encode, RegisterFile, RspuInstruction, RspuProgram, RspuSimulator,
    TaggedWord, TypeTag,
};

// --- Bounded-iteration constants (NASA Power-of-10) ---

const MAX_BENCH_INSTRS: usize = 256;
const MAX_BENCH_REGS: usize = 64;

// --- Input generators (bounded iteration, NASA-compliant) ---

fn prog(instrs: Vec<RspuInstruction>, regs: usize, guards: usize) -> RspuProgram {
    RspuProgram {
        instructions: instrs,
        registers_used: regs,
        guards_used: guards,
        register_map: vec![],
        guard_map: vec![],
        certificate: None,
    }
}

/// Small program: 5 instructions — load, immediate, ALU add, store, halt.
fn small_program() -> RspuProgram {
    prog(
        vec![
            RspuInstruction::LoadInput { dst: 0, port: 0 },
            RspuInstruction::LoadImm { dst: 1, value: 100, width: 8 },
            RspuInstruction::Alu { op: AluOp::Add, dst: 2, a: 0, b: 1 },
            RspuInstruction::StoreOutput { src: 2, port: 0 },
            RspuInstruction::Halt,
        ],
        3,
        0,
    )
}

/// Medium program: 33 instructions — alternating LoadImm / ALU pairs + halt.
fn medium_program() -> RspuProgram {
    let mut v = Vec::with_capacity(33);
    for i in 0..32_usize {
        let r = (i % 63) as u8;
        if i % 2 == 0 {
            v.push(RspuInstruction::LoadImm { dst: r, value: i as u64, width: 8 });
        } else {
            v.push(RspuInstruction::Alu { op: AluOp::Add, dst: r, a: r, b: 0 });
        }
    }
    v.push(RspuInstruction::Halt);
    prog(v, 63, 0)
}

/// Large program: 256 instructions — bounded LoadImm/Alu/Nop pattern + halt.
fn large_program() -> RspuProgram {
    let mut v = Vec::with_capacity(MAX_BENCH_INSTRS + 1);
    for i in 0..MAX_BENCH_INSTRS {
        let r = (i % 63) as u8;
        match i % 3 {
            0 => v.push(RspuInstruction::LoadImm { dst: r, value: i as u64, width: 8 }),
            1 => v.push(RspuInstruction::Alu { op: AluOp::Add, dst: r, a: r, b: 0 }),
            _ => v.push(RspuInstruction::Nop),
        }
    }
    v.push(RspuInstruction::Halt);
    prog(v, 63, 0)
}

/// One representative of every 30 R-SPU opcode for encode/decode benchmarks.
fn all_30_opcodes() -> Vec<RspuInstruction> {
    use RspuInstruction::*;
    vec![
        LoadInput { dst: 0, port: 0 },
        StoreOutput { src: 1, port: 0 },
        Mov { dst: 2, src: 3 },
        LoadImm { dst: 4, value: 42, width: 8 },
        Alu { op: AluOp::Add, dst: 5, a: 6, b: 7 },
        AluImm { op: AluOp::Sub, dst: 8, a: 9, imm: 10 },
        AluUnary { op: AluUnaryOp::Not, dst: 10, src: 11 },
        SrInit { guard: 0, length: 4, cond: 12 },
        SrTick { guard: 0 },
        SrQuery { dst: 13, guard: 0 },
        CtrInit { guard: 1, target: 8, cond: 14 },
        CtrTick { guard: 1 },
        CtrQuery { dst: 15, guard: 1 },
        GuardAnd { dst: 2, a: 0, b: 1 },
        GuardOr { dst: 3, a: 0, b: 1 },
        ReflexIf { guard: 0, dst: 16, src: 17 },
        Prev { dst: 18, signal: 19, delay: 2 },
        EmergencyStop,
        AssertAlways { cond: 20, property_id: 0 },
        AssertNever { cond: 21, property_id: 1 },
        Trap { code: 1 },
        TrapIf { cond: 22, code: 2 },
        Halt,
        ModeSwitch { mode: 0 },
        Nop,
        Fence,
        TagLoad { dst: 23, tag: 1 },
        TagCheck { src: 24, expected: 1 },
        TagRead { dst: 25, src: 26 },
        DeadlineSet { cycles: 100 },
    ]
}

/// All 14 AluOp variants for tag-checking benchmarks.
fn all_alu_ops() -> Vec<AluOp> {
    vec![
        AluOp::Add,
        AluOp::Sub,
        AluOp::Mul,
        AluOp::And,
        AluOp::Or,
        AluOp::Xor,
        AluOp::Shl,
        AluOp::Shr,
        AluOp::Eq,
        AluOp::Ne,
        AluOp::Lt,
        AluOp::Le,
        AluOp::Gt,
        AluOp::Ge,
    ]
}

// --- Benchmark groups ---

fn bench_rspu_encode(c: &mut Criterion) {
    let opcodes = all_30_opcodes();
    let mut g = c.benchmark_group("rspu_encode");
    g.bench_function("single", |b| {
        let instr = RspuInstruction::LoadInput { dst: 0, port: 0 };
        b.iter(|| encode(black_box(&instr)))
    });
    g.bench_function("all_30_opcodes", |b| {
        b.iter(|| {
            for instr in &opcodes {
                let _ = encode(black_box(instr));
            }
        })
    });
    g.finish();
}

fn bench_rspu_decode(c: &mut Criterion) {
    let encoded: Vec<u32> = all_30_opcodes().iter().map(|i| encode(i).unwrap().0).collect();
    let word = encoded[0];
    let mut g = c.benchmark_group("rspu_decode");
    g.bench_function("single", |b| b.iter(|| decode(black_box(word))));
    g.bench_function("all_30_opcodes", |b| {
        b.iter(|| {
            for &word in &encoded {
                let _ = decode(black_box(word));
            }
        })
    });
    g.finish();
}

fn bench_rspu_emit_binary(c: &mut Criterion) {
    let (s, m, l) = (small_program(), medium_program(), large_program());
    let mut g = c.benchmark_group("rspu_emit_binary");
    g.bench_function("small", |b| b.iter(|| emit_binary(black_box(&s))));
    g.bench_function("medium", |b| b.iter(|| emit_binary(black_box(&m))));
    g.bench_function("large", |b| b.iter(|| emit_binary(black_box(&l))));
    g.finish();
}

fn bench_rspu_simulate(c: &mut Criterion) {
    let (s, m, l) = (small_program(), medium_program(), large_program());
    let mut g = c.benchmark_group("rspu_simulate");
    g.bench_function("small_run", |b| {
        b.iter(|| RspuSimulator::new().run(black_box(&s), MAX_SIM_CYCLES))
    });
    g.bench_function("medium_run", |b| {
        b.iter(|| RspuSimulator::new().run(black_box(&m), MAX_SIM_CYCLES))
    });
    g.bench_function("large_run", |b| {
        b.iter(|| RspuSimulator::new().run(black_box(&l), MAX_SIM_CYCLES))
    });
    g.finish();
}

fn bench_rspu_tagged(c: &mut Criterion) {
    let ops = all_alu_ops();
    let mut g = c.benchmark_group("rspu_tagged");
    g.bench_function("register_rw", |b| {
        b.iter(|| {
            let mut rf = RegisterFile::new();
            for i in 0..MAX_BENCH_REGS {
                rf.write(
                    i as u8,
                    TaggedWord::from_literal(i as u64, TypeTag::Unsigned { width: 16 }),
                );
            }
            for i in 0..MAX_BENCH_REGS {
                black_box(rf.read(i as u8));
            }
        })
    });
    g.bench_function("alu_tag_check", |b| {
        let a = TaggedWord::from_literal(10, TypeTag::Unsigned { width: 16 });
        let bw = TaggedWord::from_literal(5, TypeTag::Unsigned { width: 16 });
        b.iter(|| {
            for op in &ops {
                let _ = check_alu_tags(black_box(&a), black_box(&bw), *op);
            }
        })
    });
    g.finish();
}

criterion_group!(
    benches,
    bench_rspu_encode,
    bench_rspu_decode,
    bench_rspu_emit_binary,
    bench_rspu_simulate,
    bench_rspu_tagged,
);
criterion_main!(benches);

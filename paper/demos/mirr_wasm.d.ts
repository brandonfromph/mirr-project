/* tslint:disable */
/* eslint-disable */

export function compile_dot(source: string): string;

export function compile_firrtl(source: string): string;

export function compile_graph_data(source: string): string;

export function compile_pipeline_stages(source: string): string;

export function compile_rspu(source: string): string;

export function compile_sexpr(source: string): string;

export function compile_verilog(source: string): string;

export function compile_verilog_sat(source: string): string;

export function infer_widths(source: string): string;

export function compile_target(source: string, target: string): string;

export function compile_verilog_with_options(source: string, target: string, dsp_threshold: number, strip_sva: boolean): string;

export function compile_json_netlist(source: string): string;

export function compile_dot_with_detail(source: string, detail_expr: boolean): string;

export function mirr_version(): string;

export function proof_status(): string;

export function simulate_mapek(source: string, ticks: number): string;

export function simulate_rspu(source: string): string;

export function simulate_waveform(source: string, cycles: number): string;

export function wasm_init(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly wasm_init: () => void;
    readonly compile_verilog: (a: number, b: number) => [number, number];
    readonly compile_firrtl: (a: number, b: number) => [number, number];
    readonly compile_sexpr: (a: number, b: number) => [number, number];
    readonly compile_dot: (a: number, b: number) => [number, number];
    readonly compile_rspu: (a: number, b: number) => [number, number];
    readonly compile_verilog_sat: (a: number, b: number) => [number, number];
    readonly simulate_waveform: (a: number, b: number, c: number) => [number, number];
    readonly compile_graph_data: (a: number, b: number) => [number, number];
    readonly infer_widths: (a: number, b: number) => [number, number];
    readonly simulate_rspu: (a: number, b: number) => [number, number];
    readonly simulate_mapek: (a: number, b: number, c: number) => [number, number];
    readonly mirr_version: () => [number, number];
    readonly compile_pipeline_stages: (a: number, b: number) => [number, number];
    readonly proof_status: () => [number, number];
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;

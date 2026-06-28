// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vreflex_soc_4core_synth.h for the primary calling header

#include "Vreflex_soc_4core_synth__pch.h"

VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___eval_static(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval_static\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.__Vtrigprevexpr___TOP__reflex_soc_4core__02Eclk__0 
        = vlSelfRef.reflex_soc_4core__02Eclk;
    vlSelfRef.__Vtrigprevexpr___TOP__rst_n__0 = vlSelfRef.rst_n;
    vlSelfRef.__Vtrigprevexpr___TOP__reflex_soc_4core__DOT__core_top_call_141_gated_clk__0 
        = vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__reflex_soc_4core__DOT__core_top_call_139_gated_clk__0 
        = vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__reflex_soc_4core__DOT__core_top_call_137_gated_clk__0 
        = vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__reflex_soc_4core__DOT__core_top_call_135_gated_clk__0 
        = vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__ram__02Eclk__0 
        = vlSelfRef.ram__02Eclk;
}

VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___eval_initial__TOP(Vreflex_soc_4core_synth___024root* vlSelf);

VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___eval_initial(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval_initial\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    Vreflex_soc_4core_synth___024root___eval_initial__TOP(vlSelf);
}

VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___eval_initial__TOP(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval_initial__TOP\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    IData/*31:0*/ ram__DOT__unnamedblk1__DOT__i;
    ram__DOT__unnamedblk1__DOT__i = 0;
    // Body
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_out = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_out = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_out = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_out = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__robot_angle_d1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angular_velocity_d1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angle_internal_d1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_add = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_invalid = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_relu_neg = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_relu_pos = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_sub = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_tag_gate = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_trap = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_data = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_prov = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_tag = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_add = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_invalid = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_relu_neg = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_relu_pos = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_sub = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_tag_gate = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_trap = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_data = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_prov = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_tag = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_add = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_invalid = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_relu_neg = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_relu_pos = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_sub = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_tag_gate = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_trap = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_data = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_prov = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_tag = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_add = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_invalid = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_relu_neg = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_relu_pos = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_sub = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_tag_gate = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_trap = 0U;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_data = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_prov = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_tag = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__controller_call_133_kd_torque = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__controller_call_133_kp_torque = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__controller_call_133_t_next = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__core_data_0 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__core_data_1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__core_data_2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__core_data_3 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__core_io_in_0 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__core_io_in_1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__core_io_in_2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__core_io_in_3 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_core_awake = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_current_instr = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_gated_clk = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_trap_signal = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_wake_timer = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_core_awake = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_current_instr = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_gated_clk = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_trap_signal = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_wake_timer = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_core_awake = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_current_instr = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_gated_clk = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_trap_signal = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_wake_timer = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_core_awake = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_current_instr = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_gated_clk = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_trap_signal = 0U;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_wake_timer = 0U;
    vlSelfRef.reflex_soc_4core__DOT__downlink_data = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__downlink_valid = 0U;
    vlSelfRef.global_trap = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_payload = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_0 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_10 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_11 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_12 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_13 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_14 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_15 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_16 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_3 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_4 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_5 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_6 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_7 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_8 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_9 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_0 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_1 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_10 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_11 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_12 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_13 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_14 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_15 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_16 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_2 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_3 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_4 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_5 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_6 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_7 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_8 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_9 = 0U;
    vlSelfRef.out_data_0 = 0ULL;
    vlSelfRef.out_data_1 = 0ULL;
    vlSelfRef.out_data_2 = 0ULL;
    vlSelfRef.out_data_3 = 0ULL;
    vlSelfRef.out_pc_0 = 0U;
    vlSelfRef.out_pc_1 = 0U;
    vlSelfRef.out_pc_2 = 0U;
    vlSelfRef.out_pc_3 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__pc_0 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__pc_1 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__pc_2 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__pc_3 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_1377_is_invalid = 0U;
    vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_1879_is_invalid = 0U;
    vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_2381_is_invalid = 0U;
    vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_2883_is_invalid = 0U;
    vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angle_internal = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angular_velocity = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_p_next = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_v_next = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__robot_angle = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__robot_torque = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_cert_g = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_cert_i = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_cert_r = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_op = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_op = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_res = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_d = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_p = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_t = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_p1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_packed = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_t1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_t2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_trap = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_val1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_val2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_if_id_instr = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_load = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_load_in = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_normal_ex = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_not_load = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_res = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_we = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pcc_fault = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pcc_valid = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pipe_pc = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_cert_g = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_cert_i = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_cert_r = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_op = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_op = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_res = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_d = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_p = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_t = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_p1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_packed = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_t1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_t2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_trap = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_val1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_val2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_if_id_instr = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_load = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_load_in = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_normal_ex = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_not_load = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_res = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_we = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_fault = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_valid = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pipe_pc = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_cert_g = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_cert_i = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_cert_r = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_op = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_op = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_res = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_d = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_p = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_t = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_p1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_packed = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_t1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_t2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_trap = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_val1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_val2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_if_id_instr = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_load = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_load_in = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_normal_ex = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_not_load = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_res = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_we = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_fault = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_valid = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_cert_g = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_cert_i = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_cert_r = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_op = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_op = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_res = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_d = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_p = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_t = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_p1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_packed = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_t1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_t2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_trap = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_val1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_val2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_if_id_instr = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_load = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_load_in = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_normal_ex = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_not_load = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_rd = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_res = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_we = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_fault = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_valid = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rx_data_0 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rx_data_1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rx_data_2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rx_data_3 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rx_valid_0 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rx_valid_1 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rx_valid_2 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rx_valid_3 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_0 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_10 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_11 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_12 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_13 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_14 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_15 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_3 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_4 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_5 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_6 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_7 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_8 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_data_9 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_0 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_1 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_10 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_11 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_12 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_13 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_14 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_15 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_2 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_3 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_4 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_5 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_6 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_7 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_8 = 0U;
    vlSelfRef.reflex_soc_4core__DOT__tx_valid_9 = 0U;
    ram__DOT__unnamedblk1__DOT__i = 0U;
    while (VL_GTS_III(32, 0x00004000U, ram__DOT__unnamedblk1__DOT__i)) {
        vlSelfRef.ram__DOT__mem[(0x00003fffU & ram__DOT__unnamedblk1__DOT__i)] = 0ULL;
        ram__DOT__unnamedblk1__DOT__i = ((IData)(1U) 
                                         + ram__DOT__unnamedblk1__DOT__i);
    }
}

VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___eval_final(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval_final\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
}

#ifdef VL_DEBUG
VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___dump_triggers__stl(const VlUnpacked<QData/*63:0*/, 1> &triggers, const std::string &tag);
#endif  // VL_DEBUG
VL_ATTR_COLD bool Vreflex_soc_4core_synth___024root___eval_phase__stl(Vreflex_soc_4core_synth___024root* vlSelf);

VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___eval_settle(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval_settle\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    IData/*31:0*/ __VstlIterCount;
    // Body
    __VstlIterCount = 0U;
    vlSelfRef.__VstlFirstIteration = 1U;
    do {
        if (VL_UNLIKELY(((0x00002710U < __VstlIterCount)))) {
#ifdef VL_DEBUG
            Vreflex_soc_4core_synth___024root___dump_triggers__stl(vlSelfRef.__VstlTriggered, "stl"s);
#endif
            VL_FATAL_MT("reflex_soc/reflex_soc_4core_synth.sv", 15, "", "DIDNOTCONVERGE: Settle region did not converge after '--converge-limit' of 10000 tries");
        }
        __VstlIterCount = ((IData)(1U) + __VstlIterCount);
        vlSelfRef.__VstlPhaseResult = Vreflex_soc_4core_synth___024root___eval_phase__stl(vlSelf);
        vlSelfRef.__VstlFirstIteration = 0U;
    } while (vlSelfRef.__VstlPhaseResult);
}

VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___eval_triggers_vec__stl(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval_triggers_vec__stl\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.__VstlTriggered[0U] = ((0xfffffffffffffffeULL 
                                      & vlSelfRef.__VstlTriggered[0U]) 
                                     | (IData)((IData)(vlSelfRef.__VstlFirstIteration)));
}

VL_ATTR_COLD bool Vreflex_soc_4core_synth___024root___trigger_anySet__stl(const VlUnpacked<QData/*63:0*/, 1> &in);

#ifdef VL_DEBUG
VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___dump_triggers__stl(const VlUnpacked<QData/*63:0*/, 1> &triggers, const std::string &tag) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___dump_triggers__stl\n"); );
    // Body
    if ((1U & (~ (IData)(Vreflex_soc_4core_synth___024root___trigger_anySet__stl(triggers))))) {
        VL_DBG_MSGS("         No '" + tag + "' region triggers active\n");
    }
    if ((1U & (IData)(triggers[0U]))) {
        VL_DBG_MSGS("         '" + tag + "' region trigger index 0 is active: Internal 'stl' trigger - first iteration\n");
    }
}
#endif  // VL_DEBUG

VL_ATTR_COLD bool Vreflex_soc_4core_synth___024root___trigger_anySet__stl(const VlUnpacked<QData/*63:0*/, 1> &in) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___trigger_anySet__stl\n"); );
    // Locals
    IData/*31:0*/ n;
    // Body
    n = 0U;
    do {
        if (in[n]) {
            return (1U);
        }
        n = ((IData)(1U) + n);
    } while ((1U > n));
    return (0U);
}

VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___stl_sequent__TOP__0(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___stl_sequent__TOP__0\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val1 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs1))];
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val2 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs2))];
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val1 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs1))];
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val2 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs2))];
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val1 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs1))];
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val2 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs2))];
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val1 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs1))];
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val2 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs2))];
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p0_alive_out 
        = ((IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_noc_l1_router_0_call_212_36410d6f6db9a7fc_out) 
           & (IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_not_noc_l1_router_0_call_76b72c6eee8e20f4_out));
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p1_alive_out 
        = ((IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_noc_l1_router_0_call_212_8cdce43d69b3cb1a_out) 
           & (IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_not_noc_l1_router_0_call_7d246c2b8ee52083_out));
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p2_alive_out 
        = ((IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_noc_l1_router_0_call_212_6b95589e1cc0689a_out) 
           & (IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_not_noc_l1_router_0_call_113ae69a62f8f2c8_out));
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p3_alive_out 
        = ((IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_noc_l1_router_0_call_212_a28addd5ea405345_out) 
           & (IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_not_noc_l1_router_0_call_af3f6c2b2ccd2e62_out));
}

VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___eval_stl(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval_stl\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if ((1ULL & vlSelfRef.__VstlTriggered[0U])) {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val1 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs1))];
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val2 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs2))];
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val1 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs1))];
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val2 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs2))];
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val1 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs1))];
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val2 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs2))];
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val1 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs1))];
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val2 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs2))];
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p0_alive_out 
            = ((IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_noc_l1_router_0_call_212_36410d6f6db9a7fc_out) 
               & (IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_not_noc_l1_router_0_call_76b72c6eee8e20f4_out));
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p1_alive_out 
            = ((IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_noc_l1_router_0_call_212_8cdce43d69b3cb1a_out) 
               & (IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_not_noc_l1_router_0_call_7d246c2b8ee52083_out));
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p2_alive_out 
            = ((IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_noc_l1_router_0_call_212_6b95589e1cc0689a_out) 
               & (IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_not_noc_l1_router_0_call_113ae69a62f8f2c8_out));
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p3_alive_out 
            = ((IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_noc_l1_router_0_call_212_a28addd5ea405345_out) 
               & (IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_not_noc_l1_router_0_call_af3f6c2b2ccd2e62_out));
    }
}

VL_ATTR_COLD bool Vreflex_soc_4core_synth___024root___eval_phase__stl(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval_phase__stl\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    CData/*0:0*/ __VstlExecute;
    // Body
    Vreflex_soc_4core_synth___024root___eval_triggers_vec__stl(vlSelf);
#ifdef VL_DEBUG
    if (VL_UNLIKELY(vlSymsp->_vm_contextp__->debug())) {
        Vreflex_soc_4core_synth___024root___dump_triggers__stl(vlSelfRef.__VstlTriggered, "stl"s);
    }
#endif
    __VstlExecute = Vreflex_soc_4core_synth___024root___trigger_anySet__stl(vlSelfRef.__VstlTriggered);
    if (__VstlExecute) {
        Vreflex_soc_4core_synth___024root___eval_stl(vlSelf);
    }
    return (__VstlExecute);
}

bool Vreflex_soc_4core_synth___024root___trigger_anySet__act(const VlUnpacked<QData/*63:0*/, 1> &in);

#ifdef VL_DEBUG
VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___dump_triggers__act(const VlUnpacked<QData/*63:0*/, 1> &triggers, const std::string &tag) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___dump_triggers__act\n"); );
    // Body
    if ((1U & (~ (IData)(Vreflex_soc_4core_synth___024root___trigger_anySet__act(triggers))))) {
        VL_DBG_MSGS("         No '" + tag + "' region triggers active\n");
    }
    if ((1U & (IData)(triggers[0U]))) {
        VL_DBG_MSGS("         '" + tag + "' region trigger index 0 is active: @(posedge reflex_soc_4core.clk)\n");
    }
    if ((1U & (IData)((triggers[0U] >> 1U)))) {
        VL_DBG_MSGS("         '" + tag + "' region trigger index 1 is active: @(negedge rst_n)\n");
    }
    if ((1U & (IData)((triggers[0U] >> 2U)))) {
        VL_DBG_MSGS("         '" + tag + "' region trigger index 2 is active: @(posedge reflex_soc_4core.core_top_call_141_gated_clk)\n");
    }
    if ((1U & (IData)((triggers[0U] >> 3U)))) {
        VL_DBG_MSGS("         '" + tag + "' region trigger index 3 is active: @(posedge reflex_soc_4core.core_top_call_139_gated_clk)\n");
    }
    if ((1U & (IData)((triggers[0U] >> 4U)))) {
        VL_DBG_MSGS("         '" + tag + "' region trigger index 4 is active: @(posedge reflex_soc_4core.core_top_call_137_gated_clk)\n");
    }
    if ((1U & (IData)((triggers[0U] >> 5U)))) {
        VL_DBG_MSGS("         '" + tag + "' region trigger index 5 is active: @(posedge reflex_soc_4core.core_top_call_135_gated_clk)\n");
    }
    if ((1U & (IData)((triggers[0U] >> 6U)))) {
        VL_DBG_MSGS("         '" + tag + "' region trigger index 6 is active: @(posedge ram.clk)\n");
    }
}
#endif  // VL_DEBUG

VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___ctor_var_reset(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___ctor_var_reset\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    const uint64_t __VscopeHash = VL_MURMUR64_HASH(vlSelf->vlNamep);
    vlSelf->reflex_soc_4core__02Eclk = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 10961058721943773748ull);
    vlSelf->rst_n = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 1638864771569018232ull);
    vlSelf->out_pc_0 = VL_SCOPED_RAND_RESET_I(32, __VscopeHash, 90854345563195349ull);
    vlSelf->out_data_0 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 5734350766571490266ull);
    vlSelf->out_pc_1 = VL_SCOPED_RAND_RESET_I(32, __VscopeHash, 11226121962894326539ull);
    vlSelf->out_data_1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10715059316092855017ull);
    vlSelf->out_pc_2 = VL_SCOPED_RAND_RESET_I(32, __VscopeHash, 5743479028646143404ull);
    vlSelf->out_data_2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 1480862724489203751ull);
    vlSelf->out_pc_3 = VL_SCOPED_RAND_RESET_I(32, __VscopeHash, 14066912936662077788ull);
    vlSelf->out_data_3 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 6168677135529287151ull);
    vlSelf->global_trap = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 286812283172483895ull);
    vlSelf->ram__02Eclk = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 15757265750297096343ull);
    vlSelf->addr = VL_SCOPED_RAND_RESET_I(14, __VscopeHash, 14934084843038794831ull);
    vlSelf->din = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 15192908731043726583ull);
    vlSelf->dout = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 11474705599699299244ull);
    vlSelf->reflex_soc_4core__DOT__pc_0 = VL_SCOPED_RAND_RESET_I(32, __VscopeHash, 15240564445303071728ull);
    vlSelf->reflex_soc_4core__DOT__core_data_0 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 12632950764867285247ull);
    vlSelf->reflex_soc_4core__DOT__core_io_in_0 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 1785226219335951086ull);
    vlSelf->reflex_soc_4core__DOT__pc_1 = VL_SCOPED_RAND_RESET_I(32, __VscopeHash, 14317033107495498782ull);
    vlSelf->reflex_soc_4core__DOT__core_data_1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 14914772192553746045ull);
    vlSelf->reflex_soc_4core__DOT__core_io_in_1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 17538917461782246627ull);
    vlSelf->reflex_soc_4core__DOT__pc_2 = VL_SCOPED_RAND_RESET_I(32, __VscopeHash, 17839723378499481340ull);
    vlSelf->reflex_soc_4core__DOT__core_data_2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 15713004369662751037ull);
    vlSelf->reflex_soc_4core__DOT__core_io_in_2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 1390808046228181029ull);
    vlSelf->reflex_soc_4core__DOT__pc_3 = VL_SCOPED_RAND_RESET_I(32, __VscopeHash, 14905654894078102481ull);
    vlSelf->reflex_soc_4core__DOT__core_data_3 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 662302034557715163ull);
    vlSelf->reflex_soc_4core__DOT__core_io_in_3 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 8312442098623499932ull);
    vlSelf->reflex_soc_4core__DOT__rx_valid_0 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 17555867910013880345ull);
    vlSelf->reflex_soc_4core__DOT__rx_data_0 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 3062802694231980282ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_0 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 9830950901083088774ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_0 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 17608114223333036138ull);
    vlSelf->reflex_soc_4core__DOT__rx_valid_1 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5282033865040105026ull);
    vlSelf->reflex_soc_4core__DOT__rx_data_1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 14210896659692160275ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_1 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 9972462159388412002ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 15327819742088107456ull);
    vlSelf->reflex_soc_4core__DOT__rx_valid_2 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 6587064116431719601ull);
    vlSelf->reflex_soc_4core__DOT__rx_data_2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 4697729720813777845ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_2 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5292800902184784331ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 9609525777112650366ull);
    vlSelf->reflex_soc_4core__DOT__rx_valid_3 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 4385493156689896999ull);
    vlSelf->reflex_soc_4core__DOT__rx_data_3 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 8215734706567544147ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_3 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 16221988846189596009ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_3 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 5503378903331113115ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_4 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 11939955249604797874ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_4 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 13140709212927744528ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_5 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 1293789822210592409ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_5 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 9326714124420654276ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_6 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 10422063384384533472ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_6 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 16208979900597876547ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_7 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 16772837482706875229ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_7 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 9363594791277279403ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_8 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5690830821828156298ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_8 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 2424531640666246109ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_9 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 412529049720301157ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_9 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 3964300044471579469ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_10 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 11234979826913786418ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_10 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 18413666607061697296ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_11 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 11301647485154997760ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_11 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 13219555656680712924ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_12 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 4002532207180717491ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_12 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 2987687530404425599ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_13 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 4189707692000820677ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_13 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 3590002867925010220ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_14 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 4494562098044195213ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_14 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 4129648650223164949ull);
    vlSelf->reflex_soc_4core__DOT__tx_valid_15 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 18191371395879840268ull);
    vlSelf->reflex_soc_4core__DOT__tx_data_15 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 6856188276445751705ull);
    vlSelf->reflex_soc_4core__DOT__robot_angle = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 4872976988072947018ull);
    vlSelf->reflex_soc_4core__DOT__robot_torque = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 5633053647694841877ull);
    vlSelf->reflex_soc_4core__DOT__downlink_valid = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 10949746367698326923ull);
    vlSelf->reflex_soc_4core__DOT__downlink_data = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 12914821147275515354ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_0 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 6901362166541195737ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_0 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 18397531875717187353ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_1 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 15508994822865902251ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 12387954152337326905ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_2 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 1253854810455473787ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 8920490825626457314ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_3 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 9651005249648380897ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_3 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 6529964579119912304ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_4 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 3641427526268567137ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_4 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 8765949144770665882ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_5 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 13536479479929255167ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_5 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10718899422072603801ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_6 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 2337396693991248861ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_6 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 17663100097172265331ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_7 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 6529000755290328609ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_7 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 14195636770461387735ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_8 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 1058047619850540459ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_8 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 6041493876337983514ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_9 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 13798674582851851031ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_9 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 1622796062965909747ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_10 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 9078808130197343593ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_10 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 9219677549253647671ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_11 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 17471502236807314253ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_11 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 17497942160568222474ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_12 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 7405666669230870133ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_12 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 13584084079804811582ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_13 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 15683931280545477782ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_13 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 3415604617410046665ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_14 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 12109367726432889687ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_14 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 12135807650194051740ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_15 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 4959662259797030579ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_15 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 11138079670371016316ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_16 = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 10321796770171321613ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_16 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 16500214180745414231ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_payload = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 4354332825205273862ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_141_trap_signal = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 3264472332594746973ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_141_current_instr = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 2554611098125636611ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_141_wake_timer = VL_SCOPED_RAND_RESET_I(3, __VscopeHash, 2088998191693381883ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_141_core_awake = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 1741982986163460885ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_141_gated_clk = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 15358925604512451356ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_if_id_instr = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 15709643830046182399ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 15916351550504056820ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_val1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 5259668104142041657ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_val2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 16404686583588175466ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 12749738561864213151ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 553038724419036700ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_res = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 16385228635840969416ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_op = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 16396609587549004736ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 12379948471270178994ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_res = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 15506624572374903933ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_we = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 11433092316131445505ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_pipe_pc = VL_SCOPED_RAND_RESET_I(32, __VscopeHash, 7180322918514396016ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_pcc_valid = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 8932839262867513211ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_pcc_fault = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 11701890077498520533ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_cert_i = VL_SCOPED_RAND_RESET_I(16, __VscopeHash, 191141489220206577ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_cert_r = VL_SCOPED_RAND_RESET_I(16, __VscopeHash, 3322589549667325873ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_cert_g = VL_SCOPED_RAND_RESET_I(16, __VscopeHash, 5651391217761008840ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 15321828306156618759ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 1263087540076239121ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 139619083588779868ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 6757643656538634156ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 8287978261884110320ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_op = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 12421796111698466898ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 9204022609881636064ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_t1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 8696530185546004513ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_p1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 4013887116941200505ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 15471540002370388979ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_t2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 1938600672657628989ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_d = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 17696411505826803539ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_t = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 2950283014034780189ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_p = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 9255607797138963492ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_trap = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 16735812005802841332ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_packed = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10144825662141477158ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_load_in = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 4113623498978679627ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_normal_ex = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 8100303008692133719ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_out = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 11055388927981149222ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_load = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 4932370782610881016ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_not_load = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 2964602166861464246ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1393_res_data = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 3979377604078391664ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1393_res_tag = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 12061334361043148482ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1393_res_prov = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 9541685178213432191ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1393_is_trap = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 6247807701899444415ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1393_is_add = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 2067050333896361867ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1393_is_sub = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 11619053041376675044ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1393_is_relu_pos = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 717285936766462994ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1393_is_relu_neg = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 655080304898988513ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1393_is_tag_gate = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 9768381991948141061ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1393_is_invalid = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 16768945405254223112ull);
    vlSelf->reflex_soc_4core__DOT__pcc_verifier_call_1377_is_invalid = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 3283821897282288489ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_139_trap_signal = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 574131531875969004ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_139_current_instr = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 27643153376713057ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_139_wake_timer = VL_SCOPED_RAND_RESET_I(3, __VscopeHash, 15046096571185816391ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_139_core_awake = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 6196807341174675736ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_139_gated_clk = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 6769713225081363889ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_if_id_instr = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 6665991210362515702ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 2854980959767199534ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_val1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10197465377075490485ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_val2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10139732355842777256ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 3464107266446036713ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 11688045467292746067ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_res = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 2760522880646300086ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_op = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10942613384992305919ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 1950754543298660764ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_res = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 1055115652386672054ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_we = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 7999417113396982715ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_pipe_pc = VL_SCOPED_RAND_RESET_I(32, __VscopeHash, 16164169162816120261ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_valid = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 1549984455345235166ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_fault = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 2630703957211854147ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_cert_i = VL_SCOPED_RAND_RESET_I(16, __VscopeHash, 10683876736983199464ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_cert_r = VL_SCOPED_RAND_RESET_I(16, __VscopeHash, 11489083322614905753ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_cert_g = VL_SCOPED_RAND_RESET_I(16, __VscopeHash, 10087336794254107375ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 15856423816356530954ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 16946881652714547887ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 14216136947907364055ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 5170585825556473643ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 3825752057992146249ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_op = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 17506805502287138300ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10945791976815419185ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_t1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 7571909746132651817ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_p1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 16244351318731686362ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 17505466824943218210ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_t2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 9913033150134138754ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_d = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 8907937764083467884ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_t = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 3123845666370617423ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_p = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 17209999827059819106ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_trap = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 17632750149923851924ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_packed = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 14014969915151655737ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_load_in = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5977648489007412705ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_normal_ex = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 6989013768762201147ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_out = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 13884057538339775024ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_load = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 8405398875054730312ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_not_load = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 8602996185320454592ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1895_res_data = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 5276155880835581357ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1895_res_tag = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 7472377610846361761ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1895_res_prov = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 2996179511271501147ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1895_is_trap = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 6130832857347492473ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1895_is_add = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5088160495709116758ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1895_is_sub = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5386289577659176882ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1895_is_relu_pos = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 12606599430718132283ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1895_is_relu_neg = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 12570105428033871585ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1895_is_tag_gate = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 12112847031055094815ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_1895_is_invalid = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 16960304173882809325ull);
    vlSelf->reflex_soc_4core__DOT__pcc_verifier_call_1879_is_invalid = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 8437053546416173936ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_137_trap_signal = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 11402020503442571277ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_137_current_instr = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 12785334710251752424ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_137_wake_timer = VL_SCOPED_RAND_RESET_I(3, __VscopeHash, 17611146006550284964ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_137_core_awake = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 7752856539271100297ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_137_gated_clk = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 1314569136499176592ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_if_id_instr = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 16226266879786018106ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 9790864791339265202ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_val1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 11108005324458865887ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_val2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 3434165664297340901ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 7295923450495547159ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 13019180993931219373ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_res = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 11295005436198801076ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_op = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10992822688078131957ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 502526290547926502ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_res = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 2442120946937336050ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_we = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5600573390086344808ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc = VL_SCOPED_RAND_RESET_I(32, __VscopeHash, 11824135597423821982ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_valid = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 14471687323342428965ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_fault = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 13642316121311668616ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_cert_i = VL_SCOPED_RAND_RESET_I(16, __VscopeHash, 16079273573520375034ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_cert_r = VL_SCOPED_RAND_RESET_I(16, __VscopeHash, 17753815905631150390ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_cert_g = VL_SCOPED_RAND_RESET_I(16, __VscopeHash, 409589469667986266ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 7137856056401319532ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 11856254272161011584ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 8462001746698543142ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 736577469413613848ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 260448246156666549ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_op = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 18022760309820932737ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 7520008286583867010ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_t1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 411335522419666771ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_p1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 5464487737652044159ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 7599083306426832935ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_t2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 13654671157873972274ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_d = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 9955932166383399687ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_t = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 7690101676016672980ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_p = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 9925442524902885378ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_trap = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 17373863096685987748ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_packed = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 11752675785818272906ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_load_in = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 8293217066819686593ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_normal_ex = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5723496926557335867ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_out = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 16920101884698698030ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_load = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 9189446277781001788ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_not_load = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 1651561705386292258ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2397_res_data = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 9105838925899540984ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2397_res_tag = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 6370805963900156161ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2397_res_prov = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 11368031645640205432ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2397_is_trap = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 7636995247628043664ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2397_is_add = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 6442809869753393472ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2397_is_sub = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 4088358372469434442ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2397_is_relu_pos = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 18329601283783020436ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2397_is_relu_neg = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 14039809959805148457ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2397_is_tag_gate = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 11268613307553472703ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2397_is_invalid = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 15540423224142671860ull);
    vlSelf->reflex_soc_4core__DOT__pcc_verifier_call_2381_is_invalid = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 18311781727577842272ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_135_trap_signal = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 16591193497756343005ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_135_current_instr = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 7545771688811447060ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_135_wake_timer = VL_SCOPED_RAND_RESET_I(3, __VscopeHash, 12479875845752029550ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_135_core_awake = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5611503671646025303ull);
    vlSelf->reflex_soc_4core__DOT__core_top_call_135_gated_clk = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 625252266232678547ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_if_id_instr = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 13184827805667779991ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 17904814497162616824ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_val1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10135296447702150668ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_val2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 4726356348395575986ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 1054612531507075500ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 5788169797518865638ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_res = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 12193924931005673505ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_op = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10834558744738573220ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10133077396205684183ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_res = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 12488821579007886261ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_we = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 15928094196917820417ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc = VL_SCOPED_RAND_RESET_I(32, __VscopeHash, 5513127010442896863ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_valid = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5485339077361747491ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_fault = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 18083722170236760759ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_cert_i = VL_SCOPED_RAND_RESET_I(16, __VscopeHash, 9513340997150988366ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_cert_r = VL_SCOPED_RAND_RESET_I(16, __VscopeHash, 1709138771178974036ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_cert_g = VL_SCOPED_RAND_RESET_I(16, __VscopeHash, 15725509703541787094ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 8897464146907147660ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 13370588916808858825ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 1496629150812065345ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 7460921701875484116ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rd = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 11697837952796031526ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_op = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 11748215375879759392ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10191146472429737321ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_t1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 11641815822587497899ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_p1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10087782051671990853ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 17070638646113847504ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_t2 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 2198788211263875406ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_d = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 16137874164599747634ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_t = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10036432823663034288ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_p = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 18054505556807976872ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_trap = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 15078420575863834246ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_packed = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 206017921191288626ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_load_in = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 18041465382820877343ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_normal_ex = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5546758189006110033ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_out = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 10759092430399758979ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_load = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 17457848980042508684ull);
    vlSelf->reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_not_load = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 7900028587248054511ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2899_res_data = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 3401643858800090097ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2899_res_tag = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 17224131246085837269ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2899_res_prov = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 6858773659826716045ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2899_is_trap = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 7557011737791593384ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2899_is_add = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 6535626981120395498ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2899_is_sub = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 8029808502921571778ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2899_is_relu_pos = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5924003984954314423ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2899_is_relu_neg = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 17661832788584874033ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2899_is_tag_gate = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 7249216249804201086ull);
    vlSelf->reflex_soc_4core__DOT__alu_core_call_2899_is_invalid = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 14869413050260197886ull);
    vlSelf->reflex_soc_4core__DOT__pcc_verifier_call_2883_is_invalid = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 2520417942173361143ull);
    vlSelf->reflex_soc_4core__DOT__controller_call_133_kp_torque = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 16623813481297982147ull);
    vlSelf->reflex_soc_4core__DOT__controller_call_133_kd_torque = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 3587174472674157479ull);
    vlSelf->reflex_soc_4core__DOT__controller_call_133_t_next = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 16147661996296042902ull);
    vlSelf->reflex_soc_4core__DOT__pendulum_call_131_angular_velocity = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 125624881564630423ull);
    vlSelf->reflex_soc_4core__DOT__pendulum_call_131_angle_internal = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 5301568101010633425ull);
    vlSelf->reflex_soc_4core__DOT__pendulum_call_131_v_next = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 2466665555185193743ull);
    vlSelf->reflex_soc_4core__DOT__pendulum_call_131_p_next = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 16793068543390604727ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_noc_l1_router_0_call_212_36410d6f6db9a7fc_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 3584479172504421531ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_not_noc_l1_router_0_call_76b72c6eee8e20f4_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 6253761111254438754ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_p0_alive_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 2470580321550768981ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_noc_l1_router_0_call_212_8cdce43d69b3cb1a_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 7471834296438457501ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_not_noc_l1_router_0_call_7d246c2b8ee52083_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 7213622340218233544ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_p1_alive_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 4293712370793749093ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_noc_l1_router_0_call_212_6b95589e1cc0689a_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 9024740203014149993ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_not_noc_l1_router_0_call_113ae69a62f8f2c8_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 11727912645150197353ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_p2_alive_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 483831418419824143ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_noc_l1_router_0_call_212_a28addd5ea405345_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 12851932623184328868ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_not_noc_l1_router_0_call_af3f6c2b2ccd2e62_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 777674835868945628ull);
    vlSelf->reflex_soc_4core__DOT__noc_l1_router_0_call_212_p3_alive_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5516781448086508851ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_not_rx_valid_3_f2852242f2f15204_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 16279756854905109922ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_core_top_call_141_wake_t_bc709c7e89383f58_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 7419446142952867225ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_not_rx_valid_2_173fdd08fddcea5a_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 10957345222871966766ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_core_top_call_139_wake_t_5b097111c566bad0_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 5343137619603645250ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_not_rx_valid_1_11d0544cf873038c_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 1863852476288863495ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_core_top_call_137_wake_t_46921745bbd4f73c_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 14457738001026846430ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_not_rx_valid_0_8f5789eac3a38db5_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 16018546929047941578ull);
    vlSelf->reflex_soc_4core__DOT__sub_g_core_top_call_135_wake_t_1c9a211eeeb38950_out = VL_SCOPED_RAND_RESET_I(1, __VscopeHash, 8367242149160248693ull);
    vlSelf->reflex_soc_4core__DOT__robot_angle_d1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 5107168855480192312ull);
    vlSelf->reflex_soc_4core__DOT__pendulum_call_131_angular_velocity_d1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 12589799638434411261ull);
    vlSelf->reflex_soc_4core__DOT__pendulum_call_131_angle_internal_d1 = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 3410565497771461553ull);
    for (int __Vi0 = 0; __Vi0 < 1024; ++__Vi0) {
        vlSelf->reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs[__Vi0] = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 1844477931145674577ull);
    }
    for (int __Vi0 = 0; __Vi0 < 1024; ++__Vi0) {
        vlSelf->reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs[__Vi0] = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 18218361318289629552ull);
    }
    for (int __Vi0 = 0; __Vi0 < 1024; ++__Vi0) {
        vlSelf->reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs[__Vi0] = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 17251828647771986857ull);
    }
    for (int __Vi0 = 0; __Vi0 < 1024; ++__Vi0) {
        vlSelf->reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs[__Vi0] = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 2469395899331658494ull);
    }
    for (int __Vi0 = 0; __Vi0 < 16384; ++__Vi0) {
        vlSelf->ram__DOT__mem[__Vi0] = VL_SCOPED_RAND_RESET_Q(64, __VscopeHash, 2202823806106347730ull);
    }
    vlSelf->__Vdly__reflex_soc_4core__DOT__core_top_call_135_current_instr = 0;
    vlSelf->__Vdly__reflex_soc_4core__DOT__core_top_call_137_current_instr = 0;
    vlSelf->__Vdly__reflex_soc_4core__DOT__core_top_call_139_current_instr = 0;
    vlSelf->__Vdly__reflex_soc_4core__DOT__core_top_call_141_current_instr = 0;
    vlSelf->__Vdly__reflex_soc_4core__DOT__pendulum_call_131_p_next = 0;
    for (int __Vi0 = 0; __Vi0 < 1; ++__Vi0) {
        vlSelf->__VstlTriggered[__Vi0] = 0;
    }
    for (int __Vi0 = 0; __Vi0 < 1; ++__Vi0) {
        vlSelf->__VactTriggered[__Vi0] = 0;
    }
    vlSelf->__Vtrigprevexpr___TOP__reflex_soc_4core__02Eclk__0 = 0;
    vlSelf->__Vtrigprevexpr___TOP__rst_n__0 = 0;
    vlSelf->__Vtrigprevexpr___TOP__reflex_soc_4core__DOT__core_top_call_141_gated_clk__0 = 0;
    vlSelf->__Vtrigprevexpr___TOP__reflex_soc_4core__DOT__core_top_call_139_gated_clk__0 = 0;
    vlSelf->__Vtrigprevexpr___TOP__reflex_soc_4core__DOT__core_top_call_137_gated_clk__0 = 0;
    vlSelf->__Vtrigprevexpr___TOP__reflex_soc_4core__DOT__core_top_call_135_gated_clk__0 = 0;
    vlSelf->__Vtrigprevexpr___TOP__ram__02Eclk__0 = 0;
    for (int __Vi0 = 0; __Vi0 < 1; ++__Vi0) {
        vlSelf->__VnbaTriggered[__Vi0] = 0;
    }
}

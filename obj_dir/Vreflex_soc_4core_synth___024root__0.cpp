// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vreflex_soc_4core_synth.h for the primary calling header

#include "Vreflex_soc_4core_synth__pch.h"

void Vreflex_soc_4core_synth___024root___eval_triggers_vec__act(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval_triggers_vec__act\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.__VactTriggered[0U] = (QData)((IData)(
                                                    (((((IData)(vlSelfRef.ram__02Eclk) 
                                                        & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__ram__02Eclk__0))) 
                                                       << 6U) 
                                                      | ((((IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_gated_clk) 
                                                           & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__reflex_soc_4core__DOT__core_top_call_135_gated_clk__0))) 
                                                          << 5U) 
                                                         | (((IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_gated_clk) 
                                                             & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__reflex_soc_4core__DOT__core_top_call_137_gated_clk__0))) 
                                                            << 4U))) 
                                                     | (((((IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_gated_clk) 
                                                           & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__reflex_soc_4core__DOT__core_top_call_139_gated_clk__0))) 
                                                          << 3U) 
                                                         | (((IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_gated_clk) 
                                                             & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__reflex_soc_4core__DOT__core_top_call_141_gated_clk__0))) 
                                                            << 2U)) 
                                                        | ((((~ (IData)(vlSelfRef.rst_n)) 
                                                             & (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rst_n__0)) 
                                                            << 1U) 
                                                           | ((IData)(vlSelfRef.reflex_soc_4core__02Eclk) 
                                                              & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__reflex_soc_4core__02Eclk__0))))))));
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

bool Vreflex_soc_4core_synth___024root___trigger_anySet__act(const VlUnpacked<QData/*63:0*/, 1> &in) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___trigger_anySet__act\n"); );
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

void Vreflex_soc_4core_synth___024root___nba_sequent__TOP__0(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___nba_sequent__TOP__0\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__ram__DOT__mem__v0;
    __VdlyVal__ram__DOT__mem__v0 = 0;
    SData/*13:0*/ __VdlyDim0__ram__DOT__mem__v0;
    __VdlyDim0__ram__DOT__mem__v0 = 0;
    // Body
    __VdlyVal__ram__DOT__mem__v0 = vlSelfRef.din;
    __VdlyDim0__ram__DOT__mem__v0 = vlSelfRef.addr;
    vlSelfRef.dout = vlSelfRef.ram__DOT__mem[vlSelfRef.addr];
    vlSelfRef.ram__DOT__mem[__VdlyDim0__ram__DOT__mem__v0] 
        = __VdlyVal__ram__DOT__mem__v0;
}

void Vreflex_soc_4core_synth___024root___nba_sequent__TOP__1(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___nba_sequent__TOP__1\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __Vdly__reflex_soc_4core__DOT__pendulum_call_131_angular_velocity_d1;
    __Vdly__reflex_soc_4core__DOT__pendulum_call_131_angular_velocity_d1 = 0;
    CData/*2:0*/ __Vdly__reflex_soc_4core__DOT__core_top_call_135_wake_timer;
    __Vdly__reflex_soc_4core__DOT__core_top_call_135_wake_timer = 0;
    CData/*2:0*/ __Vdly__reflex_soc_4core__DOT__core_top_call_137_wake_timer;
    __Vdly__reflex_soc_4core__DOT__core_top_call_137_wake_timer = 0;
    CData/*2:0*/ __Vdly__reflex_soc_4core__DOT__core_top_call_139_wake_timer;
    __Vdly__reflex_soc_4core__DOT__core_top_call_139_wake_timer = 0;
    CData/*2:0*/ __Vdly__reflex_soc_4core__DOT__core_top_call_141_wake_timer;
    __Vdly__reflex_soc_4core__DOT__core_top_call_141_wake_timer = 0;
    IData/*31:0*/ __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_1175_pipe_pc;
    __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_1175_pipe_pc = 0;
    IData/*31:0*/ __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_1677_pipe_pc;
    __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_1677_pipe_pc = 0;
    IData/*31:0*/ __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc;
    __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc = 0;
    IData/*31:0*/ __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc;
    __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc = 0;
    // Body
    __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_1175_pipe_pc 
        = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pipe_pc;
    __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_1677_pipe_pc 
        = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pipe_pc;
    __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc 
        = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc;
    __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc 
        = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc;
    __Vdly__reflex_soc_4core__DOT__pendulum_call_131_angular_velocity_d1 
        = vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angular_velocity_d1;
    vlSelfRef.__Vdly__reflex_soc_4core__DOT__pendulum_call_131_p_next 
        = vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_p_next;
    __Vdly__reflex_soc_4core__DOT__core_top_call_135_wake_timer 
        = vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_wake_timer;
    __Vdly__reflex_soc_4core__DOT__core_top_call_137_wake_timer 
        = vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_wake_timer;
    __Vdly__reflex_soc_4core__DOT__core_top_call_139_wake_timer 
        = vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_wake_timer;
    __Vdly__reflex_soc_4core__DOT__core_top_call_141_wake_timer 
        = vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_wake_timer;
    vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_135_current_instr 
        = vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_current_instr;
    vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_137_current_instr 
        = vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_current_instr;
    vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_139_current_instr 
        = vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_current_instr;
    vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_141_current_instr 
        = vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_current_instr;
    vlSelfRef.global_trap = ((IData)(vlSelfRef.rst_n) 
                             && ((((IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_0) 
                                   | (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_1)) 
                                  | (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_2)) 
                                 | (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_3)));
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.reflex_soc_4core__02Eclk) 
                                        & (IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_core_awake)));
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.reflex_soc_4core__02Eclk) 
                                        & (IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_core_awake)));
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.reflex_soc_4core__02Eclk) 
                                        & (IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_core_awake)));
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.reflex_soc_4core__02Eclk) 
                                        & (IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_core_awake)));
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_wake_timer)) 
                                        | (IData)(vlSelfRef.reflex_soc_4core__DOT__rx_valid_0)));
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_wake_timer)) 
                                        | (IData)(vlSelfRef.reflex_soc_4core__DOT__rx_valid_1)));
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_wake_timer)) 
                                        | (IData)(vlSelfRef.reflex_soc_4core__DOT__rx_valid_2)));
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_wake_timer)) 
                                        | (IData)(vlSelfRef.reflex_soc_4core__DOT__rx_valid_3)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pcc_valid) {
            __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_1175_pipe_pc 
                = ((IData)(1U) + vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pipe_pc);
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_if_id_instr, 0x00000030U));
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_valid) {
            __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_1677_pipe_pc 
                = ((IData)(1U) + vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pipe_pc);
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_if_id_instr, 0x00000030U));
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_valid) {
            __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc 
                = ((IData)(1U) + vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc);
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_if_id_instr, 0x00000030U));
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_valid) {
            __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc 
                = ((IData)(1U) + vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc);
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_if_id_instr, 0x00000030U));
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_if_id_instr, 0x00000020U));
        }
        __Vdly__reflex_soc_4core__DOT__pendulum_call_131_angular_velocity_d1 
            = vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angular_velocity;
        if ((1U & (~ (IData)(vlSelfRef.rst_n)))) {
            vlSelfRef.__Vdly__reflex_soc_4core__DOT__pendulum_call_131_p_next = 0ULL;
        }
        if (vlSelfRef.rst_n) {
            vlSelfRef.__Vdly__reflex_soc_4core__DOT__pendulum_call_131_p_next 
                = (vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angle_internal_d1 
                   + VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angular_velocity, 8U));
        }
        if ((1U & (~ (IData)(vlSelfRef.rst_n)))) {
            vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angular_velocity = 0ULL;
        }
        if (vlSelfRef.rst_n) {
            vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angular_velocity 
                = vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_v_next;
        }
        if ((1U & (~ (IData)(vlSelfRef.rst_n)))) {
            vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_v_next = 0ULL;
        }
        if (vlSelfRef.rst_n) {
            vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_v_next 
                = (vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angular_velocity_d1 
                   + (VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__robot_torque, 8U) 
                      - VL_MULS_QQQ(64, 0xfffffffffffffff6ULL, 
                                    VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angle_internal_d1, 8U))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rst_n)))) {
            vlSelfRef.reflex_soc_4core__DOT__robot_torque = 0ULL;
        }
        if (vlSelfRef.rst_n) {
            vlSelfRef.reflex_soc_4core__DOT__robot_torque 
                = vlSelfRef.reflex_soc_4core__DOT__controller_call_133_t_next;
        }
        if ((1U & (~ (IData)(vlSelfRef.rst_n)))) {
            vlSelfRef.reflex_soc_4core__DOT__controller_call_133_t_next = 0ULL;
        }
        if (vlSelfRef.rst_n) {
            vlSelfRef.reflex_soc_4core__DOT__controller_call_133_t_next 
                = (vlSelfRef.reflex_soc_4core__DOT__controller_call_133_kp_torque 
                   + vlSelfRef.reflex_soc_4core__DOT__controller_call_133_kd_torque);
        }
        if ((1U & (~ (IData)(vlSelfRef.rst_n)))) {
            vlSelfRef.reflex_soc_4core__DOT__controller_call_133_kp_torque = 0ULL;
            vlSelfRef.reflex_soc_4core__DOT__controller_call_133_kd_torque = 0ULL;
        }
        if (vlSelfRef.rst_n) {
            vlSelfRef.reflex_soc_4core__DOT__controller_call_133_kp_torque 
                = VL_SHIFTR_QQI(64,64,32, (- VL_MULS_QQQ(64, 0x00000000000005dcULL, 
                                                         VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__robot_angle, 8U))), 2U);
            vlSelfRef.reflex_soc_4core__DOT__controller_call_133_kd_torque 
                = VL_MULS_QQQ(64, 0xffffffffffffff38ULL, 
                              VL_SHIFTR_QQI(64,64,32, 
                                            (vlSelfRef.reflex_soc_4core__DOT__robot_angle 
                                             - vlSelfRef.reflex_soc_4core__DOT__robot_angle_d1), 2U));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rx_valid_0) {
            __Vdly__reflex_soc_4core__DOT__core_top_call_135_wake_timer = 5U;
            vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_135_current_instr 
                = vlSelfRef.reflex_soc_4core__DOT__rx_data_0;
        }
        if (((IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_not_rx_valid_0_8f5789eac3a38db5_out) 
             & (IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_core_top_call_135_wake_t_1c9a211eeeb38950_out))) {
            __Vdly__reflex_soc_4core__DOT__core_top_call_135_wake_timer 
                = (7U & ((IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_wake_timer) 
                         - (IData)(1U)));
        }
        vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_wake_timer 
            = __Vdly__reflex_soc_4core__DOT__core_top_call_135_wake_timer;
        if (vlSelfRef.reflex_soc_4core__DOT__rx_valid_1) {
            __Vdly__reflex_soc_4core__DOT__core_top_call_137_wake_timer = 5U;
            vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_137_current_instr 
                = vlSelfRef.reflex_soc_4core__DOT__rx_data_1;
        }
        if (((IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_not_rx_valid_1_11d0544cf873038c_out) 
             & (IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_core_top_call_137_wake_t_46921745bbd4f73c_out))) {
            __Vdly__reflex_soc_4core__DOT__core_top_call_137_wake_timer 
                = (7U & ((IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_wake_timer) 
                         - (IData)(1U)));
        }
        vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_wake_timer 
            = __Vdly__reflex_soc_4core__DOT__core_top_call_137_wake_timer;
        if (vlSelfRef.reflex_soc_4core__DOT__rx_valid_2) {
            __Vdly__reflex_soc_4core__DOT__core_top_call_139_wake_timer = 5U;
            vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_139_current_instr 
                = vlSelfRef.reflex_soc_4core__DOT__rx_data_2;
        }
        if (((IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_not_rx_valid_2_173fdd08fddcea5a_out) 
             & (IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_core_top_call_139_wake_t_5b097111c566bad0_out))) {
            __Vdly__reflex_soc_4core__DOT__core_top_call_139_wake_timer 
                = (7U & ((IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_wake_timer) 
                         - (IData)(1U)));
        }
        vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_wake_timer 
            = __Vdly__reflex_soc_4core__DOT__core_top_call_139_wake_timer;
        if (vlSelfRef.reflex_soc_4core__DOT__rx_valid_3) {
            __Vdly__reflex_soc_4core__DOT__core_top_call_141_wake_timer = 5U;
            vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_141_current_instr 
                = vlSelfRef.reflex_soc_4core__DOT__rx_data_3;
        }
        if (((IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_not_rx_valid_3_f2852242f2f15204_out) 
             & (IData)(vlSelfRef.reflex_soc_4core__DOT__sub_g_core_top_call_141_wake_t_bc709c7e89383f58_out))) {
            __Vdly__reflex_soc_4core__DOT__core_top_call_141_wake_timer 
                = (7U & ((IData)(vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_wake_timer) 
                         - (IData)(1U)));
        }
        vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_wake_timer 
            = __Vdly__reflex_soc_4core__DOT__core_top_call_141_wake_timer;
        vlSelfRef.out_pc_0 = vlSelfRef.reflex_soc_4core__DOT__pc_0;
        vlSelfRef.out_pc_1 = vlSelfRef.reflex_soc_4core__DOT__pc_1;
        vlSelfRef.out_pc_2 = vlSelfRef.reflex_soc_4core__DOT__pc_2;
        vlSelfRef.out_pc_3 = vlSelfRef.reflex_soc_4core__DOT__pc_3;
        vlSelfRef.out_data_1 = vlSelfRef.reflex_soc_4core__DOT__core_data_1;
        vlSelfRef.out_data_2 = vlSelfRef.reflex_soc_4core__DOT__core_data_2;
        vlSelfRef.out_data_3 = vlSelfRef.reflex_soc_4core__DOT__core_data_3;
        vlSelfRef.out_data_0 = vlSelfRef.reflex_soc_4core__DOT__core_data_0;
        vlSelfRef.reflex_soc_4core__DOT__rx_data_0 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rx_data_1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rx_data_2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rx_data_3 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__core_data_1 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_res;
        vlSelfRef.reflex_soc_4core__DOT__core_data_2 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_res;
        vlSelfRef.reflex_soc_4core__DOT__core_data_3 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_res;
        vlSelfRef.reflex_soc_4core__DOT__core_data_0 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_res;
        vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angular_velocity_d1 
            = __Vdly__reflex_soc_4core__DOT__pendulum_call_131_angular_velocity_d1;
        if (vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p0_alive_out) {
            vlSelfRef.reflex_soc_4core__DOT__rx_data_0 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_payload;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p1_alive_out) {
            vlSelfRef.reflex_soc_4core__DOT__rx_data_1 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_payload;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p2_alive_out) {
            vlSelfRef.reflex_soc_4core__DOT__rx_data_2 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_payload;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p3_alive_out) {
            vlSelfRef.reflex_soc_4core__DOT__rx_data_3 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_payload;
        }
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_payload 
            = (0x0000ffffffffffffULL & vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_16);
        vlSelfRef.reflex_soc_4core__DOT__rx_valid_0 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p0_alive_out) {
            vlSelfRef.reflex_soc_4core__DOT__rx_valid_0 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_16;
        }
        vlSelfRef.reflex_soc_4core__DOT__rx_valid_1 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p1_alive_out) {
            vlSelfRef.reflex_soc_4core__DOT__rx_valid_1 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_16;
        }
        vlSelfRef.reflex_soc_4core__DOT__rx_valid_2 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p2_alive_out) {
            vlSelfRef.reflex_soc_4core__DOT__rx_valid_2 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_16;
        }
        vlSelfRef.reflex_soc_4core__DOT__rx_valid_3 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_p3_alive_out) {
            vlSelfRef.reflex_soc_4core__DOT__rx_valid_3 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_16;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_15) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_16 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_15;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_16 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_15)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_16 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_15;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_16 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_15;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_data_15 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_15 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_14) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_15 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_14;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_15 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_14)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_15 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_14;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_15 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_14;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_data_14 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_14 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_13) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_14 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_13;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_14 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_13)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_14 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_13;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_14 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_13;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_data_13 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_13 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__robot_angle_d1 
            = vlSelfRef.reflex_soc_4core__DOT__robot_angle;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_12) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_13 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_12;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_13 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_12)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_13 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_12;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_13 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_12;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_data_12 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_12 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_11) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_12 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_11;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_12 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_11)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_12 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_11;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_12 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_11;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_data_11 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_11 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_10) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_11 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_10;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_11 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_10)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_11 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_10;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_11 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_10;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_data_10 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_10 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_9) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_10 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_9;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_10 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_9)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_10 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_9;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_10 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_9;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_data_9 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_9 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_8) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_9 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_8;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_9 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_8)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_9 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_8;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_9 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_8;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_data_8 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_8 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_7) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_8 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_7;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_8 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_7)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_8 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_7;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_8 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_7;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_data_7 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_7 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_6) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_7 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_6;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_7 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_6)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_7 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_6;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_7 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_6;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_data_6 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_6 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_5) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_6 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_5;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_6 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_5)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_6 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_5;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_6 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_5;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_data_5 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_5 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_4) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_5 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_4;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_5 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_4)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_5 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_4;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_5 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_4;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_data_4 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_4 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_3) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_4 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_3;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_4 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_3)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_4 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_3;
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_4 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_3;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_data_3 = 0ULL;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_2) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_3 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_2;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_2)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_3 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_2;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_3 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_trap_signal) {
            vlSelfRef.reflex_soc_4core__DOT__tx_data_3 
                = (0x8000000e00000000ULL | (QData)((IData)(vlSelfRef.reflex_soc_4core__DOT__pc_3)));
            vlSelfRef.reflex_soc_4core__DOT__tx_valid_3 = 1U;
        }
    } else {
        __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_1175_pipe_pc = 0U;
        __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_1677_pipe_pc = 0U;
        __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc = 0U;
        __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc = 0U;
        __Vdly__reflex_soc_4core__DOT__pendulum_call_131_angular_velocity_d1 = 0ULL;
        vlSelfRef.__Vdly__reflex_soc_4core__DOT__pendulum_call_131_p_next = 0ULL;
        __Vdly__reflex_soc_4core__DOT__core_top_call_135_wake_timer = 0U;
        vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_wake_timer 
            = __Vdly__reflex_soc_4core__DOT__core_top_call_135_wake_timer;
        __Vdly__reflex_soc_4core__DOT__core_top_call_137_wake_timer = 0U;
        vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_wake_timer 
            = __Vdly__reflex_soc_4core__DOT__core_top_call_137_wake_timer;
        __Vdly__reflex_soc_4core__DOT__core_top_call_139_wake_timer = 0U;
        vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_wake_timer 
            = __Vdly__reflex_soc_4core__DOT__core_top_call_139_wake_timer;
        __Vdly__reflex_soc_4core__DOT__core_top_call_141_wake_timer = 0U;
        vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_wake_timer 
            = __Vdly__reflex_soc_4core__DOT__core_top_call_141_wake_timer;
        vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_135_current_instr = 0ULL;
        vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_137_current_instr = 0ULL;
        vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_139_current_instr = 0ULL;
        vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_141_current_instr = 0ULL;
        vlSelfRef.out_pc_0 = 0U;
        vlSelfRef.out_pc_1 = 0U;
        vlSelfRef.out_pc_2 = 0U;
        vlSelfRef.out_pc_3 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs2 = 0ULL;
        vlSelfRef.out_data_1 = 0ULL;
        vlSelfRef.out_data_2 = 0ULL;
        vlSelfRef.out_data_3 = 0ULL;
        vlSelfRef.out_data_0 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angular_velocity = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rx_data_0 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rx_data_1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rx_data_2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rx_data_3 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__core_data_1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__core_data_2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__core_data_3 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__core_data_0 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_v_next = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angular_velocity_d1 
            = __Vdly__reflex_soc_4core__DOT__pendulum_call_131_angular_velocity_d1;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_payload = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rx_valid_0 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__rx_valid_1 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__rx_valid_2 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__rx_valid_3 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__robot_torque = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_16 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_15 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_16 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_15 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__controller_call_133_t_next = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_15 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_14 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_15 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_14 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__controller_call_133_kp_torque = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__controller_call_133_kd_torque = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_14 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_13 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_14 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_13 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__robot_angle_d1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_13 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_12 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_13 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_12 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_12 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_11 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_12 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_11 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_11 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_10 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_11 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_10 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_10 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_9 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_10 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_9 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_9 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_8 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_9 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_8 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_8 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_7 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_8 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_7 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_7 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_6 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_7 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_6 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_6 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_5 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_6 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_5 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_5 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_4 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_5 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_4 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_4 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_4 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_3 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_3 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_3 = 0U;
    }
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pcc_fault) 
                                        | (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_2) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_3 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_2)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_3 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_2;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__pc_3 = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pipe_pc;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_3 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__pc_3 = 0U;
    }
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pipe_pc 
        = __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_1175_pipe_pc;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_1377_is_invalid));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.reflex_soc_4core__DOT__tx_data_2 = 0ULL;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_1) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_2 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_1;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_1)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_2 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_1;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_2 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_trap_signal) {
            vlSelfRef.reflex_soc_4core__DOT__tx_data_2 
                = (0x8000000e00000000ULL | (QData)((IData)(vlSelfRef.reflex_soc_4core__DOT__pc_2)));
            vlSelfRef.reflex_soc_4core__DOT__tx_valid_2 = 1U;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__tx_data_2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_2 = 0U;
    }
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_fault) 
                                        | (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_1) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_2 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_1)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_2 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_1;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__pc_2 = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pipe_pc;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_2 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__pc_2 = 0U;
    }
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pipe_pc 
        = __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_1677_pipe_pc;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_1879_is_invalid));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.reflex_soc_4core__DOT__tx_data_1 = 0ULL;
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_0) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_1 
                = vlSelfRef.reflex_soc_4core__DOT__tx_data_0;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_0)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_1 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_0;
        }
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_1 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_trap_signal) {
            vlSelfRef.reflex_soc_4core__DOT__tx_data_1 
                = (0x8000000e00000000ULL | (QData)((IData)(vlSelfRef.reflex_soc_4core__DOT__pc_1)));
            vlSelfRef.reflex_soc_4core__DOT__tx_valid_1 = 1U;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__tx_data_1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_1 = 0U;
    }
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_fault) 
                                        | (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__tx_valid_0) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_1 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__tx_valid_0)))) {
            vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_1 
                = vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_0;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_1 = 0U;
    }
    vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sv_0 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.reflex_soc_4core__DOT__downlink_valid));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__pc_1 = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc;
        }
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc 
            = __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_0 
            = vlSelfRef.reflex_soc_4core__DOT__downlink_data;
        vlSelfRef.reflex_soc_4core__DOT__downlink_data = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_0 = 0ULL;
        if (vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_trap_signal) {
            vlSelfRef.reflex_soc_4core__DOT__tx_data_0 
                = (0x8000000e00000000ULL | (QData)((IData)(vlSelfRef.reflex_soc_4core__DOT__pc_0)));
        }
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_trap = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_invalid) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_trap = 1U;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__pc_1 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc 
            = __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2179_pipe_pc;
        vlSelfRef.reflex_soc_4core__DOT__noc_l1_router_0_call_212_sd_0 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__downlink_data = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__tx_data_0 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_trap = 0U;
    }
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((((0ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op) 
                                           & (1ULL 
                                              != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op)) 
                                          & (3ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op)) 
                                         & (5ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op)) 
                                        & (6ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op)));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_2381_is_invalid));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_trap));
    vlSelfRef.reflex_soc_4core__DOT__downlink_valid = 0U;
    if (vlSelfRef.rst_n) {
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_0 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_trap_signal) {
            vlSelfRef.reflex_soc_4core__DOT__tx_valid_0 = 1U;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__tx_valid_0 = 0U;
    }
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_fault) 
                                        | (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__pc_0 = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc;
        }
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc 
            = __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_trap = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_invalid) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_trap = 1U;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__pc_0 = 0U;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc 
            = __Vdly__reflex_soc_4core__DOT__rspu_pipeline_call_2681_pipe_pc;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_trap = 0U;
    }
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((((0ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op) 
                                           & (1ULL 
                                              != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op)) 
                                          & (3ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op)) 
                                         & (5ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op)) 
                                        & (6ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op)));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_2883_is_invalid));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_trap = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_invalid) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_trap = 1U;
        }
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_trap = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_invalid) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_trap = 1U;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_trap = 0U;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_trap = 0U;
    }
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((((0ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op) 
                                           & (1ULL 
                                              != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op)) 
                                          & (3ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op)) 
                                         & (5ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op)) 
                                        & (6ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op)));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((((0ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op) 
                                           & (1ULL 
                                              != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op)) 
                                          & (3ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op)) 
                                         & (5ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op)) 
                                        & (6ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op)));
}

void Vreflex_soc_4core_synth___024root___nba_sequent__TOP__2(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___nba_sequent__TOP__2\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0;
    __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0;
    __VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0;
    __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 = 0;
    // Body
    __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 = 0U;
    if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_we) {
        __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_res;
        __VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_rd));
        __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0) {
        vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs[__VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0] 
            = __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0;
    }
}

void Vreflex_soc_4core_synth___024root___nba_sequent__TOP__3(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___nba_sequent__TOP__3\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0;
    __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0;
    __VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0;
    __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 = 0;
    // Body
    __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 = 0U;
    if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_we) {
        __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_res;
        __VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_rd));
        __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0) {
        vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs[__VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0] 
            = __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0;
    }
}

void Vreflex_soc_4core_synth___024root___nba_sequent__TOP__4(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___nba_sequent__TOP__4\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0;
    __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0;
    __VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0;
    __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 = 0;
    // Body
    __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 = 0U;
    if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_we) {
        __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_res;
        __VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_rd));
        __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0) {
        vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs[__VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0] 
            = __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0;
    }
}

void Vreflex_soc_4core_synth___024root___nba_sequent__TOP__5(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___nba_sequent__TOP__5\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0;
    __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0;
    __VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0;
    __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 = 0;
    // Body
    __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 = 0U;
    if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_we) {
        __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_res;
        __VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_rd));
        __VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0) {
        vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs[__VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0] 
            = __VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0;
    }
}

void Vreflex_soc_4core_synth___024root___nba_sequent__TOP__6(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___nba_sequent__TOP__6\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_we = 1U;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_rd 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_rd;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_load) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_res 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_out;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_not_load) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_res 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_res;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_we = 0U;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_res = 0ULL;
    }
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_op));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_not_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_we = 1U;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_rd 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_rd;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_load) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_res 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_out;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_not_load) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_res 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_res;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_we = 0U;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_res = 0ULL;
    }
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_op));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_not_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_we = 1U;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_rd 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_rd;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_load) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_res 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_out;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_not_load) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_res 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_res;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_we = 0U;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_res = 0ULL;
    }
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_op));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_not_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_we = 1U;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_rd 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_rd;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_load) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_res 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_out;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_not_load) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_res 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_res;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_we = 0U;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_res = 0ULL;
    }
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_op));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_not_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_rd 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_rd;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_res 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_packed;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_rd 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rd;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_op 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_rd 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_rd;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_res 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_packed;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_rd 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_rd;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_res 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_packed;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_rd 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_rd;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_res 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_packed;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_normal_ex) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_d));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_load_in) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & vlSelfRef.reflex_soc_4core__DOT__core_io_in_3));
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_res = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_res = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_res = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_res = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_mem_op = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_packed = 0ULL;
    }
    vlSelfRef.reflex_soc_4core__DOT__core_io_in_3 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_normal_ex 
        = ((IData)(vlSelfRef.rst_n) && (0ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_rd 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rd;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_op 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_normal_ex) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_d));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_load_in) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & vlSelfRef.reflex_soc_4core__DOT__core_io_in_2));
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_mem_op = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_packed = 0ULL;
    }
    vlSelfRef.reflex_soc_4core__DOT__core_io_in_2 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_normal_ex 
        = ((IData)(vlSelfRef.rst_n) && (0ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_rd 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rd;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_op 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_normal_ex) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_d));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_load_in) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & vlSelfRef.reflex_soc_4core__DOT__core_io_in_1));
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_mem_op = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_packed = 0ULL;
    }
    vlSelfRef.reflex_soc_4core__DOT__core_io_in_1 = 0ULL;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_normal_ex 
        = ((IData)(vlSelfRef.rst_n) && (0ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_rd 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rd;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_op 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_normal_ex) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_d));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_load_in) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & vlSelfRef.reflex_soc_4core__DOT__core_io_in_0));
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_mem_op = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_packed = 0ULL;
    }
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_normal_ex 
        = ((IData)(vlSelfRef.rst_n) && (0ULL != vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_if_id_instr, 0x00000010U));
        }
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_prov);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_tag);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_data);
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_if_id_instr, 0x00000010U));
        }
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_prov);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_tag);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_data);
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_if_id_instr, 0x00000010U));
        }
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_prov);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_tag);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_data);
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_if_id_instr, 0x00000010U));
        }
        vlSelfRef.reflex_soc_4core__DOT__core_io_in_0 
            = vlSelfRef.reflex_soc_4core__DOT__robot_angle;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_prov);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_tag);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_data);
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_prov 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_p1;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_tag 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_t1;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_data 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d2;
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_add) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d2));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_sub) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d2));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_relu_pos) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_data 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d1;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_relu_neg) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_data = 0ULL;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_tag_gate) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_p = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_t = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_out_d = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_p = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_t = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_out_d = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_p = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_t = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_out_d = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rd = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__core_io_in_0 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_p = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_t = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_out_d = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_prov = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_tag = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_res_data = 0ULL;
    }
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d1, 0x0000001fU))));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d1, 0x0000001fU))));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op) 
                                        & (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_t1 
                                           == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_t2)));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1393_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op) 
                                        & (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_t1 
                                           == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_prov 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_p1;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_tag 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_t1;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_data 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d2;
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_add) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d2));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_sub) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d2));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_relu_pos) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_data 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d1;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_relu_neg) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_data = 0ULL;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_tag_gate) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_prov = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_tag = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_res_data = 0ULL;
    }
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d1, 0x0000001fU))));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d1, 0x0000001fU))));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op) 
                                        & (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_t1 
                                           == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_t2)));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_1895_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op) 
                                        & (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_t1 
                                           == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_prov 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_p1;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_tag 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_t1;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_data 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d2;
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_add) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d2));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_sub) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d2));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_relu_pos) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_data 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d1;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_relu_neg) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_data = 0ULL;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_tag_gate) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_prov = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_tag = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_res_data = 0ULL;
    }
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d1, 0x0000001fU))));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d1, 0x0000001fU))));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op) 
                                        & (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_t1 
                                           == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_t2)));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2397_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op) 
                                        & (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_t1 
                                           == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_t2)));
    if (vlSelfRef.rst_n) {
        if ((1U & (~ (IData)(vlSelfRef.rst_n)))) {
            vlSelfRef.reflex_soc_4core__DOT__robot_angle = 0ULL;
        }
        if (vlSelfRef.rst_n) {
            vlSelfRef.reflex_soc_4core__DOT__robot_angle 
                = vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angle_internal_d1;
        }
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_prov 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_p1;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_tag 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_t1;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_data 
            = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d2;
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_add) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d2));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_sub) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d2));
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_relu_pos) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_data 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d1;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_relu_neg) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_data = 0ULL;
        }
        if (vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_tag_gate) {
            vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__robot_angle = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_prov = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_tag = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_res_data = 0ULL;
    }
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d1, 0x0000001fU))));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d1, 0x0000001fU))));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op) 
                                        & (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_t1 
                                           == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_t2)));
    vlSelfRef.reflex_soc_4core__DOT__alu_core_call_2899_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op) 
                                        & (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_t1 
                                           == vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_val2);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_val1, 0x00000024U));
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_val2);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_val1, 0x00000024U));
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_val2);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_val1, 0x00000024U));
        vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angle_internal_d1 
            = vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angle_internal;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_val2);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_val1, 0x00000024U));
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_val1);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_val2, 0x00000020U));
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_val1, 0x00000020U));
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_op;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_val2 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val2;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_val1 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val1;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_if_id_instr);
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_if_id_instr 
                = vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_current_instr;
        }
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_val1);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_val2, 0x00000020U));
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_val1, 0x00000020U));
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_op;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_val2 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val2;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_val1 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val1;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_if_id_instr);
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_if_id_instr 
                = vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_current_instr;
        }
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_val1);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_val2, 0x00000020U));
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_val1, 0x00000020U));
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_op;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_val2 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val2;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_val1 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val1;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_if_id_instr);
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_if_id_instr 
                = vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_current_instr;
        }
        if ((1U & (~ (IData)(vlSelfRef.rst_n)))) {
            vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angle_internal = 0ULL;
        }
        if (vlSelfRef.rst_n) {
            vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angle_internal 
                = vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_p_next;
        }
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_val1);
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_val2, 0x00000020U));
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_val1, 0x00000020U));
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_valid) {
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_op;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_val2 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val2;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_val1 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val1;
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_if_id_instr);
            vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_if_id_instr 
                = vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_current_instr;
        }
    } else {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_p1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_p1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_p1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angle_internal_d1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_p1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_d1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_t2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_ex_t1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_op = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_d1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_t2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_ex_t1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_op = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_d1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_t2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_ex_t1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_op = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_angle_internal = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_d1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_t2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_ex_t1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_op = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_val2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_ex_val1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_op = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_val2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_ex_val1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_op = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_val2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_ex_val1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_op = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_val2 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_ex_val1 = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_op = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_if_id_instr = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_if_id_instr = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_if_id_instr = 0ULL;
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_if_id_instr = 0ULL;
    }
    vlSelfRef.reflex_soc_4core__DOT__pendulum_call_131_p_next 
        = vlSelfRef.__Vdly__reflex_soc_4core__DOT__pendulum_call_131_p_next;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_141_current_instr 
        = vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_141_current_instr;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_139_current_instr 
        = vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_139_current_instr;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_137_current_instr 
        = vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_137_current_instr;
    vlSelfRef.reflex_soc_4core__DOT__core_top_call_135_current_instr 
        = vlSelfRef.__Vdly__reflex_soc_4core__DOT__core_top_call_135_current_instr;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_1377_is_invalid))));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_1879_is_invalid))));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_2381_is_invalid))));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_2883_is_invalid))));
    vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_1377_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_cert_g))));
    vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_1879_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_cert_g))));
    vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_2381_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_cert_g))));
    vlSelfRef.reflex_soc_4core__DOT__pcc_verifier_call_2883_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_cert_g))));
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_cert_i = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_cert_r = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_cert_g = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_cert_i = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_cert_r = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_cert_g = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_cert_i = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_cert_r = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_cert_g = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_cert_i = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_cert_r = 0U;
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_cert_g = 0U;
}

void Vreflex_soc_4core_synth___024root___nba_comb__TOP__0(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___nba_comb__TOP__0\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val2 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs2))];
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val1 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs1))];
}

void Vreflex_soc_4core_synth___024root___nba_comb__TOP__1(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___nba_comb__TOP__1\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val2 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs2))];
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val1 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs1))];
}

void Vreflex_soc_4core_synth___024root___nba_comb__TOP__2(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___nba_comb__TOP__2\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val2 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs2))];
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val1 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs1))];
}

void Vreflex_soc_4core_synth___024root___nba_comb__TOP__3(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___nba_comb__TOP__3\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val2 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs2))];
    vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val1 
        = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs1))];
}

void Vreflex_soc_4core_synth___024root___eval_nba(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval_nba\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __Vinline__nba_sequent__TOP__0___VdlyVal__ram__DOT__mem__v0;
    __Vinline__nba_sequent__TOP__0___VdlyVal__ram__DOT__mem__v0 = 0;
    SData/*13:0*/ __Vinline__nba_sequent__TOP__0___VdlyDim0__ram__DOT__mem__v0;
    __Vinline__nba_sequent__TOP__0___VdlyDim0__ram__DOT__mem__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__2___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__2___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__2___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__2___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__2___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__2___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__3___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__3___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__3___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__3___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__3___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__3___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__4___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__4___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__4___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__4___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__4___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__4___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__5___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__5___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__5___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__5___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__5___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__5___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 = 0;
    // Body
    if ((0x0000000000000040ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__0___VdlyVal__ram__DOT__mem__v0 
            = vlSelfRef.din;
        __Vinline__nba_sequent__TOP__0___VdlyDim0__ram__DOT__mem__v0 
            = vlSelfRef.addr;
        vlSelfRef.dout = vlSelfRef.ram__DOT__mem[vlSelfRef.addr];
        vlSelfRef.ram__DOT__mem[__Vinline__nba_sequent__TOP__0___VdlyDim0__ram__DOT__mem__v0] 
            = __Vinline__nba_sequent__TOP__0___VdlyVal__ram__DOT__mem__v0;
    }
    if ((3ULL & vlSelfRef.__VnbaTriggered[0U])) {
        Vreflex_soc_4core_synth___024root___nba_sequent__TOP__1(vlSelf);
    }
    if ((4ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__2___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_we) {
            __Vinline__nba_sequent__TOP__2___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_res;
            __Vinline__nba_sequent__TOP__2___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_mem_wb_rd));
            __Vinline__nba_sequent__TOP__2___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__2___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0) {
            vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs[__Vinline__nba_sequent__TOP__2___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__2___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs__v0;
        }
    }
    if ((8ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__3___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_we) {
            __Vinline__nba_sequent__TOP__3___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_res;
            __Vinline__nba_sequent__TOP__3___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_mem_wb_rd));
            __Vinline__nba_sequent__TOP__3___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__3___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0) {
            vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs[__Vinline__nba_sequent__TOP__3___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__3___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs__v0;
        }
    }
    if ((0x0000000000000010ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__4___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_we) {
            __Vinline__nba_sequent__TOP__4___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_res;
            __Vinline__nba_sequent__TOP__4___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_mem_wb_rd));
            __Vinline__nba_sequent__TOP__4___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__4___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0) {
            vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs[__Vinline__nba_sequent__TOP__4___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__4___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs__v0;
        }
    }
    if ((0x0000000000000020ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__5___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 = 0U;
        if (vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_we) {
            __Vinline__nba_sequent__TOP__5___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 
                = vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_res;
            __Vinline__nba_sequent__TOP__5___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_mem_wb_rd));
            __Vinline__nba_sequent__TOP__5___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__5___VdlySet__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0) {
            vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs[__Vinline__nba_sequent__TOP__5___VdlyDim0__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__5___VdlyVal__reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs__v0;
        }
    }
    if ((3ULL & vlSelfRef.__VnbaTriggered[0U])) {
        Vreflex_soc_4core_synth___024root___nba_sequent__TOP__6(vlSelf);
    }
    if ((7ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val2 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs2))];
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_id_val1 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1379__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1175_dec_rs1))];
    }
    if ((0x000000000000000bULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val2 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs2))];
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_id_val1 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_1881__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_1677_dec_rs1))];
    }
    if ((0x0000000000000013ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val2 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs2))];
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_id_val1 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2383__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2179_dec_rs1))];
    }
    if ((0x0000000000000023ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val2 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs2))];
        vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_id_val1 
            = vlSelfRef.reflex_soc_4core__DOT__reg_regfile_inst_2885__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.reflex_soc_4core__DOT__rspu_pipeline_call_2681_dec_rs1))];
    }
}

void Vreflex_soc_4core_synth___024root___trigger_orInto__act_vec_vec(VlUnpacked<QData/*63:0*/, 1> &out, const VlUnpacked<QData/*63:0*/, 1> &in) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___trigger_orInto__act_vec_vec\n"); );
    // Locals
    IData/*31:0*/ n;
    // Body
    n = 0U;
    do {
        out[n] = (out[n] | in[n]);
        n = ((IData)(1U) + n);
    } while ((0U >= n));
}

#ifdef VL_DEBUG
VL_ATTR_COLD void Vreflex_soc_4core_synth___024root___dump_triggers__act(const VlUnpacked<QData/*63:0*/, 1> &triggers, const std::string &tag);
#endif  // VL_DEBUG

bool Vreflex_soc_4core_synth___024root___eval_phase__act(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval_phase__act\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    Vreflex_soc_4core_synth___024root___eval_triggers_vec__act(vlSelf);
#ifdef VL_DEBUG
    if (VL_UNLIKELY(vlSymsp->_vm_contextp__->debug())) {
        Vreflex_soc_4core_synth___024root___dump_triggers__act(vlSelfRef.__VactTriggered, "act"s);
    }
#endif
    Vreflex_soc_4core_synth___024root___trigger_orInto__act_vec_vec(vlSelfRef.__VnbaTriggered, vlSelfRef.__VactTriggered);
    return (0U);
}

void Vreflex_soc_4core_synth___024root___trigger_clear__act(VlUnpacked<QData/*63:0*/, 1> &out) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___trigger_clear__act\n"); );
    // Locals
    IData/*31:0*/ n;
    // Body
    n = 0U;
    do {
        out[n] = 0ULL;
        n = ((IData)(1U) + n);
    } while ((1U > n));
}

bool Vreflex_soc_4core_synth___024root___eval_phase__nba(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval_phase__nba\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    CData/*0:0*/ __VnbaExecute;
    // Body
    __VnbaExecute = Vreflex_soc_4core_synth___024root___trigger_anySet__act(vlSelfRef.__VnbaTriggered);
    if (__VnbaExecute) {
        Vreflex_soc_4core_synth___024root___eval_nba(vlSelf);
        Vreflex_soc_4core_synth___024root___trigger_clear__act(vlSelfRef.__VnbaTriggered);
    }
    return (__VnbaExecute);
}

void Vreflex_soc_4core_synth___024root___eval(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    IData/*31:0*/ __VnbaIterCount;
    // Body
    __VnbaIterCount = 0U;
    do {
        if (VL_UNLIKELY(((0x00002710U < __VnbaIterCount)))) {
#ifdef VL_DEBUG
            Vreflex_soc_4core_synth___024root___dump_triggers__act(vlSelfRef.__VnbaTriggered, "nba"s);
#endif
            VL_FATAL_MT("reflex_soc/reflex_soc_4core_synth.sv", 15, "", "DIDNOTCONVERGE: NBA region did not converge after '--converge-limit' of 10000 tries");
        }
        __VnbaIterCount = ((IData)(1U) + __VnbaIterCount);
        vlSelfRef.__VactIterCount = 0U;
        do {
            if (VL_UNLIKELY(((0x00002710U < vlSelfRef.__VactIterCount)))) {
#ifdef VL_DEBUG
                Vreflex_soc_4core_synth___024root___dump_triggers__act(vlSelfRef.__VactTriggered, "act"s);
#endif
                VL_FATAL_MT("reflex_soc/reflex_soc_4core_synth.sv", 15, "", "DIDNOTCONVERGE: Active region did not converge after '--converge-limit' of 10000 tries");
            }
            vlSelfRef.__VactIterCount = ((IData)(1U) 
                                         + vlSelfRef.__VactIterCount);
            vlSelfRef.__VactPhaseResult = Vreflex_soc_4core_synth___024root___eval_phase__act(vlSelf);
        } while (vlSelfRef.__VactPhaseResult);
        vlSelfRef.__VnbaPhaseResult = Vreflex_soc_4core_synth___024root___eval_phase__nba(vlSelf);
    } while (vlSelfRef.__VnbaPhaseResult);
}

#ifdef VL_DEBUG
void Vreflex_soc_4core_synth___024root___eval_debug_assertions(Vreflex_soc_4core_synth___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vreflex_soc_4core_synth___024root___eval_debug_assertions\n"); );
    Vreflex_soc_4core_synth__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if (VL_UNLIKELY(((vlSelfRef.reflex_soc_4core__02Eclk 
                      & 0xfeU)))) {
        Verilated::overWidthError("reflex_soc_4core.clk");
    }
    if (VL_UNLIKELY(((vlSelfRef.rst_n & 0xfeU)))) {
        Verilated::overWidthError("rst_n");
    }
    if (VL_UNLIKELY(((vlSelfRef.ram__02Eclk & 0xfeU)))) {
        Verilated::overWidthError("ram.clk");
    }
    if (VL_UNLIKELY(((vlSelfRef.addr & 0xc000U)))) {
        Verilated::overWidthError("addr");
    }
}
#endif  // VL_DEBUG

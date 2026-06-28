// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vrspu_top.h for the primary calling header

#include "Vrspu_top__pch.h"

void Vrspu_top___024root___nba_sequent__TOP__1(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__1\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_5227_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_864_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_trap)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_31238_pipe_pc;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_22678_pipe_pc;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_14118_pipe_pc;
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_61 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_61 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_5558_pipe_pc;
    vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_31442_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_766_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_22882_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_798_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_14322_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_830_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_5762_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_60 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_60 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_862_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_60 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_60 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_60)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_60 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_60 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_862_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_59) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_12 
                = vlSelfRef.rspu_top__DOT__tx_data_59;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_59)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_12 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_11;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_10) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_11 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_11 
                = vlSelfRef.rspu_top__DOT__tx_data_10;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_10)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_11 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_10;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_11 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_10;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_12 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_31773_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_11 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_11 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_764_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_11 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_11 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_11)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_26) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_11 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_11 
                = vlSelfRef.rspu_top__DOT__tx_data_26;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_26)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_11 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_10;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_11 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_10;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_28 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_23213_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_27 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_27 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_796_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_27 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_27 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_27)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_42) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_11 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_11 
                = vlSelfRef.rspu_top__DOT__tx_data_42;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_42)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_11 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_10;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_11 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_10;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_44 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_14653_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_43 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_43 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_828_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_43 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_43 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_43)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_58) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_11 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_58)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_11 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_10;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_60 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_12 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_11 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_12 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_31773_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_11 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_11 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_11 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_11 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_28 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_23213_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_27 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_27 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_11 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_11 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_44 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_14653_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_43 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_43 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_11 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_11 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_60 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_6093_pipe_pc;
    vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_31977_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_764_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_23417_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_796_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_14857_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_828_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_6297_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_59 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_59 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_860_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_59 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_59 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_59)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_59 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_59 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_860_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_58) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_11 
                = vlSelfRef.rspu_top__DOT__tx_data_58;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_58)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_11 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_10;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_9) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_10 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_10 
                = vlSelfRef.rspu_top__DOT__tx_data_9;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_9)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_10 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_9;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_10 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_9;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_11 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_32308_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_10 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_10 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_762_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_10 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_10 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_10)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_25) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_10 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_10 
                = vlSelfRef.rspu_top__DOT__tx_data_25;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_25)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_10 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_9;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_10 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_9;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_27 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_23748_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_26 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_26 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_794_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_26 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_26 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_26)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_41) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_10 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_10 
                = vlSelfRef.rspu_top__DOT__tx_data_41;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_41)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_10 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_9;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_10 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_9;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_43 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_15188_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_42 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_42 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_826_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_42 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_42 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_42)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_57) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_10 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_57)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_10 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_9;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_59 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_11 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_10 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_11 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_32308_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_10 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_10 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_10 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_10 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_27 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_23748_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_26 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_26 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_10 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_10 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_43 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_15188_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_42 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_42 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_10 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_10 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_59 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_6628_pipe_pc;
    vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_32512_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_762_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_23952_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_794_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_15392_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_826_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_6832_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_58 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_58 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_858_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_58 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_58 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_58)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_58 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_58 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_858_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_57) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_10 
                = vlSelfRef.rspu_top__DOT__tx_data_57;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_57)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_10 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_9;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_8) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_9 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_9 
                = vlSelfRef.rspu_top__DOT__tx_data_8;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_8)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_9 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_8;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_9 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_8;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_10 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_32843_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_9 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_9 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_760_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_9 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_9 = (0x8000000e00000000ULL 
                                                  | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_9)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_24) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_9 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_9 
                = vlSelfRef.rspu_top__DOT__tx_data_24;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_24)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_9 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_8;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_9 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_8;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_26 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_24283_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_25 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_25 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_792_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_25 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_25 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_25)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_40) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_9 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_9 
                = vlSelfRef.rspu_top__DOT__tx_data_40;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_40)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_9 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_8;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_9 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_8;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_42 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_15723_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_41 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_41 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_824_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_41 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_41 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_41)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_56) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_9 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_56)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_9 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_8;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_58 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_10 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_9 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_10 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_32843_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_9 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_9 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_9 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_9 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_26 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_24283_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_25 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_25 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_9 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_9 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_42 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_15723_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_41 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_41 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_9 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_9 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_58 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_7163_pipe_pc;
    vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_33047_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_760_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_24487_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_792_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_15927_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_824_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_7367_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_57 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_57 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_856_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_57 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_57 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_57)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_57 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_57 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_856_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_56) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_9 
                = vlSelfRef.rspu_top__DOT__tx_data_56;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_56)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_9 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_8;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_7) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_8 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_8 
                = vlSelfRef.rspu_top__DOT__tx_data_7;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_7)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_8 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_7;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_8 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_7;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_9 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_33378_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_8 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_8 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_758_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_8 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_8 = (0x8000000e00000000ULL 
                                                  | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_8)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_23) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_8 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_8 
                = vlSelfRef.rspu_top__DOT__tx_data_23;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_23)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_8 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_7;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_8 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_7;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_25 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_24818_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_24 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_24 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_790_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_24 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_24 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_24)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_39) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_8 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_8 
                = vlSelfRef.rspu_top__DOT__tx_data_39;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_39)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_8 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_7;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_8 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_7;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_41 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_16258_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_40 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_40 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_822_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_40 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_40 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_40)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_55) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_8 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_55)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_8 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_7;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_57 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_9 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_8 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_9 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_33378_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_8 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_8 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_8 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_8 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_25 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_24818_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_24 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_24 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_8 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_8 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_41 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_16258_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_40 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_40 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_8 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_8 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_57 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_7698_pipe_pc;
    vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_33582_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_758_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_25022_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_790_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_16462_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_822_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_7902_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_56 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_56 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_854_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_56 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_56 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_56)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_56 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_56 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_854_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_55) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_8 
                = vlSelfRef.rspu_top__DOT__tx_data_55;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_55)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_8 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_7;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_6) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_7 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_7 
                = vlSelfRef.rspu_top__DOT__tx_data_6;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_6)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_7 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_6;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_7 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_6;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_8 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_33913_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_7 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_7 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_756_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_7 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_7 = (0x8000000e00000000ULL 
                                                  | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_7)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_22) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_7 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_7 
                = vlSelfRef.rspu_top__DOT__tx_data_22;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_22)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_7 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_6;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_7 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_6;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_24 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_25353_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_23 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_23 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_788_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_23 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_23 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_23)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_38) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_7 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_7 
                = vlSelfRef.rspu_top__DOT__tx_data_38;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_38)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_7 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_6;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_7 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_6;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_40 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_16793_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_39 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_39 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_820_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_39 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_39 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_39)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_54) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_7 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_54)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_7 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_6;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_56 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_8 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_7 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_8 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_33913_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_7 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_7 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_7 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_7 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_24 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_25353_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_23 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_23 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_7 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_7 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_40 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_16793_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_39 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_39 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_7 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_7 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_56 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_8233_pipe_pc;
    vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_34117_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_756_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_25557_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_788_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_16997_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_820_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_8437_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_55 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_55 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_852_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_55 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_55 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_55)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_55 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_55 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_852_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_54) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_7 
                = vlSelfRef.rspu_top__DOT__tx_data_54;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_54)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_7 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_6;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_5) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_6 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_6 
                = vlSelfRef.rspu_top__DOT__tx_data_5;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_5)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_6 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_5;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_6 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_5;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_7 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_34448_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_6 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_6 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_754_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_6 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_6 = (0x8000000e00000000ULL 
                                                  | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_6)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_21) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_6 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_6 
                = vlSelfRef.rspu_top__DOT__tx_data_21;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_21)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_6 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_5;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_6 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_5;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_23 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_25888_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_22 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_22 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_786_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_22 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_22 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_22)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_37) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_6 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_6 
                = vlSelfRef.rspu_top__DOT__tx_data_37;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_37)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_6 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_5;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_6 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_5;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_39 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_17328_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_38 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_38 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_818_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_38 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_38 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_38)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_53) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_6 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_53)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_6 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_5;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_55 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_7 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_6 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_7 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_34448_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_6 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_6 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_6 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_6 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_23 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_25888_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_22 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_22 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_6 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_6 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_39 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_17328_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_38 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_38 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_6 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_6 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_55 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_8768_pipe_pc;
    vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_34652_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_754_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_26092_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_786_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_17532_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_818_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_8972_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_54 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_54 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_850_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_54 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_54 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_54)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_54 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_54 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_850_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_53) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_6 
                = vlSelfRef.rspu_top__DOT__tx_data_53;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_53)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_6 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_5;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_4) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_5 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_5 
                = vlSelfRef.rspu_top__DOT__tx_data_4;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_4)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_5 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_4;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_5 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_4;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_6 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_34983_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_5 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_5 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_752_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_5 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_5 = (0x8000000e00000000ULL 
                                                  | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_5)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_20) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_5 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_5 
                = vlSelfRef.rspu_top__DOT__tx_data_20;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_20)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_5 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_4;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_5 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_4;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_22 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_26423_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_21 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_21 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_784_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_21 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_21 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_21)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_36) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_5 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_5 
                = vlSelfRef.rspu_top__DOT__tx_data_36;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_36)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_5 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_4;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_5 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_4;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_38 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_17863_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_37 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_37 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_816_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_37 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_37 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_37)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_52) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_5 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_52)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_5 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_4;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_54 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_6 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_5 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_6 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_34983_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_5 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_5 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_5 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_5 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_22 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_26423_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_21 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_21 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_5 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_5 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_38 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_17863_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_37 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_37 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_5 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_5 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_54 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_9303_pipe_pc;
    vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_35187_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_752_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_26627_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_784_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_18067_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_816_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_9507_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_53 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_53 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_848_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_53 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_53 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_53)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_53 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_53 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_848_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_52) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_5 
                = vlSelfRef.rspu_top__DOT__tx_data_52;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_52)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_5 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_4;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_3) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_4 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_4 
                = vlSelfRef.rspu_top__DOT__tx_data_3;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_3)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_4 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_3;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_4 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_3;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_5 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_35518_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_4 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_4 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_750_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_4 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_4 = (0x8000000e00000000ULL 
                                                  | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_4)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_19) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_4 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_4 
                = vlSelfRef.rspu_top__DOT__tx_data_19;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_19)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_4 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_3;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_4 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_3;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_21 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_26958_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_20 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_20 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_782_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_20 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_20 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_20)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_35) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_4 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_4 
                = vlSelfRef.rspu_top__DOT__tx_data_35;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_35)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_4 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_3;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_4 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_3;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_37 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_18398_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_36 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_36 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_814_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_36 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_36 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_36)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_51) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_4 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_51)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_4 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_3;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_53 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_5 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_4 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_5 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_35518_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_4 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_4 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_4 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_4 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_21 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_26958_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_20 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_20 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_4 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_4 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_37 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_18398_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_36 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_36 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_4 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_4 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_53 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_9838_pipe_pc;
    vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_35722_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_750_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_27162_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_782_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_18602_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_814_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_10042_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_52 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_52 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_846_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_52 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_52 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_52)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_52 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_52 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_846_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_51) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_4 
                = vlSelfRef.rspu_top__DOT__tx_data_51;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_51)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_4 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_3;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_2) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_3 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_3 
                = vlSelfRef.rspu_top__DOT__tx_data_2;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_2)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_3 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_2;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_3 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_2;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_4 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_36053_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_3 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_3 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_748_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_3 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_3 = (0x8000000e00000000ULL 
                                                  | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_3)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_18) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_3 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_3 
                = vlSelfRef.rspu_top__DOT__tx_data_18;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_18)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_3 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_2;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_3 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_2;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_20 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_27493_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_19 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_19 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_780_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_19 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_19 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_19)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_34) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_3 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_3 
                = vlSelfRef.rspu_top__DOT__tx_data_34;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_34)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_3 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_2;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_3 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_2;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_36 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_18933_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_35 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_35 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_812_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_35 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_35 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_35)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_50) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_3 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_50)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_3 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_2;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_52 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_4 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_3 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_4 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_36053_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_3 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_3 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_3 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_3 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_20 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_27493_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_19 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_19 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_3 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_3 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_36 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_18933_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_35 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_35 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_3 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_3 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_52 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_10373_pipe_pc;
    vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_36257_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_748_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_27697_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_780_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_19137_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_812_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_10577_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_51 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_51 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_844_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_51 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_51 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_51)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_51 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_51 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_844_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_50) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_3 
                = vlSelfRef.rspu_top__DOT__tx_data_50;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_50)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_3 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_2;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_1) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_2 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_2 
                = vlSelfRef.rspu_top__DOT__tx_data_1;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_1)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_2 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_1;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_2 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_1;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_3 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_36588_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_2 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_2 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_746_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_2 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_2 = (0x8000000e00000000ULL 
                                                  | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_2)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_17) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_2 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_2 
                = vlSelfRef.rspu_top__DOT__tx_data_17;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_17)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_2 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_1;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_2 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_1;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_19 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_28028_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_18 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_18 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_778_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_18 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_18 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_18)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_33) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_2 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_2 
                = vlSelfRef.rspu_top__DOT__tx_data_33;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_33)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_2 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_1;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_2 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_1;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_35 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_19468_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_34 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_34 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_810_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_34 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_34 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_34)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_49) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_2 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_49)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_2 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_1;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_51 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_3 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_2 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_3 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_36588_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_2 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_2 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_2 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_2 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_19 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_28028_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_18 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_18 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_2 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_2 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_35 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_19468_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_34 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_34 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_2 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_2 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_51 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_10908_pipe_pc;
    vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_36792_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_746_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_28232_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_778_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_19672_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_810_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_11112_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_50 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_50 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_842_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_50 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_50 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_50)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_50 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_50 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_842_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_49) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_2 
                = vlSelfRef.rspu_top__DOT__tx_data_49;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_49)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_2 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_1;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_0) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_1 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_1 
                = vlSelfRef.rspu_top__DOT__tx_data_0;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_0)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_1 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_0;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_1 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_0;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_2 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_37123_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_1 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_1 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_744_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_1 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_1 = (0x8000000e00000000ULL 
                                                  | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_1)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_16) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_1 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_1 
                = vlSelfRef.rspu_top__DOT__tx_data_16;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_16)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_1 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_0;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_1 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_0;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_18 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_28563_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_17 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_17 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_776_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_17 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_17 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_17)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_32) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_1 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_1 
                = vlSelfRef.rspu_top__DOT__tx_data_32;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_32)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_1 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_0;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_1 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_0;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_34 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_20003_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_33 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_33 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_808_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_33 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_33 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_33)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_48) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_1 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_48)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_1 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_0;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_50 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_2 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_1 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_2 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_37123_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_1 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_1 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_1 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_1 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_18 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_28563_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_17 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_17 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_1 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_1 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_34 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_20003_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_33 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_33 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_1 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_1 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_50 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_11443_pipe_pc;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_0 
        = ((IData)(vlSelfRef.rst_n) && (1U & (IData)(vlSelfRef.rspu_top__DOT__downlink_valid_0_sync)));
    vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_37327_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_744_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_trap)));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_0 
        = ((IData)(vlSelfRef.rst_n) && (1U & (IData)(vlSelfRef.rspu_top__DOT__downlink_valid_1_sync)));
    vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_28767_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_776_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_trap)));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_0 
        = ((IData)(vlSelfRef.rst_n) && (1U & (IData)(vlSelfRef.rspu_top__DOT__downlink_valid_2_sync)));
    vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_20207_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_808_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_trap)));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_0 
        = ((IData)(vlSelfRef.rst_n) && (1U & (IData)(vlSelfRef.rspu_top__DOT__downlink_valid_3_sync)));
    vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_11647_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_49 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_49 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_840_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_49 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_49 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_49)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_49 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_49 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_840_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_trap)));
    vlSelfRef.rspu_top__DOT__downlink_valid_0_sync 
        = vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_0_sync;
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_48) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_1 
                = vlSelfRef.rspu_top__DOT__tx_data_48;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_48)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_1 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_0;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_1 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_37658_pipe_pc;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_0 
            = (((QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_0_sync[1U])) 
                << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_0_sync[0U])));
        vlSelfRef.rspu_top__DOT__downlink_data_0_sync[0U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[0U];
        vlSelfRef.rspu_top__DOT__downlink_data_0_sync[1U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[1U];
        vlSelfRef.rspu_top__DOT__downlink_data_0_sync[2U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[2U];
        vlSelfRef.rspu_top__DOT__downlink_data_0_sync[3U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[3U];
        vlSelfRef.rspu_top__DOT__downlink_valid_1_sync 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_1_sync;
        vlSelfRef.rspu_top__DOT__tx_valid_0 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_0 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_742_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_0 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_0 = (0x8000000e00000000ULL 
                                                  | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_0)));
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_17 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_29098_pipe_pc;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_0 
            = (((QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_1_sync[1U])) 
                << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_1_sync[0U])));
        vlSelfRef.rspu_top__DOT__downlink_data_1_sync[0U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[0U];
        vlSelfRef.rspu_top__DOT__downlink_data_1_sync[1U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[1U];
        vlSelfRef.rspu_top__DOT__downlink_data_1_sync[2U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[2U];
        vlSelfRef.rspu_top__DOT__downlink_data_1_sync[3U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[3U];
        vlSelfRef.rspu_top__DOT__downlink_valid_2_sync 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_2_sync;
        vlSelfRef.rspu_top__DOT__tx_valid_16 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_16 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_774_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_16 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_16 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_16)));
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_33 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_20538_pipe_pc;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_0 
            = (((QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_2_sync[1U])) 
                << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_2_sync[0U])));
        vlSelfRef.rspu_top__DOT__downlink_data_2_sync[0U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[0U];
        vlSelfRef.rspu_top__DOT__downlink_data_2_sync[1U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[1U];
        vlSelfRef.rspu_top__DOT__downlink_data_2_sync[2U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[2U];
        vlSelfRef.rspu_top__DOT__downlink_data_2_sync[3U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[3U];
        vlSelfRef.rspu_top__DOT__downlink_valid_3_sync 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_3_sync;
        vlSelfRef.rspu_top__DOT__tx_valid_32 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_32 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_806_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_32 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_32 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_32)));
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_49 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_11978_pipe_pc;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_0 
            = (((QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_3_sync[1U])) 
                << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_3_sync[0U])));
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_1 = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_1 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_37658_pipe_pc;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_0 = 0ULL;
        vlSelfRef.rspu_top__DOT__downlink_data_0_sync[0U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[0U];
        vlSelfRef.rspu_top__DOT__downlink_data_0_sync[1U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[1U];
        vlSelfRef.rspu_top__DOT__downlink_data_0_sync[2U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[2U];
        vlSelfRef.rspu_top__DOT__downlink_data_0_sync[3U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[3U];
        vlSelfRef.rspu_top__DOT__downlink_valid_1_sync 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_1_sync;
        vlSelfRef.rspu_top__DOT__tx_valid_0 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_0 = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_17 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_29098_pipe_pc;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_0 = 0ULL;
        vlSelfRef.rspu_top__DOT__downlink_data_1_sync[0U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[0U];
        vlSelfRef.rspu_top__DOT__downlink_data_1_sync[1U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[1U];
        vlSelfRef.rspu_top__DOT__downlink_data_1_sync[2U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[2U];
        vlSelfRef.rspu_top__DOT__downlink_data_1_sync[3U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[3U];
        vlSelfRef.rspu_top__DOT__downlink_valid_2_sync 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_2_sync;
        vlSelfRef.rspu_top__DOT__tx_valid_16 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_16 = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_33 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_20538_pipe_pc;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_0 = 0ULL;
        vlSelfRef.rspu_top__DOT__downlink_data_2_sync[0U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[0U];
        vlSelfRef.rspu_top__DOT__downlink_data_2_sync[1U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[1U];
        vlSelfRef.rspu_top__DOT__downlink_data_2_sync[2U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[2U];
        vlSelfRef.rspu_top__DOT__downlink_data_2_sync[3U] 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[3U];
        vlSelfRef.rspu_top__DOT__downlink_valid_3_sync 
            = vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_3_sync;
        vlSelfRef.rspu_top__DOT__tx_valid_32 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_32 = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_49 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_11978_pipe_pc;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_0 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__downlink_data_3_sync[0U] 
        = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[0U];
    vlSelfRef.rspu_top__DOT__downlink_data_3_sync[1U] 
        = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[1U];
    vlSelfRef.rspu_top__DOT__downlink_data_3_sync[2U] 
        = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[2U];
    vlSelfRef.rspu_top__DOT__downlink_data_3_sync[3U] 
        = vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[3U];
    vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_37862_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_742_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_29302_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_774_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_20742_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_806_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_trap)));
    vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_12182_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_48 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_48 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_838_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_48 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_48 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_48)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_48 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_48 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_838_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_trap)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_0 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_38193_pipe_pc;
        vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_16 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_29633_pipe_pc;
        vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_32 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_21073_pipe_pc;
        vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_48 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_0 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_38193_pipe_pc;
        vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_16 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_29633_pipe_pc;
        vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_32 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_pipe_pc 
            = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_21073_pipe_pc;
        vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_48 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_pipe_pc 
        = vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_12513_pipe_pc;
    vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_38397_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_trap));
    vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_29837_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_trap));
    vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_21277_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_trap));
    vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_12717_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_trap = 1U;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_trap = 1U;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_trap = 1U;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_trap = 1U;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_trap = 1U;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_trap = 1U;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_trap = 1U;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_trap = 1U;
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_trap = 0U;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op)));
    vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op)));
    vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op)));
    vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op)));
    vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op)));
    vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op)));
    vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op)));
    vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && ((((((0ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op) 
                                            & (1ULL 
                                               != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op)) 
                                           & (2ULL 
                                              != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op)) 
                                          & (3ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op)) 
                                         & (5ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op)) 
                                        & (6ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op)));
}

void Vrspu_top___024root___nba_sequent__TOP__2(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__2\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_4694__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__3(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__3\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_5229__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__4(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__4\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_5764__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__5(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__5\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_6299__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__6(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__6\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_6834__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__7(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__7\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_7369__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__8(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__8\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_7904__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__9(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__9\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_8439__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__10(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__10\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_8974__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__11(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__11\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_9509__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__12(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__12\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_10044__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__13(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__13\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_10579__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__14(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__14\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_11114__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__15(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__15\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_11649__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__16(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__16\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_12184__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__17(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__17\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_12719__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__18(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__18\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_13254__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__19(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__19\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_13789__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__20(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__20\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_14324__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__21(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__21\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_14859__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__22(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__22\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_15394__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__23(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__23\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_15929__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__24(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__24\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_16464__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__25(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__25\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_16999__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__26(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__26\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_17534__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__27(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__27\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_18069__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__28(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__28\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_18604__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__29(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__29\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_19139__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__30(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__30\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_19674__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__31(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__31\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_20209__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__32(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__32\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_20744__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__33(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__33\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_21279__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__34(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__34\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_21814__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__35(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__35\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_22349__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__36(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__36\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_22884__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__37(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__37\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_23419__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__38(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__38\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_23954__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__39(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__39\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_24489__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__40(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__40\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_25024__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__41(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__41\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_25559__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__42(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__42\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_26094__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__43(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__43\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_26629__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__44(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__44\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_27164__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__45(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__45\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_27699__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__46(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__46\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_28234__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__47(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__47\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_28769__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__48(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__48\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_29304__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__49(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__49\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_29839__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__50(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__50\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_30374__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__51(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__51\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_30909__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__52(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__52\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_31444__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__53(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__53\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_31979__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__54(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__54\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_32514__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__55(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__55\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_33049__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__56(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__56\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_33584__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__57(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__57\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_34119__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__58(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__58\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_34654__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__59(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__59\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_35189__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__60(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__60\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_35724__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__61(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__61\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_36259__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__62(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__62\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_36794__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__63(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__63\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_37329__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__64(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__64\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_37864__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__65(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__65\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __VdlyVal__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0;
    __VdlyVal__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 = 0;
    SData/*9:0*/ __VdlyDim0__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0;
    __VdlyDim0__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 = 0;
    CData/*0:0*/ __VdlySet__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0;
    __VdlySet__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 = 0;
    // Body
    __VdlySet__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 = 0U;
    if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_we) {
        __VdlyVal__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_res;
        __VdlyDim0__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 
            = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_rd));
        __VdlySet__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 = 1U;
    }
    if (__VdlySet__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0) {
        vlSelfRef.rspu_top__DOT__reg_regfile_inst_38399__DOT__regs[__VdlyDim0__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0] 
            = __VdlyVal__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0;
    }
}

void Vrspu_top___024root___nba_sequent__TOP__66(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__66\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_mem_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_we 
                = ((0x00000000000000ffULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_mem_op) 
                   & (1ULL != vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_mem_op));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_is_load)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_mem_res;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_is_load) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_out;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_we = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_res = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_is_load 
        = ((IData)(vlSelfRef.rst_n) && (6ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_mem_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_if_id_instr, 0x00000010U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_mem_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_mem_res 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_packed;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_rd 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_rd;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_mem_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_rd 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_if_id_instr, 0x00000010U));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_63_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_62_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_61_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_60_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_59_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_58_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_57_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_56_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_55_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_54_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_53_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_52_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_51_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_50_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_49_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_48_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_47_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_46_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_45_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_44_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_43_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_42_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_41_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_40_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_39_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_38_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_37_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_36_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_35_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_34_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_33_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_32_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_31_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_30_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_29_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_28_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_27_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_26_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_25_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_24_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_23_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_22_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_21_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_20_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_19_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_18_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_17_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_16_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_15_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_14_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_13_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_12_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_11_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_10_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_9_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_8_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_7_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_6_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_5_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_4_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_3_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_2_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_1_sync[0U]))));
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_is_load_in)))) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_packed 
                = (((0x000000f000000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_out_p, 0x00000024U)) 
                    | (0x0000000f00000000ULL & VL_SHIFTL_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_out_t, 0x00000020U))) 
                   | (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_out_d));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_is_load_in) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_packed 
                = (0x0000000100000000ULL | (0x00000000ffffffffULL 
                                            & (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_0_sync[0U]))));
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_data);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_out_p 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_prov);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_out_t 
            = (0x000000000000000fULL & vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_tag);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_out_d 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_data);
        vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_p1)), 4U);
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_t1;
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_mem_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_mem_res = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_mem_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_packed = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_rd = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_out_p = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_out_t = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_out_d = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_4708_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_5243_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_5778_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_6313_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_6848_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_7383_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_7918_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_8453_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_8988_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_9523_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_10058_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_10593_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_11128_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_11663_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_12198_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_12733_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_13268_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_13803_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_14338_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_14873_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_15408_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_15943_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_16478_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_17013_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_17548_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_18083_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_18618_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_19153_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_19688_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_20223_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_20758_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_21293_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_21828_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_22363_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_22898_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_23433_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_23968_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_24503_res_data = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_tag = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__io_in_63_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_63_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_63_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_63_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_62_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_62_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_62_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_62_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_61_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_61_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_61_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_61_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_60_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_60_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_60_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_60_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_59_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_59_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_59_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_59_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_58_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_58_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_58_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_58_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_57_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_57_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_57_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_57_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_56_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_56_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_56_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_56_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_55_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_55_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_55_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_55_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_54_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_54_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_54_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_54_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_53_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_53_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_53_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_53_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_52_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_52_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_52_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_52_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_51_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_51_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_51_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_51_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_50_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_50_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_50_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_50_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_49_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_49_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_49_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_49_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_48_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_48_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_48_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_48_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_47_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_47_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_47_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_47_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_46_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_46_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_46_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_46_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_45_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_45_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_45_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_45_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_44_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_44_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_44_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_44_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_43_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_43_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_43_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_43_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_42_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_42_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_42_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_42_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_41_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_41_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_41_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_41_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_40_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_40_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_40_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_40_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_39_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_39_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_39_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_39_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_38_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_38_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_38_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_38_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_37_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_37_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_37_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_37_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_36_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_36_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_36_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_36_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_35_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_35_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_35_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_35_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_34_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_34_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_34_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_34_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_33_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_33_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_33_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_33_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_32_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_32_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_32_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_32_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_31_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_31_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_31_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_31_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_30_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_30_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_30_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_30_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_29_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_29_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_29_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_29_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_28_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_28_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_28_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_28_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_27_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_27_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_27_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_27_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_26_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_26_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_26_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_26_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_25_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_25_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_25_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_25_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_24_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_24_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_24_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_24_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_23_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_23_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_23_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_23_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_22_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_22_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_22_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_22_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_21_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_21_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_21_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_21_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_20_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_20_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_20_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_20_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_19_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_19_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_19_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_19_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_18_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_18_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_18_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_18_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_17_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_17_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_17_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_17_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_16_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_16_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_16_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_16_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_15_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_15_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_15_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_15_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_14_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_14_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_14_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_14_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_13_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_13_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_13_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_13_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_12_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_12_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_12_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_12_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_11_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_11_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_11_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_11_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_10_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_10_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_10_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_10_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_9_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_9_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_9_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_9_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_8_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_8_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_8_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_8_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_7_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_7_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_7_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_7_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_6_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_6_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_6_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_6_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_5_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_5_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_5_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_5_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_4_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_4_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_4_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_4_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_3_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_3_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_3_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_3_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_2_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_2_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_2_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_2_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_1_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_1_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_1_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_1_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[3U];
    vlSelfRef.rspu_top__DOT__io_in_0_sync[0U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[0U];
    vlSelfRef.rspu_top__DOT__io_in_0_sync[1U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[1U];
    vlSelfRef.rspu_top__DOT__io_in_0_sync[2U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[2U];
    vlSelfRef.rspu_top__DOT__io_in_0_sync[3U] = vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[3U];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_is_load_in 
        = ((IData)(vlSelfRef.rst_n) && (0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op));
}

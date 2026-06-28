// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vrspu_top.h for the primary calling header

#include "Vrspu_top__pch.h"

void Vrspu_top___024root___nba_sequent__TOP__67(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__67\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_5243_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_5778_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_6313_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_6848_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_7383_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_7918_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_8453_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_8988_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_9523_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_10058_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_10593_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_11128_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_11663_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_12198_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_12733_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_14338_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_14873_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_15408_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_15943_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_16478_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_17013_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_17548_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_18083_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_18618_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_19153_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_19688_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_20223_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_20758_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_21293_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_22898_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_23433_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_23968_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_24503_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_25038_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_25038_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_25573_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_25573_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_26108_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_26108_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_26643_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_26643_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_27178_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_27178_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_27713_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_27713_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_28248_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_28248_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_28783_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_28783_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_29318_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_29318_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_29853_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_29853_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_30388_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_30923_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_31458_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_31458_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_31993_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_31993_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_32528_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_32528_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_33063_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_33063_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_33598_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_33598_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_34133_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_34133_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_34668_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_34668_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_35203_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_35203_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_35738_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_35738_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_36273_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_36273_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_36808_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_36808_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_37343_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_37343_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_37878_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_37878_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_prov 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_p1;
        vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_tag 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_t1;
        vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_data 
            = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d2;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_add) {
            vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d1) 
                   + (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_sub) {
            vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_data 
                = ((0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d1) 
                   - (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d2));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_mul) {
            vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_data 
                = (0x00000000ffffffffULL & ((0x00000000ffffffffULL 
                                             & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d1) 
                                            * (0x00000000ffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d2)));
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_relu_pos) {
            vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_data 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d1;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_relu_neg) {
            vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_data = 0ULL;
        }
        if (vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_tag_gate) {
            vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_data 
                = VL_SHIFTR_QQI(64,64,32, ((0x00000000ffffffffULL 
                                            & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d1) 
                                           * (0x000000000000000fULL 
                                              & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_p1)), 4U);
        }
    } else {
        vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_prov = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_tag = 0ULL;
        vlSelfRef.rspu_top__DOT__alu_core_call_38413_res_data = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_tag_gate 
        = ((IData)(vlSelfRef.rst_n) && (5ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op));
    vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_relu_pos 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op) 
                                        & (0ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_relu_neg 
        = ((IData)(vlSelfRef.rst_n) && ((3ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op) 
                                        & (1ULL == 
                                           VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d1, 0x0000001fU))));
    vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_add 
        = ((IData)(vlSelfRef.rst_n) && ((0ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_sub 
        = ((IData)(vlSelfRef.rst_n) && ((1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_t2)));
    vlSelfRef.rspu_top__DOT__alu_core_call_38413_is_mul 
        = ((IData)(vlSelfRef.rst_n) && ((2ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op) 
                                        & (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_t1 
                                           == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_t2)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d2 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_val2);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_p1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_val1, 0x00000024U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_868_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_866_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_864_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_862_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_860_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_858_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_856_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_854_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_852_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_850_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_848_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_846_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_844_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_842_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_840_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_838_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_836_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_834_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_832_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_830_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_828_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_826_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_824_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_822_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_820_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_818_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_816_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_814_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_812_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_810_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_808_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_806_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_804_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_802_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_800_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_798_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_796_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_794_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_792_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_790_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_788_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_786_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_784_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_782_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_780_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_778_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_776_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_774_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_772_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_770_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_768_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_766_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_764_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_762_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_760_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_758_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_756_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_754_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_752_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_750_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_748_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_746_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_744_current_instr;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d1 
            = (0x00000000ffffffffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_val1);
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_t2 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_val2, 0x00000020U));
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_t1 
            = (0x000000000000000fULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_val1, 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_pcc_valid) {
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_op;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_val2 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_val2;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_val1 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_val1;
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_op 
                = (0x00000000000000ffULL & vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_if_id_instr);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_if_id_instr 
                = vlSelfRef.rspu_top__DOT__core_top_call_742_current_instr;
        }
    } else {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_p1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_d1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_t2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_ex_t1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_val2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_val1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_op = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_if_id_instr = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_if_id_instr = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_868_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_868_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_866_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_866_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_864_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_864_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_862_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_862_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_860_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_860_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_858_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_858_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_856_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_856_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_854_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_854_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_852_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_852_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_850_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_850_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_848_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_848_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_846_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_846_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_844_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_844_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_842_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_842_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_840_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_840_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_838_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_838_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_836_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_836_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_834_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_834_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_832_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_832_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_830_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_830_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_828_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_828_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_826_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_826_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_824_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_824_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_822_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_822_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_820_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_820_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_818_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_818_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_816_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_816_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_814_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_814_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_812_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_812_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_810_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_810_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_808_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_808_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_806_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_806_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_804_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_804_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_802_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_802_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_800_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_800_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_798_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_798_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_796_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_796_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_794_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_794_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_792_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_792_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_790_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_790_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_788_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_788_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_786_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_786_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_784_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_784_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_782_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_782_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_780_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_780_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_778_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_778_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_776_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_776_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_774_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_774_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_772_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_772_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_770_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_770_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_768_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_768_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_766_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_766_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_764_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_764_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_762_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_762_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_760_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_760_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_758_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_758_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_756_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_756_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_754_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_754_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_752_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_752_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_750_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_750_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_748_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_748_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_746_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_746_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_744_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_744_current_instr;
    vlSelfRef.rspu_top__DOT__core_top_call_742_current_instr 
        = vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_742_current_instr;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_4692_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_5227_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_5762_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_6297_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_6832_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_7367_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_7902_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_8437_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_8972_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_9507_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_10042_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_10577_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_11112_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_11647_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_12182_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_12717_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_13252_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_13787_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_14322_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_14857_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_15392_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_15927_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_16462_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_16997_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_17532_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_18067_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_18602_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_19137_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_19672_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_20207_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_20742_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_21277_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_21812_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_22347_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_22882_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_23417_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_23952_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_24487_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_25022_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_25557_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_26092_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_26627_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_27162_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_27697_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_28232_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_28767_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_29302_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_29837_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_30372_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_30907_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_31442_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_31977_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_32512_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_33047_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_33582_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_34117_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_34652_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_35187_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_35722_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_36257_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_36792_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_37327_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_37862_is_invalid))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_pcc_valid 
        = ((IData)(vlSelfRef.rst_n) && (1U & (~ (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_38397_is_invalid))));
}

void Vrspu_top___024root___nba_sequent__TOP__68(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__68\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_4692_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_5227_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_5762_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_6297_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_6832_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_7367_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_7902_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_8437_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_8972_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_9507_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_10042_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_10577_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_11112_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_11647_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_12182_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_12717_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_13252_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_13787_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_14322_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_14857_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_15392_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_15927_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_16462_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_16997_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_17532_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_18067_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_18602_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_19137_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_19672_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_20207_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_20742_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_21277_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_21812_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_22347_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_22882_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_23417_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_23952_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_24487_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_25022_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_25557_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_26092_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_26627_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_27162_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_27697_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_28232_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_28767_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_29302_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_29837_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_30372_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_30907_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_31442_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_31977_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_32512_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_33047_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_33582_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_34117_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_34652_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_35187_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_35722_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_36257_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_36792_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_37327_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_37862_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_cert_g))));
    vlSelfRef.rspu_top__DOT__pcc_verifier_call_38397_is_invalid 
        = ((IData)(vlSelfRef.rst_n) && (((0x0400U < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_cert_i)) 
                                         | (0x0100U 
                                            < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_cert_r))) 
                                        | (0x0020U 
                                           < (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_cert_g))));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_cert_g = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_cert_i = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_cert_r = 0U;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_cert_g = 0U;
}

void Vrspu_top___024root___nba_comb__TOP__0(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__0\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_4694__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_4694__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__1(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__1\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_5229__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_5229__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__2(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__2\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_5764__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_5764__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__3(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__3\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_6299__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_6299__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__4(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__4\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_6834__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_6834__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__5(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__5\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_7369__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_7369__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__6(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__6\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_7904__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_7904__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__7(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__7\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_8439__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_8439__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__8(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__8\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_8974__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_8974__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__9(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__9\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_9509__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_9509__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__10(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__10\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_10044__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_10044__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__11(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__11\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_10579__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_10579__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__12(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__12\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_11114__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_11114__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__13(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__13\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_11649__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_11649__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__14(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__14\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_12184__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_12184__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__15(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__15\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_12719__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_12719__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__16(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__16\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_13254__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_13254__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__17(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__17\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_13789__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_13789__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__18(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__18\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_14324__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_14324__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__19(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__19\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_14859__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_14859__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__20(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__20\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_15394__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_15394__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__21(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__21\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_15929__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_15929__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__22(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__22\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_16464__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_16464__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__23(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__23\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_16999__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_16999__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__24(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__24\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_17534__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_17534__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__25(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__25\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_18069__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_18069__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__26(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__26\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_18604__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_18604__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__27(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__27\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_19139__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_19139__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__28(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__28\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_19674__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_19674__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__29(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__29\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_20209__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_20209__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__30(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__30\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_20744__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_20744__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__31(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__31\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_21279__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_21279__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__32(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__32\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_21814__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_21814__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__33(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__33\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_22349__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_22349__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__34(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__34\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_22884__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_22884__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__35(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__35\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_23419__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_23419__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__36(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__36\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_23954__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_23954__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__37(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__37\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_24489__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_24489__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__38(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__38\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_25024__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_25024__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__39(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__39\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_25559__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_25559__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__40(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__40\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_26094__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_26094__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__41(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__41\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_26629__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_26629__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__42(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__42\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_27164__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_27164__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__43(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__43\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_27699__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_27699__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__44(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__44\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_28234__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_28234__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__45(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__45\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_28769__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_28769__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__46(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__46\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_29304__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_29304__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__47(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__47\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_29839__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_29839__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__48(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__48\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_30374__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_30374__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__49(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__49\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_30909__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_30909__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__50(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__50\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_31444__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_31444__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__51(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__51\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_31979__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_31979__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__52(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__52\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_32514__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_32514__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__53(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__53\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_33049__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_33049__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__54(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__54\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_33584__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_33584__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__55(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__55\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_34119__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_34119__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__56(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__56\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_34654__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_34654__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__57(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__57\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_35189__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_35189__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__58(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__58\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_35724__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_35724__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__59(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__59\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_36259__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_36259__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__60(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__60\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_36794__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_36794__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__61(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__61\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_37329__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_37329__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__62(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__62\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_37864__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_37864__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_rs1))];
}

void Vrspu_top___024root___nba_comb__TOP__63(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_comb__TOP__63\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_val2 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_38399__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_rs2))];
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_val1 
        = vlSelfRef.rspu_top__DOT__reg_regfile_inst_38399__DOT__regs
        [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_rs1))];
}

void Vrspu_top___024root___nba_sequent__TOP__0(Vrspu_top___024root* vlSelf);
void Vrspu_top___024root___nba_sequent__TOP__1(Vrspu_top___024root* vlSelf);
void Vrspu_top___024root___nba_sequent__TOP__66(Vrspu_top___024root* vlSelf);

void Vrspu_top___024root___eval_nba(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___eval_nba\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    QData/*63:0*/ __Vinline__nba_sequent__TOP__2___VdlyVal__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__2___VdlyVal__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__2___VdlyDim0__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__2___VdlyDim0__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__2___VdlySet__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__2___VdlySet__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__3___VdlyVal__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__3___VdlyVal__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__3___VdlyDim0__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__3___VdlyDim0__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__3___VdlySet__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__3___VdlySet__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__4___VdlyVal__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__4___VdlyVal__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__4___VdlyDim0__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__4___VdlyDim0__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__4___VdlySet__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__4___VdlySet__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__5___VdlyVal__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__5___VdlyVal__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__5___VdlyDim0__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__5___VdlyDim0__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__5___VdlySet__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__5___VdlySet__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__6___VdlyVal__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__6___VdlyVal__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__6___VdlyDim0__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__6___VdlyDim0__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__6___VdlySet__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__6___VdlySet__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__7___VdlyVal__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__7___VdlyVal__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__7___VdlyDim0__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__7___VdlyDim0__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__7___VdlySet__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__7___VdlySet__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__8___VdlyVal__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__8___VdlyVal__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__8___VdlyDim0__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__8___VdlyDim0__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__8___VdlySet__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__8___VdlySet__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__9___VdlyVal__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__9___VdlyVal__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__9___VdlyDim0__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__9___VdlyDim0__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__9___VdlySet__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__9___VdlySet__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__10___VdlyVal__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__10___VdlyVal__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__10___VdlyDim0__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__10___VdlyDim0__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__10___VdlySet__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__10___VdlySet__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__11___VdlyVal__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__11___VdlyVal__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__11___VdlyDim0__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__11___VdlyDim0__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__11___VdlySet__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__11___VdlySet__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__12___VdlyVal__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__12___VdlyVal__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__12___VdlyDim0__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__12___VdlyDim0__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__12___VdlySet__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__12___VdlySet__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__13___VdlyVal__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__13___VdlyVal__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__13___VdlyDim0__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__13___VdlyDim0__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__13___VdlySet__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__13___VdlySet__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__14___VdlyVal__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__14___VdlyVal__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__14___VdlyDim0__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__14___VdlyDim0__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__14___VdlySet__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__14___VdlySet__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__15___VdlyVal__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__15___VdlyVal__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__15___VdlyDim0__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__15___VdlyDim0__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__15___VdlySet__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__15___VdlySet__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__16___VdlyVal__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__16___VdlyVal__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__16___VdlyDim0__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__16___VdlyDim0__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__16___VdlySet__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__16___VdlySet__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__17___VdlyVal__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__17___VdlyVal__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__17___VdlyDim0__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__17___VdlyDim0__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__17___VdlySet__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__17___VdlySet__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__18___VdlyVal__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__18___VdlyVal__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__18___VdlyDim0__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__18___VdlyDim0__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__18___VdlySet__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__18___VdlySet__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__19___VdlyVal__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__19___VdlyVal__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__19___VdlyDim0__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__19___VdlyDim0__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__19___VdlySet__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__19___VdlySet__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__20___VdlyVal__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__20___VdlyVal__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__20___VdlyDim0__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__20___VdlyDim0__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__20___VdlySet__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__20___VdlySet__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__21___VdlyVal__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__21___VdlyVal__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__21___VdlyDim0__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__21___VdlyDim0__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__21___VdlySet__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__21___VdlySet__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__22___VdlyVal__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__22___VdlyVal__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__22___VdlyDim0__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__22___VdlyDim0__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__22___VdlySet__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__22___VdlySet__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__23___VdlyVal__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__23___VdlyVal__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__23___VdlyDim0__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__23___VdlyDim0__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__23___VdlySet__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__23___VdlySet__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__24___VdlyVal__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__24___VdlyVal__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__24___VdlyDim0__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__24___VdlyDim0__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__24___VdlySet__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__24___VdlySet__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__25___VdlyVal__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__25___VdlyVal__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__25___VdlyDim0__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__25___VdlyDim0__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__25___VdlySet__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__25___VdlySet__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__26___VdlyVal__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__26___VdlyVal__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__26___VdlyDim0__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__26___VdlyDim0__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__26___VdlySet__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__26___VdlySet__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__27___VdlyVal__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__27___VdlyVal__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__27___VdlyDim0__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__27___VdlyDim0__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__27___VdlySet__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__27___VdlySet__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__28___VdlyVal__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__28___VdlyVal__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__28___VdlyDim0__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__28___VdlyDim0__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__28___VdlySet__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__28___VdlySet__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__29___VdlyVal__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__29___VdlyVal__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__29___VdlyDim0__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__29___VdlyDim0__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__29___VdlySet__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__29___VdlySet__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__30___VdlyVal__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__30___VdlyVal__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__30___VdlyDim0__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__30___VdlyDim0__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__30___VdlySet__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__30___VdlySet__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__31___VdlyVal__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__31___VdlyVal__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__31___VdlyDim0__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__31___VdlyDim0__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__31___VdlySet__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__31___VdlySet__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__32___VdlyVal__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__32___VdlyVal__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__32___VdlyDim0__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__32___VdlyDim0__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__32___VdlySet__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__32___VdlySet__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__33___VdlyVal__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__33___VdlyVal__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__33___VdlyDim0__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__33___VdlyDim0__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__33___VdlySet__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__33___VdlySet__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__34___VdlyVal__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__34___VdlyVal__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__34___VdlyDim0__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__34___VdlyDim0__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__34___VdlySet__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__34___VdlySet__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__35___VdlyVal__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__35___VdlyVal__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__35___VdlyDim0__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__35___VdlyDim0__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__35___VdlySet__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__35___VdlySet__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__36___VdlyVal__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__36___VdlyVal__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__36___VdlyDim0__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__36___VdlyDim0__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__36___VdlySet__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__36___VdlySet__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__37___VdlyVal__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__37___VdlyVal__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__37___VdlyDim0__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__37___VdlyDim0__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__37___VdlySet__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__37___VdlySet__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__38___VdlyVal__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__38___VdlyVal__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__38___VdlyDim0__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__38___VdlyDim0__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__38___VdlySet__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__38___VdlySet__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__39___VdlyVal__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__39___VdlyVal__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__39___VdlyDim0__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__39___VdlyDim0__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__39___VdlySet__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__39___VdlySet__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__40___VdlyVal__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__40___VdlyVal__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__40___VdlyDim0__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__40___VdlyDim0__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__40___VdlySet__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__40___VdlySet__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__41___VdlyVal__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__41___VdlyVal__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__41___VdlyDim0__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__41___VdlyDim0__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__41___VdlySet__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__41___VdlySet__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__42___VdlyVal__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__42___VdlyVal__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__42___VdlyDim0__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__42___VdlyDim0__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__42___VdlySet__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__42___VdlySet__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__43___VdlyVal__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__43___VdlyVal__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__43___VdlyDim0__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__43___VdlyDim0__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__43___VdlySet__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__43___VdlySet__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__44___VdlyVal__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__44___VdlyVal__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__44___VdlyDim0__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__44___VdlyDim0__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__44___VdlySet__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__44___VdlySet__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__45___VdlyVal__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__45___VdlyVal__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__45___VdlyDim0__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__45___VdlyDim0__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__45___VdlySet__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__45___VdlySet__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__46___VdlyVal__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__46___VdlyVal__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__46___VdlyDim0__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__46___VdlyDim0__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__46___VdlySet__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__46___VdlySet__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__47___VdlyVal__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__47___VdlyVal__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__47___VdlyDim0__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__47___VdlyDim0__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__47___VdlySet__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__47___VdlySet__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__48___VdlyVal__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__48___VdlyVal__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__48___VdlyDim0__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__48___VdlyDim0__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__48___VdlySet__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__48___VdlySet__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__49___VdlyVal__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__49___VdlyVal__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__49___VdlyDim0__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__49___VdlyDim0__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__49___VdlySet__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__49___VdlySet__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__50___VdlyVal__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__50___VdlyVal__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__50___VdlyDim0__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__50___VdlyDim0__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__50___VdlySet__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__50___VdlySet__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__51___VdlyVal__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__51___VdlyVal__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__51___VdlyDim0__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__51___VdlyDim0__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__51___VdlySet__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__51___VdlySet__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__52___VdlyVal__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__52___VdlyVal__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__52___VdlyDim0__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__52___VdlyDim0__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__52___VdlySet__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__52___VdlySet__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__53___VdlyVal__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__53___VdlyVal__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__53___VdlyDim0__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__53___VdlyDim0__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__53___VdlySet__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__53___VdlySet__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__54___VdlyVal__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__54___VdlyVal__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__54___VdlyDim0__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__54___VdlyDim0__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__54___VdlySet__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__54___VdlySet__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__55___VdlyVal__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__55___VdlyVal__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__55___VdlyDim0__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__55___VdlyDim0__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__55___VdlySet__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__55___VdlySet__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__56___VdlyVal__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__56___VdlyVal__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__56___VdlyDim0__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__56___VdlyDim0__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__56___VdlySet__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__56___VdlySet__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__57___VdlyVal__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__57___VdlyVal__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__57___VdlyDim0__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__57___VdlyDim0__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__57___VdlySet__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__57___VdlySet__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__58___VdlyVal__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__58___VdlyVal__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__58___VdlyDim0__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__58___VdlyDim0__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__58___VdlySet__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__58___VdlySet__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__59___VdlyVal__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__59___VdlyVal__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__59___VdlyDim0__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__59___VdlyDim0__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__59___VdlySet__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__59___VdlySet__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__60___VdlyVal__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__60___VdlyVal__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__60___VdlyDim0__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__60___VdlyDim0__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__60___VdlySet__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__60___VdlySet__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__61___VdlyVal__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__61___VdlyVal__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__61___VdlyDim0__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__61___VdlyDim0__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__61___VdlySet__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__61___VdlySet__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__62___VdlyVal__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__62___VdlyVal__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__62___VdlyDim0__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__62___VdlyDim0__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__62___VdlySet__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__62___VdlySet__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__63___VdlyVal__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__63___VdlyVal__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__63___VdlyDim0__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__63___VdlyDim0__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__63___VdlySet__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__63___VdlySet__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__64___VdlyVal__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__64___VdlyVal__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__64___VdlyDim0__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__64___VdlyDim0__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__64___VdlySet__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__64___VdlySet__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 = 0;
    QData/*63:0*/ __Vinline__nba_sequent__TOP__65___VdlyVal__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__65___VdlyVal__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 = 0;
    SData/*9:0*/ __Vinline__nba_sequent__TOP__65___VdlyDim0__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__65___VdlyDim0__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 = 0;
    CData/*0:0*/ __Vinline__nba_sequent__TOP__65___VdlySet__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0;
    __Vinline__nba_sequent__TOP__65___VdlySet__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 = 0;
    // Body
    if ((3ULL & vlSelfRef.__VnbaTriggered[0U])) {
        Vrspu_top___024root___nba_sequent__TOP__0(vlSelf);
        Vrspu_top___024root___nba_sequent__TOP__1(vlSelf);
    }
    if ((4ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__2___VdlySet__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_we) {
            __Vinline__nba_sequent__TOP__2___VdlyVal__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_res;
            __Vinline__nba_sequent__TOP__2___VdlyDim0__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_rd));
            __Vinline__nba_sequent__TOP__2___VdlySet__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__2___VdlySet__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_4694__DOT__regs[__Vinline__nba_sequent__TOP__2___VdlyDim0__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__2___VdlyVal__rspu_top__DOT__reg_regfile_inst_4694__DOT__regs__v0;
        }
    }
    if ((8ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__3___VdlySet__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_we) {
            __Vinline__nba_sequent__TOP__3___VdlyVal__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_res;
            __Vinline__nba_sequent__TOP__3___VdlyDim0__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_rd));
            __Vinline__nba_sequent__TOP__3___VdlySet__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__3___VdlySet__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_5229__DOT__regs[__Vinline__nba_sequent__TOP__3___VdlyDim0__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__3___VdlyVal__rspu_top__DOT__reg_regfile_inst_5229__DOT__regs__v0;
        }
    }
    if ((0x0000000000000010ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__4___VdlySet__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_we) {
            __Vinline__nba_sequent__TOP__4___VdlyVal__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_res;
            __Vinline__nba_sequent__TOP__4___VdlyDim0__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_rd));
            __Vinline__nba_sequent__TOP__4___VdlySet__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__4___VdlySet__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_5764__DOT__regs[__Vinline__nba_sequent__TOP__4___VdlyDim0__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__4___VdlyVal__rspu_top__DOT__reg_regfile_inst_5764__DOT__regs__v0;
        }
    }
    if ((0x0000000000000020ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__5___VdlySet__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_we) {
            __Vinline__nba_sequent__TOP__5___VdlyVal__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_res;
            __Vinline__nba_sequent__TOP__5___VdlyDim0__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_rd));
            __Vinline__nba_sequent__TOP__5___VdlySet__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__5___VdlySet__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_6299__DOT__regs[__Vinline__nba_sequent__TOP__5___VdlyDim0__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__5___VdlyVal__rspu_top__DOT__reg_regfile_inst_6299__DOT__regs__v0;
        }
    }
    if ((0x0000000000000040ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__6___VdlySet__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_we) {
            __Vinline__nba_sequent__TOP__6___VdlyVal__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_res;
            __Vinline__nba_sequent__TOP__6___VdlyDim0__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_rd));
            __Vinline__nba_sequent__TOP__6___VdlySet__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__6___VdlySet__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_6834__DOT__regs[__Vinline__nba_sequent__TOP__6___VdlyDim0__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__6___VdlyVal__rspu_top__DOT__reg_regfile_inst_6834__DOT__regs__v0;
        }
    }
    if ((0x0000000000000080ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__7___VdlySet__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_we) {
            __Vinline__nba_sequent__TOP__7___VdlyVal__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_res;
            __Vinline__nba_sequent__TOP__7___VdlyDim0__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_rd));
            __Vinline__nba_sequent__TOP__7___VdlySet__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__7___VdlySet__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_7369__DOT__regs[__Vinline__nba_sequent__TOP__7___VdlyDim0__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__7___VdlyVal__rspu_top__DOT__reg_regfile_inst_7369__DOT__regs__v0;
        }
    }
    if ((0x0000000000000100ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__8___VdlySet__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_we) {
            __Vinline__nba_sequent__TOP__8___VdlyVal__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_res;
            __Vinline__nba_sequent__TOP__8___VdlyDim0__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_rd));
            __Vinline__nba_sequent__TOP__8___VdlySet__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__8___VdlySet__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_7904__DOT__regs[__Vinline__nba_sequent__TOP__8___VdlyDim0__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__8___VdlyVal__rspu_top__DOT__reg_regfile_inst_7904__DOT__regs__v0;
        }
    }
    if ((0x0000000000000200ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__9___VdlySet__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_we) {
            __Vinline__nba_sequent__TOP__9___VdlyVal__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_res;
            __Vinline__nba_sequent__TOP__9___VdlyDim0__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_rd));
            __Vinline__nba_sequent__TOP__9___VdlySet__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__9___VdlySet__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_8439__DOT__regs[__Vinline__nba_sequent__TOP__9___VdlyDim0__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__9___VdlyVal__rspu_top__DOT__reg_regfile_inst_8439__DOT__regs__v0;
        }
    }
    if ((0x0000000000000400ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__10___VdlySet__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_we) {
            __Vinline__nba_sequent__TOP__10___VdlyVal__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_res;
            __Vinline__nba_sequent__TOP__10___VdlyDim0__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_rd));
            __Vinline__nba_sequent__TOP__10___VdlySet__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__10___VdlySet__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_8974__DOT__regs[__Vinline__nba_sequent__TOP__10___VdlyDim0__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__10___VdlyVal__rspu_top__DOT__reg_regfile_inst_8974__DOT__regs__v0;
        }
    }
    if ((0x0000000000000800ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__11___VdlySet__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_we) {
            __Vinline__nba_sequent__TOP__11___VdlyVal__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_res;
            __Vinline__nba_sequent__TOP__11___VdlyDim0__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_rd));
            __Vinline__nba_sequent__TOP__11___VdlySet__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__11___VdlySet__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_9509__DOT__regs[__Vinline__nba_sequent__TOP__11___VdlyDim0__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__11___VdlyVal__rspu_top__DOT__reg_regfile_inst_9509__DOT__regs__v0;
        }
    }
    if ((0x0000000000001000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__12___VdlySet__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_we) {
            __Vinline__nba_sequent__TOP__12___VdlyVal__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_res;
            __Vinline__nba_sequent__TOP__12___VdlyDim0__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_rd));
            __Vinline__nba_sequent__TOP__12___VdlySet__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__12___VdlySet__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_10044__DOT__regs[__Vinline__nba_sequent__TOP__12___VdlyDim0__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__12___VdlyVal__rspu_top__DOT__reg_regfile_inst_10044__DOT__regs__v0;
        }
    }
    if ((0x0000000000002000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__13___VdlySet__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_we) {
            __Vinline__nba_sequent__TOP__13___VdlyVal__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_res;
            __Vinline__nba_sequent__TOP__13___VdlyDim0__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_rd));
            __Vinline__nba_sequent__TOP__13___VdlySet__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__13___VdlySet__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_10579__DOT__regs[__Vinline__nba_sequent__TOP__13___VdlyDim0__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__13___VdlyVal__rspu_top__DOT__reg_regfile_inst_10579__DOT__regs__v0;
        }
    }
    if ((0x0000000000004000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__14___VdlySet__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_we) {
            __Vinline__nba_sequent__TOP__14___VdlyVal__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_res;
            __Vinline__nba_sequent__TOP__14___VdlyDim0__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_rd));
            __Vinline__nba_sequent__TOP__14___VdlySet__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__14___VdlySet__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_11114__DOT__regs[__Vinline__nba_sequent__TOP__14___VdlyDim0__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__14___VdlyVal__rspu_top__DOT__reg_regfile_inst_11114__DOT__regs__v0;
        }
    }
    if ((0x0000000000008000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__15___VdlySet__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_we) {
            __Vinline__nba_sequent__TOP__15___VdlyVal__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_res;
            __Vinline__nba_sequent__TOP__15___VdlyDim0__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_rd));
            __Vinline__nba_sequent__TOP__15___VdlySet__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__15___VdlySet__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_11649__DOT__regs[__Vinline__nba_sequent__TOP__15___VdlyDim0__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__15___VdlyVal__rspu_top__DOT__reg_regfile_inst_11649__DOT__regs__v0;
        }
    }
    if ((0x0000000000010000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__16___VdlySet__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_we) {
            __Vinline__nba_sequent__TOP__16___VdlyVal__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_res;
            __Vinline__nba_sequent__TOP__16___VdlyDim0__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_rd));
            __Vinline__nba_sequent__TOP__16___VdlySet__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__16___VdlySet__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_12184__DOT__regs[__Vinline__nba_sequent__TOP__16___VdlyDim0__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__16___VdlyVal__rspu_top__DOT__reg_regfile_inst_12184__DOT__regs__v0;
        }
    }
    if ((0x0000000000020000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__17___VdlySet__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_we) {
            __Vinline__nba_sequent__TOP__17___VdlyVal__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_res;
            __Vinline__nba_sequent__TOP__17___VdlyDim0__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_rd));
            __Vinline__nba_sequent__TOP__17___VdlySet__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__17___VdlySet__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_12719__DOT__regs[__Vinline__nba_sequent__TOP__17___VdlyDim0__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__17___VdlyVal__rspu_top__DOT__reg_regfile_inst_12719__DOT__regs__v0;
        }
    }
    if ((0x0000000000040000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__18___VdlySet__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_we) {
            __Vinline__nba_sequent__TOP__18___VdlyVal__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_res;
            __Vinline__nba_sequent__TOP__18___VdlyDim0__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_rd));
            __Vinline__nba_sequent__TOP__18___VdlySet__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__18___VdlySet__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_13254__DOT__regs[__Vinline__nba_sequent__TOP__18___VdlyDim0__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__18___VdlyVal__rspu_top__DOT__reg_regfile_inst_13254__DOT__regs__v0;
        }
    }
    if ((0x0000000000080000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__19___VdlySet__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_we) {
            __Vinline__nba_sequent__TOP__19___VdlyVal__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_res;
            __Vinline__nba_sequent__TOP__19___VdlyDim0__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_rd));
            __Vinline__nba_sequent__TOP__19___VdlySet__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__19___VdlySet__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_13789__DOT__regs[__Vinline__nba_sequent__TOP__19___VdlyDim0__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__19___VdlyVal__rspu_top__DOT__reg_regfile_inst_13789__DOT__regs__v0;
        }
    }
    if ((0x0000000000100000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__20___VdlySet__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_we) {
            __Vinline__nba_sequent__TOP__20___VdlyVal__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_res;
            __Vinline__nba_sequent__TOP__20___VdlyDim0__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_rd));
            __Vinline__nba_sequent__TOP__20___VdlySet__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__20___VdlySet__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_14324__DOT__regs[__Vinline__nba_sequent__TOP__20___VdlyDim0__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__20___VdlyVal__rspu_top__DOT__reg_regfile_inst_14324__DOT__regs__v0;
        }
    }
    if ((0x0000000000200000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__21___VdlySet__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_we) {
            __Vinline__nba_sequent__TOP__21___VdlyVal__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_res;
            __Vinline__nba_sequent__TOP__21___VdlyDim0__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_rd));
            __Vinline__nba_sequent__TOP__21___VdlySet__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__21___VdlySet__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_14859__DOT__regs[__Vinline__nba_sequent__TOP__21___VdlyDim0__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__21___VdlyVal__rspu_top__DOT__reg_regfile_inst_14859__DOT__regs__v0;
        }
    }
    if ((0x0000000000400000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__22___VdlySet__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_we) {
            __Vinline__nba_sequent__TOP__22___VdlyVal__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_res;
            __Vinline__nba_sequent__TOP__22___VdlyDim0__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_rd));
            __Vinline__nba_sequent__TOP__22___VdlySet__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__22___VdlySet__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_15394__DOT__regs[__Vinline__nba_sequent__TOP__22___VdlyDim0__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__22___VdlyVal__rspu_top__DOT__reg_regfile_inst_15394__DOT__regs__v0;
        }
    }
    if ((0x0000000000800000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__23___VdlySet__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_we) {
            __Vinline__nba_sequent__TOP__23___VdlyVal__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_res;
            __Vinline__nba_sequent__TOP__23___VdlyDim0__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_rd));
            __Vinline__nba_sequent__TOP__23___VdlySet__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__23___VdlySet__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_15929__DOT__regs[__Vinline__nba_sequent__TOP__23___VdlyDim0__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__23___VdlyVal__rspu_top__DOT__reg_regfile_inst_15929__DOT__regs__v0;
        }
    }
    if ((0x0000000001000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__24___VdlySet__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_we) {
            __Vinline__nba_sequent__TOP__24___VdlyVal__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_res;
            __Vinline__nba_sequent__TOP__24___VdlyDim0__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_rd));
            __Vinline__nba_sequent__TOP__24___VdlySet__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__24___VdlySet__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_16464__DOT__regs[__Vinline__nba_sequent__TOP__24___VdlyDim0__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__24___VdlyVal__rspu_top__DOT__reg_regfile_inst_16464__DOT__regs__v0;
        }
    }
    if ((0x0000000002000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__25___VdlySet__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_we) {
            __Vinline__nba_sequent__TOP__25___VdlyVal__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_res;
            __Vinline__nba_sequent__TOP__25___VdlyDim0__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_rd));
            __Vinline__nba_sequent__TOP__25___VdlySet__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__25___VdlySet__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_16999__DOT__regs[__Vinline__nba_sequent__TOP__25___VdlyDim0__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__25___VdlyVal__rspu_top__DOT__reg_regfile_inst_16999__DOT__regs__v0;
        }
    }
    if ((0x0000000004000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__26___VdlySet__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_we) {
            __Vinline__nba_sequent__TOP__26___VdlyVal__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_res;
            __Vinline__nba_sequent__TOP__26___VdlyDim0__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_rd));
            __Vinline__nba_sequent__TOP__26___VdlySet__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__26___VdlySet__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_17534__DOT__regs[__Vinline__nba_sequent__TOP__26___VdlyDim0__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__26___VdlyVal__rspu_top__DOT__reg_regfile_inst_17534__DOT__regs__v0;
        }
    }
    if ((0x0000000008000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__27___VdlySet__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_we) {
            __Vinline__nba_sequent__TOP__27___VdlyVal__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_res;
            __Vinline__nba_sequent__TOP__27___VdlyDim0__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_rd));
            __Vinline__nba_sequent__TOP__27___VdlySet__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__27___VdlySet__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_18069__DOT__regs[__Vinline__nba_sequent__TOP__27___VdlyDim0__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__27___VdlyVal__rspu_top__DOT__reg_regfile_inst_18069__DOT__regs__v0;
        }
    }
    if ((0x0000000010000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__28___VdlySet__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_we) {
            __Vinline__nba_sequent__TOP__28___VdlyVal__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_res;
            __Vinline__nba_sequent__TOP__28___VdlyDim0__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_rd));
            __Vinline__nba_sequent__TOP__28___VdlySet__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__28___VdlySet__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_18604__DOT__regs[__Vinline__nba_sequent__TOP__28___VdlyDim0__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__28___VdlyVal__rspu_top__DOT__reg_regfile_inst_18604__DOT__regs__v0;
        }
    }
    if ((0x0000000020000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__29___VdlySet__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_we) {
            __Vinline__nba_sequent__TOP__29___VdlyVal__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_res;
            __Vinline__nba_sequent__TOP__29___VdlyDim0__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_rd));
            __Vinline__nba_sequent__TOP__29___VdlySet__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__29___VdlySet__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_19139__DOT__regs[__Vinline__nba_sequent__TOP__29___VdlyDim0__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__29___VdlyVal__rspu_top__DOT__reg_regfile_inst_19139__DOT__regs__v0;
        }
    }
    if ((0x0000000040000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__30___VdlySet__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_we) {
            __Vinline__nba_sequent__TOP__30___VdlyVal__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_res;
            __Vinline__nba_sequent__TOP__30___VdlyDim0__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_rd));
            __Vinline__nba_sequent__TOP__30___VdlySet__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__30___VdlySet__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_19674__DOT__regs[__Vinline__nba_sequent__TOP__30___VdlyDim0__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__30___VdlyVal__rspu_top__DOT__reg_regfile_inst_19674__DOT__regs__v0;
        }
    }
    if ((0x0000000080000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__31___VdlySet__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_we) {
            __Vinline__nba_sequent__TOP__31___VdlyVal__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_res;
            __Vinline__nba_sequent__TOP__31___VdlyDim0__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_rd));
            __Vinline__nba_sequent__TOP__31___VdlySet__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__31___VdlySet__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_20209__DOT__regs[__Vinline__nba_sequent__TOP__31___VdlyDim0__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__31___VdlyVal__rspu_top__DOT__reg_regfile_inst_20209__DOT__regs__v0;
        }
    }
    if ((0x0000000100000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__32___VdlySet__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_we) {
            __Vinline__nba_sequent__TOP__32___VdlyVal__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_res;
            __Vinline__nba_sequent__TOP__32___VdlyDim0__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_rd));
            __Vinline__nba_sequent__TOP__32___VdlySet__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__32___VdlySet__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_20744__DOT__regs[__Vinline__nba_sequent__TOP__32___VdlyDim0__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__32___VdlyVal__rspu_top__DOT__reg_regfile_inst_20744__DOT__regs__v0;
        }
    }
    if ((0x0000000200000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__33___VdlySet__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_we) {
            __Vinline__nba_sequent__TOP__33___VdlyVal__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_res;
            __Vinline__nba_sequent__TOP__33___VdlyDim0__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_rd));
            __Vinline__nba_sequent__TOP__33___VdlySet__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__33___VdlySet__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_21279__DOT__regs[__Vinline__nba_sequent__TOP__33___VdlyDim0__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__33___VdlyVal__rspu_top__DOT__reg_regfile_inst_21279__DOT__regs__v0;
        }
    }
    if ((0x0000000400000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__34___VdlySet__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_we) {
            __Vinline__nba_sequent__TOP__34___VdlyVal__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_res;
            __Vinline__nba_sequent__TOP__34___VdlyDim0__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_rd));
            __Vinline__nba_sequent__TOP__34___VdlySet__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__34___VdlySet__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_21814__DOT__regs[__Vinline__nba_sequent__TOP__34___VdlyDim0__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__34___VdlyVal__rspu_top__DOT__reg_regfile_inst_21814__DOT__regs__v0;
        }
    }
    if ((0x0000000800000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__35___VdlySet__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_we) {
            __Vinline__nba_sequent__TOP__35___VdlyVal__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_res;
            __Vinline__nba_sequent__TOP__35___VdlyDim0__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_rd));
            __Vinline__nba_sequent__TOP__35___VdlySet__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__35___VdlySet__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_22349__DOT__regs[__Vinline__nba_sequent__TOP__35___VdlyDim0__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__35___VdlyVal__rspu_top__DOT__reg_regfile_inst_22349__DOT__regs__v0;
        }
    }
    if ((0x0000001000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__36___VdlySet__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_we) {
            __Vinline__nba_sequent__TOP__36___VdlyVal__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_res;
            __Vinline__nba_sequent__TOP__36___VdlyDim0__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_rd));
            __Vinline__nba_sequent__TOP__36___VdlySet__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__36___VdlySet__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_22884__DOT__regs[__Vinline__nba_sequent__TOP__36___VdlyDim0__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__36___VdlyVal__rspu_top__DOT__reg_regfile_inst_22884__DOT__regs__v0;
        }
    }
    if ((0x0000002000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__37___VdlySet__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_we) {
            __Vinline__nba_sequent__TOP__37___VdlyVal__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_res;
            __Vinline__nba_sequent__TOP__37___VdlyDim0__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_rd));
            __Vinline__nba_sequent__TOP__37___VdlySet__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__37___VdlySet__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_23419__DOT__regs[__Vinline__nba_sequent__TOP__37___VdlyDim0__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__37___VdlyVal__rspu_top__DOT__reg_regfile_inst_23419__DOT__regs__v0;
        }
    }
    if ((0x0000004000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__38___VdlySet__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_we) {
            __Vinline__nba_sequent__TOP__38___VdlyVal__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_res;
            __Vinline__nba_sequent__TOP__38___VdlyDim0__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_rd));
            __Vinline__nba_sequent__TOP__38___VdlySet__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__38___VdlySet__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_23954__DOT__regs[__Vinline__nba_sequent__TOP__38___VdlyDim0__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__38___VdlyVal__rspu_top__DOT__reg_regfile_inst_23954__DOT__regs__v0;
        }
    }
    if ((0x0000008000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__39___VdlySet__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_we) {
            __Vinline__nba_sequent__TOP__39___VdlyVal__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_res;
            __Vinline__nba_sequent__TOP__39___VdlyDim0__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_rd));
            __Vinline__nba_sequent__TOP__39___VdlySet__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__39___VdlySet__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_24489__DOT__regs[__Vinline__nba_sequent__TOP__39___VdlyDim0__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__39___VdlyVal__rspu_top__DOT__reg_regfile_inst_24489__DOT__regs__v0;
        }
    }
    if ((0x0000010000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__40___VdlySet__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_we) {
            __Vinline__nba_sequent__TOP__40___VdlyVal__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_res;
            __Vinline__nba_sequent__TOP__40___VdlyDim0__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_rd));
            __Vinline__nba_sequent__TOP__40___VdlySet__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__40___VdlySet__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_25024__DOT__regs[__Vinline__nba_sequent__TOP__40___VdlyDim0__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__40___VdlyVal__rspu_top__DOT__reg_regfile_inst_25024__DOT__regs__v0;
        }
    }
    if ((0x0000020000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__41___VdlySet__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_we) {
            __Vinline__nba_sequent__TOP__41___VdlyVal__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_res;
            __Vinline__nba_sequent__TOP__41___VdlyDim0__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_rd));
            __Vinline__nba_sequent__TOP__41___VdlySet__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__41___VdlySet__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_25559__DOT__regs[__Vinline__nba_sequent__TOP__41___VdlyDim0__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__41___VdlyVal__rspu_top__DOT__reg_regfile_inst_25559__DOT__regs__v0;
        }
    }
    if ((0x0000040000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__42___VdlySet__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_we) {
            __Vinline__nba_sequent__TOP__42___VdlyVal__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_res;
            __Vinline__nba_sequent__TOP__42___VdlyDim0__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_rd));
            __Vinline__nba_sequent__TOP__42___VdlySet__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__42___VdlySet__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_26094__DOT__regs[__Vinline__nba_sequent__TOP__42___VdlyDim0__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__42___VdlyVal__rspu_top__DOT__reg_regfile_inst_26094__DOT__regs__v0;
        }
    }
    if ((0x0000080000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__43___VdlySet__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_we) {
            __Vinline__nba_sequent__TOP__43___VdlyVal__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_res;
            __Vinline__nba_sequent__TOP__43___VdlyDim0__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_rd));
            __Vinline__nba_sequent__TOP__43___VdlySet__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__43___VdlySet__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_26629__DOT__regs[__Vinline__nba_sequent__TOP__43___VdlyDim0__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__43___VdlyVal__rspu_top__DOT__reg_regfile_inst_26629__DOT__regs__v0;
        }
    }
    if ((0x0000100000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__44___VdlySet__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_we) {
            __Vinline__nba_sequent__TOP__44___VdlyVal__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_res;
            __Vinline__nba_sequent__TOP__44___VdlyDim0__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_rd));
            __Vinline__nba_sequent__TOP__44___VdlySet__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__44___VdlySet__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_27164__DOT__regs[__Vinline__nba_sequent__TOP__44___VdlyDim0__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__44___VdlyVal__rspu_top__DOT__reg_regfile_inst_27164__DOT__regs__v0;
        }
    }
    if ((0x0000200000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__45___VdlySet__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_we) {
            __Vinline__nba_sequent__TOP__45___VdlyVal__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_res;
            __Vinline__nba_sequent__TOP__45___VdlyDim0__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_rd));
            __Vinline__nba_sequent__TOP__45___VdlySet__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__45___VdlySet__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_27699__DOT__regs[__Vinline__nba_sequent__TOP__45___VdlyDim0__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__45___VdlyVal__rspu_top__DOT__reg_regfile_inst_27699__DOT__regs__v0;
        }
    }
    if ((0x0000400000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__46___VdlySet__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_we) {
            __Vinline__nba_sequent__TOP__46___VdlyVal__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_res;
            __Vinline__nba_sequent__TOP__46___VdlyDim0__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_rd));
            __Vinline__nba_sequent__TOP__46___VdlySet__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__46___VdlySet__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_28234__DOT__regs[__Vinline__nba_sequent__TOP__46___VdlyDim0__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__46___VdlyVal__rspu_top__DOT__reg_regfile_inst_28234__DOT__regs__v0;
        }
    }
    if ((0x0000800000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__47___VdlySet__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_we) {
            __Vinline__nba_sequent__TOP__47___VdlyVal__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_res;
            __Vinline__nba_sequent__TOP__47___VdlyDim0__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_rd));
            __Vinline__nba_sequent__TOP__47___VdlySet__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__47___VdlySet__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_28769__DOT__regs[__Vinline__nba_sequent__TOP__47___VdlyDim0__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__47___VdlyVal__rspu_top__DOT__reg_regfile_inst_28769__DOT__regs__v0;
        }
    }
    if ((0x0001000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__48___VdlySet__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_we) {
            __Vinline__nba_sequent__TOP__48___VdlyVal__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_res;
            __Vinline__nba_sequent__TOP__48___VdlyDim0__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_rd));
            __Vinline__nba_sequent__TOP__48___VdlySet__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__48___VdlySet__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_29304__DOT__regs[__Vinline__nba_sequent__TOP__48___VdlyDim0__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__48___VdlyVal__rspu_top__DOT__reg_regfile_inst_29304__DOT__regs__v0;
        }
    }
    if ((0x0002000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__49___VdlySet__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_we) {
            __Vinline__nba_sequent__TOP__49___VdlyVal__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_res;
            __Vinline__nba_sequent__TOP__49___VdlyDim0__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_rd));
            __Vinline__nba_sequent__TOP__49___VdlySet__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__49___VdlySet__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_29839__DOT__regs[__Vinline__nba_sequent__TOP__49___VdlyDim0__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__49___VdlyVal__rspu_top__DOT__reg_regfile_inst_29839__DOT__regs__v0;
        }
    }
    if ((0x0004000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__50___VdlySet__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_we) {
            __Vinline__nba_sequent__TOP__50___VdlyVal__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_res;
            __Vinline__nba_sequent__TOP__50___VdlyDim0__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_rd));
            __Vinline__nba_sequent__TOP__50___VdlySet__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__50___VdlySet__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_30374__DOT__regs[__Vinline__nba_sequent__TOP__50___VdlyDim0__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__50___VdlyVal__rspu_top__DOT__reg_regfile_inst_30374__DOT__regs__v0;
        }
    }
    if ((0x0008000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__51___VdlySet__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_we) {
            __Vinline__nba_sequent__TOP__51___VdlyVal__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_res;
            __Vinline__nba_sequent__TOP__51___VdlyDim0__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_rd));
            __Vinline__nba_sequent__TOP__51___VdlySet__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__51___VdlySet__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_30909__DOT__regs[__Vinline__nba_sequent__TOP__51___VdlyDim0__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__51___VdlyVal__rspu_top__DOT__reg_regfile_inst_30909__DOT__regs__v0;
        }
    }
    if ((0x0010000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__52___VdlySet__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_we) {
            __Vinline__nba_sequent__TOP__52___VdlyVal__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_res;
            __Vinline__nba_sequent__TOP__52___VdlyDim0__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_rd));
            __Vinline__nba_sequent__TOP__52___VdlySet__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__52___VdlySet__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_31444__DOT__regs[__Vinline__nba_sequent__TOP__52___VdlyDim0__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__52___VdlyVal__rspu_top__DOT__reg_regfile_inst_31444__DOT__regs__v0;
        }
    }
    if ((0x0020000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__53___VdlySet__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_we) {
            __Vinline__nba_sequent__TOP__53___VdlyVal__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_res;
            __Vinline__nba_sequent__TOP__53___VdlyDim0__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_rd));
            __Vinline__nba_sequent__TOP__53___VdlySet__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__53___VdlySet__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_31979__DOT__regs[__Vinline__nba_sequent__TOP__53___VdlyDim0__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__53___VdlyVal__rspu_top__DOT__reg_regfile_inst_31979__DOT__regs__v0;
        }
    }
    if ((0x0040000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__54___VdlySet__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_we) {
            __Vinline__nba_sequent__TOP__54___VdlyVal__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_res;
            __Vinline__nba_sequent__TOP__54___VdlyDim0__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_rd));
            __Vinline__nba_sequent__TOP__54___VdlySet__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__54___VdlySet__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_32514__DOT__regs[__Vinline__nba_sequent__TOP__54___VdlyDim0__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__54___VdlyVal__rspu_top__DOT__reg_regfile_inst_32514__DOT__regs__v0;
        }
    }
    if ((0x0080000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__55___VdlySet__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_we) {
            __Vinline__nba_sequent__TOP__55___VdlyVal__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_res;
            __Vinline__nba_sequent__TOP__55___VdlyDim0__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_rd));
            __Vinline__nba_sequent__TOP__55___VdlySet__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__55___VdlySet__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_33049__DOT__regs[__Vinline__nba_sequent__TOP__55___VdlyDim0__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__55___VdlyVal__rspu_top__DOT__reg_regfile_inst_33049__DOT__regs__v0;
        }
    }
    if ((0x0100000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__56___VdlySet__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_we) {
            __Vinline__nba_sequent__TOP__56___VdlyVal__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_res;
            __Vinline__nba_sequent__TOP__56___VdlyDim0__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_rd));
            __Vinline__nba_sequent__TOP__56___VdlySet__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__56___VdlySet__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_33584__DOT__regs[__Vinline__nba_sequent__TOP__56___VdlyDim0__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__56___VdlyVal__rspu_top__DOT__reg_regfile_inst_33584__DOT__regs__v0;
        }
    }
    if ((0x0200000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__57___VdlySet__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_we) {
            __Vinline__nba_sequent__TOP__57___VdlyVal__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_res;
            __Vinline__nba_sequent__TOP__57___VdlyDim0__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_rd));
            __Vinline__nba_sequent__TOP__57___VdlySet__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__57___VdlySet__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_34119__DOT__regs[__Vinline__nba_sequent__TOP__57___VdlyDim0__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__57___VdlyVal__rspu_top__DOT__reg_regfile_inst_34119__DOT__regs__v0;
        }
    }
    if ((0x0400000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__58___VdlySet__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_we) {
            __Vinline__nba_sequent__TOP__58___VdlyVal__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_res;
            __Vinline__nba_sequent__TOP__58___VdlyDim0__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_rd));
            __Vinline__nba_sequent__TOP__58___VdlySet__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__58___VdlySet__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_34654__DOT__regs[__Vinline__nba_sequent__TOP__58___VdlyDim0__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__58___VdlyVal__rspu_top__DOT__reg_regfile_inst_34654__DOT__regs__v0;
        }
    }
    if ((0x0800000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__59___VdlySet__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_we) {
            __Vinline__nba_sequent__TOP__59___VdlyVal__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_res;
            __Vinline__nba_sequent__TOP__59___VdlyDim0__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_rd));
            __Vinline__nba_sequent__TOP__59___VdlySet__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__59___VdlySet__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_35189__DOT__regs[__Vinline__nba_sequent__TOP__59___VdlyDim0__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__59___VdlyVal__rspu_top__DOT__reg_regfile_inst_35189__DOT__regs__v0;
        }
    }
    if ((0x1000000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__60___VdlySet__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_we) {
            __Vinline__nba_sequent__TOP__60___VdlyVal__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_res;
            __Vinline__nba_sequent__TOP__60___VdlyDim0__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_rd));
            __Vinline__nba_sequent__TOP__60___VdlySet__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__60___VdlySet__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_35724__DOT__regs[__Vinline__nba_sequent__TOP__60___VdlyDim0__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__60___VdlyVal__rspu_top__DOT__reg_regfile_inst_35724__DOT__regs__v0;
        }
    }
    if ((0x2000000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__61___VdlySet__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_we) {
            __Vinline__nba_sequent__TOP__61___VdlyVal__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_res;
            __Vinline__nba_sequent__TOP__61___VdlyDim0__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_rd));
            __Vinline__nba_sequent__TOP__61___VdlySet__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__61___VdlySet__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_36259__DOT__regs[__Vinline__nba_sequent__TOP__61___VdlyDim0__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__61___VdlyVal__rspu_top__DOT__reg_regfile_inst_36259__DOT__regs__v0;
        }
    }
    if ((0x4000000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__62___VdlySet__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_we) {
            __Vinline__nba_sequent__TOP__62___VdlyVal__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_res;
            __Vinline__nba_sequent__TOP__62___VdlyDim0__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_rd));
            __Vinline__nba_sequent__TOP__62___VdlySet__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__62___VdlySet__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_36794__DOT__regs[__Vinline__nba_sequent__TOP__62___VdlyDim0__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__62___VdlyVal__rspu_top__DOT__reg_regfile_inst_36794__DOT__regs__v0;
        }
    }
    if ((0x8000000000000000ULL & vlSelfRef.__VnbaTriggered[0U])) {
        __Vinline__nba_sequent__TOP__63___VdlySet__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_we) {
            __Vinline__nba_sequent__TOP__63___VdlyVal__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_res;
            __Vinline__nba_sequent__TOP__63___VdlyDim0__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_rd));
            __Vinline__nba_sequent__TOP__63___VdlySet__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__63___VdlySet__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_37329__DOT__regs[__Vinline__nba_sequent__TOP__63___VdlyDim0__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__63___VdlyVal__rspu_top__DOT__reg_regfile_inst_37329__DOT__regs__v0;
        }
    }
    if ((1ULL & vlSelfRef.__VnbaTriggered[1U])) {
        __Vinline__nba_sequent__TOP__64___VdlySet__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_we) {
            __Vinline__nba_sequent__TOP__64___VdlyVal__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_res;
            __Vinline__nba_sequent__TOP__64___VdlyDim0__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_rd));
            __Vinline__nba_sequent__TOP__64___VdlySet__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__64___VdlySet__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_37864__DOT__regs[__Vinline__nba_sequent__TOP__64___VdlyDim0__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__64___VdlyVal__rspu_top__DOT__reg_regfile_inst_37864__DOT__regs__v0;
        }
    }
    if ((2ULL & vlSelfRef.__VnbaTriggered[1U])) {
        __Vinline__nba_sequent__TOP__65___VdlySet__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 = 0U;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_we) {
            __Vinline__nba_sequent__TOP__65___VdlyVal__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 
                = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_res;
            __Vinline__nba_sequent__TOP__65___VdlyDim0__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 
                = (0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_rd));
            __Vinline__nba_sequent__TOP__65___VdlySet__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0 = 1U;
        }
        if (__Vinline__nba_sequent__TOP__65___VdlySet__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0) {
            vlSelfRef.rspu_top__DOT__reg_regfile_inst_38399__DOT__regs[__Vinline__nba_sequent__TOP__65___VdlyDim0__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0] 
                = __Vinline__nba_sequent__TOP__65___VdlyVal__rspu_top__DOT__reg_regfile_inst_38399__DOT__regs__v0;
        }
    }
    if ((3ULL & vlSelfRef.__VnbaTriggered[0U])) {
        Vrspu_top___024root___nba_sequent__TOP__66(vlSelf);
        Vrspu_top___024root___nba_sequent__TOP__67(vlSelf);
        Vrspu_top___024root___nba_sequent__TOP__68(vlSelf);
    }
    if ((7ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_4694__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_4694__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_rs1))];
    }
    if ((0x000000000000000bULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_5229__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_5229__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_rs1))];
    }
    if ((0x0000000000000013ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_5764__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_5764__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_rs1))];
    }
    if ((0x0000000000000023ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_6299__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_6299__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_rs1))];
    }
    if ((0x0000000000000043ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_6834__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_6834__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_rs1))];
    }
    if ((0x0000000000000083ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_7369__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_7369__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_rs1))];
    }
    if ((0x0000000000000103ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_7904__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_7904__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_rs1))];
    }
    if ((0x0000000000000203ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_8439__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_8439__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_rs1))];
    }
    if ((0x0000000000000403ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_8974__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_8974__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_rs1))];
    }
    if ((0x0000000000000803ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_9509__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_9509__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_rs1))];
    }
    if ((0x0000000000001003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_10044__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_10044__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_rs1))];
    }
    if ((0x0000000000002003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_10579__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_10579__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_rs1))];
    }
    if ((0x0000000000004003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_11114__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_11114__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_rs1))];
    }
    if ((0x0000000000008003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_11649__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_11649__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_rs1))];
    }
    if ((0x0000000000010003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_12184__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_12184__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_rs1))];
    }
    if ((0x0000000000020003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_12719__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_12719__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_rs1))];
    }
    if ((0x0000000000040003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_13254__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_13254__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_rs1))];
    }
    if ((0x0000000000080003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_13789__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_13789__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_rs1))];
    }
    if ((0x0000000000100003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_14324__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_14324__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_rs1))];
    }
    if ((0x0000000000200003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_14859__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_14859__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_rs1))];
    }
    if ((0x0000000000400003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_15394__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_15394__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_rs1))];
    }
    if ((0x0000000000800003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_15929__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_15929__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_rs1))];
    }
    if ((0x0000000001000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_16464__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_16464__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_rs1))];
    }
    if ((0x0000000002000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_16999__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_16999__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_rs1))];
    }
    if ((0x0000000004000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_17534__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_17534__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_rs1))];
    }
    if ((0x0000000008000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_18069__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_18069__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_rs1))];
    }
    if ((0x0000000010000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_18604__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_18604__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_rs1))];
    }
    if ((0x0000000020000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_19139__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_19139__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_rs1))];
    }
    if ((0x0000000040000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_19674__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_19674__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_rs1))];
    }
    if ((0x0000000080000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_20209__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_20209__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_rs1))];
    }
    if ((0x0000000100000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_20744__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_20744__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_rs1))];
    }
    if ((0x0000000200000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_21279__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_21279__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_rs1))];
    }
    if ((0x0000000400000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_21814__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_21814__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_rs1))];
    }
    if ((0x0000000800000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_22349__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_22349__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_rs1))];
    }
    if ((0x0000001000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_22884__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_22884__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_rs1))];
    }
    if ((0x0000002000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_23419__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_23419__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_rs1))];
    }
    if ((0x0000004000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_23954__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_23954__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_rs1))];
    }
    if ((0x0000008000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_24489__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_24489__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_rs1))];
    }
    if ((0x0000010000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_25024__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_25024__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_rs1))];
    }
    if ((0x0000020000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_25559__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_25559__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_rs1))];
    }
    if ((0x0000040000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_26094__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_26094__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_rs1))];
    }
    if ((0x0000080000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_26629__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_26629__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_rs1))];
    }
    if ((0x0000100000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_27164__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_27164__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_rs1))];
    }
    if ((0x0000200000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_27699__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_27699__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_rs1))];
    }
    if ((0x0000400000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_28234__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_28234__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_rs1))];
    }
    if ((0x0000800000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_28769__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_28769__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_rs1))];
    }
    if ((0x0001000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_29304__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_29304__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_rs1))];
    }
    if ((0x0002000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_29839__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_29839__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_rs1))];
    }
    if ((0x0004000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_30374__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_30374__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_rs1))];
    }
    if ((0x0008000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_30909__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_30909__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_rs1))];
    }
    if ((0x0010000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_31444__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_31444__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_rs1))];
    }
    if ((0x0020000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_31979__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_31979__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_rs1))];
    }
    if ((0x0040000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_32514__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_32514__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_rs1))];
    }
    if ((0x0080000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_33049__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_33049__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_rs1))];
    }
    if ((0x0100000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_33584__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_33584__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_rs1))];
    }
    if ((0x0200000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_34119__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_34119__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_rs1))];
    }
    if ((0x0400000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_34654__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_34654__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_rs1))];
    }
    if ((0x0800000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_35189__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_35189__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_rs1))];
    }
    if ((0x1000000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_35724__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_35724__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_rs1))];
    }
    if ((0x2000000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_36259__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_36259__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_rs1))];
    }
    if ((0x4000000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_36794__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_36794__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_rs1))];
    }
    if ((0x8000000000000003ULL & vlSelfRef.__VnbaTriggered[0U])) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_37329__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_37329__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_rs1))];
    }
    if (((1ULL & vlSelfRef.__VnbaTriggered[1U]) | (3ULL 
                                                   & vlSelfRef.__VnbaTriggered[0U]))) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_37864__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_37864__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_rs1))];
    }
    if (((2ULL & vlSelfRef.__VnbaTriggered[1U]) | (3ULL 
                                                   & vlSelfRef.__VnbaTriggered[0U]))) {
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_val2 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_38399__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_rs2))];
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_val1 
            = vlSelfRef.rspu_top__DOT__reg_regfile_inst_38399__DOT__regs
            [(0x000003ffU & (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_rs1))];
    }
}

void Vrspu_top___024root___trigger_orInto__act_vec_vec(VlUnpacked<QData/*63:0*/, 2> &out, const VlUnpacked<QData/*63:0*/, 2> &in) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___trigger_orInto__act_vec_vec\n"); );
    // Locals
    IData/*31:0*/ n;
    // Body
    n = 0U;
    do {
        out[n] = (out[n] | in[n]);
        n = ((IData)(1U) + n);
    } while ((1U >= n));
}

void Vrspu_top___024root___eval_triggers_vec__act(Vrspu_top___024root* vlSelf);
#ifdef VL_DEBUG
VL_ATTR_COLD void Vrspu_top___024root___dump_triggers__act(const VlUnpacked<QData/*63:0*/, 2> &triggers, const std::string &tag);
#endif  // VL_DEBUG

bool Vrspu_top___024root___eval_phase__act(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___eval_phase__act\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    Vrspu_top___024root___eval_triggers_vec__act(vlSelf);
#ifdef VL_DEBUG
    if (VL_UNLIKELY(vlSymsp->_vm_contextp__->debug())) {
        Vrspu_top___024root___dump_triggers__act(vlSelfRef.__VactTriggered, "act"s);
    }
#endif
    Vrspu_top___024root___trigger_orInto__act_vec_vec(vlSelfRef.__VnbaTriggered, vlSelfRef.__VactTriggered);
    return (0U);
}

void Vrspu_top___024root___trigger_clear__act(VlUnpacked<QData/*63:0*/, 2> &out) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___trigger_clear__act\n"); );
    // Locals
    IData/*31:0*/ n;
    // Body
    n = 0U;
    do {
        out[n] = 0ULL;
        n = ((IData)(1U) + n);
    } while ((2U > n));
}

bool Vrspu_top___024root___trigger_anySet__act(const VlUnpacked<QData/*63:0*/, 2> &in);

bool Vrspu_top___024root___eval_phase__nba(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___eval_phase__nba\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    CData/*0:0*/ __VnbaExecute;
    // Body
    __VnbaExecute = Vrspu_top___024root___trigger_anySet__act(vlSelfRef.__VnbaTriggered);
    if (__VnbaExecute) {
        Vrspu_top___024root___eval_nba(vlSelf);
        Vrspu_top___024root___trigger_clear__act(vlSelfRef.__VnbaTriggered);
    }
    return (__VnbaExecute);
}

void Vrspu_top___024root___eval(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___eval\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    IData/*31:0*/ __VnbaIterCount;
    // Body
    __VnbaIterCount = 0U;
    do {
        if (VL_UNLIKELY(((0x00002710U < __VnbaIterCount)))) {
#ifdef VL_DEBUG
            Vrspu_top___024root___dump_triggers__act(vlSelfRef.__VnbaTriggered, "nba"s);
#endif
            VL_FATAL_MT("reflex_soc/rspu_top_synth.sv", 15, "", "DIDNOTCONVERGE: NBA region did not converge after '--converge-limit' of 10000 tries");
        }
        __VnbaIterCount = ((IData)(1U) + __VnbaIterCount);
        vlSelfRef.__VactIterCount = 0U;
        do {
            if (VL_UNLIKELY(((0x00002710U < vlSelfRef.__VactIterCount)))) {
#ifdef VL_DEBUG
                Vrspu_top___024root___dump_triggers__act(vlSelfRef.__VactTriggered, "act"s);
#endif
                VL_FATAL_MT("reflex_soc/rspu_top_synth.sv", 15, "", "DIDNOTCONVERGE: Active region did not converge after '--converge-limit' of 10000 tries");
            }
            vlSelfRef.__VactIterCount = ((IData)(1U) 
                                         + vlSelfRef.__VactIterCount);
            vlSelfRef.__VactPhaseResult = Vrspu_top___024root___eval_phase__act(vlSelf);
        } while (vlSelfRef.__VactPhaseResult);
        vlSelfRef.__VnbaPhaseResult = Vrspu_top___024root___eval_phase__nba(vlSelf);
    } while (vlSelfRef.__VnbaPhaseResult);
}

#ifdef VL_DEBUG
void Vrspu_top___024root___eval_debug_assertions(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___eval_debug_assertions\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    if (VL_UNLIKELY(((vlSelfRef.clk & 0xfeU)))) {
        Verilated::overWidthError("clk");
    }
    if (VL_UNLIKELY(((vlSelfRef.rst_n & 0xfeU)))) {
        Verilated::overWidthError("rst_n");
    }
    if (VL_UNLIKELY(((vlSelfRef.downlink_valid_0 & 0xfeU)))) {
        Verilated::overWidthError("downlink_valid_0");
    }
    if (VL_UNLIKELY(((vlSelfRef.downlink_valid_1 & 0xfeU)))) {
        Verilated::overWidthError("downlink_valid_1");
    }
    if (VL_UNLIKELY(((vlSelfRef.downlink_valid_2 & 0xfeU)))) {
        Verilated::overWidthError("downlink_valid_2");
    }
    if (VL_UNLIKELY(((vlSelfRef.downlink_valid_3 & 0xfeU)))) {
        Verilated::overWidthError("downlink_valid_3");
    }
}
#endif  // VL_DEBUG

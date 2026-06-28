// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vrspu_top.h for the primary calling header

#include "Vrspu_top__pch.h"

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
void Vrspu_top___024root___eval_nba(Vrspu_top___024root* vlSelf);

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

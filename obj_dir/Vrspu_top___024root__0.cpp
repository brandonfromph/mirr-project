// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vrspu_top.h for the primary calling header

#include "Vrspu_top__pch.h"

void Vrspu_top___024root___eval_triggers_vec__act(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___eval_triggers_vec__act\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Body
    vlSelfRef.__VactTriggered[0U] = (((QData)((IData)(
                                                      ((((((((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_746_gated_clk) 
                                                               & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_746_gated_clk__0))) 
                                                              << 3U) 
                                                             | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_748_gated_clk) 
                                                                 & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_748_gated_clk__0))) 
                                                                << 2U)) 
                                                            | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_750_gated_clk) 
                                                                 & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_750_gated_clk__0))) 
                                                                << 1U) 
                                                               | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_752_gated_clk) 
                                                                  & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_752_gated_clk__0))))) 
                                                           << 0x0000000cU) 
                                                          | ((((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_754_gated_clk) 
                                                                 & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_754_gated_clk__0))) 
                                                                << 3U) 
                                                               | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_756_gated_clk) 
                                                                   & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_756_gated_clk__0))) 
                                                                  << 2U)) 
                                                              | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_758_gated_clk) 
                                                                   & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_758_gated_clk__0))) 
                                                                  << 1U) 
                                                                 | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_760_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_760_gated_clk__0))))) 
                                                             << 8U)) 
                                                         | (((((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_762_gated_clk) 
                                                                 & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_762_gated_clk__0))) 
                                                                << 3U) 
                                                               | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_764_gated_clk) 
                                                                   & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_764_gated_clk__0))) 
                                                                  << 2U)) 
                                                              | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_766_gated_clk) 
                                                                   & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_766_gated_clk__0))) 
                                                                  << 1U) 
                                                                 | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_768_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_768_gated_clk__0))))) 
                                                             << 4U) 
                                                            | (((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_770_gated_clk) 
                                                                  & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_770_gated_clk__0))) 
                                                                 << 3U) 
                                                                | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_772_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_772_gated_clk__0))) 
                                                                   << 2U)) 
                                                               | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_774_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_774_gated_clk__0))) 
                                                                   << 1U) 
                                                                  | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_776_gated_clk) 
                                                                     & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_776_gated_clk__0))))))) 
                                                        << 0x00000010U) 
                                                       | ((((((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_778_gated_clk) 
                                                                & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_778_gated_clk__0))) 
                                                               << 3U) 
                                                              | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_780_gated_clk) 
                                                                  & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_780_gated_clk__0))) 
                                                                 << 2U)) 
                                                             | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_782_gated_clk) 
                                                                  & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_782_gated_clk__0))) 
                                                                 << 1U) 
                                                                | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_784_gated_clk) 
                                                                   & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_784_gated_clk__0))))) 
                                                            << 0x0000000cU) 
                                                           | ((((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_786_gated_clk) 
                                                                  & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_786_gated_clk__0))) 
                                                                 << 3U) 
                                                                | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_788_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_788_gated_clk__0))) 
                                                                   << 2U)) 
                                                               | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_790_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_790_gated_clk__0))) 
                                                                   << 1U) 
                                                                  | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_792_gated_clk) 
                                                                     & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_792_gated_clk__0))))) 
                                                              << 8U)) 
                                                          | (((((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_794_gated_clk) 
                                                                  & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_794_gated_clk__0))) 
                                                                 << 3U) 
                                                                | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_796_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_796_gated_clk__0))) 
                                                                   << 2U)) 
                                                               | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_798_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_798_gated_clk__0))) 
                                                                   << 1U) 
                                                                  | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_800_gated_clk) 
                                                                     & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_800_gated_clk__0))))) 
                                                              << 4U) 
                                                             | (((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_802_gated_clk) 
                                                                   & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_802_gated_clk__0))) 
                                                                  << 3U) 
                                                                 | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_804_gated_clk) 
                                                                     & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_804_gated_clk__0))) 
                                                                    << 2U)) 
                                                                | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_806_gated_clk) 
                                                                     & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_806_gated_clk__0))) 
                                                                    << 1U) 
                                                                   | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_808_gated_clk) 
                                                                      & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_808_gated_clk__0)))))))))) 
                                      << 0x00000020U) 
                                     | (QData)((IData)(
                                                       ((((((((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_810_gated_clk) 
                                                                & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_810_gated_clk__0))) 
                                                               << 3U) 
                                                              | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_812_gated_clk) 
                                                                  & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_812_gated_clk__0))) 
                                                                 << 2U)) 
                                                             | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_814_gated_clk) 
                                                                  & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_814_gated_clk__0))) 
                                                                 << 1U) 
                                                                | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_816_gated_clk) 
                                                                   & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_816_gated_clk__0))))) 
                                                            << 0x0000000cU) 
                                                           | ((((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_818_gated_clk) 
                                                                  & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_818_gated_clk__0))) 
                                                                 << 3U) 
                                                                | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_820_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_820_gated_clk__0))) 
                                                                   << 2U)) 
                                                               | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_822_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_822_gated_clk__0))) 
                                                                   << 1U) 
                                                                  | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_824_gated_clk) 
                                                                     & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_824_gated_clk__0))))) 
                                                              << 8U)) 
                                                          | (((((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_826_gated_clk) 
                                                                  & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_826_gated_clk__0))) 
                                                                 << 3U) 
                                                                | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_828_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_828_gated_clk__0))) 
                                                                   << 2U)) 
                                                               | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_830_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_830_gated_clk__0))) 
                                                                   << 1U) 
                                                                  | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_832_gated_clk) 
                                                                     & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_832_gated_clk__0))))) 
                                                              << 4U) 
                                                             | (((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_834_gated_clk) 
                                                                   & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_834_gated_clk__0))) 
                                                                  << 3U) 
                                                                 | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_836_gated_clk) 
                                                                     & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_836_gated_clk__0))) 
                                                                    << 2U)) 
                                                                | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_838_gated_clk) 
                                                                     & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_838_gated_clk__0))) 
                                                                    << 1U) 
                                                                   | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_840_gated_clk) 
                                                                      & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_840_gated_clk__0))))))) 
                                                         << 0x00000010U) 
                                                        | ((((((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_842_gated_clk) 
                                                                 & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_842_gated_clk__0))) 
                                                                << 3U) 
                                                               | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_844_gated_clk) 
                                                                   & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_844_gated_clk__0))) 
                                                                  << 2U)) 
                                                              | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_846_gated_clk) 
                                                                   & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_846_gated_clk__0))) 
                                                                  << 1U) 
                                                                 | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_848_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_848_gated_clk__0))))) 
                                                             << 0x0000000cU) 
                                                            | ((((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_850_gated_clk) 
                                                                   & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_850_gated_clk__0))) 
                                                                  << 3U) 
                                                                 | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_852_gated_clk) 
                                                                     & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_852_gated_clk__0))) 
                                                                    << 2U)) 
                                                                | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_854_gated_clk) 
                                                                     & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_854_gated_clk__0))) 
                                                                    << 1U) 
                                                                   | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_856_gated_clk) 
                                                                      & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_856_gated_clk__0))))) 
                                                               << 8U)) 
                                                           | (((((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_858_gated_clk) 
                                                                   & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_858_gated_clk__0))) 
                                                                  << 3U) 
                                                                 | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_860_gated_clk) 
                                                                     & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_860_gated_clk__0))) 
                                                                    << 2U)) 
                                                                | ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_862_gated_clk) 
                                                                     & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_862_gated_clk__0))) 
                                                                    << 1U) 
                                                                   | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_864_gated_clk) 
                                                                      & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_864_gated_clk__0))))) 
                                                               << 4U) 
                                                              | (((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_866_gated_clk) 
                                                                    & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_866_gated_clk__0))) 
                                                                   << 3U) 
                                                                  | (((IData)(vlSelfRef.rspu_top__DOT__core_top_call_868_gated_clk) 
                                                                      & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_868_gated_clk__0))) 
                                                                     << 2U)) 
                                                                 | ((((~ (IData)(vlSelfRef.rst_n)) 
                                                                      & (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rst_n__0)) 
                                                                     << 1U) 
                                                                    | ((IData)(vlSelfRef.clk) 
                                                                       & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__clk__0)))))))))));
    vlSelfRef.__VactTriggered[1U] = (QData)((IData)(
                                                    ((((IData)(vlSelfRef.rspu_top__DOT__core_top_call_742_gated_clk) 
                                                       & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_742_gated_clk__0))) 
                                                      << 1U) 
                                                     | ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_744_gated_clk) 
                                                        & (~ (IData)(vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_744_gated_clk__0))))));
    vlSelfRef.__Vtrigprevexpr___TOP__clk__0 = vlSelfRef.clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rst_n__0 = vlSelfRef.rst_n;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_868_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_868_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_866_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_866_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_864_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_864_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_862_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_862_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_860_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_860_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_858_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_858_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_856_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_856_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_854_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_854_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_852_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_852_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_850_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_850_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_848_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_848_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_846_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_846_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_844_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_844_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_842_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_842_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_840_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_840_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_838_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_838_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_836_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_836_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_834_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_834_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_832_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_832_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_830_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_830_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_828_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_828_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_826_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_826_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_824_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_824_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_822_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_822_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_820_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_820_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_818_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_818_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_816_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_816_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_814_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_814_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_812_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_812_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_810_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_810_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_808_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_808_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_806_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_806_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_804_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_804_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_802_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_802_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_800_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_800_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_798_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_798_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_796_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_796_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_794_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_794_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_792_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_792_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_790_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_790_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_788_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_788_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_786_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_786_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_784_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_784_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_782_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_782_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_780_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_780_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_778_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_778_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_776_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_776_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_774_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_774_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_772_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_772_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_770_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_770_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_768_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_768_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_766_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_766_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_764_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_764_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_762_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_762_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_760_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_760_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_758_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_758_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_756_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_756_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_754_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_754_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_752_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_752_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_750_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_750_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_748_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_748_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_746_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_746_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_744_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_744_gated_clk;
    vlSelfRef.__Vtrigprevexpr___TOP__rspu_top__DOT__core_top_call_742_gated_clk__0 
        = vlSelfRef.rspu_top__DOT__core_top_call_742_gated_clk;
}

bool Vrspu_top___024root___trigger_anySet__act(const VlUnpacked<QData/*63:0*/, 2> &in) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___trigger_anySet__act\n"); );
    // Locals
    IData/*31:0*/ n;
    // Body
    n = 0U;
    do {
        if (in[n]) {
            return (1U);
        }
        n = ((IData)(1U) + n);
    } while ((2U > n));
    return (0U);
}

void Vrspu_top___024root___nba_sequent__TOP__0(Vrspu_top___024root* vlSelf) {
    VL_DEBUG_IF(VL_DBG_MSGF("+    Vrspu_top___024root___nba_sequent__TOP__0\n"); );
    Vrspu_top__Syms* const __restrict vlSymsp VL_ATTR_UNUSED = vlSelf->vlSymsp;
    auto& vlSelfRef = std::ref(*vlSelf).get();
    // Locals
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_742_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_742_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_744_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_744_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_746_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_746_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_748_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_748_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_750_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_750_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_752_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_752_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_754_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_754_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_756_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_756_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_758_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_758_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_760_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_760_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_762_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_762_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_764_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_764_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_766_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_766_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_768_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_768_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_770_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_770_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_772_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_772_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_774_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_774_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_776_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_776_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_778_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_778_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_780_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_780_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_782_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_782_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_784_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_784_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_786_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_786_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_788_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_788_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_790_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_790_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_792_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_792_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_794_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_794_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_796_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_796_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_798_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_798_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_800_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_800_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_802_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_802_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_804_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_804_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_806_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_806_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_808_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_808_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_810_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_810_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_812_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_812_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_814_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_814_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_816_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_816_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_818_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_818_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_820_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_820_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_822_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_822_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_824_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_824_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_826_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_826_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_828_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_828_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_830_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_830_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_832_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_832_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_834_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_834_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_836_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_836_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_838_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_838_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_840_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_840_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_842_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_842_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_844_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_844_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_846_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_846_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_848_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_848_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_850_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_850_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_852_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_852_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_854_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_854_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_856_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_856_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_858_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_858_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_860_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_860_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_862_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_862_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_864_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_864_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_866_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_866_wake_timer = 0;
    CData/*2:0*/ __Vdly__rspu_top__DOT__core_top_call_868_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_868_wake_timer = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_0;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_0 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_1;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_1 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_10;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_10 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_11;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_11 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_12;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_12 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_13;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_13 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_14;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_14 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_15;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_15 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_2;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_2 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_3;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_3 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_4;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_4 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_5;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_5 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_6;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_6 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_7;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_7 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_8;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_8 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_9;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_9 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_0;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_0 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_1;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_1 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_10;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_10 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_11;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_11 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_12;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_12 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_13;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_13 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_14;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_14 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_15;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_15 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_2;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_2 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_3;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_3 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_4;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_4 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_5;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_5 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_6;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_6 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_7;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_7 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_8;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_8 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_9;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_9 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_0;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_0 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_1;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_1 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_10;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_10 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_11;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_11 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_12;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_12 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_13;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_13 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_14;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_14 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_15;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_15 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_2;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_2 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_3;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_3 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_4;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_4 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_5;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_5 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_6;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_6 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_7;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_7 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_8;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_8 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_9;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_9 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_0;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_0 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_1;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_1 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_10;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_10 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_11;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_11 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_12;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_12 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_13;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_13 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_14;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_14 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_15;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_15 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_2;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_2 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_3;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_3 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_4;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_4 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_5;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_5 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_6;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_6 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_7;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_7 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_8;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_8 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_9;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_9 = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__rspu_pipeline_call_13048_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_13048_pipe_pc = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__rspu_pipeline_call_13583_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_13583_pipe_pc = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__rspu_pipeline_call_21608_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_21608_pipe_pc = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__rspu_pipeline_call_22143_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_22143_pipe_pc = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__rspu_pipeline_call_30168_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_30168_pipe_pc = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__rspu_pipeline_call_30703_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_30703_pipe_pc = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__rspu_pipeline_call_4488_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_4488_pipe_pc = 0;
    IData/*31:0*/ __Vdly__rspu_top__DOT__rspu_pipeline_call_5023_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_5023_pipe_pc = 0;
    // Body
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_0_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_0_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_0_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_0_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_1_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_1_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_1_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_1_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_2_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_2_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_2_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_2_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_3_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_3_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_3_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_3_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_4_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_4_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_4_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_4_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_5_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_5_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_5_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_5_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_6_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_6_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_6_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_6_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_7_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_7_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_7_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_7_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_8_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_8_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_8_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_8_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_9_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_9_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_9_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_9_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_10_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_10_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_10_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_10_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_11_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_11_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_11_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_11_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_12_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_12_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_12_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_12_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_13_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_13_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_13_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_13_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_14_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_14_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_14_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_14_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_15_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_15_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_15_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_15_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_16_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_16_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_16_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_16_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_17_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_17_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_17_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_17_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_18_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_18_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_18_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_18_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_19_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_19_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_19_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_19_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_20_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_20_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_20_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_20_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_21_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_21_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_21_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_21_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_22_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_22_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_22_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_22_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_23_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_23_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_23_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_23_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_24_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_24_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_24_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_24_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_25_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_25_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_25_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_25_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_26_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_26_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_26_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_26_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_27_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_27_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_27_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_27_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_28_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_28_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_28_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_28_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_29_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_29_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_29_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_29_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_30_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_30_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_30_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_30_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_31_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_31_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_31_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_31_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_32_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_32_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_32_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_32_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_33_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_33_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_33_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_33_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_34_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_34_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_34_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_34_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_35_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_35_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_35_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_35_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_36_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_36_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_36_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_36_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_37_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_37_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_37_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_37_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_38_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_38_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_38_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_38_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_39_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_39_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_39_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_39_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_40_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_40_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_40_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_40_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_41_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_41_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_41_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_41_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_42_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_42_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_42_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_42_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_43_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_43_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_43_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_43_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_44_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_44_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_44_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_44_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_45_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_45_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_45_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_45_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_46_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_46_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_46_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_46_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_47_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_47_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_47_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_47_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_48_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_48_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_48_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_48_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_49_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_49_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_49_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_49_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_50_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_50_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_50_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_50_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_51_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_51_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_51_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_51_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_52_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_52_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_52_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_52_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_53_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_53_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_53_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_53_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_54_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_54_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_54_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_54_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_55_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_55_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_55_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_55_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_56_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_56_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_56_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_56_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_57_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_57_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_57_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_57_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_58_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_58_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_58_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_58_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_59_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_59_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_59_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_59_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_60_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_60_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_60_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_60_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_61_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_61_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_61_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_61_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_62_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_62_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_62_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_62_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[0U] 
        = vlSelfRef.rspu_top__DOT__io_in_63_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[1U] 
        = vlSelfRef.rspu_top__DOT__io_in_63_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[2U] 
        = vlSelfRef.rspu_top__DOT__io_in_63_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[3U] 
        = vlSelfRef.rspu_top__DOT__io_in_63_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_0_sync 
        = vlSelfRef.rspu_top__DOT__downlink_valid_0_sync;
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[0U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_0_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[1U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_0_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[2U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_0_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[3U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_0_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_1_sync 
        = vlSelfRef.rspu_top__DOT__downlink_valid_1_sync;
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[0U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_1_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[1U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_1_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[2U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_1_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[3U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_1_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_2_sync 
        = vlSelfRef.rspu_top__DOT__downlink_valid_2_sync;
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[0U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_2_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[1U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_2_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[2U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_2_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[3U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_2_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_3_sync 
        = vlSelfRef.rspu_top__DOT__downlink_valid_3_sync;
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[0U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_3_sync[0U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[1U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_3_sync[1U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[2U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_3_sync[2U];
    vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[3U] 
        = vlSelfRef.rspu_top__DOT__downlink_data_3_sync[3U];
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_10373_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_10908_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_11443_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_11978_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_12513_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_13048_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_13583_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_14118_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_14653_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_15188_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_15723_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_16258_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_16793_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_17328_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_17863_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_18398_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_18933_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_19468_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_20003_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_20538_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_21073_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_21608_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_22143_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_22678_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_23213_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_23748_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_24283_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_24818_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_25353_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_25888_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_26423_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_26958_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_27493_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_28028_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_28563_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_29098_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_29633_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_30168_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_30703_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_31238_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_31773_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_32308_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_32843_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_33378_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_33913_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_34448_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_34983_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_35518_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_36053_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_36588_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_37123_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_37658_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_38193_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_4488_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_pipe_pc;
    __Vdly__rspu_top__DOT__rspu_pipeline_call_5023_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_5558_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_6093_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_6628_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_7163_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_7698_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_8233_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_8768_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_9303_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_pipe_pc;
    vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_9838_pipe_pc 
        = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_pipe_pc;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_0 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_0;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_1 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_1;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_10 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_10;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_11 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_11;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_12 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_12;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_13 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_13;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_14 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_14;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_15 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_15;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_2 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_2;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_3 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_3;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_4 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_4;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_5 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_5;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_6 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_6;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_7 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_7;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_8 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_8;
    __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_9 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_9;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_0 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_0;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_1 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_1;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_10 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_10;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_11 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_11;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_12 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_12;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_13 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_13;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_14 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_14;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_15 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_15;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_2 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_2;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_3 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_3;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_4 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_4;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_5 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_5;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_6 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_6;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_7 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_7;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_8 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_8;
    __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_9 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_9;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_0 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_0;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_1 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_1;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_10 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_10;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_11 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_11;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_12 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_12;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_13 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_13;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_14 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_14;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_15 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_15;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_2 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_2;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_3 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_3;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_4 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_4;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_5 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_5;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_6 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_6;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_7 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_7;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_8 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_8;
    __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_9 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_9;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_0 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_0;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_1 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_1;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_10 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_10;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_11 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_11;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_12 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_12;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_13 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_13;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_14 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_14;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_15 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_15;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_2 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_2;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_3 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_3;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_4 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_4;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_5 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_5;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_6 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_6;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_7 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_7;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_8 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_8;
    __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_9 
        = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_9;
    __Vdly__rspu_top__DOT__core_top_call_742_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_742_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_744_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_744_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_746_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_746_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_748_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_748_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_750_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_750_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_752_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_752_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_754_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_754_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_756_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_756_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_758_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_758_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_760_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_760_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_762_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_762_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_764_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_764_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_766_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_766_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_768_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_768_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_770_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_770_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_772_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_772_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_774_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_774_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_776_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_776_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_778_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_778_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_780_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_780_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_782_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_782_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_784_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_784_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_786_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_786_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_788_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_788_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_790_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_790_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_792_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_792_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_794_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_794_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_796_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_796_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_798_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_798_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_800_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_800_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_802_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_802_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_804_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_804_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_806_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_806_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_808_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_808_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_810_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_810_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_812_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_812_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_814_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_814_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_816_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_816_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_818_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_818_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_820_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_820_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_822_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_822_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_824_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_824_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_826_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_826_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_828_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_828_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_830_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_830_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_832_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_832_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_834_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_834_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_836_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_836_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_838_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_838_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_840_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_840_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_842_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_842_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_844_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_844_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_846_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_846_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_848_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_848_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_850_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_850_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_852_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_852_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_854_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_854_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_856_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_856_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_858_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_858_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_860_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_860_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_862_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_862_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_864_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_864_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_866_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_866_wake_timer;
    __Vdly__rspu_top__DOT__core_top_call_868_wake_timer 
        = vlSelfRef.rspu_top__DOT__core_top_call_868_wake_timer;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_742_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_742_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_744_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_744_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_746_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_746_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_748_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_748_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_750_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_750_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_752_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_752_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_754_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_754_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_756_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_756_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_758_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_758_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_760_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_760_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_762_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_762_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_764_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_764_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_766_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_766_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_768_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_768_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_770_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_770_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_772_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_772_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_774_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_774_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_776_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_776_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_778_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_778_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_780_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_780_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_782_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_782_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_784_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_784_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_786_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_786_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_788_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_788_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_790_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_790_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_792_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_792_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_794_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_794_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_796_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_796_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_798_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_798_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_800_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_800_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_802_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_802_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_804_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_804_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_806_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_806_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_808_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_808_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_810_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_810_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_812_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_812_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_814_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_814_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_816_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_816_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_818_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_818_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_820_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_820_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_822_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_822_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_824_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_824_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_826_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_826_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_828_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_828_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_830_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_830_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_832_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_832_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_834_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_834_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_836_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_836_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_838_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_838_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_840_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_840_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_842_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_842_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_844_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_844_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_846_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_846_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_848_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_848_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_850_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_850_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_852_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_852_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_854_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_854_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_856_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_856_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_858_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_858_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_860_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_860_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_862_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_862_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_864_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_864_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_866_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_866_current_instr;
    vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_868_current_instr 
        = vlSelfRef.rspu_top__DOT__core_top_call_868_current_instr;
    vlSelfRef.uplink_valid_0 = ((IData)(vlSelfRef.rst_n) 
                                && ((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                    & (0x0000000000000010ULL 
                                       <= (0x0000000000000fffULL 
                                           & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))));
    vlSelfRef.uplink_valid_1 = ((IData)(vlSelfRef.rst_n) 
                                && ((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                    & (~ ((0x0000000000000010ULL 
                                           <= (0x0000000000000fffULL 
                                               & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U))) 
                                          & (0x0000000000000020ULL 
                                             > (0x0000000000000fffULL 
                                                & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))))));
    vlSelfRef.uplink_valid_2 = ((IData)(vlSelfRef.rst_n) 
                                && ((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                    & (~ ((0x0000000000000020ULL 
                                           <= (0x0000000000000fffULL 
                                               & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U))) 
                                          & (0x0000000000000030ULL 
                                             > (0x0000000000000fffULL 
                                                & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))))));
    vlSelfRef.uplink_valid_3 = ((IData)(vlSelfRef.rst_n) 
                                && ((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                    & (~ ((0x0000000000000030ULL 
                                           <= (0x0000000000000fffULL 
                                               & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U))) 
                                          & (0x0000000000000040ULL 
                                             > (0x0000000000000fffULL 
                                                & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))))));
    vlSelfRef.rspu_top__DOT__core_top_call_742_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_742_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_744_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_744_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_746_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_746_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_748_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_748_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_750_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_750_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_752_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_752_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_754_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_754_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_756_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_756_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_758_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_758_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_760_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_760_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_762_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_762_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_764_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_764_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_766_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_766_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_768_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_768_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_770_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_770_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_772_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_772_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_774_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_774_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_776_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_776_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_778_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_778_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_780_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_780_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_782_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_782_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_784_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_784_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_786_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_786_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_788_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_788_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_790_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_790_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_792_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_792_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_794_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_794_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_796_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_796_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_798_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_798_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_800_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_800_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_802_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_802_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_804_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_804_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_806_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_806_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_808_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_808_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_810_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_810_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_812_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_812_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_814_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_814_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_816_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_816_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_818_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_818_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_820_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_820_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_822_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_822_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_824_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_824_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_826_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_826_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_828_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_828_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_830_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_830_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_832_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_832_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_834_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_834_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_836_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_836_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_838_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_838_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_840_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_840_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_842_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_842_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_844_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_844_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_846_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_846_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_848_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_848_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_850_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_850_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_852_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_852_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_854_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_854_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_856_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_856_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_858_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_858_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_860_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_860_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_862_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_862_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_864_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_864_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_866_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_866_core_awake)));
    vlSelfRef.rspu_top__DOT__core_top_call_868_gated_clk 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.clk) 
                                        & (IData)(vlSelfRef.rspu_top__DOT__core_top_call_868_core_awake)));
    vlSelfRef.global_trap = ((IData)(vlSelfRef.rst_n) 
                             && ((((IData)(vlSelfRef.rspu_top__DOT__trap_0) 
                                   | (IData)(vlSelfRef.rspu_top__DOT__trap_1)) 
                                  | (IData)(vlSelfRef.rspu_top__DOT__trap_2)) 
                                 | (IData)(vlSelfRef.rspu_top__DOT__trap_3)));
    if (vlSelfRef.rst_n) {
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_0_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_0_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_0_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_0_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[2U] 
            = (IData)(vlSelfRef.io_in_0);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[3U] 
            = (IData)((vlSelfRef.io_in_0 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_1_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_1_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_1_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_1_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[2U] 
            = (IData)(vlSelfRef.io_in_1);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[3U] 
            = (IData)((vlSelfRef.io_in_1 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_2_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_2_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_2_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_2_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[2U] 
            = (IData)(vlSelfRef.io_in_2);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[3U] 
            = (IData)((vlSelfRef.io_in_2 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_3_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_3_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_3_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_3_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[2U] 
            = (IData)(vlSelfRef.io_in_3);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[3U] 
            = (IData)((vlSelfRef.io_in_3 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_4_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_4_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_4_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_4_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[2U] 
            = (IData)(vlSelfRef.io_in_4);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[3U] 
            = (IData)((vlSelfRef.io_in_4 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_5_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_5_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_5_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_5_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[2U] 
            = (IData)(vlSelfRef.io_in_5);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[3U] 
            = (IData)((vlSelfRef.io_in_5 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_6_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_6_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_6_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_6_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[2U] 
            = (IData)(vlSelfRef.io_in_6);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[3U] 
            = (IData)((vlSelfRef.io_in_6 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_7_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_7_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_7_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_7_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[2U] 
            = (IData)(vlSelfRef.io_in_7);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[3U] 
            = (IData)((vlSelfRef.io_in_7 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_8_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_8_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_8_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_8_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[2U] 
            = (IData)(vlSelfRef.io_in_8);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[3U] 
            = (IData)((vlSelfRef.io_in_8 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_9_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_9_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_9_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_9_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[2U] 
            = (IData)(vlSelfRef.io_in_9);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[3U] 
            = (IData)((vlSelfRef.io_in_9 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_10_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_10_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_10_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_10_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[2U] 
            = (IData)(vlSelfRef.io_in_10);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[3U] 
            = (IData)((vlSelfRef.io_in_10 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_11_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_11_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_11_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_11_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[2U] 
            = (IData)(vlSelfRef.io_in_11);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[3U] 
            = (IData)((vlSelfRef.io_in_11 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_12_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_12_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_12_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_12_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[2U] 
            = (IData)(vlSelfRef.io_in_12);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[3U] 
            = (IData)((vlSelfRef.io_in_12 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_13_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_13_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_13_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_13_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[2U] 
            = (IData)(vlSelfRef.io_in_13);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[3U] 
            = (IData)((vlSelfRef.io_in_13 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_14_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_14_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_14_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_14_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[2U] 
            = (IData)(vlSelfRef.io_in_14);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[3U] 
            = (IData)((vlSelfRef.io_in_14 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_15_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_15_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_15_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_15_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[2U] 
            = (IData)(vlSelfRef.io_in_15);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[3U] 
            = (IData)((vlSelfRef.io_in_15 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_16_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_16_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_16_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_16_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[2U] 
            = (IData)(vlSelfRef.io_in_16);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[3U] 
            = (IData)((vlSelfRef.io_in_16 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_17_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_17_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_17_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_17_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[2U] 
            = (IData)(vlSelfRef.io_in_17);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[3U] 
            = (IData)((vlSelfRef.io_in_17 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_18_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_18_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_18_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_18_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[2U] 
            = (IData)(vlSelfRef.io_in_18);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[3U] 
            = (IData)((vlSelfRef.io_in_18 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_19_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_19_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_19_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_19_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[2U] 
            = (IData)(vlSelfRef.io_in_19);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[3U] 
            = (IData)((vlSelfRef.io_in_19 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_20_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_20_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_20_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_20_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[2U] 
            = (IData)(vlSelfRef.io_in_20);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[3U] 
            = (IData)((vlSelfRef.io_in_20 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_21_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_21_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_21_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_21_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[2U] 
            = (IData)(vlSelfRef.io_in_21);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[3U] 
            = (IData)((vlSelfRef.io_in_21 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_22_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_22_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_22_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_22_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[2U] 
            = (IData)(vlSelfRef.io_in_22);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[3U] 
            = (IData)((vlSelfRef.io_in_22 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_23_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_23_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_23_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_23_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[2U] 
            = (IData)(vlSelfRef.io_in_23);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[3U] 
            = (IData)((vlSelfRef.io_in_23 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_24_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_24_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_24_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_24_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[2U] 
            = (IData)(vlSelfRef.io_in_24);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[3U] 
            = (IData)((vlSelfRef.io_in_24 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_25_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_25_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_25_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_25_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[2U] 
            = (IData)(vlSelfRef.io_in_25);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[3U] 
            = (IData)((vlSelfRef.io_in_25 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_26_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_26_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_26_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_26_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[2U] 
            = (IData)(vlSelfRef.io_in_26);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[3U] 
            = (IData)((vlSelfRef.io_in_26 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_27_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_27_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_27_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_27_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[2U] 
            = (IData)(vlSelfRef.io_in_27);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[3U] 
            = (IData)((vlSelfRef.io_in_27 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_28_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_28_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_28_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_28_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[2U] 
            = (IData)(vlSelfRef.io_in_28);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[3U] 
            = (IData)((vlSelfRef.io_in_28 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_29_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_29_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_29_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_29_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[2U] 
            = (IData)(vlSelfRef.io_in_29);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[3U] 
            = (IData)((vlSelfRef.io_in_29 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_30_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_30_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_30_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_30_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[2U] 
            = (IData)(vlSelfRef.io_in_30);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[3U] 
            = (IData)((vlSelfRef.io_in_30 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_31_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_31_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_31_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_31_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[2U] 
            = (IData)(vlSelfRef.io_in_31);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[3U] 
            = (IData)((vlSelfRef.io_in_31 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_32_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_32_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_32_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_32_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[2U] 
            = (IData)(vlSelfRef.io_in_32);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[3U] 
            = (IData)((vlSelfRef.io_in_32 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_33_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_33_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_33_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_33_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[2U] 
            = (IData)(vlSelfRef.io_in_33);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[3U] 
            = (IData)((vlSelfRef.io_in_33 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_34_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_34_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_34_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_34_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[2U] 
            = (IData)(vlSelfRef.io_in_34);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[3U] 
            = (IData)((vlSelfRef.io_in_34 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_35_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_35_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_35_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_35_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[2U] 
            = (IData)(vlSelfRef.io_in_35);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[3U] 
            = (IData)((vlSelfRef.io_in_35 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_36_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_36_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_36_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_36_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[2U] 
            = (IData)(vlSelfRef.io_in_36);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[3U] 
            = (IData)((vlSelfRef.io_in_36 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_37_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_37_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_37_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_37_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[2U] 
            = (IData)(vlSelfRef.io_in_37);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[3U] 
            = (IData)((vlSelfRef.io_in_37 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_38_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_38_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_38_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_38_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[2U] 
            = (IData)(vlSelfRef.io_in_38);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[3U] 
            = (IData)((vlSelfRef.io_in_38 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_39_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_39_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_39_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_39_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[2U] 
            = (IData)(vlSelfRef.io_in_39);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[3U] 
            = (IData)((vlSelfRef.io_in_39 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_40_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_40_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_40_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_40_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[2U] 
            = (IData)(vlSelfRef.io_in_40);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[3U] 
            = (IData)((vlSelfRef.io_in_40 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_41_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_41_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_41_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_41_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[2U] 
            = (IData)(vlSelfRef.io_in_41);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[3U] 
            = (IData)((vlSelfRef.io_in_41 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_42_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_42_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_42_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_42_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[2U] 
            = (IData)(vlSelfRef.io_in_42);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[3U] 
            = (IData)((vlSelfRef.io_in_42 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_43_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_43_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_43_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_43_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[2U] 
            = (IData)(vlSelfRef.io_in_43);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[3U] 
            = (IData)((vlSelfRef.io_in_43 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_44_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_44_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_44_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_44_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[2U] 
            = (IData)(vlSelfRef.io_in_44);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[3U] 
            = (IData)((vlSelfRef.io_in_44 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_45_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_45_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_45_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_45_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[2U] 
            = (IData)(vlSelfRef.io_in_45);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[3U] 
            = (IData)((vlSelfRef.io_in_45 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_46_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_46_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_46_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_46_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[2U] 
            = (IData)(vlSelfRef.io_in_46);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[3U] 
            = (IData)((vlSelfRef.io_in_46 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_47_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_47_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_47_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_47_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[2U] 
            = (IData)(vlSelfRef.io_in_47);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[3U] 
            = (IData)((vlSelfRef.io_in_47 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_48_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_48_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_48_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_48_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[2U] 
            = (IData)(vlSelfRef.io_in_48);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[3U] 
            = (IData)((vlSelfRef.io_in_48 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_49_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_49_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_49_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_49_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[2U] 
            = (IData)(vlSelfRef.io_in_49);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[3U] 
            = (IData)((vlSelfRef.io_in_49 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_50_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_50_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_50_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_50_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[2U] 
            = (IData)(vlSelfRef.io_in_50);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[3U] 
            = (IData)((vlSelfRef.io_in_50 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_51_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_51_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_51_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_51_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[2U] 
            = (IData)(vlSelfRef.io_in_51);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[3U] 
            = (IData)((vlSelfRef.io_in_51 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_52_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_52_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_52_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_52_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[2U] 
            = (IData)(vlSelfRef.io_in_52);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[3U] 
            = (IData)((vlSelfRef.io_in_52 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_53_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_53_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_53_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_53_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[2U] 
            = (IData)(vlSelfRef.io_in_53);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[3U] 
            = (IData)((vlSelfRef.io_in_53 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_54_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_54_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_54_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_54_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[2U] 
            = (IData)(vlSelfRef.io_in_54);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[3U] 
            = (IData)((vlSelfRef.io_in_54 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_55_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_55_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_55_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_55_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[2U] 
            = (IData)(vlSelfRef.io_in_55);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[3U] 
            = (IData)((vlSelfRef.io_in_55 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_56_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_56_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_56_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_56_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[2U] 
            = (IData)(vlSelfRef.io_in_56);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[3U] 
            = (IData)((vlSelfRef.io_in_56 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_57_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_57_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_57_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_57_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[2U] 
            = (IData)(vlSelfRef.io_in_57);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[3U] 
            = (IData)((vlSelfRef.io_in_57 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_58_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_58_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_58_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_58_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[2U] 
            = (IData)(vlSelfRef.io_in_58);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[3U] 
            = (IData)((vlSelfRef.io_in_58 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_59_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_59_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_59_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_59_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[2U] 
            = (IData)(vlSelfRef.io_in_59);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[3U] 
            = (IData)((vlSelfRef.io_in_59 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_60_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_60_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_60_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_60_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[2U] 
            = (IData)(vlSelfRef.io_in_60);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[3U] 
            = (IData)((vlSelfRef.io_in_60 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_61_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_61_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_61_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_61_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[2U] 
            = (IData)(vlSelfRef.io_in_61);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[3U] 
            = (IData)((vlSelfRef.io_in_61 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_62_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_62_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_62_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_62_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[2U] 
            = (IData)(vlSelfRef.io_in_62);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[3U] 
            = (IData)((vlSelfRef.io_in_62 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_63_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_63_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_63_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__io_in_63_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[2U] 
            = (IData)(vlSelfRef.io_in_63);
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[3U] 
            = (IData)((vlSelfRef.io_in_63 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_0_sync 
            = (((IData)(vlSelfRef.downlink_valid_0) 
                << 1U) | (1U & ((IData)(vlSelfRef.rspu_top__DOT__downlink_valid_0_sync) 
                                >> 1U)));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_0_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_0_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_0_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_0_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[2U] 
            = (IData)(vlSelfRef.downlink_data_0);
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[3U] 
            = (IData)((vlSelfRef.downlink_data_0 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_1_sync 
            = (((IData)(vlSelfRef.downlink_valid_1) 
                << 1U) | (1U & ((IData)(vlSelfRef.rspu_top__DOT__downlink_valid_1_sync) 
                                >> 1U)));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_1_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_1_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_1_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_1_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[2U] 
            = (IData)(vlSelfRef.downlink_data_1);
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[3U] 
            = (IData)((vlSelfRef.downlink_data_1 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_2_sync 
            = (((IData)(vlSelfRef.downlink_valid_2) 
                << 1U) | (1U & ((IData)(vlSelfRef.rspu_top__DOT__downlink_valid_2_sync) 
                                >> 1U)));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_2_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_2_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_2_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_2_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[2U] 
            = (IData)(vlSelfRef.downlink_data_2);
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[3U] 
            = (IData)((vlSelfRef.downlink_data_2 >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_3_sync 
            = (((IData)(vlSelfRef.downlink_valid_3) 
                << 1U) | (1U & ((IData)(vlSelfRef.rspu_top__DOT__downlink_valid_3_sync) 
                                >> 1U)));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[0U] 
            = (IData)((((QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_3_sync[3U])) 
                        << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_3_sync[2U]))));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[1U] 
            = (IData)(((((QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_3_sync[3U])) 
                         << 0x00000020U) | (QData)((IData)(vlSelfRef.rspu_top__DOT__downlink_data_3_sync[2U]))) 
                       >> 0x00000020U));
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[2U] 
            = (IData)(vlSelfRef.downlink_data_3);
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[3U] 
            = (IData)((vlSelfRef.downlink_data_3 >> 0x00000020U));
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_10373_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_10908_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_11443_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_11978_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_12513_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_pcc_valid) {
            __Vdly__rspu_top__DOT__rspu_pipeline_call_13048_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_pcc_valid) {
            __Vdly__rspu_top__DOT__rspu_pipeline_call_13583_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_14118_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_14653_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_15188_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_15723_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_16258_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_16793_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_17328_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_17863_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_18398_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_18933_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_19468_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_20003_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_20538_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_21073_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_pcc_valid) {
            __Vdly__rspu_top__DOT__rspu_pipeline_call_21608_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_pcc_valid) {
            __Vdly__rspu_top__DOT__rspu_pipeline_call_22143_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_22678_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_23213_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_23748_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_24283_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_24818_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_25353_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_25888_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_26423_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_26958_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_27493_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_28028_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_28563_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_29098_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_29633_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_pcc_valid) {
            __Vdly__rspu_top__DOT__rspu_pipeline_call_30168_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_pcc_valid) {
            __Vdly__rspu_top__DOT__rspu_pipeline_call_30703_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_31238_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_31773_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_32308_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_32843_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_33378_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_33913_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_34448_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_34983_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_35518_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_36053_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_36588_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_37123_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_37658_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_38193_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_pcc_valid) {
            __Vdly__rspu_top__DOT__rspu_pipeline_call_4488_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_pcc_valid) {
            __Vdly__rspu_top__DOT__rspu_pipeline_call_5023_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_5558_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_6093_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_6628_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_7163_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_7698_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_8233_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_8768_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_9303_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_pcc_valid) {
            vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_9838_pipe_pc 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_pipe_pc);
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_rs1 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_if_id_instr, 0x00000030U));
            vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_rs2 
                = (0x00000000000003ffULL & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_if_id_instr, 0x00000020U));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_0) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_0 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_0)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_0 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_0);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_1) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_1 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_1)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_1 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_1);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_10) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_10 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_10)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_10 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_10);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_11) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_11 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_11)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_11 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_11);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_12) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_12 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_12)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_12 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_12);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_13) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_13 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_13)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_13 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_13);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_14) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_14 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_14)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_14 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_14);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_15) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_15 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_15)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_15 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_15);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_2) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_2 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_2)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_2 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_2);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_3) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_3 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_3)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_3 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_3);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_4) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_4 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_4)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_4 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_4);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_5) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_5 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_5)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_5 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_5);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_6) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_6 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_6)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_6 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_6);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_7) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_7 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_7)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_7 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_7);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_8) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_8 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_8)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_8 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_8);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_9) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_9 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_9)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_9 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_9);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_16) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_0 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_16)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_0 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_0);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_17) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_1 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_17)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_1 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_1);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_26) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_10 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_26)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_10 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_10);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_27) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_11 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_27)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_11 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_11);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_28) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_12 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_28)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_12 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_12);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_29) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_13 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_29)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_13 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_13);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_30) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_14 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_30)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_14 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_14);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_31) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_15 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_31)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_15 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_15);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_18) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_2 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_18)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_2 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_2);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_19) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_3 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_19)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_3 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_3);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_20) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_4 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_20)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_4 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_4);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_21) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_5 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_21)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_5 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_5);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_22) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_6 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_22)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_6 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_6);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_23) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_7 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_23)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_7 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_7);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_24) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_8 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_24)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_8 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_8);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_25) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_9 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_25)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_9 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_9);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_32) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_0 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_32)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_0 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_0);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_33) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_1 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_33)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_1 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_1);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_42) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_10 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_42)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_10 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_10);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_43) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_11 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_43)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_11 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_11);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_44) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_12 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_44)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_12 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_12);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_45) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_13 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_45)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_13 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_13);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_46) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_14 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_46)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_14 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_14);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_47) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_15 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_47)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_15 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_15);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_34) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_2 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_34)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_2 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_2);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_35) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_3 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_35)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_3 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_3);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_36) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_4 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_36)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_4 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_4);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_37) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_5 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_37)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_5 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_5);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_38) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_6 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_38)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_6 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_6);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_39) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_7 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_39)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_7 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_7);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_40) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_8 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_40)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_8 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_8);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_41) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_9 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_41)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_9 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_9);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_48) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_0 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_48)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_0 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_0);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_49) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_1 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_49)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_1 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_1);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_58) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_10 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_58)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_10 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_10);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_59) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_11 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_59)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_11 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_11);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_60) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_12 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_60)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_12 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_12);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_61) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_13 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_61)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_13 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_13);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_62) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_14 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_62)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_14 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_14);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_63) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_15 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_63)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_15 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_15);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_50) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_2 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_50)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_2 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_2);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_51) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_3 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_51)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_3 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_3);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_52) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_4 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_52)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_4 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_4);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_53) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_5 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_53)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_5 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_5);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_54) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_6 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_54)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_6 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_6);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_55) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_7 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_55)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_7 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_7);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_56) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_8 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_56)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_8 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_8);
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_57) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_9 = 0U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_57)))) {
            __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_9 
                = ((IData)(1U) + vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_9);
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_742_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_742_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_742_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_742_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_0) {
            __Vdly__rspu_top__DOT__core_top_call_742_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_742_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_0;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_744_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_744_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_744_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_744_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_1) {
            __Vdly__rspu_top__DOT__core_top_call_744_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_744_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_1;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_746_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_746_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_746_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_746_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_2) {
            __Vdly__rspu_top__DOT__core_top_call_746_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_746_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_2;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_748_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_748_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_748_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_748_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_3) {
            __Vdly__rspu_top__DOT__core_top_call_748_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_748_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_3;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_750_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_750_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_750_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_750_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_4) {
            __Vdly__rspu_top__DOT__core_top_call_750_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_750_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_4;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_752_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_752_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_752_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_752_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_5) {
            __Vdly__rspu_top__DOT__core_top_call_752_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_752_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_5;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_754_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_754_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_754_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_754_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_6) {
            __Vdly__rspu_top__DOT__core_top_call_754_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_754_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_6;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_756_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_756_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_756_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_756_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_7) {
            __Vdly__rspu_top__DOT__core_top_call_756_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_756_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_7;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_758_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_758_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_758_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_758_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_8) {
            __Vdly__rspu_top__DOT__core_top_call_758_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_758_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_8;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_760_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_760_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_760_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_760_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_9) {
            __Vdly__rspu_top__DOT__core_top_call_760_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_760_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_9;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_762_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_762_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_762_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_762_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_10) {
            __Vdly__rspu_top__DOT__core_top_call_762_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_762_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_10;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_764_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_764_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_764_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_764_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_11) {
            __Vdly__rspu_top__DOT__core_top_call_764_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_764_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_11;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_766_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_766_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_766_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_766_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_12) {
            __Vdly__rspu_top__DOT__core_top_call_766_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_766_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_12;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_768_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_768_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_768_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_768_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_13) {
            __Vdly__rspu_top__DOT__core_top_call_768_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_768_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_13;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_770_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_770_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_770_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_770_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_14) {
            __Vdly__rspu_top__DOT__core_top_call_770_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_770_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_14;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_772_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_772_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_772_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_772_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_15) {
            __Vdly__rspu_top__DOT__core_top_call_772_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_772_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_15;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_774_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_774_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_774_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_774_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_16) {
            __Vdly__rspu_top__DOT__core_top_call_774_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_774_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_16;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_776_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_776_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_776_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_776_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_17) {
            __Vdly__rspu_top__DOT__core_top_call_776_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_776_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_17;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_778_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_778_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_778_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_778_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_18) {
            __Vdly__rspu_top__DOT__core_top_call_778_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_778_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_18;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_780_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_780_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_780_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_780_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_19) {
            __Vdly__rspu_top__DOT__core_top_call_780_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_780_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_19;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_782_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_782_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_782_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_782_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_20) {
            __Vdly__rspu_top__DOT__core_top_call_782_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_782_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_20;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_784_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_784_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_784_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_784_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_21) {
            __Vdly__rspu_top__DOT__core_top_call_784_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_784_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_21;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_786_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_786_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_786_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_786_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_22) {
            __Vdly__rspu_top__DOT__core_top_call_786_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_786_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_22;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_788_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_788_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_788_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_788_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_23) {
            __Vdly__rspu_top__DOT__core_top_call_788_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_788_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_23;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_790_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_790_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_790_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_790_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_24) {
            __Vdly__rspu_top__DOT__core_top_call_790_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_790_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_24;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_792_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_792_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_792_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_792_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_25) {
            __Vdly__rspu_top__DOT__core_top_call_792_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_792_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_25;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_794_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_794_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_794_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_794_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_26) {
            __Vdly__rspu_top__DOT__core_top_call_794_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_794_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_26;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_796_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_796_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_796_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_796_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_27) {
            __Vdly__rspu_top__DOT__core_top_call_796_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_796_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_27;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_798_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_798_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_798_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_798_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_28) {
            __Vdly__rspu_top__DOT__core_top_call_798_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_798_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_28;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_800_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_800_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_800_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_800_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_29) {
            __Vdly__rspu_top__DOT__core_top_call_800_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_800_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_29;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_802_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_802_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_802_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_802_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_30) {
            __Vdly__rspu_top__DOT__core_top_call_802_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_802_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_30;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_804_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_804_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_804_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_804_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_31) {
            __Vdly__rspu_top__DOT__core_top_call_804_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_804_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_31;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_806_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_806_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_806_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_806_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_32) {
            __Vdly__rspu_top__DOT__core_top_call_806_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_806_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_32;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_808_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_808_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_808_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_808_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_33) {
            __Vdly__rspu_top__DOT__core_top_call_808_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_808_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_33;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_810_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_810_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_810_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_810_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_34) {
            __Vdly__rspu_top__DOT__core_top_call_810_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_810_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_34;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_812_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_812_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_812_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_812_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_35) {
            __Vdly__rspu_top__DOT__core_top_call_812_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_812_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_35;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_814_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_814_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_814_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_814_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_36) {
            __Vdly__rspu_top__DOT__core_top_call_814_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_814_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_36;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_816_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_816_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_816_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_816_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_37) {
            __Vdly__rspu_top__DOT__core_top_call_816_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_816_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_37;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_818_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_818_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_818_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_818_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_38) {
            __Vdly__rspu_top__DOT__core_top_call_818_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_818_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_38;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_820_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_820_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_820_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_820_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_39) {
            __Vdly__rspu_top__DOT__core_top_call_820_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_820_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_39;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_822_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_822_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_822_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_822_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_40) {
            __Vdly__rspu_top__DOT__core_top_call_822_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_822_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_40;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_824_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_824_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_824_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_824_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_41) {
            __Vdly__rspu_top__DOT__core_top_call_824_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_824_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_41;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_826_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_826_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_826_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_826_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_42) {
            __Vdly__rspu_top__DOT__core_top_call_826_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_826_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_42;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_828_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_828_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_828_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_828_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_43) {
            __Vdly__rspu_top__DOT__core_top_call_828_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_828_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_43;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_830_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_830_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_830_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_830_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_44) {
            __Vdly__rspu_top__DOT__core_top_call_830_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_830_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_44;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_832_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_832_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_832_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_832_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_45) {
            __Vdly__rspu_top__DOT__core_top_call_832_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_832_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_45;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_834_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_834_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_834_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_834_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_46) {
            __Vdly__rspu_top__DOT__core_top_call_834_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_834_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_46;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_836_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_836_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_836_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_836_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_47) {
            __Vdly__rspu_top__DOT__core_top_call_836_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_836_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_47;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_838_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_838_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_838_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_838_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_48) {
            __Vdly__rspu_top__DOT__core_top_call_838_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_838_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_48;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_840_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_840_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_840_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_840_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_49) {
            __Vdly__rspu_top__DOT__core_top_call_840_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_840_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_49;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_842_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_842_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_842_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_842_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_50) {
            __Vdly__rspu_top__DOT__core_top_call_842_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_842_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_50;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_844_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_844_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_844_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_844_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_51) {
            __Vdly__rspu_top__DOT__core_top_call_844_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_844_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_51;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_846_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_846_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_846_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_846_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_52) {
            __Vdly__rspu_top__DOT__core_top_call_846_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_846_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_52;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_848_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_848_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_848_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_848_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_53) {
            __Vdly__rspu_top__DOT__core_top_call_848_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_848_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_53;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_850_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_850_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_850_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_850_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_54) {
            __Vdly__rspu_top__DOT__core_top_call_850_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_850_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_54;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_852_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_852_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_852_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_852_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_55) {
            __Vdly__rspu_top__DOT__core_top_call_852_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_852_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_55;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_854_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_854_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_854_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_854_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_56) {
            __Vdly__rspu_top__DOT__core_top_call_854_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_854_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_56;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_856_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_856_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_856_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_856_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_57) {
            __Vdly__rspu_top__DOT__core_top_call_856_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_856_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_57;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_858_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_858_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_858_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_858_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_58) {
            __Vdly__rspu_top__DOT__core_top_call_858_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_858_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_58;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_860_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_860_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_860_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_860_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_59) {
            __Vdly__rspu_top__DOT__core_top_call_860_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_860_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_59;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_862_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_862_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_862_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_862_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_60) {
            __Vdly__rspu_top__DOT__core_top_call_862_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_862_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_60;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_864_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_864_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_864_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_864_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_61) {
            __Vdly__rspu_top__DOT__core_top_call_864_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_864_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_61;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_866_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_866_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_866_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_866_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_62) {
            __Vdly__rspu_top__DOT__core_top_call_866_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_866_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_62;
        }
        if (vlSelfRef.rspu_top__DOT__core_top_call_868_auto_g_1_out) {
            __Vdly__rspu_top__DOT__core_top_call_868_wake_timer 
                = (7U & ((IData)(vlSelfRef.rspu_top__DOT__core_top_call_868_wake_timer) 
                         - (IData)(1U)));
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_868_current_instr = 0x00000000000000ffULL;
        }
        if (vlSelfRef.rspu_top__DOT__rx_valid_63) {
            __Vdly__rspu_top__DOT__core_top_call_868_wake_timer = 6U;
            vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_868_current_instr 
                = vlSelfRef.rspu_top__DOT__rx_data_63;
        }
        vlSelfRef.out_pc_0 = vlSelfRef.rspu_top__DOT__pc_0;
        vlSelfRef.out_pc_1 = vlSelfRef.rspu_top__DOT__pc_1;
        vlSelfRef.out_pc_10 = vlSelfRef.rspu_top__DOT__pc_10;
        vlSelfRef.out_pc_11 = vlSelfRef.rspu_top__DOT__pc_11;
        vlSelfRef.out_pc_12 = vlSelfRef.rspu_top__DOT__pc_12;
        vlSelfRef.out_pc_13 = vlSelfRef.rspu_top__DOT__pc_13;
        vlSelfRef.out_pc_14 = vlSelfRef.rspu_top__DOT__pc_14;
        vlSelfRef.out_pc_15 = vlSelfRef.rspu_top__DOT__pc_15;
        vlSelfRef.out_pc_16 = vlSelfRef.rspu_top__DOT__pc_16;
        vlSelfRef.out_pc_17 = vlSelfRef.rspu_top__DOT__pc_17;
        vlSelfRef.out_pc_18 = vlSelfRef.rspu_top__DOT__pc_18;
        vlSelfRef.out_pc_19 = vlSelfRef.rspu_top__DOT__pc_19;
        vlSelfRef.out_pc_2 = vlSelfRef.rspu_top__DOT__pc_2;
        vlSelfRef.out_pc_20 = vlSelfRef.rspu_top__DOT__pc_20;
        vlSelfRef.out_pc_21 = vlSelfRef.rspu_top__DOT__pc_21;
        vlSelfRef.out_pc_22 = vlSelfRef.rspu_top__DOT__pc_22;
        vlSelfRef.out_pc_23 = vlSelfRef.rspu_top__DOT__pc_23;
        vlSelfRef.out_pc_24 = vlSelfRef.rspu_top__DOT__pc_24;
        vlSelfRef.out_pc_25 = vlSelfRef.rspu_top__DOT__pc_25;
        vlSelfRef.out_pc_26 = vlSelfRef.rspu_top__DOT__pc_26;
        vlSelfRef.out_pc_27 = vlSelfRef.rspu_top__DOT__pc_27;
        vlSelfRef.out_pc_28 = vlSelfRef.rspu_top__DOT__pc_28;
        vlSelfRef.out_pc_29 = vlSelfRef.rspu_top__DOT__pc_29;
        vlSelfRef.out_pc_3 = vlSelfRef.rspu_top__DOT__pc_3;
        vlSelfRef.out_pc_30 = vlSelfRef.rspu_top__DOT__pc_30;
        vlSelfRef.out_pc_31 = vlSelfRef.rspu_top__DOT__pc_31;
        vlSelfRef.out_pc_32 = vlSelfRef.rspu_top__DOT__pc_32;
        vlSelfRef.out_pc_33 = vlSelfRef.rspu_top__DOT__pc_33;
        vlSelfRef.out_pc_34 = vlSelfRef.rspu_top__DOT__pc_34;
        vlSelfRef.out_pc_35 = vlSelfRef.rspu_top__DOT__pc_35;
        vlSelfRef.out_pc_36 = vlSelfRef.rspu_top__DOT__pc_36;
        vlSelfRef.out_pc_37 = vlSelfRef.rspu_top__DOT__pc_37;
        vlSelfRef.out_pc_38 = vlSelfRef.rspu_top__DOT__pc_38;
        vlSelfRef.out_pc_39 = vlSelfRef.rspu_top__DOT__pc_39;
        vlSelfRef.out_pc_4 = vlSelfRef.rspu_top__DOT__pc_4;
        vlSelfRef.out_pc_40 = vlSelfRef.rspu_top__DOT__pc_40;
        vlSelfRef.out_pc_41 = vlSelfRef.rspu_top__DOT__pc_41;
        vlSelfRef.out_pc_42 = vlSelfRef.rspu_top__DOT__pc_42;
        vlSelfRef.out_pc_43 = vlSelfRef.rspu_top__DOT__pc_43;
        vlSelfRef.out_pc_44 = vlSelfRef.rspu_top__DOT__pc_44;
        vlSelfRef.out_pc_45 = vlSelfRef.rspu_top__DOT__pc_45;
        vlSelfRef.out_pc_46 = vlSelfRef.rspu_top__DOT__pc_46;
        vlSelfRef.out_pc_47 = vlSelfRef.rspu_top__DOT__pc_47;
        vlSelfRef.out_pc_48 = vlSelfRef.rspu_top__DOT__pc_48;
        vlSelfRef.out_pc_49 = vlSelfRef.rspu_top__DOT__pc_49;
        vlSelfRef.out_pc_5 = vlSelfRef.rspu_top__DOT__pc_5;
        vlSelfRef.out_pc_50 = vlSelfRef.rspu_top__DOT__pc_50;
        vlSelfRef.out_pc_51 = vlSelfRef.rspu_top__DOT__pc_51;
        vlSelfRef.out_pc_52 = vlSelfRef.rspu_top__DOT__pc_52;
        vlSelfRef.out_pc_53 = vlSelfRef.rspu_top__DOT__pc_53;
        vlSelfRef.out_pc_54 = vlSelfRef.rspu_top__DOT__pc_54;
        vlSelfRef.out_pc_55 = vlSelfRef.rspu_top__DOT__pc_55;
        vlSelfRef.out_pc_56 = vlSelfRef.rspu_top__DOT__pc_56;
        vlSelfRef.out_pc_57 = vlSelfRef.rspu_top__DOT__pc_57;
        vlSelfRef.out_pc_58 = vlSelfRef.rspu_top__DOT__pc_58;
        vlSelfRef.out_pc_59 = vlSelfRef.rspu_top__DOT__pc_59;
        vlSelfRef.out_pc_6 = vlSelfRef.rspu_top__DOT__pc_6;
        vlSelfRef.out_pc_60 = vlSelfRef.rspu_top__DOT__pc_60;
        vlSelfRef.out_pc_61 = vlSelfRef.rspu_top__DOT__pc_61;
        vlSelfRef.out_pc_62 = vlSelfRef.rspu_top__DOT__pc_62;
        vlSelfRef.out_pc_63 = vlSelfRef.rspu_top__DOT__pc_63;
        vlSelfRef.out_pc_7 = vlSelfRef.rspu_top__DOT__pc_7;
        vlSelfRef.out_pc_8 = vlSelfRef.rspu_top__DOT__pc_8;
        vlSelfRef.out_pc_9 = vlSelfRef.rspu_top__DOT__pc_9;
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_is_store_out) {
            vlSelfRef.io_out_0 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_val1;
        }
    } else {
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_0_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_1_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_2_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_3_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_4_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_5_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_6_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_7_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_8_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_9_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_10_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_11_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_12_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_13_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_14_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_15_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_16_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_17_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_18_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_19_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_20_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_21_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_22_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_23_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_24_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_25_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_26_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_27_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_28_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_29_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_30_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_31_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_32_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_33_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_34_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_35_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_36_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_37_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_38_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_39_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_40_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_41_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_42_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_43_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_44_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_45_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_46_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_47_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_48_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_49_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_50_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_51_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_52_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_53_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_54_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_55_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_56_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_57_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_58_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_59_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_60_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_61_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_62_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__io_in_63_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_0_sync = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_0_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_1_sync = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_1_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_2_sync = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_2_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_valid_3_sync = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[0U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[1U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[2U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__downlink_data_3_sync[3U] = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_10373_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_10908_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_11443_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_11978_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_12513_pipe_pc = 0U;
        __Vdly__rspu_top__DOT__rspu_pipeline_call_13048_pipe_pc = 0U;
        __Vdly__rspu_top__DOT__rspu_pipeline_call_13583_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_14118_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_14653_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_15188_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_15723_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_16258_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_16793_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_17328_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_17863_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_18398_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_18933_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_19468_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_20003_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_20538_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_21073_pipe_pc = 0U;
        __Vdly__rspu_top__DOT__rspu_pipeline_call_21608_pipe_pc = 0U;
        __Vdly__rspu_top__DOT__rspu_pipeline_call_22143_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_22678_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_23213_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_23748_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_24283_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_24818_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_25353_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_25888_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_26423_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_26958_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_27493_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_28028_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_28563_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_29098_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_29633_pipe_pc = 0U;
        __Vdly__rspu_top__DOT__rspu_pipeline_call_30168_pipe_pc = 0U;
        __Vdly__rspu_top__DOT__rspu_pipeline_call_30703_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_31238_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_31773_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_32308_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_32843_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_33378_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_33913_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_34448_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_34983_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_35518_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_36053_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_36588_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_37123_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_37658_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_38193_pipe_pc = 0U;
        __Vdly__rspu_top__DOT__rspu_pipeline_call_4488_pipe_pc = 0U;
        __Vdly__rspu_top__DOT__rspu_pipeline_call_5023_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_5558_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_6093_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_6628_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_7163_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_7698_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_8233_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_8768_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_9303_pipe_pc = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__rspu_pipeline_call_9838_pipe_pc = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_0 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_1 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_10 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_11 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_12 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_13 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_14 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_15 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_2 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_3 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_4 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_5 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_6 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_7 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_8 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_9 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_0 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_1 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_10 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_11 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_12 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_13 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_14 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_15 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_2 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_3 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_4 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_5 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_6 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_7 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_8 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_9 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_0 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_1 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_10 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_11 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_12 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_13 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_14 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_15 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_2 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_3 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_4 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_5 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_6 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_7 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_8 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_9 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_0 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_1 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_10 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_11 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_12 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_13 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_14 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_15 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_2 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_3 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_4 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_5 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_6 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_7 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_8 = 0U;
        __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_9 = 0U;
        __Vdly__rspu_top__DOT__core_top_call_742_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_744_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_746_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_748_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_750_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_752_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_754_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_756_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_758_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_760_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_762_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_764_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_766_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_768_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_770_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_772_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_774_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_776_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_778_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_780_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_782_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_784_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_786_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_788_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_790_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_792_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_794_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_796_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_798_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_800_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_802_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_804_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_806_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_808_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_810_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_812_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_814_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_816_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_818_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_820_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_822_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_824_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_826_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_828_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_830_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_832_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_834_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_836_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_838_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_840_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_842_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_844_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_846_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_848_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_850_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_852_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_854_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_856_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_858_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_860_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_862_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_864_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_866_wake_timer = 0U;
        __Vdly__rspu_top__DOT__core_top_call_868_wake_timer = 0U;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_742_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_744_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_746_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_748_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_750_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_752_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_754_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_756_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_758_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_760_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_762_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_764_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_766_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_768_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_770_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_772_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_774_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_776_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_778_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_780_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_782_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_784_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_786_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_788_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_790_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_792_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_794_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_796_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_798_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_800_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_802_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_804_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_806_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_808_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_810_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_812_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_814_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_816_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_818_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_820_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_822_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_824_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_826_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_828_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_830_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_832_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_834_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_836_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_838_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_840_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_842_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_844_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_846_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_848_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_850_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_852_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_854_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_856_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_858_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_860_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_862_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_864_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_866_current_instr = 0ULL;
        vlSelfRef.__Vdly__rspu_top__DOT__core_top_call_868_current_instr = 0ULL;
        vlSelfRef.out_pc_0 = 0U;
        vlSelfRef.out_pc_1 = 0U;
        vlSelfRef.out_pc_10 = 0U;
        vlSelfRef.out_pc_11 = 0U;
        vlSelfRef.out_pc_12 = 0U;
        vlSelfRef.out_pc_13 = 0U;
        vlSelfRef.out_pc_14 = 0U;
        vlSelfRef.out_pc_15 = 0U;
        vlSelfRef.out_pc_16 = 0U;
        vlSelfRef.out_pc_17 = 0U;
        vlSelfRef.out_pc_18 = 0U;
        vlSelfRef.out_pc_19 = 0U;
        vlSelfRef.out_pc_2 = 0U;
        vlSelfRef.out_pc_20 = 0U;
        vlSelfRef.out_pc_21 = 0U;
        vlSelfRef.out_pc_22 = 0U;
        vlSelfRef.out_pc_23 = 0U;
        vlSelfRef.out_pc_24 = 0U;
        vlSelfRef.out_pc_25 = 0U;
        vlSelfRef.out_pc_26 = 0U;
        vlSelfRef.out_pc_27 = 0U;
        vlSelfRef.out_pc_28 = 0U;
        vlSelfRef.out_pc_29 = 0U;
        vlSelfRef.out_pc_3 = 0U;
        vlSelfRef.out_pc_30 = 0U;
        vlSelfRef.out_pc_31 = 0U;
        vlSelfRef.out_pc_32 = 0U;
        vlSelfRef.out_pc_33 = 0U;
        vlSelfRef.out_pc_34 = 0U;
        vlSelfRef.out_pc_35 = 0U;
        vlSelfRef.out_pc_36 = 0U;
        vlSelfRef.out_pc_37 = 0U;
        vlSelfRef.out_pc_38 = 0U;
        vlSelfRef.out_pc_39 = 0U;
        vlSelfRef.out_pc_4 = 0U;
        vlSelfRef.out_pc_40 = 0U;
        vlSelfRef.out_pc_41 = 0U;
        vlSelfRef.out_pc_42 = 0U;
        vlSelfRef.out_pc_43 = 0U;
        vlSelfRef.out_pc_44 = 0U;
        vlSelfRef.out_pc_45 = 0U;
        vlSelfRef.out_pc_46 = 0U;
        vlSelfRef.out_pc_47 = 0U;
        vlSelfRef.out_pc_48 = 0U;
        vlSelfRef.out_pc_49 = 0U;
        vlSelfRef.out_pc_5 = 0U;
        vlSelfRef.out_pc_50 = 0U;
        vlSelfRef.out_pc_51 = 0U;
        vlSelfRef.out_pc_52 = 0U;
        vlSelfRef.out_pc_53 = 0U;
        vlSelfRef.out_pc_54 = 0U;
        vlSelfRef.out_pc_55 = 0U;
        vlSelfRef.out_pc_56 = 0U;
        vlSelfRef.out_pc_57 = 0U;
        vlSelfRef.out_pc_58 = 0U;
        vlSelfRef.out_pc_59 = 0U;
        vlSelfRef.out_pc_6 = 0U;
        vlSelfRef.out_pc_60 = 0U;
        vlSelfRef.out_pc_61 = 0U;
        vlSelfRef.out_pc_62 = 0U;
        vlSelfRef.out_pc_63 = 0U;
        vlSelfRef.out_pc_7 = 0U;
        vlSelfRef.out_pc_8 = 0U;
        vlSelfRef.out_pc_9 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_dec_rs2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_rs1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_dec_rs2 = 0ULL;
        vlSelfRef.io_out_0 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_is_store_out) {
            vlSelfRef.io_out_1 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_1 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_is_store_out) {
            vlSelfRef.io_out_10 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_10 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_is_store_out) {
            vlSelfRef.io_out_11 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_11 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_is_store_out) {
            vlSelfRef.io_out_12 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_12 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_is_store_out) {
            vlSelfRef.io_out_13 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_13 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_is_store_out) {
            vlSelfRef.io_out_14 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_14 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_is_store_out) {
            vlSelfRef.io_out_15 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_15 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_is_store_out) {
            vlSelfRef.io_out_16 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_16 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_is_store_out) {
            vlSelfRef.io_out_17 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_17 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_is_store_out) {
            vlSelfRef.io_out_18 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_18 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_is_store_out) {
            vlSelfRef.io_out_19 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_19 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_is_store_out) {
            vlSelfRef.io_out_2 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_2 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_is_store_out) {
            vlSelfRef.io_out_20 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_20 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_is_store_out) {
            vlSelfRef.io_out_21 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_21 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_is_store_out) {
            vlSelfRef.io_out_22 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_22 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_is_store_out) {
            vlSelfRef.io_out_23 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_23 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_is_store_out) {
            vlSelfRef.io_out_24 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_24 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_is_store_out) {
            vlSelfRef.io_out_25 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_25 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_is_store_out) {
            vlSelfRef.io_out_26 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_26 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_is_store_out) {
            vlSelfRef.io_out_27 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_27 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_is_store_out) {
            vlSelfRef.io_out_28 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_28 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_is_store_out) {
            vlSelfRef.io_out_29 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_29 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_is_store_out) {
            vlSelfRef.io_out_3 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_3 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_is_store_out) {
            vlSelfRef.io_out_30 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_30 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_is_store_out) {
            vlSelfRef.io_out_31 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_31 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_is_store_out) {
            vlSelfRef.io_out_32 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_32 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_is_store_out) {
            vlSelfRef.io_out_33 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_33 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_is_store_out) {
            vlSelfRef.io_out_34 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_34 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_is_store_out) {
            vlSelfRef.io_out_35 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_35 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_is_store_out) {
            vlSelfRef.io_out_36 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_36 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_is_store_out) {
            vlSelfRef.io_out_37 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_37 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_is_store_out) {
            vlSelfRef.io_out_38 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_38 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_is_store_out) {
            vlSelfRef.io_out_39 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_39 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_is_store_out) {
            vlSelfRef.io_out_4 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_4 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_is_store_out) {
            vlSelfRef.io_out_40 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_40 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_is_store_out) {
            vlSelfRef.io_out_41 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_41 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_is_store_out) {
            vlSelfRef.io_out_42 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_42 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_is_store_out) {
            vlSelfRef.io_out_43 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_43 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_is_store_out) {
            vlSelfRef.io_out_44 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_44 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_is_store_out) {
            vlSelfRef.io_out_45 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_45 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_is_store_out) {
            vlSelfRef.io_out_46 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_46 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_is_store_out) {
            vlSelfRef.io_out_47 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_47 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_is_store_out) {
            vlSelfRef.io_out_48 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_48 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_is_store_out) {
            vlSelfRef.io_out_49 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_49 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_is_store_out) {
            vlSelfRef.io_out_5 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_5 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_is_store_out) {
            vlSelfRef.io_out_50 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_50 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_is_store_out) {
            vlSelfRef.io_out_51 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_51 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_is_store_out) {
            vlSelfRef.io_out_52 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_52 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_is_store_out) {
            vlSelfRef.io_out_53 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_53 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_is_store_out) {
            vlSelfRef.io_out_54 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_54 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_is_store_out) {
            vlSelfRef.io_out_55 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_55 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_is_store_out) {
            vlSelfRef.io_out_56 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_56 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_is_store_out) {
            vlSelfRef.io_out_57 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_57 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_is_store_out) {
            vlSelfRef.io_out_58 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_58 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_is_store_out) {
            vlSelfRef.io_out_59 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_59 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_is_store_out) {
            vlSelfRef.io_out_6 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_6 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_is_store_out) {
            vlSelfRef.io_out_60 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_60 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_is_store_out) {
            vlSelfRef.io_out_61 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_61 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_is_store_out) {
            vlSelfRef.io_out_62 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_62 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_is_store_out) {
            vlSelfRef.io_out_63 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_63 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_is_store_out) {
            vlSelfRef.io_out_7 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_7 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_is_store_out) {
            vlSelfRef.io_out_8 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_8 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_id_ex_op));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_is_store_out) {
            vlSelfRef.io_out_9 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_val1;
        }
    } else {
        vlSelfRef.io_out_9 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_is_store_out 
        = ((IData)(vlSelfRef.rst_n) && (1ULL == vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_id_ex_op));
    vlSelfRef.rspu_top__DOT__core_top_call_742_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_742_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_0)));
    vlSelfRef.rspu_top__DOT__core_top_call_744_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_744_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_1)));
    vlSelfRef.rspu_top__DOT__core_top_call_746_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_746_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_2)));
    vlSelfRef.rspu_top__DOT__core_top_call_748_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_748_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_3)));
    vlSelfRef.rspu_top__DOT__core_top_call_750_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_750_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_4)));
    vlSelfRef.rspu_top__DOT__core_top_call_752_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_752_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_5)));
    vlSelfRef.rspu_top__DOT__core_top_call_754_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_754_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_6)));
    vlSelfRef.rspu_top__DOT__core_top_call_756_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_756_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_7)));
    vlSelfRef.rspu_top__DOT__core_top_call_758_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_758_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_8)));
    vlSelfRef.rspu_top__DOT__core_top_call_760_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_760_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_9)));
    vlSelfRef.rspu_top__DOT__core_top_call_762_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_762_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_10)));
    vlSelfRef.rspu_top__DOT__core_top_call_764_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_764_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_11)));
    vlSelfRef.rspu_top__DOT__core_top_call_766_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_766_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_12)));
    vlSelfRef.rspu_top__DOT__core_top_call_768_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_768_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_13)));
    vlSelfRef.rspu_top__DOT__core_top_call_770_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_770_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_14)));
    vlSelfRef.rspu_top__DOT__core_top_call_772_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_772_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_15)));
    vlSelfRef.rspu_top__DOT__core_top_call_774_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_774_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_16)));
    vlSelfRef.rspu_top__DOT__core_top_call_776_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_776_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_17)));
    vlSelfRef.rspu_top__DOT__core_top_call_778_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_778_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_18)));
    vlSelfRef.rspu_top__DOT__core_top_call_780_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_780_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_19)));
    vlSelfRef.rspu_top__DOT__core_top_call_782_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_782_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_20)));
    vlSelfRef.rspu_top__DOT__core_top_call_784_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_784_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_21)));
    vlSelfRef.rspu_top__DOT__core_top_call_786_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_786_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_22)));
    vlSelfRef.rspu_top__DOT__core_top_call_788_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_788_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_23)));
    vlSelfRef.rspu_top__DOT__core_top_call_790_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_790_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_24)));
    vlSelfRef.rspu_top__DOT__core_top_call_792_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_792_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_25)));
    vlSelfRef.rspu_top__DOT__core_top_call_794_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_794_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_26)));
    vlSelfRef.rspu_top__DOT__core_top_call_796_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_796_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_27)));
    vlSelfRef.rspu_top__DOT__core_top_call_798_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_798_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_28)));
    vlSelfRef.rspu_top__DOT__core_top_call_800_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_800_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_29)));
    vlSelfRef.rspu_top__DOT__core_top_call_802_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_802_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_30)));
    vlSelfRef.rspu_top__DOT__core_top_call_804_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_804_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_31)));
    vlSelfRef.rspu_top__DOT__core_top_call_806_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_806_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_32)));
    vlSelfRef.rspu_top__DOT__core_top_call_808_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_808_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_33)));
    vlSelfRef.rspu_top__DOT__core_top_call_810_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_810_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_34)));
    vlSelfRef.rspu_top__DOT__core_top_call_812_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_812_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_35)));
    vlSelfRef.rspu_top__DOT__core_top_call_814_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_814_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_36)));
    vlSelfRef.rspu_top__DOT__core_top_call_816_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_816_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_37)));
    vlSelfRef.rspu_top__DOT__core_top_call_818_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_818_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_38)));
    vlSelfRef.rspu_top__DOT__core_top_call_820_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_820_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_39)));
    vlSelfRef.rspu_top__DOT__core_top_call_822_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_822_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_40)));
    vlSelfRef.rspu_top__DOT__core_top_call_824_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_824_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_41)));
    vlSelfRef.rspu_top__DOT__core_top_call_826_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_826_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_42)));
    vlSelfRef.rspu_top__DOT__core_top_call_828_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_828_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_43)));
    vlSelfRef.rspu_top__DOT__core_top_call_830_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_830_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_44)));
    vlSelfRef.rspu_top__DOT__core_top_call_832_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_832_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_45)));
    vlSelfRef.rspu_top__DOT__core_top_call_834_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_834_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_46)));
    vlSelfRef.rspu_top__DOT__core_top_call_836_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_836_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_47)));
    vlSelfRef.rspu_top__DOT__core_top_call_838_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_838_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_48)));
    vlSelfRef.rspu_top__DOT__core_top_call_840_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_840_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_49)));
    vlSelfRef.rspu_top__DOT__core_top_call_842_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_842_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_50)));
    vlSelfRef.rspu_top__DOT__core_top_call_844_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_844_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_51)));
    vlSelfRef.rspu_top__DOT__core_top_call_846_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_846_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_52)));
    vlSelfRef.rspu_top__DOT__core_top_call_848_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_848_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_53)));
    vlSelfRef.rspu_top__DOT__core_top_call_850_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_850_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_54)));
    vlSelfRef.rspu_top__DOT__core_top_call_852_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_852_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_55)));
    vlSelfRef.rspu_top__DOT__core_top_call_854_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_854_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_56)));
    vlSelfRef.rspu_top__DOT__core_top_call_856_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_856_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_57)));
    vlSelfRef.rspu_top__DOT__core_top_call_858_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_858_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_58)));
    vlSelfRef.rspu_top__DOT__core_top_call_860_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_860_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_59)));
    vlSelfRef.rspu_top__DOT__core_top_call_862_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_862_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_60)));
    vlSelfRef.rspu_top__DOT__core_top_call_864_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_864_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_61)));
    vlSelfRef.rspu_top__DOT__core_top_call_866_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_866_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_62)));
    vlSelfRef.rspu_top__DOT__core_top_call_868_core_awake 
        = ((IData)(vlSelfRef.rst_n) && ((0U < (IData)(vlSelfRef.rspu_top__DOT__core_top_call_868_wake_timer)) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rx_valid_63)));
    vlSelfRef.rspu_top__DOT__trap_0 = ((IData)(vlSelfRef.rst_n) 
                                       && ((((((((((((((((IData)(vlSelfRef.rspu_top__DOT__tx_valid_0) 
                                                         | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_1)) 
                                                        | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_2)) 
                                                       | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_3)) 
                                                      | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_4)) 
                                                     | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_5)) 
                                                    | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_6)) 
                                                   | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_7)) 
                                                  | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_8)) 
                                                 | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_9)) 
                                                | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_10)) 
                                               | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_11)) 
                                              | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_12)) 
                                             | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_13)) 
                                            | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_14)) 
                                           | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_15)));
    vlSelfRef.rspu_top__DOT__trap_1 = ((IData)(vlSelfRef.rst_n) 
                                       && ((((((((((((((((IData)(vlSelfRef.rspu_top__DOT__tx_valid_16) 
                                                         | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_17)) 
                                                        | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_18)) 
                                                       | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_19)) 
                                                      | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_20)) 
                                                     | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_21)) 
                                                    | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_22)) 
                                                   | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_23)) 
                                                  | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_24)) 
                                                 | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_25)) 
                                                | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_26)) 
                                               | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_27)) 
                                              | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_28)) 
                                             | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_29)) 
                                            | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_30)) 
                                           | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_31)));
    vlSelfRef.rspu_top__DOT__trap_2 = ((IData)(vlSelfRef.rst_n) 
                                       && ((((((((((((((((IData)(vlSelfRef.rspu_top__DOT__tx_valid_32) 
                                                         | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_33)) 
                                                        | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_34)) 
                                                       | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_35)) 
                                                      | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_36)) 
                                                     | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_37)) 
                                                    | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_38)) 
                                                   | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_39)) 
                                                  | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_40)) 
                                                 | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_41)) 
                                                | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_42)) 
                                               | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_43)) 
                                              | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_44)) 
                                             | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_45)) 
                                            | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_46)) 
                                           | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_47)));
    vlSelfRef.rspu_top__DOT__trap_3 = ((IData)(vlSelfRef.rst_n) 
                                       && ((((((((((((((((IData)(vlSelfRef.rspu_top__DOT__tx_valid_48) 
                                                         | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_49)) 
                                                        | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_50)) 
                                                       | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_51)) 
                                                      | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_52)) 
                                                     | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_53)) 
                                                    | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_54)) 
                                                   | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_55)) 
                                                  | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_56)) 
                                                 | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_57)) 
                                                | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_58)) 
                                               | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_59)) 
                                              | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_60)) 
                                             | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_61)) 
                                            | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_62)) 
                                           | (IData)(vlSelfRef.rspu_top__DOT__tx_valid_63)));
    vlSelfRef.rspu_top__DOT__core_top_call_742_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_742_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_744_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_744_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_746_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_746_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_748_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_748_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_750_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_750_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_752_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_752_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_754_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_754_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_756_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_756_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_758_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_758_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_760_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_760_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_762_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_762_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_764_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_764_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_766_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_766_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_768_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_768_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_770_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_770_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_772_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_772_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_774_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_774_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_776_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_776_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_778_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_778_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_780_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_780_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_782_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_782_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_784_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_784_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_786_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_786_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_788_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_788_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_790_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_790_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_792_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_792_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_794_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_794_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_796_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_796_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_798_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_798_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_800_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_800_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_802_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_802_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_804_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_804_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_806_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_806_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_808_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_808_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_810_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_810_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_812_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_812_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_814_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_814_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_816_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_816_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_818_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_818_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_820_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_820_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_822_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_822_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_824_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_824_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_826_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_826_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_828_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_828_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_830_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_830_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_832_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_832_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_834_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_834_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_836_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_836_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_838_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_838_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_840_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_840_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_842_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_842_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_844_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_844_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_846_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_846_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_848_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_848_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_850_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_850_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_852_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_852_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_854_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_854_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_856_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_856_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_858_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_858_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_860_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_860_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_862_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_862_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_864_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_864_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_866_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_866_wake_timer;
    vlSelfRef.rspu_top__DOT__core_top_call_868_wake_timer 
        = __Vdly__rspu_top__DOT__core_top_call_868_wake_timer;
    vlSelfRef.rspu_top__DOT__rx_valid_0 = ((IData)(vlSelfRef.rst_n) 
                                           && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                & (0ULL 
                                                   == 
                                                   (0x0000000000000fffULL 
                                                    & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                               & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_0))));
    vlSelfRef.rspu_top__DOT__rx_valid_1 = ((IData)(vlSelfRef.rst_n) 
                                           && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                & (1ULL 
                                                   == 
                                                   (0x0000000000000fffULL 
                                                    & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                               & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_1))));
    vlSelfRef.rspu_top__DOT__rx_valid_2 = ((IData)(vlSelfRef.rst_n) 
                                           && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                & (2ULL 
                                                   == 
                                                   (0x0000000000000fffULL 
                                                    & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                               & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_2))));
    vlSelfRef.rspu_top__DOT__rx_valid_3 = ((IData)(vlSelfRef.rst_n) 
                                           && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                & (3ULL 
                                                   == 
                                                   (0x0000000000000fffULL 
                                                    & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                               & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_3))));
    vlSelfRef.rspu_top__DOT__rx_valid_4 = ((IData)(vlSelfRef.rst_n) 
                                           && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                & (4ULL 
                                                   == 
                                                   (0x0000000000000fffULL 
                                                    & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                               & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_4))));
    vlSelfRef.rspu_top__DOT__rx_valid_5 = ((IData)(vlSelfRef.rst_n) 
                                           && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                & (5ULL 
                                                   == 
                                                   (0x0000000000000fffULL 
                                                    & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                               & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_5))));
    vlSelfRef.rspu_top__DOT__rx_valid_6 = ((IData)(vlSelfRef.rst_n) 
                                           && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                & (6ULL 
                                                   == 
                                                   (0x0000000000000fffULL 
                                                    & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                               & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_6))));
    vlSelfRef.rspu_top__DOT__rx_valid_7 = ((IData)(vlSelfRef.rst_n) 
                                           && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                & (7ULL 
                                                   == 
                                                   (0x0000000000000fffULL 
                                                    & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                               & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_7))));
    vlSelfRef.rspu_top__DOT__rx_valid_8 = ((IData)(vlSelfRef.rst_n) 
                                           && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                & (8ULL 
                                                   == 
                                                   (0x0000000000000fffULL 
                                                    & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                               & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_8))));
    vlSelfRef.rspu_top__DOT__rx_valid_9 = ((IData)(vlSelfRef.rst_n) 
                                           && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                & (9ULL 
                                                   == 
                                                   (0x0000000000000fffULL 
                                                    & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                               & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_9))));
    vlSelfRef.rspu_top__DOT__rx_valid_10 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                 & (0x000000000000000aULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_10))));
    vlSelfRef.rspu_top__DOT__rx_valid_11 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                 & (0x000000000000000bULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_11))));
    vlSelfRef.rspu_top__DOT__rx_valid_12 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                 & (0x000000000000000cULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_12))));
    vlSelfRef.rspu_top__DOT__rx_valid_13 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                 & (0x000000000000000dULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_13))));
    vlSelfRef.rspu_top__DOT__rx_valid_14 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                 & (0x000000000000000eULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_14))));
    vlSelfRef.rspu_top__DOT__rx_valid_15 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16) 
                                                 & (0x000000000000000fULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_15))));
    vlSelfRef.rspu_top__DOT__rx_valid_16 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x0000000000000010ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_0))));
    vlSelfRef.rspu_top__DOT__rx_valid_17 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x0000000000000011ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_1))));
    vlSelfRef.rspu_top__DOT__rx_valid_18 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x0000000000000012ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_2))));
    vlSelfRef.rspu_top__DOT__rx_valid_19 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x0000000000000013ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_3))));
    vlSelfRef.rspu_top__DOT__rx_valid_20 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x0000000000000014ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_4))));
    vlSelfRef.rspu_top__DOT__rx_valid_21 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x0000000000000015ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_5))));
    vlSelfRef.rspu_top__DOT__rx_valid_22 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x0000000000000016ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_6))));
    vlSelfRef.rspu_top__DOT__rx_valid_23 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x0000000000000017ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_7))));
    vlSelfRef.rspu_top__DOT__rx_valid_24 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x0000000000000018ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_8))));
    vlSelfRef.rspu_top__DOT__rx_valid_25 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x0000000000000019ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_9))));
    vlSelfRef.rspu_top__DOT__rx_valid_26 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x000000000000001aULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_10))));
    vlSelfRef.rspu_top__DOT__rx_valid_27 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x000000000000001bULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_11))));
    vlSelfRef.rspu_top__DOT__rx_valid_28 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x000000000000001cULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_12))));
    vlSelfRef.rspu_top__DOT__rx_valid_29 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x000000000000001dULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_13))));
    vlSelfRef.rspu_top__DOT__rx_valid_30 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x000000000000001eULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_14))));
    vlSelfRef.rspu_top__DOT__rx_valid_31 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16) 
                                                 & (0x000000000000001fULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_15))));
    vlSelfRef.rspu_top__DOT__rx_valid_32 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x0000000000000020ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_0))));
    vlSelfRef.rspu_top__DOT__rx_valid_33 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x0000000000000021ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_1))));
    vlSelfRef.rspu_top__DOT__rx_valid_34 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x0000000000000022ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_2))));
    vlSelfRef.rspu_top__DOT__rx_valid_35 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x0000000000000023ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_3))));
    vlSelfRef.rspu_top__DOT__rx_valid_36 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x0000000000000024ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_4))));
    vlSelfRef.rspu_top__DOT__rx_valid_37 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x0000000000000025ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_5))));
    vlSelfRef.rspu_top__DOT__rx_valid_38 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x0000000000000026ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_6))));
    vlSelfRef.rspu_top__DOT__rx_valid_39 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x0000000000000027ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_7))));
    vlSelfRef.rspu_top__DOT__rx_valid_40 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x0000000000000028ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_8))));
    vlSelfRef.rspu_top__DOT__rx_valid_41 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x0000000000000029ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_9))));
    vlSelfRef.rspu_top__DOT__rx_valid_42 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x000000000000002aULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_10))));
    vlSelfRef.rspu_top__DOT__rx_valid_43 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x000000000000002bULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_11))));
    vlSelfRef.rspu_top__DOT__rx_valid_44 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x000000000000002cULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_12))));
    vlSelfRef.rspu_top__DOT__rx_valid_45 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x000000000000002dULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_13))));
    vlSelfRef.rspu_top__DOT__rx_valid_46 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x000000000000002eULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_14))));
    vlSelfRef.rspu_top__DOT__rx_valid_47 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16) 
                                                 & (0x000000000000002fULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_15))));
    vlSelfRef.rspu_top__DOT__rx_valid_48 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x0000000000000030ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_0))));
    vlSelfRef.rspu_top__DOT__rx_valid_49 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x0000000000000031ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_1))));
    vlSelfRef.rspu_top__DOT__rx_valid_50 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x0000000000000032ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_2))));
    vlSelfRef.rspu_top__DOT__rx_valid_51 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x0000000000000033ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_3))));
    vlSelfRef.rspu_top__DOT__rx_valid_52 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x0000000000000034ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_4))));
    vlSelfRef.rspu_top__DOT__rx_valid_53 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x0000000000000035ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_5))));
    vlSelfRef.rspu_top__DOT__rx_valid_54 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x0000000000000036ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_6))));
    vlSelfRef.rspu_top__DOT__rx_valid_55 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x0000000000000037ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_7))));
    vlSelfRef.rspu_top__DOT__rx_valid_56 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x0000000000000038ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_8))));
    vlSelfRef.rspu_top__DOT__rx_valid_57 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x0000000000000039ULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_9))));
    vlSelfRef.rspu_top__DOT__rx_valid_58 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x000000000000003aULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_10))));
    vlSelfRef.rspu_top__DOT__rx_valid_59 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x000000000000003bULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_11))));
    vlSelfRef.rspu_top__DOT__rx_valid_60 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x000000000000003cULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_12))));
    vlSelfRef.rspu_top__DOT__rx_valid_61 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x000000000000003dULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_13))));
    vlSelfRef.rspu_top__DOT__rx_valid_62 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x000000000000003eULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_14))));
    vlSelfRef.rspu_top__DOT__rx_valid_63 = ((IData)(vlSelfRef.rst_n) 
                                            && (((IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16) 
                                                 & (0x000000000000003fULL 
                                                    == 
                                                    (0x0000000000000fffULL 
                                                     & VL_SHIFTR_QQI(64,64,32, vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16, 0x00000030U)))) 
                                                & (~ (IData)(vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_15))));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_0 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_0));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_1 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_1));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_2 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_2));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_3 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_3));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_4 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_4));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_5 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_5));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_6 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_6));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_7 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_7));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_8 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_8));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_9 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_9));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_10 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_10));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_11 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_11));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_12 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_12));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_13 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_13));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_14 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_14));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_core_dead_15 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_15));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_0 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_0));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_1 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_1));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_2 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_2));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_3 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_3));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_4 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_4));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_5 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_5));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_6 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_6));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_7 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_7));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_8 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_8));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_9 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_9));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_10 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_10));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_11 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_11));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_12 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_12));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_13 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_13));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_14 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_14));
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_core_dead_15 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_15));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_0 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_0));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_1 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_1));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_2 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_2));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_3 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_3));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_4 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_4));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_5 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_5));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_6 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_6));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_7 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_7));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_8 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_8));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_9 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_9));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_10 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_10));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_11 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_11));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_12 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_12));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_13 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_13));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_14 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_14));
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_core_dead_15 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_15));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_0 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_0));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_1 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_1));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_2 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_2));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_3 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_3));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_4 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_4));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_5 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_5));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_6 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_6));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_7 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_7));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_8 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_8));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_9 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_9));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_10 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_10));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_11 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_11));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_12 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_12));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_13 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_13));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_14 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_14));
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_core_dead_15 
        = ((IData)(vlSelfRef.rst_n) && (0x00000400U 
                                        < vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_15));
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_0 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_0;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_1 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_1;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_2 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_2;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_3 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_3;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_4 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_4;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_5 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_5;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_6 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_6;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_7 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_7;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_8 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_8;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_9 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_9;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_10 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_10;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_11 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_11;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_12 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_12;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_13 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_13;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_14 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_14;
    vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_15 
        = __Vdly__rspu_top__DOT__noc_l1_router_0_call_886_heartbeat_15;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_0 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_0;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_1 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_1;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_2 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_2;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_3 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_3;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_4 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_4;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_5 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_5;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_6 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_6;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_7 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_7;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_8 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_8;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_9 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_9;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_10 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_10;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_11 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_11;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_12 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_12;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_13 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_13;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_14 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_14;
    vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_15 
        = __Vdly__rspu_top__DOT__noc_l1_router_1_call_888_heartbeat_15;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_0 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_0;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_1 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_1;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_2 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_2;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_3 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_3;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_4 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_4;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_5 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_5;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_6 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_6;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_7 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_7;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_8 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_8;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_9 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_9;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_10 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_10;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_11 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_11;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_12 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_12;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_13 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_13;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_14 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_14;
    vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_15 
        = __Vdly__rspu_top__DOT__noc_l1_router_2_call_890_heartbeat_15;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_0 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_0;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_1 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_1;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_2 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_2;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_3 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_3;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_4 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_4;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_5 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_5;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_6 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_6;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_7 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_7;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_8 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_8;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_9 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_9;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_10 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_10;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_11 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_11;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_12 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_12;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_13 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_13;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_14 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_14;
    vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_15 
        = __Vdly__rspu_top__DOT__noc_l1_router_3_call_892_heartbeat_15;
    if (vlSelfRef.rst_n) {
        vlSelfRef.out_data_0 = vlSelfRef.rspu_top__DOT__core_data_0;
        vlSelfRef.out_data_1 = vlSelfRef.rspu_top__DOT__core_data_1;
        vlSelfRef.out_data_10 = vlSelfRef.rspu_top__DOT__core_data_10;
        vlSelfRef.out_data_11 = vlSelfRef.rspu_top__DOT__core_data_11;
        vlSelfRef.out_data_12 = vlSelfRef.rspu_top__DOT__core_data_12;
        vlSelfRef.out_data_13 = vlSelfRef.rspu_top__DOT__core_data_13;
        vlSelfRef.out_data_14 = vlSelfRef.rspu_top__DOT__core_data_14;
        vlSelfRef.out_data_15 = vlSelfRef.rspu_top__DOT__core_data_15;
        vlSelfRef.out_data_16 = vlSelfRef.rspu_top__DOT__core_data_16;
        vlSelfRef.out_data_17 = vlSelfRef.rspu_top__DOT__core_data_17;
        vlSelfRef.out_data_18 = vlSelfRef.rspu_top__DOT__core_data_18;
        vlSelfRef.out_data_19 = vlSelfRef.rspu_top__DOT__core_data_19;
        vlSelfRef.out_data_2 = vlSelfRef.rspu_top__DOT__core_data_2;
        vlSelfRef.out_data_20 = vlSelfRef.rspu_top__DOT__core_data_20;
        vlSelfRef.out_data_21 = vlSelfRef.rspu_top__DOT__core_data_21;
        vlSelfRef.out_data_22 = vlSelfRef.rspu_top__DOT__core_data_22;
        vlSelfRef.out_data_23 = vlSelfRef.rspu_top__DOT__core_data_23;
        vlSelfRef.out_data_24 = vlSelfRef.rspu_top__DOT__core_data_24;
        vlSelfRef.out_data_25 = vlSelfRef.rspu_top__DOT__core_data_25;
        vlSelfRef.out_data_26 = vlSelfRef.rspu_top__DOT__core_data_26;
        vlSelfRef.out_data_27 = vlSelfRef.rspu_top__DOT__core_data_27;
        vlSelfRef.out_data_28 = vlSelfRef.rspu_top__DOT__core_data_28;
        vlSelfRef.out_data_29 = vlSelfRef.rspu_top__DOT__core_data_29;
        vlSelfRef.out_data_3 = vlSelfRef.rspu_top__DOT__core_data_3;
        vlSelfRef.out_data_30 = vlSelfRef.rspu_top__DOT__core_data_30;
        vlSelfRef.out_data_31 = vlSelfRef.rspu_top__DOT__core_data_31;
        vlSelfRef.out_data_32 = vlSelfRef.rspu_top__DOT__core_data_32;
        vlSelfRef.out_data_33 = vlSelfRef.rspu_top__DOT__core_data_33;
        vlSelfRef.out_data_34 = vlSelfRef.rspu_top__DOT__core_data_34;
        vlSelfRef.out_data_35 = vlSelfRef.rspu_top__DOT__core_data_35;
        vlSelfRef.out_data_36 = vlSelfRef.rspu_top__DOT__core_data_36;
        vlSelfRef.out_data_37 = vlSelfRef.rspu_top__DOT__core_data_37;
        vlSelfRef.out_data_38 = vlSelfRef.rspu_top__DOT__core_data_38;
        vlSelfRef.out_data_39 = vlSelfRef.rspu_top__DOT__core_data_39;
        vlSelfRef.out_data_4 = vlSelfRef.rspu_top__DOT__core_data_4;
        vlSelfRef.out_data_40 = vlSelfRef.rspu_top__DOT__core_data_40;
        vlSelfRef.out_data_41 = vlSelfRef.rspu_top__DOT__core_data_41;
        vlSelfRef.out_data_42 = vlSelfRef.rspu_top__DOT__core_data_42;
        vlSelfRef.out_data_43 = vlSelfRef.rspu_top__DOT__core_data_43;
        vlSelfRef.out_data_44 = vlSelfRef.rspu_top__DOT__core_data_44;
        vlSelfRef.out_data_45 = vlSelfRef.rspu_top__DOT__core_data_45;
        vlSelfRef.out_data_46 = vlSelfRef.rspu_top__DOT__core_data_46;
        vlSelfRef.out_data_47 = vlSelfRef.rspu_top__DOT__core_data_47;
        vlSelfRef.out_data_48 = vlSelfRef.rspu_top__DOT__core_data_48;
        vlSelfRef.out_data_49 = vlSelfRef.rspu_top__DOT__core_data_49;
        vlSelfRef.out_data_5 = vlSelfRef.rspu_top__DOT__core_data_5;
        vlSelfRef.out_data_50 = vlSelfRef.rspu_top__DOT__core_data_50;
        vlSelfRef.out_data_51 = vlSelfRef.rspu_top__DOT__core_data_51;
        vlSelfRef.out_data_52 = vlSelfRef.rspu_top__DOT__core_data_52;
        vlSelfRef.out_data_53 = vlSelfRef.rspu_top__DOT__core_data_53;
        vlSelfRef.out_data_54 = vlSelfRef.rspu_top__DOT__core_data_54;
        vlSelfRef.out_data_55 = vlSelfRef.rspu_top__DOT__core_data_55;
        vlSelfRef.out_data_56 = vlSelfRef.rspu_top__DOT__core_data_56;
        vlSelfRef.out_data_57 = vlSelfRef.rspu_top__DOT__core_data_57;
        vlSelfRef.out_data_58 = vlSelfRef.rspu_top__DOT__core_data_58;
        vlSelfRef.out_data_59 = vlSelfRef.rspu_top__DOT__core_data_59;
        vlSelfRef.out_data_6 = vlSelfRef.rspu_top__DOT__core_data_6;
        vlSelfRef.out_data_60 = vlSelfRef.rspu_top__DOT__core_data_60;
        vlSelfRef.out_data_61 = vlSelfRef.rspu_top__DOT__core_data_61;
        vlSelfRef.out_data_62 = vlSelfRef.rspu_top__DOT__core_data_62;
        vlSelfRef.out_data_63 = vlSelfRef.rspu_top__DOT__core_data_63;
        vlSelfRef.out_data_7 = vlSelfRef.rspu_top__DOT__core_data_7;
        vlSelfRef.out_data_8 = vlSelfRef.rspu_top__DOT__core_data_8;
        vlSelfRef.out_data_9 = vlSelfRef.rspu_top__DOT__core_data_9;
        vlSelfRef.uplink_data_0 = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16;
        vlSelfRef.uplink_data_1 = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16;
        vlSelfRef.uplink_data_2 = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16;
        vlSelfRef.uplink_data_3 = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16;
        vlSelfRef.rspu_top__DOT__rx_data_0 = (0x0000ffffffffffffULL 
                                              & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_1 = (0x0000ffffffffffffULL 
                                              & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_2 = (0x0000ffffffffffffULL 
                                              & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_3 = (0x0000ffffffffffffULL 
                                              & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_4 = (0x0000ffffffffffffULL 
                                              & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_5 = (0x0000ffffffffffffULL 
                                              & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_6 = (0x0000ffffffffffffULL 
                                              & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_7 = (0x0000ffffffffffffULL 
                                              & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_8 = (0x0000ffffffffffffULL 
                                              & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_9 = (0x0000ffffffffffffULL 
                                              & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_10 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_11 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_12 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_13 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_14 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_15 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_16 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_17 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_18 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_19 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_20 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_21 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_22 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_23 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_24 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_25 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_26 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_27 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_28 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_29 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_30 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_31 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_32 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_33 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_34 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_35 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_36 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_37 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_38 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_39 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_40 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_41 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_42 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_43 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_44 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_45 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_46 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_47 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_48 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_49 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_50 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_51 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_52 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_53 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_54 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_55 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_56 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_57 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_58 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_59 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_60 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_61 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_62 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__rx_data_63 = (0x0000ffffffffffffULL 
                                               & vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16);
        vlSelfRef.rspu_top__DOT__core_data_0 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_38193_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_1 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37658_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_10 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32843_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_11 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_32308_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_12 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31773_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_13 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_14 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_15 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_16 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29633_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_17 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_29098_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_18 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28563_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_19 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_28028_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_2 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_37123_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_20 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_27493_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_21 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26958_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_22 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_26423_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_23 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25888_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_24 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_25353_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_25 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24818_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_26 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_24283_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_27 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23748_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_28 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_23213_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_29 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_3 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36588_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_30 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_31 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_32 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21073_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_33 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20538_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_34 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_20003_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_35 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_19468_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_36 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18933_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_37 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_18398_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_38 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17863_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_39 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_17328_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_4 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_36053_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_40 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16793_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_41 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_16258_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_42 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15723_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_43 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_15188_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_44 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14653_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_45 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_46 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_47 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_48 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_12513_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_49 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11978_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_5 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_35518_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_50 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_11443_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_51 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10908_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_52 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_10373_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_53 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9838_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_54 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_9303_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_55 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8768_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_56 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_8233_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_57 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7698_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_58 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_7163_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_59 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6628_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_6 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34983_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_60 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_6093_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_61 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5558_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_62 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_63 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_7 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_34448_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_8 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33913_mem_wb_res;
        vlSelfRef.rspu_top__DOT__core_data_9 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_33378_mem_wb_res;
        if (vlSelfRef.rspu_top__DOT__tx_valid_15) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16 
                = vlSelfRef.rspu_top__DOT__tx_data_15;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_15)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_15;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_15;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_31) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16 
                = vlSelfRef.rspu_top__DOT__tx_data_31;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_31)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_15;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_15;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_47) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16 
                = vlSelfRef.rspu_top__DOT__tx_data_47;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_47)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_15;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_15;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_63) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16 
                = vlSelfRef.rspu_top__DOT__tx_data_63;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_63)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_15;
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_15;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_14) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_15 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_14)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_15 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_14;
        }
        vlSelfRef.rspu_top__DOT__tx_valid_15 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_15 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_772_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_15 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_15 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_15)));
        }
    } else {
        vlSelfRef.out_data_0 = 0ULL;
        vlSelfRef.out_data_1 = 0ULL;
        vlSelfRef.out_data_10 = 0ULL;
        vlSelfRef.out_data_11 = 0ULL;
        vlSelfRef.out_data_12 = 0ULL;
        vlSelfRef.out_data_13 = 0ULL;
        vlSelfRef.out_data_14 = 0ULL;
        vlSelfRef.out_data_15 = 0ULL;
        vlSelfRef.out_data_16 = 0ULL;
        vlSelfRef.out_data_17 = 0ULL;
        vlSelfRef.out_data_18 = 0ULL;
        vlSelfRef.out_data_19 = 0ULL;
        vlSelfRef.out_data_2 = 0ULL;
        vlSelfRef.out_data_20 = 0ULL;
        vlSelfRef.out_data_21 = 0ULL;
        vlSelfRef.out_data_22 = 0ULL;
        vlSelfRef.out_data_23 = 0ULL;
        vlSelfRef.out_data_24 = 0ULL;
        vlSelfRef.out_data_25 = 0ULL;
        vlSelfRef.out_data_26 = 0ULL;
        vlSelfRef.out_data_27 = 0ULL;
        vlSelfRef.out_data_28 = 0ULL;
        vlSelfRef.out_data_29 = 0ULL;
        vlSelfRef.out_data_3 = 0ULL;
        vlSelfRef.out_data_30 = 0ULL;
        vlSelfRef.out_data_31 = 0ULL;
        vlSelfRef.out_data_32 = 0ULL;
        vlSelfRef.out_data_33 = 0ULL;
        vlSelfRef.out_data_34 = 0ULL;
        vlSelfRef.out_data_35 = 0ULL;
        vlSelfRef.out_data_36 = 0ULL;
        vlSelfRef.out_data_37 = 0ULL;
        vlSelfRef.out_data_38 = 0ULL;
        vlSelfRef.out_data_39 = 0ULL;
        vlSelfRef.out_data_4 = 0ULL;
        vlSelfRef.out_data_40 = 0ULL;
        vlSelfRef.out_data_41 = 0ULL;
        vlSelfRef.out_data_42 = 0ULL;
        vlSelfRef.out_data_43 = 0ULL;
        vlSelfRef.out_data_44 = 0ULL;
        vlSelfRef.out_data_45 = 0ULL;
        vlSelfRef.out_data_46 = 0ULL;
        vlSelfRef.out_data_47 = 0ULL;
        vlSelfRef.out_data_48 = 0ULL;
        vlSelfRef.out_data_49 = 0ULL;
        vlSelfRef.out_data_5 = 0ULL;
        vlSelfRef.out_data_50 = 0ULL;
        vlSelfRef.out_data_51 = 0ULL;
        vlSelfRef.out_data_52 = 0ULL;
        vlSelfRef.out_data_53 = 0ULL;
        vlSelfRef.out_data_54 = 0ULL;
        vlSelfRef.out_data_55 = 0ULL;
        vlSelfRef.out_data_56 = 0ULL;
        vlSelfRef.out_data_57 = 0ULL;
        vlSelfRef.out_data_58 = 0ULL;
        vlSelfRef.out_data_59 = 0ULL;
        vlSelfRef.out_data_6 = 0ULL;
        vlSelfRef.out_data_60 = 0ULL;
        vlSelfRef.out_data_61 = 0ULL;
        vlSelfRef.out_data_62 = 0ULL;
        vlSelfRef.out_data_63 = 0ULL;
        vlSelfRef.out_data_7 = 0ULL;
        vlSelfRef.out_data_8 = 0ULL;
        vlSelfRef.out_data_9 = 0ULL;
        vlSelfRef.uplink_data_0 = 0ULL;
        vlSelfRef.uplink_data_1 = 0ULL;
        vlSelfRef.uplink_data_2 = 0ULL;
        vlSelfRef.uplink_data_3 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_0 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_1 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_2 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_3 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_4 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_5 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_6 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_7 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_8 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_9 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_10 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_11 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_12 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_13 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_14 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_15 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_16 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_17 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_18 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_19 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_20 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_21 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_22 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_23 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_24 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_25 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_26 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_27 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_28 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_29 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_30 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_31 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_32 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_33 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_34 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_35 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_36 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_37 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_38 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_39 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_40 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_41 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_42 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_43 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_44 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_45 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_46 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_47 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_48 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_49 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_50 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_51 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_52 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_53 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_54 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_55 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_56 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_57 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_58 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_59 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_60 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_61 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_62 = 0ULL;
        vlSelfRef.rspu_top__DOT__rx_data_63 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_0 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_1 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_10 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_11 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_12 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_13 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_14 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_15 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_16 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_17 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_18 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_19 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_2 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_20 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_21 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_22 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_23 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_24 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_25 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_26 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_27 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_28 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_29 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_3 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_30 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_31 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_32 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_33 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_34 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_35 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_36 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_37 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_38 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_39 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_4 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_40 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_41 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_42 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_43 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_44 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_45 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_46 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_47 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_48 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_49 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_5 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_50 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_51 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_52 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_53 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_54 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_55 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_56 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_57 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_58 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_59 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_6 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_60 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_61 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_62 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_63 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_7 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_8 = 0ULL;
        vlSelfRef.rspu_top__DOT__core_data_9 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_16 = 0U;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_16 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_16 = 0U;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_16 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_16 = 0U;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_16 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_16 = 0U;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_16 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_15 = 0U;
        vlSelfRef.rspu_top__DOT__tx_valid_15 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_15 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_772_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_14) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_15 
                = vlSelfRef.rspu_top__DOT__tx_data_14;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_14)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_15 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_14;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_30) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_15 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_30)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_15 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_14;
        }
        vlSelfRef.rspu_top__DOT__tx_valid_31 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_31 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_804_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_31 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_31 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_31)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_15 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_15 = 0U;
        vlSelfRef.rspu_top__DOT__tx_valid_31 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_31 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_804_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_30) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_15 
                = vlSelfRef.rspu_top__DOT__tx_data_30;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_30)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_15 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_14;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_46) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_15 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_46)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_15 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_14;
        }
        vlSelfRef.rspu_top__DOT__tx_valid_47 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_47 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_836_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_47 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_47 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_47)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_15 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_15 = 0U;
        vlSelfRef.rspu_top__DOT__tx_valid_47 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_47 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_836_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_46) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_15 
                = vlSelfRef.rspu_top__DOT__tx_data_46;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_46)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_15 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_14;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_62) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_15 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_62)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_15 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_14;
        }
        vlSelfRef.rspu_top__DOT__tx_valid_63 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_63 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_868_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_63 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_63 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_63)));
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_15 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_15 = 0U;
        vlSelfRef.rspu_top__DOT__tx_valid_63 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_63 = 0ULL;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_868_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_trap)));
    if (vlSelfRef.rst_n) {
        if (vlSelfRef.rspu_top__DOT__tx_valid_62) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_15 
                = vlSelfRef.rspu_top__DOT__tx_data_62;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_62)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_15 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_14;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_13) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_14 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_14 
                = vlSelfRef.rspu_top__DOT__tx_data_13;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_13)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_14 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_13;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_14 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_13;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_15 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_pipe_pc 
            = __Vdly__rspu_top__DOT__rspu_pipeline_call_30168_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_14 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_14 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_770_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_14 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_14 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_14)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_29) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_14 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_14 
                = vlSelfRef.rspu_top__DOT__tx_data_29;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_29)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_14 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_13;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_14 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_13;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_31 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_pipe_pc 
            = __Vdly__rspu_top__DOT__rspu_pipeline_call_21608_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_30 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_30 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_802_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_30 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_30 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_30)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_45) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_14 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_14 
                = vlSelfRef.rspu_top__DOT__tx_data_45;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_45)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_14 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_13;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_14 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_13;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_47 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_pipe_pc 
            = __Vdly__rspu_top__DOT__rspu_pipeline_call_13048_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_46 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_46 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_834_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_46 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_46 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_46)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_61) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_14 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_61)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_14 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_13;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_63 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_pipe_pc;
        }
    } else {
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_15 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_14 = 0U;
        vlSelfRef.rspu_top__DOT__pc_15 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_pipe_pc 
            = __Vdly__rspu_top__DOT__rspu_pipeline_call_30168_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_14 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_14 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_14 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_14 = 0U;
        vlSelfRef.rspu_top__DOT__pc_31 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_pipe_pc 
            = __Vdly__rspu_top__DOT__rspu_pipeline_call_21608_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_30 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_30 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_14 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_14 = 0U;
        vlSelfRef.rspu_top__DOT__pc_47 = 0U;
        vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_pipe_pc 
            = __Vdly__rspu_top__DOT__rspu_pipeline_call_13048_pipe_pc;
        vlSelfRef.rspu_top__DOT__tx_valid_46 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_46 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_14 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_14 = 0U;
        vlSelfRef.rspu_top__DOT__pc_63 = 0U;
    }
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_pipe_pc 
        = __Vdly__rspu_top__DOT__rspu_pipeline_call_4488_pipe_pc;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_30372_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30168_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_770_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_trap)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_21812_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_21608_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_802_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_trap)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_13252_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13048_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_trap));
    if (vlSelfRef.rst_n) {
        vlSelfRef.rspu_top__DOT__tx_valid_62 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_62 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_866_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_62 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_62 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_62)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_61) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_14 
                = vlSelfRef.rspu_top__DOT__tx_data_61;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_61)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_14 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_13;
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_12) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_13 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_13 
                = vlSelfRef.rspu_top__DOT__tx_data_12;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_12)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_13 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_12;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_13 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_12;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_14 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__tx_valid_13 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_13 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_768_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_13 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_13 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_13)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_28) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_13 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_13 
                = vlSelfRef.rspu_top__DOT__tx_data_28;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_28)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_13 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_12;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_13 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_12;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_30 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__tx_valid_29 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_29 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_800_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_29 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_29 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_29)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_44) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_13 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_13 
                = vlSelfRef.rspu_top__DOT__tx_data_44;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_44)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_13 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_12;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_13 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_12;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_46 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__tx_valid_45 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_45 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_832_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_45 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_45 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_45)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_60) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_13 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_13 
                = vlSelfRef.rspu_top__DOT__tx_data_60;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_60)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_13 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_12;
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_13 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_12;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_62 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__tx_valid_61 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_61 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_864_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_61 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_61 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_61)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_11) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_12 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_12 
                = vlSelfRef.rspu_top__DOT__tx_data_11;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_11)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_12 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_11;
            vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_12 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_11;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_13 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__tx_valid_12 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_12 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_766_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_12 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_12 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_12)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_27) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_12 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_12 
                = vlSelfRef.rspu_top__DOT__tx_data_27;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_27)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_12 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_11;
            vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_12 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_11;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_29 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__tx_valid_28 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_28 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_798_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_28 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_28 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_28)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_43) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_12 = 1U;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_12 
                = vlSelfRef.rspu_top__DOT__tx_data_43;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_43)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_12 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_11;
            vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_12 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_11;
        }
        vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_trap = 0U;
        if (vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_invalid) {
            vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_trap = 1U;
        }
        if (vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_pcc_valid) {
            vlSelfRef.rspu_top__DOT__pc_45 = vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_pipe_pc;
        }
        vlSelfRef.rspu_top__DOT__tx_valid_44 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_44 = 0ULL;
        if (vlSelfRef.rspu_top__DOT__core_top_call_830_trap_signal) {
            vlSelfRef.rspu_top__DOT__tx_valid_44 = 1U;
            vlSelfRef.rspu_top__DOT__tx_data_44 = (0x8000000e00000000ULL 
                                                   | (QData)((IData)(vlSelfRef.rspu_top__DOT__pc_44)));
        }
        if (vlSelfRef.rspu_top__DOT__tx_valid_59) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_12 = 1U;
        }
        if ((1U & (~ (IData)(vlSelfRef.rspu_top__DOT__tx_valid_59)))) {
            vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_12 
                = vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_11;
        }
    } else {
        vlSelfRef.rspu_top__DOT__tx_valid_62 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_62 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_14 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_13 = 0U;
        vlSelfRef.rspu_top__DOT__pc_14 = 0U;
        vlSelfRef.rspu_top__DOT__tx_valid_13 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_13 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_13 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_13 = 0U;
        vlSelfRef.rspu_top__DOT__pc_30 = 0U;
        vlSelfRef.rspu_top__DOT__tx_valid_29 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_29 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_13 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_13 = 0U;
        vlSelfRef.rspu_top__DOT__pc_46 = 0U;
        vlSelfRef.rspu_top__DOT__tx_valid_45 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_45 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_13 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_13 = 0U;
        vlSelfRef.rspu_top__DOT__pc_62 = 0U;
        vlSelfRef.rspu_top__DOT__tx_valid_61 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_61 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sd_13 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sv_12 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_30388_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_13 = 0U;
        vlSelfRef.rspu_top__DOT__tx_valid_12 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_12 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_0_call_886_sd_12 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sv_12 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_21828_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_29 = 0U;
        vlSelfRef.rspu_top__DOT__tx_valid_28 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_28 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_1_call_888_sd_12 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sv_12 = 0U;
        vlSelfRef.rspu_top__DOT__alu_core_call_13268_is_trap = 0U;
        vlSelfRef.rspu_top__DOT__pc_45 = 0U;
        vlSelfRef.rspu_top__DOT__tx_valid_44 = 0U;
        vlSelfRef.rspu_top__DOT__tx_data_44 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_2_call_890_sd_12 = 0ULL;
        vlSelfRef.rspu_top__DOT__noc_l1_router_3_call_892_sv_12 = 0U;
    }
    vlSelfRef.rspu_top__DOT__core_top_call_834_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_trap)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_4692_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_4488_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_4708_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_866_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_ex_trap)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_pipe_pc 
        = __Vdly__rspu_top__DOT__rspu_pipeline_call_30703_pipe_pc;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_pipe_pc 
        = __Vdly__rspu_top__DOT__rspu_pipeline_call_22143_pipe_pc;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_pipe_pc 
        = __Vdly__rspu_top__DOT__rspu_pipeline_call_13583_pipe_pc;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_5023_pipe_pc 
        = __Vdly__rspu_top__DOT__rspu_pipeline_call_5023_pipe_pc;
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_30907_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_30703_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_30923_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_768_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_31238_ex_trap)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_22347_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22143_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_22363_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_800_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_22678_ex_trap)));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_pcc_fault 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__pcc_verifier_call_13787_is_invalid));
    vlSelfRef.rspu_top__DOT__rspu_pipeline_call_13583_ex_trap 
        = ((IData)(vlSelfRef.rst_n) && (IData)(vlSelfRef.rspu_top__DOT__alu_core_call_13803_is_trap));
    vlSelfRef.rspu_top__DOT__core_top_call_832_trap_signal 
        = ((IData)(vlSelfRef.rst_n) && ((IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_pcc_fault) 
                                        | (IData)(vlSelfRef.rspu_top__DOT__rspu_pipeline_call_14118_ex_trap)));
}

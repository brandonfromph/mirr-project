// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vreflex_soc_4core.h for the primary calling header

#include "Vreflex_soc_4core__pch.h"

void Vreflex_soc_4core___024root___ctor_var_reset(Vreflex_soc_4core___024root* vlSelf);

Vreflex_soc_4core___024root::Vreflex_soc_4core___024root(Vreflex_soc_4core__Syms* symsp, const char* namep)
 {
    vlSymsp = symsp;
    vlNamep = strdup(namep);
    // Reset structure values
    Vreflex_soc_4core___024root___ctor_var_reset(this);
}

void Vreflex_soc_4core___024root::__Vconfigure(bool first) {
    (void)first;  // Prevent unused variable warning
}

Vreflex_soc_4core___024root::~Vreflex_soc_4core___024root() {
    VL_DO_DANGLING(std::free(const_cast<char*>(vlNamep)), vlNamep);
}

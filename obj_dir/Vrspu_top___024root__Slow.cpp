// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Design implementation internals
// See Vrspu_top.h for the primary calling header

#include "Vrspu_top__pch.h"

void Vrspu_top___024root___ctor_var_reset(Vrspu_top___024root* vlSelf);

Vrspu_top___024root::Vrspu_top___024root(Vrspu_top__Syms* symsp, const char* namep)
 {
    vlSymsp = symsp;
    vlNamep = strdup(namep);
    // Reset structure values
    Vrspu_top___024root___ctor_var_reset(this);
}

void Vrspu_top___024root::__Vconfigure(bool first) {
    (void)first;  // Prevent unused variable warning
}

Vrspu_top___024root::~Vrspu_top___024root() {
    VL_DO_DANGLING(std::free(const_cast<char*>(vlNamep)), vlNamep);
}

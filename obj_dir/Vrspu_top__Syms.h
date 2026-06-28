// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Symbol table internal header
//
// Internal details; most calling programs do not need this header,
// unless using verilator public meta comments.

#ifndef VERILATED_VRSPU_TOP__SYMS_H_
#define VERILATED_VRSPU_TOP__SYMS_H_  // guard

#include "verilated.h"

// INCLUDE MODEL CLASS

#include "Vrspu_top.h"

// INCLUDE MODULE CLASSES
#include "Vrspu_top___024root.h"

// SYMS CLASS (contains all model state)
class alignas(VL_CACHE_LINE_BYTES) Vrspu_top__Syms final : public VerilatedSyms {
  public:
    // INTERNAL STATE
    Vrspu_top* const __Vm_modelp;
    VlDeleter __Vm_deleter;
    bool __Vm_didInit = false;

    // MODULE INSTANCE STATE
    Vrspu_top___024root            TOP;

    // CONSTRUCTORS
    Vrspu_top__Syms(VerilatedContext* contextp, const char* namep, Vrspu_top* modelp);
    ~Vrspu_top__Syms();

    // METHODS
    const char* name() const { return TOP.vlNamep; }
};

#endif  // guard

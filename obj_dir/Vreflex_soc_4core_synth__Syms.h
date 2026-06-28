// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Symbol table internal header
//
// Internal details; most calling programs do not need this header,
// unless using verilator public meta comments.

#ifndef VERILATED_VREFLEX_SOC_4CORE_SYNTH__SYMS_H_
#define VERILATED_VREFLEX_SOC_4CORE_SYNTH__SYMS_H_  // guard

#include "verilated.h"

// INCLUDE MODEL CLASS

#include "Vreflex_soc_4core_synth.h"

// INCLUDE MODULE CLASSES
#include "Vreflex_soc_4core_synth___024root.h"

// SYMS CLASS (contains all model state)
class alignas(VL_CACHE_LINE_BYTES) Vreflex_soc_4core_synth__Syms final : public VerilatedSyms {
  public:
    // INTERNAL STATE
    Vreflex_soc_4core_synth* const __Vm_modelp;
    VlDeleter __Vm_deleter;
    bool __Vm_didInit = false;

    // MODULE INSTANCE STATE
    Vreflex_soc_4core_synth___024root TOP;

    // CONSTRUCTORS
    Vreflex_soc_4core_synth__Syms(VerilatedContext* contextp, const char* namep, Vreflex_soc_4core_synth* modelp);
    ~Vreflex_soc_4core_synth__Syms();

    // METHODS
    const char* name() const { return TOP.vlNamep; }
};

#endif  // guard

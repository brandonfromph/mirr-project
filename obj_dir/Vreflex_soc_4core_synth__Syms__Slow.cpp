// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Symbol table implementation internals

#include "Vreflex_soc_4core_synth__pch.h"

Vreflex_soc_4core_synth__Syms::Vreflex_soc_4core_synth__Syms(VerilatedContext* contextp, const char* namep, Vreflex_soc_4core_synth* modelp)
    : VerilatedSyms{contextp}
    // Setup internal state of the Syms class
    , __Vm_modelp{modelp}
    // Setup top module instance
    , TOP{this, namep}
{
    // Check resources
    Verilated::stackCheck(356);
    // Setup sub module instances
    // Configure time unit / time precision
    _vm_contextp__->timeunit(-12);
    _vm_contextp__->timeprecision(-12);
    // Setup each module's pointers to their submodules
    // Setup each module's pointer back to symbol table (for public functions)
    TOP.__Vconfigure(true);
    // Setup scopes
}

Vreflex_soc_4core_synth__Syms::~Vreflex_soc_4core_synth__Syms() {
    // Tear down scopes
    // Tear down sub module instances
}

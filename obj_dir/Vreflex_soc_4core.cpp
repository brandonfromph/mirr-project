// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Model implementation (design independent parts)

#include "Vreflex_soc_4core__pch.h"

//============================================================
// Constructors

Vreflex_soc_4core::Vreflex_soc_4core(VerilatedContext* _vcontextp__, const char* _vcname__)
    : VerilatedModel{*_vcontextp__}
    , vlSymsp{new Vreflex_soc_4core__Syms(contextp(), _vcname__, this)}
    , clk{vlSymsp->TOP.clk}
    , rst_n{vlSymsp->TOP.rst_n}
    , global_trap{vlSymsp->TOP.global_trap}
    , out_pc_0{vlSymsp->TOP.out_pc_0}
    , out_pc_1{vlSymsp->TOP.out_pc_1}
    , out_pc_2{vlSymsp->TOP.out_pc_2}
    , out_pc_3{vlSymsp->TOP.out_pc_3}
    , out_data_0{vlSymsp->TOP.out_data_0}
    , out_data_1{vlSymsp->TOP.out_data_1}
    , out_data_2{vlSymsp->TOP.out_data_2}
    , out_data_3{vlSymsp->TOP.out_data_3}
    , rootp{&(vlSymsp->TOP)}
{
    // Register model with the context
    contextp()->addModel(this);
}

Vreflex_soc_4core::Vreflex_soc_4core(const char* _vcname__)
    : Vreflex_soc_4core(Verilated::threadContextp(), _vcname__)
{
}

//============================================================
// Destructor

Vreflex_soc_4core::~Vreflex_soc_4core() {
    delete vlSymsp;
}

//============================================================
// Evaluation function

#ifdef VL_DEBUG
void Vreflex_soc_4core___024root___eval_debug_assertions(Vreflex_soc_4core___024root* vlSelf);
#endif  // VL_DEBUG
void Vreflex_soc_4core___024root___eval_static(Vreflex_soc_4core___024root* vlSelf);
void Vreflex_soc_4core___024root___eval_initial(Vreflex_soc_4core___024root* vlSelf);
void Vreflex_soc_4core___024root___eval_settle(Vreflex_soc_4core___024root* vlSelf);
void Vreflex_soc_4core___024root___eval(Vreflex_soc_4core___024root* vlSelf);

void Vreflex_soc_4core::eval_step() {
    VL_DEBUG_IF(VL_DBG_MSGF("+++++TOP Evaluate Vreflex_soc_4core::eval_step\n"); );
#ifdef VL_DEBUG
    // Debug assertions
    Vreflex_soc_4core___024root___eval_debug_assertions(&(vlSymsp->TOP));
#endif  // VL_DEBUG
    vlSymsp->__Vm_deleter.deleteAll();
    if (VL_UNLIKELY(!vlSymsp->__Vm_didInit)) {
        VL_DEBUG_IF(VL_DBG_MSGF("+ Initial\n"););
        Vreflex_soc_4core___024root___eval_static(&(vlSymsp->TOP));
        Vreflex_soc_4core___024root___eval_initial(&(vlSymsp->TOP));
        Vreflex_soc_4core___024root___eval_settle(&(vlSymsp->TOP));
        vlSymsp->__Vm_didInit = true;
    }
    VL_DEBUG_IF(VL_DBG_MSGF("+ Eval\n"););
    Vreflex_soc_4core___024root___eval(&(vlSymsp->TOP));
    // Evaluate cleanup
    Verilated::endOfEval(vlSymsp->__Vm_evalMsgQp);
}

//============================================================
// Events and timing
bool Vreflex_soc_4core::eventsPending() { return false; }

uint64_t Vreflex_soc_4core::nextTimeSlot() {
    VL_FATAL_MT(__FILE__, __LINE__, "", "No delays in the design");
    return 0;
}

//============================================================
// Utilities

const char* Vreflex_soc_4core::name() const {
    return vlSymsp->name();
}

//============================================================
// Invoke final blocks

void Vreflex_soc_4core___024root___eval_final(Vreflex_soc_4core___024root* vlSelf);

VL_ATTR_COLD void Vreflex_soc_4core::final() {
    contextp()->executingFinal(true);
    Vreflex_soc_4core___024root___eval_final(&(vlSymsp->TOP));
    contextp()->executingFinal(false);
}

//============================================================
// Implementations of abstract methods from VerilatedModel

const char* Vreflex_soc_4core::hierName() const { return vlSymsp->name(); }
const char* Vreflex_soc_4core::modelName() const { return "Vreflex_soc_4core"; }
unsigned Vreflex_soc_4core::threads() const { return 1; }
void Vreflex_soc_4core::prepareClone() const { contextp()->prepareClone(); }
void Vreflex_soc_4core::atClone() const {
    contextp()->threadPoolpOnClone();
}

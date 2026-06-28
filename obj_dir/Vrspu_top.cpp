// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Model implementation (design independent parts)

#include "Vrspu_top__pch.h"

//============================================================
// Constructors

Vrspu_top::Vrspu_top(VerilatedContext* _vcontextp__, const char* _vcname__)
    : VerilatedModel{*_vcontextp__}
    , vlSymsp{new Vrspu_top__Syms(contextp(), _vcname__, this)}
    , clk{vlSymsp->TOP.clk}
    , rst_n{vlSymsp->TOP.rst_n}
    , global_trap{vlSymsp->TOP.global_trap}
    , uplink_valid_0{vlSymsp->TOP.uplink_valid_0}
    , downlink_valid_0{vlSymsp->TOP.downlink_valid_0}
    , uplink_valid_1{vlSymsp->TOP.uplink_valid_1}
    , downlink_valid_1{vlSymsp->TOP.downlink_valid_1}
    , uplink_valid_2{vlSymsp->TOP.uplink_valid_2}
    , downlink_valid_2{vlSymsp->TOP.downlink_valid_2}
    , uplink_valid_3{vlSymsp->TOP.uplink_valid_3}
    , downlink_valid_3{vlSymsp->TOP.downlink_valid_3}
    , out_pc_0{vlSymsp->TOP.out_pc_0}
    , out_pc_1{vlSymsp->TOP.out_pc_1}
    , out_pc_2{vlSymsp->TOP.out_pc_2}
    , out_pc_3{vlSymsp->TOP.out_pc_3}
    , out_pc_4{vlSymsp->TOP.out_pc_4}
    , out_pc_5{vlSymsp->TOP.out_pc_5}
    , out_pc_6{vlSymsp->TOP.out_pc_6}
    , out_pc_7{vlSymsp->TOP.out_pc_7}
    , out_pc_8{vlSymsp->TOP.out_pc_8}
    , out_pc_9{vlSymsp->TOP.out_pc_9}
    , out_pc_10{vlSymsp->TOP.out_pc_10}
    , out_pc_11{vlSymsp->TOP.out_pc_11}
    , out_pc_12{vlSymsp->TOP.out_pc_12}
    , out_pc_13{vlSymsp->TOP.out_pc_13}
    , out_pc_14{vlSymsp->TOP.out_pc_14}
    , out_pc_15{vlSymsp->TOP.out_pc_15}
    , out_pc_16{vlSymsp->TOP.out_pc_16}
    , out_pc_17{vlSymsp->TOP.out_pc_17}
    , out_pc_18{vlSymsp->TOP.out_pc_18}
    , out_pc_19{vlSymsp->TOP.out_pc_19}
    , out_pc_20{vlSymsp->TOP.out_pc_20}
    , out_pc_21{vlSymsp->TOP.out_pc_21}
    , out_pc_22{vlSymsp->TOP.out_pc_22}
    , out_pc_23{vlSymsp->TOP.out_pc_23}
    , out_pc_24{vlSymsp->TOP.out_pc_24}
    , out_pc_25{vlSymsp->TOP.out_pc_25}
    , out_pc_26{vlSymsp->TOP.out_pc_26}
    , out_pc_27{vlSymsp->TOP.out_pc_27}
    , out_pc_28{vlSymsp->TOP.out_pc_28}
    , out_pc_29{vlSymsp->TOP.out_pc_29}
    , out_pc_30{vlSymsp->TOP.out_pc_30}
    , out_pc_31{vlSymsp->TOP.out_pc_31}
    , out_pc_32{vlSymsp->TOP.out_pc_32}
    , out_pc_33{vlSymsp->TOP.out_pc_33}
    , out_pc_34{vlSymsp->TOP.out_pc_34}
    , out_pc_35{vlSymsp->TOP.out_pc_35}
    , out_pc_36{vlSymsp->TOP.out_pc_36}
    , out_pc_37{vlSymsp->TOP.out_pc_37}
    , out_pc_38{vlSymsp->TOP.out_pc_38}
    , out_pc_39{vlSymsp->TOP.out_pc_39}
    , out_pc_40{vlSymsp->TOP.out_pc_40}
    , out_pc_41{vlSymsp->TOP.out_pc_41}
    , out_pc_42{vlSymsp->TOP.out_pc_42}
    , out_pc_43{vlSymsp->TOP.out_pc_43}
    , out_pc_44{vlSymsp->TOP.out_pc_44}
    , out_pc_45{vlSymsp->TOP.out_pc_45}
    , out_pc_46{vlSymsp->TOP.out_pc_46}
    , out_pc_47{vlSymsp->TOP.out_pc_47}
    , out_pc_48{vlSymsp->TOP.out_pc_48}
    , out_pc_49{vlSymsp->TOP.out_pc_49}
    , out_pc_50{vlSymsp->TOP.out_pc_50}
    , out_pc_51{vlSymsp->TOP.out_pc_51}
    , out_pc_52{vlSymsp->TOP.out_pc_52}
    , out_pc_53{vlSymsp->TOP.out_pc_53}
    , out_pc_54{vlSymsp->TOP.out_pc_54}
    , out_pc_55{vlSymsp->TOP.out_pc_55}
    , out_pc_56{vlSymsp->TOP.out_pc_56}
    , out_pc_57{vlSymsp->TOP.out_pc_57}
    , out_pc_58{vlSymsp->TOP.out_pc_58}
    , out_pc_59{vlSymsp->TOP.out_pc_59}
    , out_pc_60{vlSymsp->TOP.out_pc_60}
    , out_pc_61{vlSymsp->TOP.out_pc_61}
    , out_pc_62{vlSymsp->TOP.out_pc_62}
    , out_pc_63{vlSymsp->TOP.out_pc_63}
    , out_data_0{vlSymsp->TOP.out_data_0}
    , io_in_0{vlSymsp->TOP.io_in_0}
    , io_out_0{vlSymsp->TOP.io_out_0}
    , out_data_1{vlSymsp->TOP.out_data_1}
    , io_in_1{vlSymsp->TOP.io_in_1}
    , io_out_1{vlSymsp->TOP.io_out_1}
    , out_data_2{vlSymsp->TOP.out_data_2}
    , io_in_2{vlSymsp->TOP.io_in_2}
    , io_out_2{vlSymsp->TOP.io_out_2}
    , out_data_3{vlSymsp->TOP.out_data_3}
    , io_in_3{vlSymsp->TOP.io_in_3}
    , io_out_3{vlSymsp->TOP.io_out_3}
    , out_data_4{vlSymsp->TOP.out_data_4}
    , io_in_4{vlSymsp->TOP.io_in_4}
    , io_out_4{vlSymsp->TOP.io_out_4}
    , out_data_5{vlSymsp->TOP.out_data_5}
    , io_in_5{vlSymsp->TOP.io_in_5}
    , io_out_5{vlSymsp->TOP.io_out_5}
    , out_data_6{vlSymsp->TOP.out_data_6}
    , io_in_6{vlSymsp->TOP.io_in_6}
    , io_out_6{vlSymsp->TOP.io_out_6}
    , out_data_7{vlSymsp->TOP.out_data_7}
    , io_in_7{vlSymsp->TOP.io_in_7}
    , io_out_7{vlSymsp->TOP.io_out_7}
    , out_data_8{vlSymsp->TOP.out_data_8}
    , io_in_8{vlSymsp->TOP.io_in_8}
    , io_out_8{vlSymsp->TOP.io_out_8}
    , out_data_9{vlSymsp->TOP.out_data_9}
    , io_in_9{vlSymsp->TOP.io_in_9}
    , io_out_9{vlSymsp->TOP.io_out_9}
    , out_data_10{vlSymsp->TOP.out_data_10}
    , io_in_10{vlSymsp->TOP.io_in_10}
    , io_out_10{vlSymsp->TOP.io_out_10}
    , out_data_11{vlSymsp->TOP.out_data_11}
    , io_in_11{vlSymsp->TOP.io_in_11}
    , io_out_11{vlSymsp->TOP.io_out_11}
    , out_data_12{vlSymsp->TOP.out_data_12}
    , io_in_12{vlSymsp->TOP.io_in_12}
    , io_out_12{vlSymsp->TOP.io_out_12}
    , out_data_13{vlSymsp->TOP.out_data_13}
    , io_in_13{vlSymsp->TOP.io_in_13}
    , io_out_13{vlSymsp->TOP.io_out_13}
    , out_data_14{vlSymsp->TOP.out_data_14}
    , io_in_14{vlSymsp->TOP.io_in_14}
    , io_out_14{vlSymsp->TOP.io_out_14}
    , out_data_15{vlSymsp->TOP.out_data_15}
    , io_in_15{vlSymsp->TOP.io_in_15}
    , io_out_15{vlSymsp->TOP.io_out_15}
    , out_data_16{vlSymsp->TOP.out_data_16}
    , io_in_16{vlSymsp->TOP.io_in_16}
    , io_out_16{vlSymsp->TOP.io_out_16}
    , out_data_17{vlSymsp->TOP.out_data_17}
    , io_in_17{vlSymsp->TOP.io_in_17}
    , io_out_17{vlSymsp->TOP.io_out_17}
    , out_data_18{vlSymsp->TOP.out_data_18}
    , io_in_18{vlSymsp->TOP.io_in_18}
    , io_out_18{vlSymsp->TOP.io_out_18}
    , out_data_19{vlSymsp->TOP.out_data_19}
    , io_in_19{vlSymsp->TOP.io_in_19}
    , io_out_19{vlSymsp->TOP.io_out_19}
    , out_data_20{vlSymsp->TOP.out_data_20}
    , io_in_20{vlSymsp->TOP.io_in_20}
    , io_out_20{vlSymsp->TOP.io_out_20}
    , out_data_21{vlSymsp->TOP.out_data_21}
    , io_in_21{vlSymsp->TOP.io_in_21}
    , io_out_21{vlSymsp->TOP.io_out_21}
    , out_data_22{vlSymsp->TOP.out_data_22}
    , io_in_22{vlSymsp->TOP.io_in_22}
    , io_out_22{vlSymsp->TOP.io_out_22}
    , out_data_23{vlSymsp->TOP.out_data_23}
    , io_in_23{vlSymsp->TOP.io_in_23}
    , io_out_23{vlSymsp->TOP.io_out_23}
    , out_data_24{vlSymsp->TOP.out_data_24}
    , io_in_24{vlSymsp->TOP.io_in_24}
    , io_out_24{vlSymsp->TOP.io_out_24}
    , out_data_25{vlSymsp->TOP.out_data_25}
    , io_in_25{vlSymsp->TOP.io_in_25}
    , io_out_25{vlSymsp->TOP.io_out_25}
    , out_data_26{vlSymsp->TOP.out_data_26}
    , io_in_26{vlSymsp->TOP.io_in_26}
    , io_out_26{vlSymsp->TOP.io_out_26}
    , out_data_27{vlSymsp->TOP.out_data_27}
    , io_in_27{vlSymsp->TOP.io_in_27}
    , io_out_27{vlSymsp->TOP.io_out_27}
    , out_data_28{vlSymsp->TOP.out_data_28}
    , io_in_28{vlSymsp->TOP.io_in_28}
    , io_out_28{vlSymsp->TOP.io_out_28}
    , out_data_29{vlSymsp->TOP.out_data_29}
    , io_in_29{vlSymsp->TOP.io_in_29}
    , io_out_29{vlSymsp->TOP.io_out_29}
    , out_data_30{vlSymsp->TOP.out_data_30}
    , io_in_30{vlSymsp->TOP.io_in_30}
    , io_out_30{vlSymsp->TOP.io_out_30}
    , out_data_31{vlSymsp->TOP.out_data_31}
    , io_in_31{vlSymsp->TOP.io_in_31}
    , io_out_31{vlSymsp->TOP.io_out_31}
    , out_data_32{vlSymsp->TOP.out_data_32}
    , io_in_32{vlSymsp->TOP.io_in_32}
    , io_out_32{vlSymsp->TOP.io_out_32}
    , out_data_33{vlSymsp->TOP.out_data_33}
    , io_in_33{vlSymsp->TOP.io_in_33}
    , io_out_33{vlSymsp->TOP.io_out_33}
    , out_data_34{vlSymsp->TOP.out_data_34}
    , io_in_34{vlSymsp->TOP.io_in_34}
    , io_out_34{vlSymsp->TOP.io_out_34}
    , out_data_35{vlSymsp->TOP.out_data_35}
    , io_in_35{vlSymsp->TOP.io_in_35}
    , io_out_35{vlSymsp->TOP.io_out_35}
    , out_data_36{vlSymsp->TOP.out_data_36}
    , io_in_36{vlSymsp->TOP.io_in_36}
    , io_out_36{vlSymsp->TOP.io_out_36}
    , out_data_37{vlSymsp->TOP.out_data_37}
    , io_in_37{vlSymsp->TOP.io_in_37}
    , io_out_37{vlSymsp->TOP.io_out_37}
    , out_data_38{vlSymsp->TOP.out_data_38}
    , io_in_38{vlSymsp->TOP.io_in_38}
    , io_out_38{vlSymsp->TOP.io_out_38}
    , out_data_39{vlSymsp->TOP.out_data_39}
    , io_in_39{vlSymsp->TOP.io_in_39}
    , io_out_39{vlSymsp->TOP.io_out_39}
    , out_data_40{vlSymsp->TOP.out_data_40}
    , io_in_40{vlSymsp->TOP.io_in_40}
    , io_out_40{vlSymsp->TOP.io_out_40}
    , out_data_41{vlSymsp->TOP.out_data_41}
    , io_in_41{vlSymsp->TOP.io_in_41}
    , io_out_41{vlSymsp->TOP.io_out_41}
    , out_data_42{vlSymsp->TOP.out_data_42}
    , io_in_42{vlSymsp->TOP.io_in_42}
    , io_out_42{vlSymsp->TOP.io_out_42}
    , out_data_43{vlSymsp->TOP.out_data_43}
    , io_in_43{vlSymsp->TOP.io_in_43}
    , io_out_43{vlSymsp->TOP.io_out_43}
    , out_data_44{vlSymsp->TOP.out_data_44}
    , io_in_44{vlSymsp->TOP.io_in_44}
    , io_out_44{vlSymsp->TOP.io_out_44}
    , out_data_45{vlSymsp->TOP.out_data_45}
    , io_in_45{vlSymsp->TOP.io_in_45}
    , io_out_45{vlSymsp->TOP.io_out_45}
    , out_data_46{vlSymsp->TOP.out_data_46}
    , io_in_46{vlSymsp->TOP.io_in_46}
    , io_out_46{vlSymsp->TOP.io_out_46}
    , out_data_47{vlSymsp->TOP.out_data_47}
    , io_in_47{vlSymsp->TOP.io_in_47}
    , io_out_47{vlSymsp->TOP.io_out_47}
    , out_data_48{vlSymsp->TOP.out_data_48}
    , io_in_48{vlSymsp->TOP.io_in_48}
    , io_out_48{vlSymsp->TOP.io_out_48}
    , out_data_49{vlSymsp->TOP.out_data_49}
    , io_in_49{vlSymsp->TOP.io_in_49}
    , io_out_49{vlSymsp->TOP.io_out_49}
    , out_data_50{vlSymsp->TOP.out_data_50}
    , io_in_50{vlSymsp->TOP.io_in_50}
    , io_out_50{vlSymsp->TOP.io_out_50}
    , out_data_51{vlSymsp->TOP.out_data_51}
    , io_in_51{vlSymsp->TOP.io_in_51}
    , io_out_51{vlSymsp->TOP.io_out_51}
    , out_data_52{vlSymsp->TOP.out_data_52}
    , io_in_52{vlSymsp->TOP.io_in_52}
    , io_out_52{vlSymsp->TOP.io_out_52}
    , out_data_53{vlSymsp->TOP.out_data_53}
    , io_in_53{vlSymsp->TOP.io_in_53}
    , io_out_53{vlSymsp->TOP.io_out_53}
    , out_data_54{vlSymsp->TOP.out_data_54}
    , io_in_54{vlSymsp->TOP.io_in_54}
    , io_out_54{vlSymsp->TOP.io_out_54}
    , out_data_55{vlSymsp->TOP.out_data_55}
    , io_in_55{vlSymsp->TOP.io_in_55}
    , io_out_55{vlSymsp->TOP.io_out_55}
    , out_data_56{vlSymsp->TOP.out_data_56}
    , io_in_56{vlSymsp->TOP.io_in_56}
    , io_out_56{vlSymsp->TOP.io_out_56}
    , out_data_57{vlSymsp->TOP.out_data_57}
    , io_in_57{vlSymsp->TOP.io_in_57}
    , io_out_57{vlSymsp->TOP.io_out_57}
    , out_data_58{vlSymsp->TOP.out_data_58}
    , io_in_58{vlSymsp->TOP.io_in_58}
    , io_out_58{vlSymsp->TOP.io_out_58}
    , out_data_59{vlSymsp->TOP.out_data_59}
    , io_in_59{vlSymsp->TOP.io_in_59}
    , io_out_59{vlSymsp->TOP.io_out_59}
    , out_data_60{vlSymsp->TOP.out_data_60}
    , io_in_60{vlSymsp->TOP.io_in_60}
    , io_out_60{vlSymsp->TOP.io_out_60}
    , out_data_61{vlSymsp->TOP.out_data_61}
    , io_in_61{vlSymsp->TOP.io_in_61}
    , io_out_61{vlSymsp->TOP.io_out_61}
    , out_data_62{vlSymsp->TOP.out_data_62}
    , io_in_62{vlSymsp->TOP.io_in_62}
    , io_out_62{vlSymsp->TOP.io_out_62}
    , out_data_63{vlSymsp->TOP.out_data_63}
    , io_in_63{vlSymsp->TOP.io_in_63}
    , io_out_63{vlSymsp->TOP.io_out_63}
    , uplink_data_0{vlSymsp->TOP.uplink_data_0}
    , downlink_data_0{vlSymsp->TOP.downlink_data_0}
    , uplink_data_1{vlSymsp->TOP.uplink_data_1}
    , downlink_data_1{vlSymsp->TOP.downlink_data_1}
    , uplink_data_2{vlSymsp->TOP.uplink_data_2}
    , downlink_data_2{vlSymsp->TOP.downlink_data_2}
    , uplink_data_3{vlSymsp->TOP.uplink_data_3}
    , downlink_data_3{vlSymsp->TOP.downlink_data_3}
    , rootp{&(vlSymsp->TOP)}
{
    // Register model with the context
    contextp()->addModel(this);
}

Vrspu_top::Vrspu_top(const char* _vcname__)
    : Vrspu_top(Verilated::threadContextp(), _vcname__)
{
}

//============================================================
// Destructor

Vrspu_top::~Vrspu_top() {
    delete vlSymsp;
}

//============================================================
// Evaluation function

#ifdef VL_DEBUG
void Vrspu_top___024root___eval_debug_assertions(Vrspu_top___024root* vlSelf);
#endif  // VL_DEBUG
void Vrspu_top___024root___eval_static(Vrspu_top___024root* vlSelf);
void Vrspu_top___024root___eval_initial(Vrspu_top___024root* vlSelf);
void Vrspu_top___024root___eval_settle(Vrspu_top___024root* vlSelf);
void Vrspu_top___024root___eval(Vrspu_top___024root* vlSelf);

void Vrspu_top::eval_step() {
    VL_DEBUG_IF(VL_DBG_MSGF("+++++TOP Evaluate Vrspu_top::eval_step\n"); );
#ifdef VL_DEBUG
    // Debug assertions
    Vrspu_top___024root___eval_debug_assertions(&(vlSymsp->TOP));
#endif  // VL_DEBUG
    vlSymsp->__Vm_deleter.deleteAll();
    if (VL_UNLIKELY(!vlSymsp->__Vm_didInit)) {
        VL_DEBUG_IF(VL_DBG_MSGF("+ Initial\n"););
        Vrspu_top___024root___eval_static(&(vlSymsp->TOP));
        Vrspu_top___024root___eval_initial(&(vlSymsp->TOP));
        Vrspu_top___024root___eval_settle(&(vlSymsp->TOP));
        vlSymsp->__Vm_didInit = true;
    }
    VL_DEBUG_IF(VL_DBG_MSGF("+ Eval\n"););
    Vrspu_top___024root___eval(&(vlSymsp->TOP));
    // Evaluate cleanup
    Verilated::endOfEval(vlSymsp->__Vm_evalMsgQp);
}

//============================================================
// Events and timing
bool Vrspu_top::eventsPending() { return false; }

uint64_t Vrspu_top::nextTimeSlot() {
    VL_FATAL_MT(__FILE__, __LINE__, "", "No delays in the design");
    return 0;
}

//============================================================
// Utilities

const char* Vrspu_top::name() const {
    return vlSymsp->name();
}

//============================================================
// Invoke final blocks

void Vrspu_top___024root___eval_final(Vrspu_top___024root* vlSelf);

VL_ATTR_COLD void Vrspu_top::final() {
    contextp()->executingFinal(true);
    Vrspu_top___024root___eval_final(&(vlSymsp->TOP));
    contextp()->executingFinal(false);
}

//============================================================
// Implementations of abstract methods from VerilatedModel

const char* Vrspu_top::hierName() const { return vlSymsp->name(); }
const char* Vrspu_top::modelName() const { return "Vrspu_top"; }
unsigned Vrspu_top::threads() const { return 1; }
void Vrspu_top::prepareClone() const { contextp()->prepareClone(); }
void Vrspu_top::atClone() const {
    contextp()->threadPoolpOnClone();
}

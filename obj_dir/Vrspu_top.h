// Verilated -*- C++ -*-
// DESCRIPTION: Verilator output: Primary model header
//
// This header should be included by all source files instantiating the design.
// The class here is then constructed to instantiate the design.
// See the Verilator manual for examples.

#ifndef VERILATED_VRSPU_TOP_H_
#define VERILATED_VRSPU_TOP_H_  // guard

#include "verilated.h"

class Vrspu_top__Syms;
class Vrspu_top___024root;

// This class is the main interface to the Verilated model
class alignas(VL_CACHE_LINE_BYTES) Vrspu_top VL_NOT_FINAL : public VerilatedModel {
  private:
    // Symbol table holding complete model state (owned by this class)
    Vrspu_top__Syms* const vlSymsp;

  public:

    // CONSTEXPR CAPABILITIES
    // Verilated with --trace?
    static constexpr bool traceCapable = false;

    // PORTS
    // The application code writes and reads these signals to
    // propagate new values into/out from the Verilated model.
    VL_IN8(&clk,0,0);
    VL_IN8(&rst_n,0,0);
    VL_OUT8(&global_trap,0,0);
    VL_OUT8(&uplink_valid_0,0,0);
    VL_IN8(&downlink_valid_0,0,0);
    VL_OUT8(&uplink_valid_1,0,0);
    VL_IN8(&downlink_valid_1,0,0);
    VL_OUT8(&uplink_valid_2,0,0);
    VL_IN8(&downlink_valid_2,0,0);
    VL_OUT8(&uplink_valid_3,0,0);
    VL_IN8(&downlink_valid_3,0,0);
    VL_OUT(&out_pc_0,31,0);
    VL_OUT(&out_pc_1,31,0);
    VL_OUT(&out_pc_2,31,0);
    VL_OUT(&out_pc_3,31,0);
    VL_OUT(&out_pc_4,31,0);
    VL_OUT(&out_pc_5,31,0);
    VL_OUT(&out_pc_6,31,0);
    VL_OUT(&out_pc_7,31,0);
    VL_OUT(&out_pc_8,31,0);
    VL_OUT(&out_pc_9,31,0);
    VL_OUT(&out_pc_10,31,0);
    VL_OUT(&out_pc_11,31,0);
    VL_OUT(&out_pc_12,31,0);
    VL_OUT(&out_pc_13,31,0);
    VL_OUT(&out_pc_14,31,0);
    VL_OUT(&out_pc_15,31,0);
    VL_OUT(&out_pc_16,31,0);
    VL_OUT(&out_pc_17,31,0);
    VL_OUT(&out_pc_18,31,0);
    VL_OUT(&out_pc_19,31,0);
    VL_OUT(&out_pc_20,31,0);
    VL_OUT(&out_pc_21,31,0);
    VL_OUT(&out_pc_22,31,0);
    VL_OUT(&out_pc_23,31,0);
    VL_OUT(&out_pc_24,31,0);
    VL_OUT(&out_pc_25,31,0);
    VL_OUT(&out_pc_26,31,0);
    VL_OUT(&out_pc_27,31,0);
    VL_OUT(&out_pc_28,31,0);
    VL_OUT(&out_pc_29,31,0);
    VL_OUT(&out_pc_30,31,0);
    VL_OUT(&out_pc_31,31,0);
    VL_OUT(&out_pc_32,31,0);
    VL_OUT(&out_pc_33,31,0);
    VL_OUT(&out_pc_34,31,0);
    VL_OUT(&out_pc_35,31,0);
    VL_OUT(&out_pc_36,31,0);
    VL_OUT(&out_pc_37,31,0);
    VL_OUT(&out_pc_38,31,0);
    VL_OUT(&out_pc_39,31,0);
    VL_OUT(&out_pc_40,31,0);
    VL_OUT(&out_pc_41,31,0);
    VL_OUT(&out_pc_42,31,0);
    VL_OUT(&out_pc_43,31,0);
    VL_OUT(&out_pc_44,31,0);
    VL_OUT(&out_pc_45,31,0);
    VL_OUT(&out_pc_46,31,0);
    VL_OUT(&out_pc_47,31,0);
    VL_OUT(&out_pc_48,31,0);
    VL_OUT(&out_pc_49,31,0);
    VL_OUT(&out_pc_50,31,0);
    VL_OUT(&out_pc_51,31,0);
    VL_OUT(&out_pc_52,31,0);
    VL_OUT(&out_pc_53,31,0);
    VL_OUT(&out_pc_54,31,0);
    VL_OUT(&out_pc_55,31,0);
    VL_OUT(&out_pc_56,31,0);
    VL_OUT(&out_pc_57,31,0);
    VL_OUT(&out_pc_58,31,0);
    VL_OUT(&out_pc_59,31,0);
    VL_OUT(&out_pc_60,31,0);
    VL_OUT(&out_pc_61,31,0);
    VL_OUT(&out_pc_62,31,0);
    VL_OUT(&out_pc_63,31,0);
    VL_OUT64(&out_data_0,63,0);
    VL_IN64(&io_in_0,63,0);
    VL_OUT64(&io_out_0,63,0);
    VL_OUT64(&out_data_1,63,0);
    VL_IN64(&io_in_1,63,0);
    VL_OUT64(&io_out_1,63,0);
    VL_OUT64(&out_data_2,63,0);
    VL_IN64(&io_in_2,63,0);
    VL_OUT64(&io_out_2,63,0);
    VL_OUT64(&out_data_3,63,0);
    VL_IN64(&io_in_3,63,0);
    VL_OUT64(&io_out_3,63,0);
    VL_OUT64(&out_data_4,63,0);
    VL_IN64(&io_in_4,63,0);
    VL_OUT64(&io_out_4,63,0);
    VL_OUT64(&out_data_5,63,0);
    VL_IN64(&io_in_5,63,0);
    VL_OUT64(&io_out_5,63,0);
    VL_OUT64(&out_data_6,63,0);
    VL_IN64(&io_in_6,63,0);
    VL_OUT64(&io_out_6,63,0);
    VL_OUT64(&out_data_7,63,0);
    VL_IN64(&io_in_7,63,0);
    VL_OUT64(&io_out_7,63,0);
    VL_OUT64(&out_data_8,63,0);
    VL_IN64(&io_in_8,63,0);
    VL_OUT64(&io_out_8,63,0);
    VL_OUT64(&out_data_9,63,0);
    VL_IN64(&io_in_9,63,0);
    VL_OUT64(&io_out_9,63,0);
    VL_OUT64(&out_data_10,63,0);
    VL_IN64(&io_in_10,63,0);
    VL_OUT64(&io_out_10,63,0);
    VL_OUT64(&out_data_11,63,0);
    VL_IN64(&io_in_11,63,0);
    VL_OUT64(&io_out_11,63,0);
    VL_OUT64(&out_data_12,63,0);
    VL_IN64(&io_in_12,63,0);
    VL_OUT64(&io_out_12,63,0);
    VL_OUT64(&out_data_13,63,0);
    VL_IN64(&io_in_13,63,0);
    VL_OUT64(&io_out_13,63,0);
    VL_OUT64(&out_data_14,63,0);
    VL_IN64(&io_in_14,63,0);
    VL_OUT64(&io_out_14,63,0);
    VL_OUT64(&out_data_15,63,0);
    VL_IN64(&io_in_15,63,0);
    VL_OUT64(&io_out_15,63,0);
    VL_OUT64(&out_data_16,63,0);
    VL_IN64(&io_in_16,63,0);
    VL_OUT64(&io_out_16,63,0);
    VL_OUT64(&out_data_17,63,0);
    VL_IN64(&io_in_17,63,0);
    VL_OUT64(&io_out_17,63,0);
    VL_OUT64(&out_data_18,63,0);
    VL_IN64(&io_in_18,63,0);
    VL_OUT64(&io_out_18,63,0);
    VL_OUT64(&out_data_19,63,0);
    VL_IN64(&io_in_19,63,0);
    VL_OUT64(&io_out_19,63,0);
    VL_OUT64(&out_data_20,63,0);
    VL_IN64(&io_in_20,63,0);
    VL_OUT64(&io_out_20,63,0);
    VL_OUT64(&out_data_21,63,0);
    VL_IN64(&io_in_21,63,0);
    VL_OUT64(&io_out_21,63,0);
    VL_OUT64(&out_data_22,63,0);
    VL_IN64(&io_in_22,63,0);
    VL_OUT64(&io_out_22,63,0);
    VL_OUT64(&out_data_23,63,0);
    VL_IN64(&io_in_23,63,0);
    VL_OUT64(&io_out_23,63,0);
    VL_OUT64(&out_data_24,63,0);
    VL_IN64(&io_in_24,63,0);
    VL_OUT64(&io_out_24,63,0);
    VL_OUT64(&out_data_25,63,0);
    VL_IN64(&io_in_25,63,0);
    VL_OUT64(&io_out_25,63,0);
    VL_OUT64(&out_data_26,63,0);
    VL_IN64(&io_in_26,63,0);
    VL_OUT64(&io_out_26,63,0);
    VL_OUT64(&out_data_27,63,0);
    VL_IN64(&io_in_27,63,0);
    VL_OUT64(&io_out_27,63,0);
    VL_OUT64(&out_data_28,63,0);
    VL_IN64(&io_in_28,63,0);
    VL_OUT64(&io_out_28,63,0);
    VL_OUT64(&out_data_29,63,0);
    VL_IN64(&io_in_29,63,0);
    VL_OUT64(&io_out_29,63,0);
    VL_OUT64(&out_data_30,63,0);
    VL_IN64(&io_in_30,63,0);
    VL_OUT64(&io_out_30,63,0);
    VL_OUT64(&out_data_31,63,0);
    VL_IN64(&io_in_31,63,0);
    VL_OUT64(&io_out_31,63,0);
    VL_OUT64(&out_data_32,63,0);
    VL_IN64(&io_in_32,63,0);
    VL_OUT64(&io_out_32,63,0);
    VL_OUT64(&out_data_33,63,0);
    VL_IN64(&io_in_33,63,0);
    VL_OUT64(&io_out_33,63,0);
    VL_OUT64(&out_data_34,63,0);
    VL_IN64(&io_in_34,63,0);
    VL_OUT64(&io_out_34,63,0);
    VL_OUT64(&out_data_35,63,0);
    VL_IN64(&io_in_35,63,0);
    VL_OUT64(&io_out_35,63,0);
    VL_OUT64(&out_data_36,63,0);
    VL_IN64(&io_in_36,63,0);
    VL_OUT64(&io_out_36,63,0);
    VL_OUT64(&out_data_37,63,0);
    VL_IN64(&io_in_37,63,0);
    VL_OUT64(&io_out_37,63,0);
    VL_OUT64(&out_data_38,63,0);
    VL_IN64(&io_in_38,63,0);
    VL_OUT64(&io_out_38,63,0);
    VL_OUT64(&out_data_39,63,0);
    VL_IN64(&io_in_39,63,0);
    VL_OUT64(&io_out_39,63,0);
    VL_OUT64(&out_data_40,63,0);
    VL_IN64(&io_in_40,63,0);
    VL_OUT64(&io_out_40,63,0);
    VL_OUT64(&out_data_41,63,0);
    VL_IN64(&io_in_41,63,0);
    VL_OUT64(&io_out_41,63,0);
    VL_OUT64(&out_data_42,63,0);
    VL_IN64(&io_in_42,63,0);
    VL_OUT64(&io_out_42,63,0);
    VL_OUT64(&out_data_43,63,0);
    VL_IN64(&io_in_43,63,0);
    VL_OUT64(&io_out_43,63,0);
    VL_OUT64(&out_data_44,63,0);
    VL_IN64(&io_in_44,63,0);
    VL_OUT64(&io_out_44,63,0);
    VL_OUT64(&out_data_45,63,0);
    VL_IN64(&io_in_45,63,0);
    VL_OUT64(&io_out_45,63,0);
    VL_OUT64(&out_data_46,63,0);
    VL_IN64(&io_in_46,63,0);
    VL_OUT64(&io_out_46,63,0);
    VL_OUT64(&out_data_47,63,0);
    VL_IN64(&io_in_47,63,0);
    VL_OUT64(&io_out_47,63,0);
    VL_OUT64(&out_data_48,63,0);
    VL_IN64(&io_in_48,63,0);
    VL_OUT64(&io_out_48,63,0);
    VL_OUT64(&out_data_49,63,0);
    VL_IN64(&io_in_49,63,0);
    VL_OUT64(&io_out_49,63,0);
    VL_OUT64(&out_data_50,63,0);
    VL_IN64(&io_in_50,63,0);
    VL_OUT64(&io_out_50,63,0);
    VL_OUT64(&out_data_51,63,0);
    VL_IN64(&io_in_51,63,0);
    VL_OUT64(&io_out_51,63,0);
    VL_OUT64(&out_data_52,63,0);
    VL_IN64(&io_in_52,63,0);
    VL_OUT64(&io_out_52,63,0);
    VL_OUT64(&out_data_53,63,0);
    VL_IN64(&io_in_53,63,0);
    VL_OUT64(&io_out_53,63,0);
    VL_OUT64(&out_data_54,63,0);
    VL_IN64(&io_in_54,63,0);
    VL_OUT64(&io_out_54,63,0);
    VL_OUT64(&out_data_55,63,0);
    VL_IN64(&io_in_55,63,0);
    VL_OUT64(&io_out_55,63,0);
    VL_OUT64(&out_data_56,63,0);
    VL_IN64(&io_in_56,63,0);
    VL_OUT64(&io_out_56,63,0);
    VL_OUT64(&out_data_57,63,0);
    VL_IN64(&io_in_57,63,0);
    VL_OUT64(&io_out_57,63,0);
    VL_OUT64(&out_data_58,63,0);
    VL_IN64(&io_in_58,63,0);
    VL_OUT64(&io_out_58,63,0);
    VL_OUT64(&out_data_59,63,0);
    VL_IN64(&io_in_59,63,0);
    VL_OUT64(&io_out_59,63,0);
    VL_OUT64(&out_data_60,63,0);
    VL_IN64(&io_in_60,63,0);
    VL_OUT64(&io_out_60,63,0);
    VL_OUT64(&out_data_61,63,0);
    VL_IN64(&io_in_61,63,0);
    VL_OUT64(&io_out_61,63,0);
    VL_OUT64(&out_data_62,63,0);
    VL_IN64(&io_in_62,63,0);
    VL_OUT64(&io_out_62,63,0);
    VL_OUT64(&out_data_63,63,0);
    VL_IN64(&io_in_63,63,0);
    VL_OUT64(&io_out_63,63,0);
    VL_OUT64(&uplink_data_0,63,0);
    VL_IN64(&downlink_data_0,63,0);
    VL_OUT64(&uplink_data_1,63,0);
    VL_IN64(&downlink_data_1,63,0);
    VL_OUT64(&uplink_data_2,63,0);
    VL_IN64(&downlink_data_2,63,0);
    VL_OUT64(&uplink_data_3,63,0);
    VL_IN64(&downlink_data_3,63,0);

    // CELLS
    // Public to allow access to /* verilator public */ items.
    // Otherwise the application code can consider these internals.

    // Root instance pointer to allow access to model internals,
    // including inlined /* verilator public_flat_* */ items.
    Vrspu_top___024root* const rootp;

    // CONSTRUCTORS
    /// Construct the model; called by application code
    /// If contextp is null, then the model will use the default global context
    /// If name is "", then makes a wrapper with a
    /// single model invisible with respect to DPI scope names.
    explicit Vrspu_top(VerilatedContext* contextp, const char* name = "TOP");
    explicit Vrspu_top(const char* name = "TOP");
    /// Destroy the model; called (often implicitly) by application code
    virtual ~Vrspu_top();
  private:
    VL_UNCOPYABLE(Vrspu_top);  ///< Copying not allowed

  public:
    // API METHODS
    /// Evaluate the model.  Application must call when inputs change.
    void eval() { eval_step(); }
    /// Evaluate when calling multiple units/models per time step.
    void eval_step();
    /// Evaluate at end of a timestep for tracing, when using eval_step().
    /// Application must call after all eval() and before time changes.
    void eval_end_step() {}
    /// Simulation complete, run final blocks.  Application must call on completion.
    void final();
    /// Are there scheduled events to handle?
    bool eventsPending();
    /// Returns time at next time slot. Aborts if !eventsPending()
    uint64_t nextTimeSlot();
    /// Trace signals in the model; called by application code
    void trace(VerilatedTraceBaseC* tfp, int levels, int options = 0) { contextp()->trace(tfp, levels, options); }
    /// Retrieve name of this model instance (as passed to constructor).
    const char* name() const;

    // Abstract methods from VerilatedModel
    const char* hierName() const override final;
    const char* modelName() const override final;
    unsigned threads() const override final;
    /// Prepare for cloning the model at the process level (e.g. fork in Linux)
    /// Release necessary resources. Called before cloning.
    void prepareClone() const;
    /// Re-init after cloning the model at the process level (e.g. fork in Linux)
    /// Re-allocate necessary resources. Called after cloning.
    void atClone() const;
  private:
    // Internal functions - trace registration
    void traceBaseModel(VerilatedTraceBaseC* tfp, int levels, int options);
};

#endif  // guard

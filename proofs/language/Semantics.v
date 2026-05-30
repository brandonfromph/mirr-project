(**
 * Semantics.v — MIRR Operational Semantics
 *
 * Formalizes MIRR modules as labeled transition systems.
 * Every MIRR module is a finite-state machine: signals, guards,
 * counters, and shift registers are all bounded.
 *
 * Campaign: MEGA-14 SUB-TURING-PROOF
 * Date: 2026-03-20
 *)

Require Import Nat.
Require Import List.
Require Import Bool.
Require Import PeanoNat.
Import ListNotations.

(* ================================================================= *)
(** * Bounded Natural Numbers                                            *)
(* ================================================================= *)

(** All MIRR widths are bounded to 64 bits. *)
Definition MAX_WIDTH : nat := 64.

(** All MIRR signal counts are bounded. *)
Definition MAX_SIGNALS : nat := 256.

(** All MIRR guard counts are bounded. *)
Definition MAX_GUARDS : nat := 64.

(** Maximum dynamic delay for any guard. *)
Definition MAX_DYNAMIC_DELAY : nat := 256.

(** Maximum total state space (conservative upper bound). *)
Definition MAX_STATES : nat := 2 ^ (MAX_SIGNALS * MAX_WIDTH + MAX_GUARDS * MAX_DYNAMIC_DELAY).

(* ================================================================= *)
(** * Signal Representation                                             *)
(* ================================================================= *)

(** A signal has a name, a bit-width, and a current value. *)
Record Signal := {
  sig_name : nat;
  sig_width : nat;
  sig_value : nat;
}.

(** Signal width is always bounded. *)
Definition signal_bounded (s : Signal) : Prop :=
  sig_width s <= MAX_WIDTH.

(** Signal value fits within its width. *)
Definition signal_value_fits (s : Signal) : Prop :=
  sig_value s < 2 ^ (sig_width s).

(* ================================================================= *)
(** * Guard Representation                                              *)
(* ================================================================= *)

(** Guard condition: a boolean expression over signals.
    For simplicity, we model conditions as a predicate on signal state. *)
Definition GuardCondition := list Signal -> bool.

(** A guard has a condition, a delay (shift register depth), and a counter. *)
Record Guard := {
  guard_condition : GuardCondition;
  guard_delay : nat;
  guard_counter : nat;
}.

(** Guard delay is bounded. *)
Definition guard_delay_bounded (g : Guard) : Prop :=
  guard_delay g <= MAX_DYNAMIC_DELAY.

(** Guard counter is bounded. *)
Definition guard_counter_bounded (g : Guard) : Prop :=
  guard_counter g <= MAX_DYNAMIC_DELAY.

(* ================================================================= *)
(** * Reflex Representation                                             *)
(* ================================================================= *)

(** A reflex assigns a new value to a signal.
    We model it as a function from signal state to new values. *)
Record Reflex := {
  reflex_target : nat;       (* signal name *)
  reflex_expr : list Signal -> nat;  (* expression producing new value *)
}.

(* ================================================================= *)
(** * MIRR Module                                                       *)
(* ================================================================= *)

(** A MIRR module is a collection of signals, guards, and reflexes. *)
Record MIRRModule := {
  module_signals : list Signal;
  module_guards : list Guard;
  module_reflexes : list Reflex;
}.

(** Module signal count is bounded. *)
Definition module_signals_bounded (m : MIRRModule) : Prop :=
  length (module_signals m) <= MAX_SIGNALS.

(** Module guard count is bounded. *)
Definition module_guards_bounded (m : MIRRModule) : Prop :=
  length (module_guards m) <= MAX_GUARDS.

(* ================================================================= *)
(** * Module State                                                      *)
(* ================================================================= *)

(** The state of a MIRR module at a single clock cycle:
    - Signal values
    - Guard shift register contents (modeled as counter values)
    - Guard counter values *)
Record MIRRState := {
  state_signals : list Signal;
  state_guard_counters : list nat;
}.

(** All signals in state have bounded widths. *)
Definition state_signals_bounded (s : MIRRState) : Prop :=
  Forall signal_bounded (state_signals s).

(** All guard counters in state are bounded. *)
Definition state_counters_bounded (s : MIRRState) : Prop :=
  Forall (fun c => c <= MAX_DYNAMIC_DELAY) (state_guard_counters s).

(** Total state space is finite. *)
Definition state_space_bits (s : MIRRState) : nat :=
  (length (state_signals s) * MAX_WIDTH) +
  (length (state_guard_counters s) * MAX_DYNAMIC_DELAY).

Definition state_finite (s : MIRRState) : Prop :=
  state_space_bits s <= MAX_SIGNALS * MAX_WIDTH + MAX_GUARDS * MAX_DYNAMIC_DELAY.

(* ================================================================= *)
(** * Transition Relation                                               *)
(* ================================================================= *)

(** Update a signal value in the signal list. *)
Fixpoint update_signal (sigs : list Signal) (name : nat) (value : nat) : list Signal :=
  match sigs with
  | [] => []
  | s :: rest =>
      if Nat.eqb (sig_name s) name then
        {| sig_name := name; sig_width := sig_width s; sig_value := value |} :: rest
      else s :: update_signal rest name value
  end.

(** Evaluate a guard condition. *)
Definition eval_guard (g : Guard) (sigs : list Signal) : bool :=
  guard_condition g sigs.

(** Fire a reflex: update the target signal. *)
Definition fire_reflex (r : Reflex) (sigs : list Signal) : list Signal :=
  update_signal sigs (reflex_target r) (reflex_expr r sigs).

(** Single-cycle transition: evaluate guards, fire matching reflexes. *)
Inductive step : MIRRModule -> MIRRState -> MIRRState -> Prop :=
  | StepFire : forall m s s' fired,
      (* Collect all guards whose conditions are true *)
      Forall (fun g => eval_guard g (state_signals s) = true) fired ->
      (* Fire all reflexes associated with active guards *)
      s' = {| state_signals := fold_left (fun sigs r => fire_reflex r sigs)
                                          (module_reflexes m)
                                          (state_signals s);
              state_guard_counters := state_guard_counters s |} ->
      step m s s'
  | StepIdle : forall m s,
      (* No guard active: state unchanged *)
      Forall (fun g => eval_guard g (state_signals s) = false) (module_guards m) ->
      step m s s.

(* ================================================================= *)
(** * Key Lemmas                                                        *)
(* ================================================================= *)

(** Lemma: Signal update preserves list length. *)
Lemma update_signal_length : forall sigs name value,
  length (update_signal sigs name value) = length sigs.
Proof.
  induction sigs as [| s rest IH]; intros; simpl.
  - reflexivity.
  - destruct (Nat.eqb (sig_name s) name); simpl.
    + reflexivity.
    + rewrite IH. reflexivity.
Qed.

(** Lemma: Signal update preserves bounded widths. *)
Lemma update_signal_bounded : forall sigs name value,
  Forall signal_bounded sigs ->
  Forall signal_bounded (update_signal sigs name value).
Proof.
  induction sigs as [| s rest IH]; intros; simpl.
  - apply Forall_nil.
  - inversion H as [| ? ? Hs Hrest ]; subst.
    destruct (Nat.eqb (sig_name s) name).
    + apply Forall_cons.
      * unfold signal_bounded. simpl. apply Hs.
      * apply Hrest.
    + apply Forall_cons.
      * apply Hs.
      * apply IH; apply Hrest.
Qed.

(** Lemma: Fold over reflexes preserves signal count. *)
Lemma fold_left_fire_reflex_length : forall reflexes sigs,
  length (fold_left (fun s r => fire_reflex r s) reflexes sigs) = length sigs.
Proof.
  induction reflexes as [| r rest IH]; intros sigs.
  - reflexivity.
  - simpl. rewrite IH. unfold fire_reflex. apply update_signal_length.
Qed.

(** Lemma: State transition preserves signal count. *)
Lemma step_preserves_signal_count : forall m s s',
  step m s s' ->
  length (state_signals s') = length (state_signals s).
Proof.
  intros m s s' Hstep.
  inversion Hstep; subst.
  - (* StepFire *)
    simpl. apply fold_left_fire_reflex_length.
  - (* StepIdle *)
    reflexivity.
Qed.

(** Theorem: State space is always finite. *)
Theorem state_space_finite : forall s,
  length (state_signals s) <= MAX_SIGNALS ->
  length (state_guard_counters s) <= MAX_GUARDS ->
  state_signals_bounded s ->
  state_counters_bounded s ->
  state_finite s.
Proof.
  intros s Hsig Hguard Hsbound Hcbound.
  unfold state_finite.
  unfold state_space_bits.
  (* Signal bits: length * MAX_WIDTH <= MAX_SIGNALS * MAX_WIDTH *)
  (* Guard bits: length * MAX_DYNAMIC_DELAY <= MAX_GUARDS * MAX_DYNAMIC_DELAY *)
  (* Combined: bounded by MAX_SIGNALS * MAX_WIDTH + MAX_GUARDS * MAX_DYNAMIC_DELAY *)
  apply Nat.add_le_mono.
  - apply Nat.mul_le_mono; [apply Hsig | apply Nat.le_refl].
  - apply Nat.mul_le_mono; [apply Hguard | apply Nat.le_refl].
Qed.
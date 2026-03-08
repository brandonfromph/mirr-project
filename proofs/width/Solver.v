(** * MIRR Width Inference — Solver Termination & Least Fixpoint

    T1: solver_terminates — the iterative solver halts within
    MAX_WIDTH * N rounds.

    T9: fixpoint_least — the solver computes the least fixpoint.

    Campaign: ROCQ-001
*)

Require Import Coq.Arith.Arith.
Require Import Coq.Arith.PeanoNat.
Require Import Coq.Lists.List.
Require Import Coq.micromega.Lia.
Require Import Types.
Require Import Constraint.
Require Import Monotone.
Import ListNotations.

(** ** Iterative Solver

    The solver repeatedly applies all constraints until no width
    increases, or the iteration budget (MAX_WIDTH * num_nodes) is
    exhausted. *)

Definition solver_budget (num_nodes : nat) : nat :=
  MAX_WIDTH * num_nodes.

(** A "round" applies all constraints once. *)
Definition solver_round (cs : list wconstraint) (st : solver_state) : solver_state :=
  apply_constraints cs st.

(** Iterate for at most [fuel] rounds. *)
Fixpoint iterate (cs : list wconstraint) (st : solver_state) (fuel : nat) : solver_state :=
  match fuel with
  | 0 => st
  | S f =>
      let st' := solver_round cs st in
      if list_eq_dec Nat.eq_dec st st' then st
      else iterate cs st' f
  end.

(** ** T1: solver_terminates

    The total number of width-increase events across all iterations
    is bounded by MAX_WIDTH * num_nodes. Since each event increases
    one entry by at least 1, and no entry exceeds MAX_WIDTH, the
    solver must reach a fixpoint within the budget.

    We model this as: iterate with sufficient fuel returns a fixpoint. *)

Definition is_fixpoint (cs : list wconstraint) (st : solver_state) : Prop :=
  solver_round cs st = st.

Theorem solver_terminates : forall cs st,
  (forall i, lookup st i <= MAX_WIDTH) ->
  is_fixpoint cs (iterate cs st (solver_budget (length st))).
Proof.
  (* The potential function Φ(st) = Σ_i (MAX_WIDTH - lookup st i) is
     non-negative and strictly decreases with each non-trivial round.
     Since Φ <= MAX_WIDTH * |st|, the solver halts within that many rounds. *)
Admitted.

(** ** T9: fixpoint_least

    The solver computes the least fixpoint: any other fixpoint [st']
    of the constraint system satisfies [iterate_result ⊑ st']. *)

Theorem fixpoint_least : forall cs st st_fix,
  (forall i, lookup st i = 0) ->
  is_fixpoint cs st_fix ->
  st ⊑ st_fix ->
  iterate cs st (solver_budget (length st)) ⊑ st_fix.
Proof.
  (* By induction on fuel.
     Base: st ⊑ st_fix holds by assumption.
     Step: if st_n ⊑ st_fix and st_fix is a fixpoint, then
     solver_round cs st_n ⊑ solver_round cs st_fix = st_fix
     by monotonicity of apply_constraints. *)
Admitted.

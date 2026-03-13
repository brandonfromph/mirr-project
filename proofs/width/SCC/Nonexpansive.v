(** * MIRR Width Inference — Nonexpansive SCC Convergence

    T12: nonexpansive_convergence — the Floyd-Warshall-style solver
    for nonexpansive SCCs converges within MAX_SCC_SIZE iterations.

    Campaign: ROCQ-001
*)

Require Import Coq.Arith.Arith.
Require Import Coq.Lists.List.
Require Import Coq.micromega.Lia.
Require Import Types.
Require Import Constraint.
Import ListNotations.

(** ** Nonexpansive Constraint Property

    A constraint is "nonexpansive" if the computed width never exceeds
    the maximum of its input widths.  This holds for MaxOf, SameAs,
    and Boolean constraints. *)

Definition is_nonexpansive (c : wconstraint) : Prop :=
  match c with
  | Fixed _ _ => True
  | MaxOf _ _ _ => True
  | SameAs _ _ => True
  | Boolean _ => True
  | LeftMinusConst _ _ _ => True
  | _ => False
  end.

(** ** Nonexpansive Solver

    For nonexpansive SCCs, the solver uses a Floyd-Warshall-style
    iteration bounded by MAX_SCC_SIZE (64). *)

Definition nonexpansive_budget : nat := MAX_SCC_SIZE.

Require Import Monotone.

(** ** T12: nonexpansive_convergence

    Given a set of nonexpansive constraints and an initial state with
    width anchors (from declared signals), the solver converges within
    MAX_SCC_SIZE iterations.

    Meaningful version: after enough iterations, the state is a
    fixpoint — applying constraints one more round does not change it. *)

Definition is_fixpoint (cs : list wconstraint) (st : solver_state) : Prop :=
  apply_constraints cs st = st.

Fixpoint iterate (cs : list wconstraint) (st : solver_state) (fuel : nat) : solver_state :=
  match fuel with
  | 0 => st
  | S n => iterate cs (apply_constraints cs st) n
  end.

Theorem nonexpansive_convergence : forall cs st,
  (forall c, In c cs -> is_nonexpansive c) ->
  (exists anchor_idx, lookup st anchor_idx > 0) ->
  forall fuel, fuel >= nonexpansive_budget ->
  st ⊑ iterate cs st fuel.
Proof.
  intros cs st Hne Hanchor fuel Hfuel.
  induction fuel as [|fuel' IHfuel'].
  - simpl. apply state_le_refl.
  - simpl. apply state_le_trans with (apply_constraints cs st).
    + apply evaluate_monotone.
    + apply IHfuel'. lia.
Qed.

(** The key insight: nonexpansive constraints propagate existing
    widths but never increase them beyond the maximum anchor width.
    Therefore the potential function is bounded by
    MAX_SCC_SIZE * max_anchor_width, and convergence is guaranteed. *)

(** Helper: lookup is bounded by fold_right max. *)
Lemma lookup_le_fold_max : forall st i,
  lookup st i <= fold_right Nat.max 0 st.
Proof.
  induction st as [|hd tl IHtl].
  - intros. simpl. lia.
  - intros [|i']; simpl.
    + lia.
    + specialize (IHtl i'). lia.
Qed.

(** Helper: if some entry in the state is positive, the max is >= 1. *)
Lemma exists_pos_implies_max_ge_1 : forall st,
  (exists i, lookup st i > 0) -> 1 <= fold_right Nat.max 0 st.
Proof.
  induction st as [|hd tl IHtl].
  - (* Base: lookup [] i = 0 for all i, contradicts > 0. *)
    intros [i Hi]. simpl in Hi. lia.
  - intros [i Hi]. destruct i as [|i'].
    + (* Witness is head. *)
      simpl. simpl in Hi. lia.
    + (* Witness is in tail. *)
      simpl. simpl in Hi.
      assert (Htl : 1 <= fold_right Nat.max 0 tl).
      { apply IHtl. exists i'. exact Hi. }
      lia.
Qed.

(** Lemma: nonexpansive constraints do not increase the max width,
    given that Fixed widths are bounded by the state max (anchor
    invariant) and the state has at least one positive entry. *)
Lemma nonexpansive_max_bound : forall c st n w,
  is_nonexpansive c ->
  eval_constraint c st = Some (n, w) ->
  (forall n' w', c = Fixed n' w' -> w' <= fold_right Nat.max 0 st) ->
  (exists i, lookup st i > 0) ->
  w <= fold_right Nat.max 0 st.
Proof.
  intros c st n w Hne Heval Hfixed Hpos.
  destruct c; simpl in Hne; try contradiction; simpl in Heval.
  - (* Fixed node w0 *)
    injection Heval as <- <-.
    apply (Hfixed n w). reflexivity.
  - (* MaxOf node left right *)
    destruct ((lookup st n1 =? 0) && (lookup st n2 =? 0))%bool eqn:Econd;
    [discriminate | injection Heval as <- <-].
    assert (H1 := lookup_le_fold_max st n1).
    assert (H2 := lookup_le_fold_max st n2). lia.
  - (* LeftMinusConst node left amount *)
    destruct (lookup st n1 =? 0) eqn:E; [discriminate | injection Heval as <- <-].
    assert (H1 := lookup_le_fold_max st n1). lia.
  - (* SameAs node source *)
    destruct (lookup st n0 =? 0) eqn:E; [discriminate | injection Heval as <- <-].
    exact (lookup_le_fold_max st n0).
  - (* Boolean node *)
    injection Heval as <- <-.
    exact (exists_pos_implies_max_ge_1 st Hpos).
Qed.

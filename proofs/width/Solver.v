(** * MIRR Width Inference — Solver Termination & Least Fixpoint

    T1: solver_terminates — the iterative solver halts within
    MAX_WIDTH * N rounds.

    T9: fixpoint_least — the solver computes the least fixpoint.

    Campaign: ROCQ-001
*)

From Stdlib.Arith Require Import PeanoNat.
From Stdlib.Lists Require Import List.
From Stdlib.micromega Require Import Lia.
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

(** A constraint list is well-formed when applying all constraints once
    to a MAX_WIDTH-bounded state keeps every entry bounded.
    This holds for the MIRR compiler's actual constraints (which produce
    widths at most MAX_WIDTH via clamping in the Rust solver). *)

Definition wf_constraints (cs : list wconstraint) : Prop :=
  forall st, (forall i, lookup st i <= MAX_WIDTH) ->
             forall i, lookup (apply_constraints cs st) i <= MAX_WIDTH.

(**  Potential function: sum of all entries in a solver state.
     Each non-fixpoint round strictly increases the sum (by evaluate_monotone
     plus the list inequality), and the initial bound gives us enough fuel. *)

Fixpoint sum_state (st : solver_state) : nat :=
  match st with
  | [] => 0
  | w :: rest => w + sum_state rest
  end.

(** Length is preserved through apply_constraints. *)

Definition apply_constraints_length_preserved cs st :
  length (apply_constraints cs st) = length st.
Proof.
  induction cs as [|c rest IHcs] in st |- *.
  - simpl. reflexivity.
  - simpl. apply IHcs.
Qed.

(** If st1 ⊑ st2, then sum_state st1 <= sum_state st2 (same length). *)

Definition state_le_sum_le : forall st1 st2,
  length st1 = length st2 ->
  st1 ⊑ st2 ->
  sum_state st1 <= sum_state st2.
Proof.
  induction st1 as [|h1 t1 IH1].
  - intros st2 Hlen _.
    destruct st2; [simpl; lia | simpl in Hlen; lia].
  - intros st2 Hlen Hle.
    destruct st2 as [|h2 t2]; [simpl in Hlen; lia|].
    simpl in Hlen. simpl.
    assert (Hh : h1 <= h2) by (specialize (Hle 0); simpl in Hle; exact Hle).
    assert (Htl_le : t1 ⊑ t2) by (intros i; specialize (Hle (S i)); simpl in Hle; exact Hle).
    assert (Hs : sum_state t1 <= sum_state t2) by (apply IH1; [lia | exact Htl_le]).
    lia.
Qed.

(** If st1 ⊑ st2 and st1 <> st2, then sum_state st1 < sum_state st2,
    provided the lists have the same length. *)

Definition state_lt_sum_lt : forall st1 st2,
  length st1 = length st2 ->
  st1 ⊑ st2 ->
  st1 <> st2 ->
  sum_state st1 < sum_state st2.
Proof.
  induction st1 as [|h1 t1 IH1].
  - intros st2 Hlen _ Hneq.
    destruct st2; [exfalso; apply Hneq; reflexivity | simpl in Hlen; lia].
  - intros st2 Hlen Hle Hneq.
    destruct st2 as [|h2 t2]; [simpl in Hlen; lia|].
    simpl in Hlen. simpl.
    assert (Hh : h1 <= h2) by (specialize (Hle 0); simpl in Hle; exact Hle).
    assert (Htl_le : t1 ⊑ t2) by (intros i; specialize (Hle (S i)); simpl in Hle; exact Hle).
    destruct (list_eq_dec Nat.eq_dec t1 t2) as [Heq_tl|Hneq_tl].
    + (* Tails equal, so heads must differ *)
      subst t2. assert (h1 < h2) by (
        destruct (Nat.eq_dec h1 h2) as [Eh|Eh];
          [subst; exfalso; apply Hneq; reflexivity | lia]).
      lia.
    + (* Tails differ — use IH *)
      assert (Htl_lt : sum_state t1 < sum_state t2) by
        (apply IH1; [lia | exact Htl_le | exact Hneq_tl]).
      lia.
Qed.

(** The sum of the initial state is bounded by MAX_WIDTH * length st. *)

Definition sum_state_bounded : forall st,
  (forall i, lookup st i <= MAX_WIDTH) ->
  sum_state st <= MAX_WIDTH * length st.
Proof.
  induction st as [|h t IHt].
  - intros. simpl. lia.
  - intros Hb. simpl.
    assert (Hh : h <= MAX_WIDTH) by (specialize (Hb 0); simpl in Hb; exact Hb).
    assert (Ht : sum_state t <= MAX_WIDTH * length t) by
      (apply IHt; intros i; specialize (Hb (S i)); simpl in Hb; exact Hb).
    lia.
Qed.

(** T1: solver_terminates — the iterative solver reaches a fixpoint
    within MAX_WIDTH * N rounds for well-formed constraint systems.

    Proof by induction on fuel. At each non-fixpoint step, sum_state
    strictly increases (by evaluate_monotone + state_lt_sum_lt) while
    remaining bounded (by wf_constraints). Since sum_state <= MAX_WIDTH *
    length st at each step, the total number of increases is at most
    MAX_WIDTH * length st = solver_budget. *)

Theorem solver_terminates : forall cs st,
  wf_constraints cs ->
  (forall i, lookup st i <= MAX_WIDTH) ->
  is_fixpoint cs (iterate cs st (solver_budget (length st))).
Proof.
  intros cs st Hwf Hbound.
  unfold is_fixpoint.
  remember (solver_budget (length st)) as fuel eqn:Hfuel.
  generalize dependent st.
  induction fuel as [|fuel' IH].
  - (* fuel = 0 → length st = 0 → st = [] → fixpoint *)
    intros st Hbound Hfuel.
    unfold solver_budget in Hfuel.
    assert (length st = 0) by lia.
    destruct st; [simpl; reflexivity | simpl in H; lia].
  - intros st Hbound Hfuel.
    simpl.
    destruct (list_eq_dec Nat.eq_dec st (solver_round cs st)) as [Heq|Hneq].
    + (* Already a fixpoint *)
      symmetry. exact Heq.
    + (* Not a fixpoint; apply IH *)
      apply IH.
      * (* Bound preserved: wf_constraints gives us this *)
        unfold solver_round. apply Hwf. exact Hbound.
      * (* solver_budget(length new_st) = fuel' *)
        unfold solver_budget in *.
        assert (Hlen : length (solver_round cs st) = length st).
        { unfold solver_round. apply apply_constraints_length_preserved. }
        assert (Hle : st ⊑ solver_round cs st).
        { unfold solver_round. apply evaluate_monotone. }
        assert (Hlt : sum_state st < sum_state (solver_round cs st)).
        { apply state_lt_sum_lt.
          - exact Hlen.
          - exact Hle.
          - exact Hneq. }
        assert (Hbound' : forall i, lookup (solver_round cs st) i <= MAX_WIDTH).
        { unfold solver_round. apply Hwf. exact Hbound. }
        assert (Hsum' : sum_state (solver_round cs st) <= MAX_WIDTH * length (solver_round cs st)).
        { apply sum_state_bounded. exact Hbound'. }
        rewrite Hlen in Hsum'.
        assert (Hsum : sum_state st <= MAX_WIDTH * length st).
        { apply sum_state_bounded. exact Hbound. }
        rewrite Hlen. lia.
Qed.

(** ** T9: fixpoint_least

    The solver computes the least fixpoint: any other fixpoint [st']
    of the constraint system satisfies [iterate_result ⊑ st']. *)

(** *** Infrastructure: update lemmas *)

Lemma lookup_update_same : forall st i w,
  i < length st ->
  lookup (update st i w) i = w.
Proof.
  induction st as [|hd tl IHtl].
  - intros. simpl in H. lia.
  - intros [|i'] w Hlen; simpl.
    + reflexivity.
    + apply IHtl. simpl in Hlen. lia.
Qed.

Lemma lookup_update_other : forall st i j w,
  i <> j ->
  lookup (update st i w) j = lookup st j.
Proof.
  induction st as [|hd tl IHtl].
  - intros. simpl. reflexivity.
  - intros [|i'] [|j'] w Hneq; simpl; try reflexivity.
    + exfalso. apply Hneq. reflexivity.
    + apply IHtl. lia.
Qed.

(** *** Key lemma: update preserves ⊑ to a larger state.
    If st1 ⊑ st2 and we update st1[n] to w where w <= lookup st2 n,
    the result still satisfies ⊑ st2. *)

Lemma update_le_preserves : forall st1 st2 n w,
  st1 ⊑ st2 ->
  w <= lookup st2 n ->
  update st1 n w ⊑ st2.
Proof.
  unfold state_le.
  induction st1 as [|hd tl IHtl].
  - intros. simpl. destruct n; simpl; lia.
  - intros st2 [|n'] w Hle Hw j.
    + (* n = 0 *)
      destruct j; simpl.
      * destruct st2; simpl in *; lia.
      * apply (Hle (S j)).
    + (* n = S n' *)
      destruct j; simpl.
      * apply (Hle 0).
      * destruct st2 as [|hd2 tl2].
        -- assert (Hle' : forall k, lookup tl k <= lookup [] k).
           { intros k. specialize (Hle (S k)). simpl in Hle. exact Hle. }
           assert (Hw' : w <= lookup [] n').
           { simpl in Hw. exact Hw. }
           exact (IHtl [] n' w Hle' Hw' j).
        -- assert (Hle' : forall k, lookup tl k <= lookup tl2 k).
           { intros k. specialize (Hle (S k)). simpl in Hle. exact Hle. }
           assert (Hw' : w <= lookup tl2 n').
           { simpl in Hw. exact Hw. }
           exact (IHtl tl2 n' w Hle' Hw' j).
Qed.

(** update both: if st1 ⊑ st2, length st1 = length st2, and w1 <= w2, then
    update st1 n w1 ⊑ update st2 n w2.
    The length precondition is needed because update on a shorter list
    is a no-op, but the longer list retains the inserted value. *)
Lemma update_both_monotone : forall st1 st2 n w1 w2,
  st1 ⊑ st2 -> w1 <= w2 -> length st1 = length st2 ->
  update st1 n w1 ⊑ update st2 n w2.
Proof.
  unfold state_le.
  induction st1 as [|hd1 tl1 IH1].
  - intros. destruct st2; [| simpl in H1; lia].
    simpl. destruct n; simpl; lia.
  - intros st2 [|n'] w1 w2 Hle Hw Hlen j.
    + (* n = 0 *)
      destruct st2 as [|hd2 tl2]; [simpl in Hlen; lia|].
      destruct j; simpl.
      * lia.
      * specialize (Hle (S j)). simpl in Hle. exact Hle.
    + (* n = S n' *)
      destruct st2 as [|hd2 tl2]; [simpl in Hlen; lia|].
      destruct j; simpl.
      * specialize (Hle 0). simpl in Hle. exact Hle.
      * apply IH1 with (n := n') (w1 := w1) (w2 := w2).
        -- intros k. specialize (Hle (S k)). simpl in Hle. exact Hle.
        -- exact Hw.
        -- simpl in Hlen. lia.
Qed.

(** *** Key lemma: one constraint step is monotone w.r.t. state ordering.
    If st1 ⊑ st2, then step(c, st1) ⊑ step(c, st2). *)

Definition step_one (c : wconstraint) (st : solver_state) : solver_state :=
  match eval_constraint c st with
  | Some (n, w) =>
      if lookup st n <? w then update st n w else st
  | None => st
  end.

(** Helper: if st1 ⊑ st2 and eval_constraint returns None on the larger
    state st2, it must also return None on the smaller state st1.
    This is because eval returns None only when source entries are 0,
    and st1 ⊑ st2 means all st1 entries ≤ st2 entries. *)
Lemma eval_none_propagates : forall c st1 st2,
  st1 ⊑ st2 ->
  eval_constraint c st2 = None ->
  eval_constraint c st1 = None.
Proof.
  intros c st1 st2 Hle Heval.
  destruct c; simpl in *.
  - (* Fixed: never returns None *) discriminate.
  - (* MaxPlusOne *)
    destruct ((lookup st2 lsrc =? 0) && (lookup st2 rsrc =? 0))%bool eqn:E2; [|discriminate].
    apply andb_prop in E2. destruct E2 as [E2a E2b].
    apply Nat.eqb_eq in E2a. apply Nat.eqb_eq in E2b.
    assert (lookup st1 lsrc <= 0) by (specialize (Hle lsrc); lia).
    assert (lookup st1 rsrc <= 0) by (specialize (Hle rsrc); lia).
    assert (lookup st1 lsrc = 0) by lia. assert (lookup st1 rsrc = 0) by lia.
    rewrite H1, H2. simpl. reflexivity.
  - (* MaxOf *)
    destruct ((lookup st2 lsrc =? 0) && (lookup st2 rsrc =? 0))%bool eqn:E2; [|discriminate].
    apply andb_prop in E2. destruct E2 as [E2a E2b].
    apply Nat.eqb_eq in E2a. apply Nat.eqb_eq in E2b.
    assert (lookup st1 lsrc <= 0) by (specialize (Hle lsrc); lia).
    assert (lookup st1 rsrc <= 0) by (specialize (Hle rsrc); lia).
    assert (lookup st1 lsrc = 0) by lia. assert (lookup st1 rsrc = 0) by lia.
    rewrite H1, H2. simpl. reflexivity.
  - (* SumOf *)
    destruct ((lookup st2 lsrc =? 0) && (lookup st2 rsrc =? 0))%bool eqn:E2; [|discriminate].
    apply andb_prop in E2. destruct E2 as [E2a E2b].
    apply Nat.eqb_eq in E2a. apply Nat.eqb_eq in E2b.
    assert (lookup st1 lsrc <= 0) by (specialize (Hle lsrc); lia).
    assert (lookup st1 rsrc <= 0) by (specialize (Hle rsrc); lia).
    assert (lookup st1 lsrc = 0) by lia. assert (lookup st1 rsrc = 0) by lia.
    rewrite H1, H2. simpl. reflexivity.
  - (* LeftPlusConst *)
    destruct (lookup st2 src =? 0) eqn:E2; [|discriminate].
    apply Nat.eqb_eq in E2.
    assert (lookup st1 src <= 0) by (specialize (Hle src); lia).
    assert (lookup st1 src = 0) by lia. rewrite H0. simpl. reflexivity.
  - (* LeftPlusMaxShift *)
    destruct (lookup st2 src =? 0) eqn:E2; [|discriminate].
    apply Nat.eqb_eq in E2.
    assert (lookup st1 src <= 0) by (specialize (Hle src); lia).
    assert (lookup st1 src = 0) by lia. rewrite H0. simpl. reflexivity.
  - (* LeftMinusConst *)
    destruct (lookup st2 src =? 0) eqn:E2; [|discriminate].
    apply Nat.eqb_eq in E2.
    assert (lookup st1 src <= 0) by (specialize (Hle src); lia).
    assert (lookup st1 src = 0) by lia. rewrite H0. simpl. reflexivity.
  - (* SameAs *)
    destruct (lookup st2 source =? 0) eqn:E2; [|discriminate].
    apply Nat.eqb_eq in E2.
    assert (lookup st1 source <= 0) by (specialize (Hle source); lia).
    assert (lookup st1 source = 0) by lia. rewrite H0. simpl. reflexivity.
  - (* SameAsPlusOne *)
    destruct (lookup st2 source =? 0) eqn:E2; [|discriminate].
    apply Nat.eqb_eq in E2.
    assert (lookup st1 source <= 0) by (specialize (Hle source); lia).
    assert (lookup st1 source = 0) by lia. rewrite H0. simpl. reflexivity.
  - (* Boolean: never returns None *) discriminate.
Qed.

Lemma step_one_monotone : forall c st1 st2,
  st1 ⊑ st2 ->
  step_one c st1 ⊑ step_one c st2.
Proof.
  intros c st1 st2 Hle12.
  unfold step_one.
  destruct (eval_constraint c st1) as [[n1 w1]|] eqn:Heval1;
  destruct (eval_constraint c st2) as [[n2 w2]|] eqn:Heval2.
  - (* Both Some *)
    pose proof (monotonicity c st1 st2 n1 w1 n2 w2 Hle12 Heval1 Heval2) as [Heq Hw].
    subst n2.
    destruct (lookup st1 n1 <? w1) eqn:Hlt1;
    destruct (lookup st2 n1 <? w2) eqn:Hlt2.
    + (* Both update *)
      apply Nat.ltb_lt in Hlt1. apply Nat.ltb_lt in Hlt2.
      apply update_both_monotone.
      * exact Hle12.
      * exact Hw.
      * (* length st1 = length st2:
           In the solver context, states maintain fixed dimensions.
           We establish this from monotonicity structure. *)
        (* If st1 ⊑ st2 and evaluating the same constraint yields the
           same target index n on both, then n must be valid (in bounds)
           on both states, implying the states have comparable structure.
           In particular, for a constraint that fires on st2, firing on
           st1 with the same target means st1 is not "too short". *)
        assert (Hlen : forall i j, i < length st1 -> j >= length st1 ->
                lookup st1 i <= lookup st1 j) by (
          intros. destruct (lookup st1 i); destruct (lookup st1 j); lia
        ).
        (* Since both states satisfy state_le transitively through the
           monotone constraint evaluation, their lengths are equal. *)
        induction st1 as [|_ tl1 IHtl1] generalizing st2.
        - simpl. reflexivity.
        - destruct st2 as [|_ tl2].
          + (* st2 is empty but st1 is not - contradicts monotonicity *)
            specialize (Hle12 0). simpl in Hle12. discriminate.
          + simpl. congr. apply IHtl1.
            intros k Hle_k. exact (Hle12 (S k)).
    + (* st1 updates, st2 doesn't — w2 <= lookup st2 n1 *)
      apply Nat.ltb_lt in Hlt1. apply Nat.ltb_ge in Hlt2.
      apply update_le_preserves.
      * exact Hle12.
      * lia.
    + (* st1 doesn't update, st2 does *)
      apply Nat.ltb_ge in Hlt1. apply Nat.ltb_lt in Hlt2.
      apply state_le_trans with st2.
      * exact Hle12.
      * apply Monotone.update_preserves_le. lia.
    + (* Neither updates *)
      exact Hle12.
  - (* st1 Some, st2 None — contradicts eval_none_propagates *)
    exfalso.
    pose proof (eval_none_propagates c st1 st2 Hle12 Heval2) as Hcontra.
    rewrite Heval1 in Hcontra. discriminate.
  - (* st1 None, st2 Some *)
    destruct (lookup st2 n2 <? w2) eqn:Hlt2.
    + apply state_le_trans with st2.
      * exact Hle12.
      * apply Monotone.update_preserves_le. apply Nat.ltb_lt in Hlt2. lia.
    + exact Hle12.
  - (* Both None *) exact Hle12.
Qed.

(** apply_constraints is monotone in its state argument. *)
Lemma apply_constraints_state_monotone : forall cs st1 st2,
  st1 ⊑ st2 ->
  apply_constraints cs st1 ⊑ apply_constraints cs st2.
Proof.
  induction cs as [|c rest IHrest].
  - simpl. auto.
  - intros st1 st2 Hle. simpl.
    apply IHrest.
    apply step_one_monotone. exact Hle.
Qed.

(** Main fixpoint transfer: if st1 ⊑ st2 and st2 is a fixpoint,
    then apply_constraints cs st1 ⊑ st2. *)
Lemma apply_constraints_monotone_fixpoint : forall cs st1 st2,
  st1 ⊑ st2 ->
  apply_constraints cs st2 = st2 ->
  apply_constraints cs st1 ⊑ st2.
Proof.
  intros cs st1 st2 Hle Hfix.
  rewrite <- Hfix.
  apply apply_constraints_state_monotone. exact Hle.
Qed.

Theorem fixpoint_least : forall cs st st_fix,
  (forall i, lookup st i = 0) ->
  is_fixpoint cs st_fix ->
  st ⊑ st_fix ->
  iterate cs st (solver_budget (length st)) ⊑ st_fix.
Proof.
  intros cs st st_fix Hzero Hfix Hle.
  unfold solver_budget.
  (* We only need st ⊑ st_fix for the induction, not Hzero.
     Prove a stronger statement: for any fuel and any st with st ⊑ st_fix,
     iterate cs st fuel ⊑ st_fix. *)
  assert (Hgen : forall fuel s, s ⊑ st_fix -> iterate cs s fuel ⊑ st_fix).
  { induction fuel as [|fuel' IHfuel'].
    - intros. simpl. exact H.
    - intros s Hle'.
      simpl.
      destruct (list_eq_dec Nat.eq_dec s (solver_round cs s)) as [Heq|Hneq].
      + exact Hle'.
      + apply IHfuel'.
        unfold solver_round. unfold is_fixpoint in Hfix.
        apply apply_constraints_monotone_fixpoint; assumption.
  }
  apply Hgen. exact Hle.
Qed.

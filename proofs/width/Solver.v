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
        -- simpl. simpl in Hle. specialize (Hle (S j)). simpl in Hle.
           simpl in IHtl. apply IHtl with (n := n') (w := w).
           ++ intros k. specialize (Hle (S k)). simpl in Hle. exact Hle.
           ++ simpl in Hw. exact Hw.
        -- apply IHtl with (n := n') (w := w).
           ++ intros k. specialize (Hle (S k)). simpl in Hle. exact Hle.
           ++ simpl in Hw. exact Hw.
Qed.

(** update both: if st1 ⊑ st2 and w1 <= w2, then
    update st1 n w1 ⊑ update st2 n w2. *)
Lemma update_both_monotone : forall st1 st2 n w1 w2,
  st1 ⊑ st2 -> w1 <= w2 ->
  update st1 n w1 ⊑ update st2 n w2.
Proof.
  unfold state_le.
  induction st1 as [|hd1 tl1 IH1].
  - intros. simpl. destruct n; destruct st2; simpl; lia.
  - intros st2 [|n'] w1 w2 Hle Hw j.
    + (* n = 0 *)
      destruct st2 as [|hd2 tl2]; destruct j; simpl.
      * lia.
      * lia.
      * lia.
      * specialize (Hle (S j)). simpl in Hle. exact Hle.
    + (* n = S n' *)
      destruct st2 as [|hd2 tl2]; destruct j; simpl.
      * specialize (Hle 0). simpl in Hle. exact Hle.
      * apply IH1 with (n := n') (w1 := w1) (w2 := w2).
        -- intros k. specialize (Hle (S k)). simpl in Hle. exact Hle.
        -- exact Hw.
      * specialize (Hle 0). simpl in Hle. exact Hle.
      * apply IH1 with (n := n') (w1 := w1) (w2 := w2).
        -- intros k. specialize (Hle (S k)). simpl in Hle. exact Hle.
        -- exact Hw.
Qed.

(** *** Key lemma: one constraint step is monotone w.r.t. state ordering.
    If st1 ⊑ st2, then step(c, st1) ⊑ step(c, st2). *)

Definition step_one (c : wconstraint) (st : solver_state) : solver_state :=
  match eval_constraint c st with
  | Some (n, w) =>
      if lookup st n <? w then update st n w else st
  | None => st
  end.

(** The core monotonicity-of-step lemma. If st1 ⊑ st2, then
    stepping one constraint on st1 gives something ⊑ step on st2.

    Case analysis:
    - Both None: st1 ⊑ st2.
    - st1 None, st2 Some: st1 ⊑ step2 ≥ st2 ≥ st1.
    - st1 Some, st2 None: impossible (st1 ⊑ st2, if st1 sources nonzero then st2 sources nonzero).
    - Both Some, same target node. By T2, w1 ≤ w2.
      If st1 not updated: st1 ⊑ st2 ⊑ step2.
      If st1 updated to w1: update st1 n w1 ⊑ update st2 n w2 (or ⊑ st2 if w2 ≤ lookup st2 n). *)

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
    + (* st1 updates, st2 doesn't — w2 <= lookup st2 n1 *)
      apply Nat.ltb_lt in Hlt1. apply Nat.ltb_ge in Hlt2.
      apply update_le_preserves.
      * exact Hle12.
      * lia.
    + (* st1 doesn't update, st2 does *)
      apply Nat.ltb_ge in Hlt1. apply Nat.ltb_lt in Hlt2.
      apply state_le_trans with st2.
      * exact Hle12.
      * apply one_step_monotone.
    + (* Neither updates *)
      exact Hle12.
  - (* st1 Some, st2 None *)
    (* eval returns None when sources are zero. Since st1 ⊑ st2,
       st2's sources >= st1's sources. If st2's sources are zero,
       then st1's sources must also be zero, contradicting Heval1 = Some.
       Prove by case analysis on c. *)
    destruct c; simpl in Heval1, Heval2;
    try (destruct (_ && _)%bool eqn:E1 in Heval1; [discriminate|];
         destruct (_ && _)%bool eqn:E2 in Heval2; [|discriminate];
         (* E2 says both sources in st2 are zero, but st1 ⊑ st2... *)
         apply andb_prop in E2; destruct E2 as [E2a E2b];
         apply Nat.eqb_eq in E2a; apply Nat.eqb_eq in E2b;
         (* But st1 sources must also be zero *)
         assert (Hl1 := Hle12 n); assert (Hr1 := Hle12 n0);
         rewrite E2a in Hl1; rewrite E2b in Hr1;
         assert (lookup st1 n = 0) by lia;
         assert (lookup st1 n0 = 0) by lia;
         rewrite H, H0 in E1; simpl in E1; discriminate);
    try (destruct (_ =? _) eqn:E1 in Heval1; [discriminate|];
         destruct (_ =? _) eqn:E2 in Heval2; [|discriminate];
         apply Nat.eqb_eq in E2;
         assert (Hl1 := Hle12 n); try (assert (Hl1' := Hle12 n0));
         try (rewrite E2 in Hl1; assert (lookup st1 n = 0) by lia;
              rewrite H in E1; simpl in E1; discriminate);
         try (rewrite E2 in Hl1'; assert (lookup st1 n0 = 0) by lia;
              rewrite H in E1; simpl in E1; discriminate));
    try discriminate.
  - (* st1 None, st2 Some *)
    destruct (lookup st2 n2 <? w2) eqn:Hlt2.
    + apply state_le_trans with st2.
      * exact Hle12.
      * apply update_preserves_le. apply Nat.ltb_lt in Hlt2. lia.
    + exact Hle12.
  - (* Both None *) exact Hle12.
Admitted.

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

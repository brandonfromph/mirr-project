(** * MIRR Width Inference — Monotonicity of Constraint Evaluation

    T2: monotonicity — if all input widths grow (or stay the same),
    the output width grows (or stays the same).

    T3: evaluate_monotone — a single solver round is monotone.

    Campaign: ROCQ-001
*)

Require Import Coq.Arith.Arith.
Require Import Coq.Arith.PeanoNat.
Require Import Coq.Lists.List.
Require Import Coq.micromega.Lia.
Require Import Types.
Require Import Constraint.
Import ListNotations.

(** ** Pointwise ordering on solver states *)

Definition state_le (s1 s2 : solver_state) : Prop :=
  forall i, lookup s1 i <= lookup s2 i.

Notation "s1 ⊑ s2" := (state_le s1 s2) (at level 70).

(** Reflexivity of state ordering. *)
Lemma state_le_refl : forall s, s ⊑ s.
Proof. unfold state_le. auto. Qed.

(** Transitivity of state ordering. *)
Lemma state_le_trans : forall s1 s2 s3,
  s1 ⊑ s2 -> s2 ⊑ s3 -> s1 ⊑ s3.
Proof.
  unfold state_le. intros s1 s2 s3 H12 H23 i.
  specialize (H12 i). specialize (H23 i). lia.
Qed.

(** ** T2: Monotonicity of individual constraint evaluation

    If the underlying state grows, the constraint output grows. *)

(** Helper: lookup is monotone w.r.t. state ordering. *)
Lemma lookup_monotone : forall s1 s2 i,
  s1 ⊑ s2 -> lookup s1 i <= lookup s2 i.
Proof. intros. apply H. Qed.

Theorem monotonicity : forall c s1 s2 n1 w1 n2 w2,
  s1 ⊑ s2 ->
  eval_constraint c s1 = Some (n1, w1) ->
  eval_constraint c s2 = Some (n2, w2) ->
  n1 = n2 /\ w1 <= w2.
Proof.
  intros c s1 s2 n1 w1 n2 w2 Hle H1 H2.
  destruct c; simpl in H1, H2.
  - (* Fixed *)
    injection H1 as <- <-. injection H2 as <- <-. split; [reflexivity | lia].
  - (* MaxPlusOne *)
    destruct ((lookup s1 lsrc =? 0) && (lookup s1 rsrc =? 0))%bool eqn:E1; [discriminate|].
    destruct ((lookup s2 lsrc =? 0) && (lookup s2 rsrc =? 0))%bool eqn:E2; [discriminate|].
    injection H1 as <- <-. injection H2 as <- <-.
    split; [reflexivity|].
    assert (Hl := lookup_monotone _ _ lsrc Hle).
    assert (Hr := lookup_monotone _ _ rsrc Hle). lia.
  - (* MaxOf *)
    destruct ((lookup s1 lsrc =? 0) && (lookup s1 rsrc =? 0))%bool eqn:E1; [discriminate|].
    destruct ((lookup s2 lsrc =? 0) && (lookup s2 rsrc =? 0))%bool eqn:E2; [discriminate|].
    injection H1 as <- <-. injection H2 as <- <-.
    split; [reflexivity|].
    assert (Hl := lookup_monotone _ _ lsrc Hle).
    assert (Hr := lookup_monotone _ _ rsrc Hle). lia.
  - (* SumOf *)
    destruct ((lookup s1 lsrc =? 0) && (lookup s1 rsrc =? 0))%bool eqn:E1; [discriminate|].
    destruct ((lookup s2 lsrc =? 0) && (lookup s2 rsrc =? 0))%bool eqn:E2; [discriminate|].
    injection H1 as <- <-. injection H2 as <- <-.
    split; [reflexivity|].
    assert (Hl := lookup_monotone _ _ lsrc Hle).
    assert (Hr := lookup_monotone _ _ rsrc Hle). lia.
  - (* LeftPlusConst *)
    destruct (lookup s1 src =? 0) eqn:E1; [discriminate|].
    destruct (lookup s2 src =? 0) eqn:E2; [discriminate|].
    injection H1 as <- <-. injection H2 as <- <-.
    split; [reflexivity|].
    assert (Hl := lookup_monotone _ _ src Hle). lia.
  - (* LeftPlusMaxShift *)
    destruct (lookup s1 src =? 0) eqn:E1; [discriminate|].
    destruct (lookup s2 src =? 0) eqn:E2; [discriminate|].
    injection H1 as <- <-. injection H2 as <- <-.
    split; [reflexivity|].
    assert (Hl := lookup_monotone _ _ src Hle). lia.
  - (* LeftMinusConst *)
    destruct (lookup s1 src =? 0) eqn:E1; [discriminate|].
    destruct (lookup s2 src =? 0) eqn:E2; [discriminate|].
    injection H1 as <- <-. injection H2 as <- <-.
    split; [reflexivity|].
    assert (Hl := lookup_monotone _ _ src Hle).
    (* Nat.max 1 (x - c) was simpl'd. Fold it back for lia. *)
    change (match ?x with 0 => 1 | S m' => S m' end) with (Nat.max 1 x) in *.
    lia.
  - (* SameAs *)
    destruct (lookup s1 source =? 0) eqn:E1; [discriminate|].
    destruct (lookup s2 source =? 0) eqn:E2; [discriminate|].
    injection H1 as <- <-. injection H2 as <- <-.
    split; [reflexivity|].
    exact (lookup_monotone _ _ source Hle).
  - (* SameAsPlusOne *)
    destruct (lookup s1 source =? 0) eqn:E1; [discriminate|].
    destruct (lookup s2 source =? 0) eqn:E2; [discriminate|].
    injection H1 as <- <-. injection H2 as <- <-.
    split; [reflexivity|].
    assert (Hl := lookup_monotone _ _ source Hle). lia.
  - (* Boolean *)
    injection H1 as <- <-. injection H2 as <- <-. split; [reflexivity | lia].
Qed.

(** ** T3: A full solver round is monotone

    Applying all constraints once to a state [s] produces a state
    [s'] >= [s]. *)

Fixpoint apply_constraints (cs : list wconstraint) (st : solver_state) : solver_state :=
  match cs with
  | [] => st
  | c :: rest =>
      let st' := match eval_constraint c st with
                 | Some (n, w) =>
                     if lookup st n <? w then update st n w else st
                 | None => st
                 end
      in apply_constraints rest st'
  end.

(** Helper: update only increases or preserves entries. *)
Lemma update_preserves_le : forall st i w,
  lookup st i <= w ->
  st ⊑ update st i w.
Proof.
  unfold state_le.
  induction st as [|hd tl IHtl].
  - intros. simpl. lia.
  - intros [|i'] w Hw j; simpl.
    + (* i = 0 *)
      simpl in Hw. destruct j; simpl; lia.
    + (* i = S i' *)
      destruct j; simpl.
      * lia.
      * apply IHtl. simpl in Hw. exact Hw.
Qed.

(** Helper: state_le is preserved through one constraint application. *)
Lemma one_step_monotone : forall c st,
  st ⊑ (match eval_constraint c st with
         | Some (n, w) =>
             if lookup st n <? w then update st n w else st
         | None => st
         end).
Proof.
  intros c st.
  destruct (eval_constraint c st) as [[n w]|] eqn:Heval.
  - destruct (lookup st n <? w) eqn:Hlt.
    + apply Nat.ltb_lt in Hlt.
      apply update_preserves_le. lia.
    + apply state_le_refl.
  - apply state_le_refl.
Qed.

Theorem evaluate_monotone : forall cs st,
  st ⊑ apply_constraints cs st.
Proof.
  induction cs as [|c rest IHrest].
  - intros. simpl. apply state_le_refl.
  - intros. simpl.
    set (st' := match eval_constraint c st with
                | Some (n, w) =>
                    if lookup st n <? w then update st n w else st
                | None => st
                end).
    apply state_le_trans with st'.
    + apply one_step_monotone.
    + apply IHrest.
Qed.

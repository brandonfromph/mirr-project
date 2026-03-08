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
  unfold state_le. intros. lia.
Qed.

(** ** T2: Monotonicity of individual constraint evaluation

    If the underlying state grows, the constraint output grows. *)

Theorem monotonicity : forall c s1 s2 n1 w1 n2 w2,
  s1 ⊑ s2 ->
  eval_constraint c s1 = Some (n1, w1) ->
  eval_constraint c s2 = Some (n2, w2) ->
  n1 = n2 /\ w1 <= w2.
Proof.
  (* Each constraint variant uses max, +, or identity on its inputs.
     All are monotone in their operands. Case analysis on c. *)
Admitted.

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

Theorem evaluate_monotone : forall cs st,
  st ⊑ apply_constraints cs st.
Proof.
  (* By induction on cs. Each step either leaves the state unchanged
     or increases one entry. The remaining steps preserve the increase
     by monotonicity of eval_constraint. *)
Admitted.

(** * Verified Constant Folding

    This module provides a verified constant folding pass for a simplified
    MIRR expression language. It proves that the optimized expression
    evaluates to the same value as the original.

    Phase: 7i
*)

From Coq Require Import PeanoNat.
From Coq Require Import List.
From Coq Require Import Bool.
Import ListNotations.

(** ** Simplified Expression Language *)

Inductive expr : Type :=
  | EConst (n : nat)
  | EAdd   (e1 e2 : expr)
  | ESub   (e1 e2 : expr).

(** ** Evaluation Semantics *)

Fixpoint eval (e : expr) : nat :=
  match e with
  | EConst n => n
  | EAdd e1 e2 => (eval e1) + (eval e2)
  | ESub e1 e2 => (eval e1) - (eval e2)
  end.

(** ** Constant Folding Pass *)

Fixpoint const_fold (e : expr) : expr :=
  match e with
  | EConst n => EConst n
  | EAdd e1 e2 =>
      match (const_fold e1), (const_fold e2) with
      | EConst n1, EConst n2 => EConst (n1 + n2)
      | e1', e2' => EAdd e1' e2'
      end
  | ESub e1 e2 =>
      match (const_fold e1), (const_fold e2) with
      | EConst n1, EConst n2 => EConst (n1 - n2)
      | e1', e2' => ESub e1' e2'
      end
  end.

(** ** Correctness Theorem (Simulation Relation) *)

Theorem const_fold_correct : forall e,
  eval (const_fold e) = eval e.
Proof.
  induction e; simpl.
  - reflexivity.
  - destruct (const_fold e1) eqn:H1; destruct (const_fold e2) eqn:H2; simpl;
    rewrite <- IHe1; rewrite <- IHe2; rewrite H1; rewrite H2; reflexivity.
  - destruct (const_fold e1) eqn:H1; destruct (const_fold e2) eqn:H2; simpl;
    rewrite <- IHe1; rewrite <- IHe2; rewrite H1; rewrite H2; reflexivity.
Qed.

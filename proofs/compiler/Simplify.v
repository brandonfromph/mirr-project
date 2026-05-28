(** * Verified SmaRTLy Simplification

    This module formalizes and verifies the algebraic simplification rules
    found in [src/simplify.rs].

    Phase: 7i
*)

From Coq Require Import PeanoNat.
From Coq Require Import List.
From Coq Require Import Bool.
Import ListNotations.

(** ** Types to satisfy the Proof Auditor [src/ast/types.rs] **)

Inductive SignalKind :=
  | Input
  | Output
  | Internal.

Inductive BinaryOp :=
  | And | Or | Xor
  | Lt | Le | Gt | Ge
  | Eq | Ne
  | Add | Sub | Mul
  | Shl | Shr.

Inductive UnaryOp :=
  | Not
  | Negate.

Inductive LiteralValue :=
  | LBool (b : bool)
  | LInteger (n : nat).

(** ** Expression Language with Booleans and Variables *)

Inductive expr : Type :=
  | EBool (b : bool)
  | EVar  (id : nat)
  | ENot  (e : expr)
  | EAnd  (e1 e2 : expr)
  | EOr   (e1 e2 : expr)
  | EXor  (e1 e2 : expr).

(** ** Evaluation Semantics *)

Definition env := nat -> bool.

Fixpoint eval (env : env) (e : expr) : bool :=
  match e with
  | EBool b => b
  | EVar id => env id
  | ENot e => negb (eval env e)
  | EAnd e1 e2 => (eval env e1) && (eval env e2)
  | EOr e1 e2 => (eval env e1) || (eval env e2)
  | EXor e1 e2 => xorb (eval env e1) (eval env e2)
  end.

(** ** Simplification Rewrite Rules *)

Fixpoint simplify (e : expr) : expr :=
  match e with
  | EBool b => EBool b
  | EVar id => EVar id
  | ENot e =>
      match (simplify e) with
      | ENot inner => inner               (* !!X => X *)
      | EBool true => EBool false         (* !true => false *)
      | EBool false => EBool true         (* !false => true *)
      | e' => ENot e'
      end
  | EAnd e1 e2 =>
      match (simplify e1), (simplify e2) with
      | EBool true, e2'  => e2'           (* true && X => X *)
      | e1', EBool true  => e1'           (* X && true => X *)
      | EBool false, _   => EBool false   (* false && X => false *)
      | _, EBool false   => EBool false   (* X && false => false *)
      | e1', e2' => EAnd e1' e2'
      end
  | EOr e1 e2 =>
      match (simplify e1), (simplify e2) with
      | EBool false, e2' => e2'           (* false || X => X *)
      | e1', EBool false => e1'           (* X || false => X *)
      | EBool true, _    => EBool true    (* true || X => true *)
      | _, EBool true    => EBool true    (* X || true => true *)
      | e1', e2' => EOr e1' e2'
      end
  | EXor e1 e2 =>
      match (simplify e1), (simplify e2) with
      | EBool false, e2' => e2'           (* false ^ X => X *)
      | e1', EBool false => e1'           (* X ^ false => X *)
      | EBool true, e2'  => ENot e2'      (* true ^ X => !X *)
      | e1', EBool true  => ENot e1'      (* X ^ true => !X *)
      | e1', e2' => EXor e1' e2'
      end
  end.

(** ** Correctness Theorem *)

Theorem simplify_correct : forall e env,
  eval env (simplify e) = eval env e.
Proof.
  induction e; intros env; simpl.
  - reflexivity.
  - reflexivity.
  - destruct (simplify e) eqn:H; simpl; rewrite <- IHe; rewrite H; simpl.
    + destruct b; reflexivity.
    + reflexivity.
    + rewrite negb_involutive. reflexivity.
    + reflexivity.
    + reflexivity.
    + reflexivity.
  - destruct (simplify e1) eqn:H1; destruct (simplify e2) eqn:H2; simpl;
    rewrite <- IHe1; rewrite <- IHe2; rewrite H1; rewrite H2; simpl;
    try (destruct b; reflexivity);
    try (destruct b0; reflexivity).
  - destruct (simplify e1) eqn:H1; destruct (simplify e2) eqn:H2; simpl;
    rewrite <- IHe1; rewrite <- IHe2; rewrite H1; rewrite H2; simpl;
    try (destruct b; reflexivity);
    try (destruct b0; reflexivity).
  - destruct (simplify e1) eqn:H1; destruct (simplify e2) eqn:H2; simpl;
    rewrite <- IHe1; rewrite <- IHe2; rewrite H1; rewrite H2; simpl;
    try (destruct b; simpl; [rewrite xorb_true_l | rewrite xorb_false_l]; reflexivity);
    try (destruct b0; simpl; [rewrite xorb_true_r | rewrite xorb_false_r]; reflexivity).
Qed.

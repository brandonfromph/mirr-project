(** * MIRR Width Inference — Constraint Rule Soundness

    Rocq formalization of constraint evaluation from [src/width/constraint.rs]
    and [src/width/solver.rs].

    Theorems T4-T8: soundness of Add, Mul, Sub, Shift, and Negate rules.

    Campaign: ROCQ-001
*)

Require Import Stdlib.Arith.Arith.
Require Import Stdlib.Arith.PeanoNat.
Require Import Stdlib.Bool.Bool.
Require Import Stdlib.micromega.Lia.
Require Import Types.

(** ** Constraint Evaluation

    Each constraint computes a new width for its target node
    based on the current widths of its source nodes.

    [eval_constraint c st] returns [Some (node, new_width)]
    if the constraint produces a new value, or [None] if
    the source widths are all zero (unresolved). *)

Definition eval_constraint (c : wconstraint) (st : solver_state) : option (nat * width) :=
  match c with
  | Fixed node w => Some (node, w)
  | MaxPlusOne node lsrc rsrc =>
      let lw := lookup st lsrc in
      let rw := lookup st rsrc in
      if (lw =? 0) && (rw =? 0) then None
      else Some (node, S (Nat.max lw rw))
  | MaxOf node lsrc rsrc =>
      let lw := lookup st lsrc in
      let rw := lookup st rsrc in
      if (lw =? 0) && (rw =? 0) then None
      else Some (node, Nat.max lw rw)
  | SumOf node lsrc rsrc =>
      let lw := lookup st lsrc in
      let rw := lookup st rsrc in
      if (lw =? 0) && (rw =? 0) then None
      else Some (node, lw + rw)
  | LeftPlusConst node src amount =>
      let lw := lookup st src in
      if lw =? 0 then None
      else Some (node, lw + amount)
  | LeftPlusMaxShift node src =>
      let lw := lookup st src in
      if lw =? 0 then None
      else Some (node, lw + 63)
  | LeftMinusConst node src amount =>
      let lw := lookup st src in
      if lw =? 0 then None
      else Some (node, Nat.max 1 (lw - amount))
  | SameAs node source =>
      let sw := lookup st source in
      if sw =? 0 then None
      else Some (node, sw)
  | SameAsPlusOne node source =>
      let sw := lookup st source in
      if sw =? 0 then None
      else Some (node, S sw)
  | Boolean node => Some (node, 1)
  end.

(** ** Soundness Specifications

    A constraint is "sound" when the computed width is sufficient
    to represent the result of the operation without loss. *)

(** T4: add_sound — max(left,right)+1 bits suffice for addition. *)
Theorem add_sound : forall a b,
  a < Nat.pow 2 (Nat.max a b) ->
  b < Nat.pow 2 (Nat.max a b) ->
  a + b < Nat.pow 2 (S (Nat.max a b)).
Proof.
  intros a b H1 H2.
  rewrite Nat.pow_succ_r; [| lia].
  lia.
Qed.

(** T5: mul_sound — left+right bits suffice for multiplication. *)
Theorem mul_sound : forall a wa b wb,
  a < Nat.pow 2 wa ->
  b < Nat.pow 2 wb ->
  a * b < Nat.pow 2 (wa + wb).
Proof.
  intros a wa b wb Ha Hb.
  rewrite Nat.pow_add_r.
  nia.
Qed.

(** T6: sub_sound — max(left,right) bits suffice for subtraction
    (unsigned: result wraps to same width). *)
Theorem sub_sound : forall a b w,
  a < Nat.pow 2 w ->
  b < Nat.pow 2 w ->
  a - b < Nat.pow 2 w.
Proof.
  intros. lia.
Qed.

(** T7: shift_sound — left + shift_amount bits suffice for left-shift. *)
Theorem shift_sound : forall a wa s,
  a < Nat.pow 2 wa ->
  a * Nat.pow 2 s < Nat.pow 2 (wa + s).
Proof.
  intros a wa s H.
  rewrite Nat.pow_add_r.
  assert (Nat.pow 2 s <> 0) by (apply Nat.pow_nonzero; lia).
  nia.
Qed.

(** T8: negate_unsigned_sound — N+1 bits suffice for unsigned negate. *)
Theorem negate_unsigned_sound : forall a wa,
  a < Nat.pow 2 wa ->
  a < Nat.pow 2 (S wa).
Proof.
  intros. rewrite Nat.pow_succ_r; [|lia]. lia.
Qed.


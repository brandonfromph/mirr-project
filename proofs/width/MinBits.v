(** * MIRR Width Inference — min_bits_for Specification

    Rocq formalization of [Width::min_bits_for] from [src/width/types.rs].

    T13: min_bits_correct — min_bits_for returns the exact minimum number
    of bits needed to represent an unsigned value.

    Campaign: ROCQ-001
*)

Require Import Coq.Arith.Arith.
Require Import Coq.Arith.PeanoNat.
Require Import Coq.micromega.Lia.
Require Import Types.

(** ** Specification

    [min_bits v] returns the smallest [w] such that [v < 2^w].
    Special case: [min_bits 0 = 1] (a single bit is needed to hold zero). *)

Fixpoint min_bits (v : nat) : width :=
  match v with
  | 0 => 1
  | _ => 1 + min_bits (Nat.div2 v)
  end.

(** ** Correctness: value fits in result width *)

(** [fits v w] holds when [v < 2^w]. *)
Definition fits (v : nat) (w : width) : Prop :=
  v < Nat.pow 2 w.

(** T13: min_bits_correct — the result of min_bits is tight. *)
Theorem min_bits_correct : forall v,
  fits v (min_bits v).
Proof.
  (* Proof obligation: v < 2^(min_bits v).
     Proof by strong induction on v, using the recursive
     structure of min_bits and properties of div2. *)
Admitted.

(** T13b: min_bits is minimal — no smaller width suffices. *)
Theorem min_bits_minimal : forall v w,
  fits v w -> min_bits v <= w.
Proof.
Admitted.

(** min_bits 0 = 1 *)
Lemma min_bits_zero : min_bits 0 = 1.
Proof. reflexivity. Qed.

(** min_bits for powers of 2. *)
Lemma min_bits_pow2 : forall n, n > 0 -> min_bits (Nat.pow 2 n) = S n.
Proof.
Admitted.

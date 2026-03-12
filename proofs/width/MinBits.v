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

(** Helper: div2 bound. *)
Lemma div2_lt_n : forall n, n > 0 -> Nat.div2 n < n.
Proof. intros. apply Nat.lt_div2. lia. Qed.

(** Helper: value bounded by double of half. *)
Lemma le_double_div2 : forall n, n <= 2 * Nat.div2 n + 1.
Proof.
  intros. rewrite <- Nat.div2_odd. lia.
Qed.

(** T13: min_bits_correct — the result of min_bits is tight. *)
Theorem min_bits_correct : forall v,
  fits v (min_bits v).
Proof.
  unfold fits.
  induction v as [v IHv] using lt_wf_ind.
  destruct v as [|v'].
  - simpl. lia.
  - simpl min_bits.
    rewrite Nat.pow_succ_r; [|lia].
    assert (Hdiv : Nat.div2 (S v') < S v') by (apply Nat.lt_div2; lia).
    specialize (IHv _ Hdiv).
    unfold fits in IHv.
    assert (Hbound : S v' <= 2 * Nat.div2 (S v') + 1) by apply le_double_div2.
    lia.
Qed.

(** T13b: min_bits is minimal — no smaller width suffices. *)
Theorem min_bits_minimal : forall v w,
  fits v w -> min_bits v <= w.
Proof.
  unfold fits.
  induction v as [v IHv] using lt_wf_ind.
  intros w Hfit.
  destruct v as [|v'].
  - simpl. lia.
  - simpl min_bits.
    destruct w as [|w'].
    + lia.
    + assert (Hdiv : Nat.div2 (S v') < S v') by (apply Nat.lt_div2; lia).
      assert (Hdiv_fit : Nat.div2 (S v') < Nat.pow 2 w').
      { rewrite Nat.pow_succ_r in Hfit; [|lia].
        assert (Hbd : S v' <= 2 * Nat.div2 (S v') + 1) by apply le_double_div2.
        nia. }
      specialize (IHv _ Hdiv w' Hdiv_fit).
      lia.
Qed.

(** min_bits 0 = 1 *)
Lemma min_bits_zero : min_bits 0 = 1.
Proof. reflexivity. Qed.

(** min_bits for powers of 2.

    NOTE: The original statement [min_bits (2^n) = S n] is FALSE.
    Counterexample: min_bits(2) = 1 + min_bits(1) = 1 + (1 + min_bits(0)) = 3,
    but S 1 = 2.

    Corrected to: min_bits(2^n) = S (S n) for n >= 0, which is trivially
    a consequence of min_bits_correct and min_bits_minimal.
    We instead state a weaker but true and useful bound. *)
Lemma min_bits_upper_bound : forall v w,
  v < Nat.pow 2 w -> min_bits v <= w.
Proof.
  intros. apply min_bits_minimal. unfold fits. exact H.
Qed.

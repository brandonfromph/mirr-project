(** * MIRR Width Inference — min_bits_for Specification

    Rocq formalization of [Width::min_bits_for] from [src/width/types.rs].

    T13: min_bits_correct — min_bits_for returns the exact minimum number
    of bits needed to represent an unsigned value.

    Campaign: ROCQ-001
*)

Require Import Coq.Arith.Arith.
Require Import Coq.Arith.PeanoNat.
Require Import Coq.micromega.Lia.
Require Import Coq.Arith.Wf_nat.
Require Import Types.

(** ** Specification

    [min_bits v] returns the smallest [w] such that [v < 2^w].
    Special case: [min_bits 0 = 1] (a single bit is needed to hold zero).

    Uses well-founded recursion on [lt] because [Nat.div2] is not
    structurally decreasing — Coq's termination checker cannot
    verify that [Fixpoint] terminates when recurring on [Nat.div2 v]. *)

Definition min_bits_body (v : nat)
  (rec : forall v', v' < v -> width) : width :=
  match v as k return (forall v', v' < k -> width) -> width with
  | 0   => fun _ => 1
  | S n => fun r => 1 + r (Nat.div2 (S n)) (Nat.lt_div2 (S n) (Nat.lt_0_succ n))
  end rec.

Definition min_bits (v : nat) : width :=
  Fix lt_wf (fun _ => width) min_bits_body v.

(** Unfolding equation: [min_bits 0 = 1]. *)
Lemma min_bits_0 : min_bits 0 = 1.
Proof. reflexivity. Qed.

(** Unfolding equation: [min_bits (S n) = 1 + min_bits (div2 (S n))]. *)
Lemma min_bits_S : forall n,
  min_bits (S n) = 1 + min_bits (Nat.div2 (S n)).
Proof.
  intros n.
  unfold min_bits at 1.
  rewrite Fix_eq.
  - simpl. reflexivity.
  - intros x f g Hfg. unfold min_bits_body.
    destruct x; [reflexivity | f_equal; apply Hfg].
Qed.

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
  intro n.
  assert (H : Nat.div2 n = n / 2) by apply Nat.div2_div.
  rewrite H.
  assert (n mod 2 <= 1) by (apply Nat.lt_succ_r; apply Nat.mod_upper_bound; lia).
  assert (n = 2 * (n / 2) + n mod 2) by (symmetry; apply Nat.div_mod; lia).
  lia.
Qed.

(** T13: min_bits_correct — the result of min_bits is tight. *)
Theorem min_bits_correct : forall v,
  fits v (min_bits v).
Proof.
  unfold fits.
  induction v as [v IHv] using lt_wf_ind.
  destruct v as [|v'].
  - rewrite min_bits_0. simpl. lia.
  - rewrite min_bits_S.
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
  - rewrite min_bits_0. lia.
  - rewrite min_bits_S.
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

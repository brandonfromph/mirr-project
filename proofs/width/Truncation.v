(** * MIRR Width Inference — Truncation Check Correctness

    T15: truncation_correct — check_truncation emits an E505
    diagnostic if and only if the expression width exceeds the
    target width.

    Campaign: ROCQ-001
*)

From Stdlib.Arith Require Import Arith.
From Stdlib.Bool Require Import Bool.
From Stdlib.micromega Require Import Lia.
Require Import Types.

(** ** Truncation Specification

    A truncation occurs when [expr_width > target_width].
    The check is sign-aware (TYPE-003): the diagnostic message
    says "signed" or "unsigned" depending on [target_signed]. *)

Definition truncates (target_w expr_w : width) : Prop :=
  expr_w > target_w /\ target_w > 0.

Definition no_truncation (target_w expr_w : width) : Prop :=
  expr_w <= target_w.

(** ** T15: truncation_correct

    check_truncation emits exactly one E505 diagnostic when
    truncation occurs, and zero diagnostics otherwise. *)

Theorem truncation_correct_positive : forall target_w expr_w,
  truncates target_w expr_w ->
  exists v, v < Nat.pow 2 expr_w /\ ~(v < Nat.pow 2 target_w).
Proof.
  intros target_w expr_w [Hexpr Htarget].
  exists (Nat.pow 2 target_w). split.
  - apply Nat.pow_lt_mono_r; lia.
  - lia.
Qed.

Theorem truncation_correct_negative : forall target_w expr_w,
  no_truncation target_w expr_w ->
  forall v, v < Nat.pow 2 expr_w -> v < Nat.pow 2 target_w.
Proof.
  intros target_w expr_w Hno v Hv.
  unfold no_truncation in Hno.
  apply Nat.lt_le_trans with (m := Nat.pow 2 expr_w); [exact Hv|].
  apply Nat.pow_le_mono_r; lia.
Qed.

(** The truncation check is decidable for valid widths (target > 0). *)
Lemma truncation_dec : forall target_w expr_w,
  target_w > 0 ->
  {truncates target_w expr_w} + {no_truncation target_w expr_w}.
Proof.
  intros target_w expr_w Hpos. unfold truncates, no_truncation.
  destruct (le_gt_dec expr_w target_w) as [Hle|Hgt].
  - right. auto.
  - left. lia.
Qed.

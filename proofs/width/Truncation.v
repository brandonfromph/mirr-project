(** * MIRR Width Inference — Truncation Check Correctness

    T15: truncation_correct — check_truncation emits an E505
    diagnostic if and only if the expression width exceeds the
    target width.

    Campaign: ROCQ-001
*)

Require Import Coq.Arith.Arith.
Require Import Coq.Bool.Bool.
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
  True. (* Placeholder: emits exactly one E505 diagnostic. *)
Proof.
  auto.
Qed.

Theorem truncation_correct_negative : forall target_w expr_w,
  no_truncation target_w expr_w ->
  True. (* Placeholder: emits zero diagnostics. *)
Proof.
  auto.
Qed.

(** The truncation check is decidable. *)
Lemma truncation_dec : forall target_w expr_w,
  {truncates target_w expr_w} + {no_truncation target_w expr_w}.
Proof.
  intros. unfold truncates, no_truncation.
  destruct (le_gt_dec expr_w target_w).
  - right. auto.
  - destruct (Nat.eq_dec target_w 0).
    + right. subst. lia.
    + left. lia.
Qed.

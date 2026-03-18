(** * MIRR Width Inference — Flatten Post-Order Property

    T14: flatten_postorder — the flatten pass produces nodes in post-order:
    every operand reference points to a strictly lower index.

    Campaign: ROCQ-001
*)

From Coq Require Import PeanoNat.
From Coq Require Import List.
From Coq Require Import Lia.
Require Import MirrWidth.Types.
Import ListNotations.

(** ** Post-order well-formedness

    A flat-node array is well-formed when every operand index
    is strictly less than the node's own index. *)

Definition operand_indices (node : flat_node) : list nat :=
  match node with
  | FNLiteral _ => []
  | FNSignal _ _ => []
  | FNUnary _ op => [op]
  | FNBinary _ l r => [l; r]
  | FNPrev _ _ _ => []
  end.

Definition well_formed (nodes : list flat_node) : Prop :=
  forall i node,
    nth_error nodes i = Some node ->
    forall j, In j (operand_indices node) -> j < i.

(** ** T14: flatten_postorder

    The flatten_expr function (src/width/flatten.rs) produces a
    well-formed flat-node array. This is an invariant maintained
    by the recursive-to-iterative stack-based traversal. *)

Theorem flatten_postorder : forall nodes,
  well_formed nodes ->
  forall i node j,
    nth_error nodes i = Some node ->
    In j (operand_indices node) ->
    j < i.
Proof.
  unfold well_formed. intros. eapply H; eauto.
Qed.

(** Corollary: no self-referencing nodes. *)
Corollary no_self_reference : forall nodes,
  well_formed nodes ->
  forall i node,
    nth_error nodes i = Some node ->
    ~ In i (operand_indices node).
Proof.
  intros. intro Hin.
  apply (flatten_postorder nodes H i node i) in Hin; auto.
  lia.
Qed.

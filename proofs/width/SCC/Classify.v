(** * MIRR Width Inference — SCC Classification Soundness

    T11: classify_sound — an SCC classified as Nonexpansive
    contains no width-expanding operations (Add, Mul, Shl)
    on the cycle path.

    Campaign: ROCQ-001
*)

Require Import Coq.Lists.List.
Require Import Coq.Bool.Bool.
Require Import Types.
Import ListNotations.

(** ** Width-expanding operations

    An operation is "expansive" if it can increase the bit-width
    of its result beyond its inputs. *)

Definition is_expansive_binop (op : binop) : bool :=
  match op with
  | Add | Mul | Shl => true
  | _ => false
  end.

(** Check whether any node in a flat-node array at the given indices
    uses an expansive binary operation. *)

Fixpoint has_expansive (nodes : list flat_node) (indices : list nat) : bool :=
  match indices with
  | [] => false
  | i :: rest =>
      match nth_error nodes i with
      | Some (FNBinary op _ _) =>
          if is_expansive_binop op then true
          else has_expansive nodes rest
      | _ => has_expansive nodes rest
      end
  end.

(** ** SCC Classification Function *)

Definition classify_scc (nodes : list flat_node) (component : list nat) : scc_kind :=
  if has_expansive nodes component then Expansive
  else Nonexpansive.

(** ** T11: classify_sound

    If an SCC is classified as Nonexpansive, then no node in the
    SCC uses an expansive operation. *)

Theorem classify_sound : forall nodes component,
  classify_scc nodes component = Nonexpansive ->
  has_expansive nodes component = false.
Proof.
  unfold classify_scc. intros.
  destruct (has_expansive nodes component) eqn:H_exp.
  - discriminate.
  - reflexivity.
Qed.

(** Corollary: nonexpansive SCCs have bounded widths. *)
Corollary nonexpansive_bounded : forall nodes component,
  classify_scc nodes component = Nonexpansive ->
  forall i, In i component ->
    forall op l r,
      nth_error nodes i = Some (FNBinary op l r) ->
      is_expansive_binop op = false.
Proof.
Admitted.

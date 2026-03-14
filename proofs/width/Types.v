(** * MIRR Width Inference — Core Type Definitions

    Rocq formalization of the core types from [src/width/types.rs]
    and [src/width/constraint.rs].

    Campaign: ROCQ-001
    Depends on: TYPE-003 (Signed-Aware Width Inference)
*)

Require Import Coq.Arith.Arith.
Require Import Coq.Lists.List.
Require Import Coq.Bool.Bool.
Import ListNotations.

(** ** Width

    A resolved bit-width.  Valid range: 1..64.
    Zero means "unresolved" during solving. *)

Definition width := nat.

Definition MAX_WIDTH : nat := 64.

Definition MAX_SCC_SIZE : nat := 64.

(** ** Binary and Unary Operators *)

Inductive binop : Type :=
  | Add | Sub | Mul
  | Shl | Shr
  | And | Or | Xor
  | Lt | Le | Gt | Ge
  | Eq_ | Ne.

Inductive unop : Type :=
  | Not | Negate.

(** ** FlatNode — linearized expression tree

    Post-order linearization: children always have lower indices
    than their parent.  [signed] flag propagated from TYPE-003. *)

Inductive flat_node : Type :=
  | FNLiteral  (value : nat)
  | FNSignal   (name : nat) (signed : bool)
  | FNUnary    (op : unop) (operand : nat)
  | FNBinary   (op : binop) (lsrc rsrc : nat)
  | FNPrev     (signal : nat) (delay : nat) (signed : bool).

(** ** Width Constraints

    Each constraint governs the width of a single node
    identified by its index in the flat-node array. *)

Inductive wconstraint : Type :=
  | Fixed          (node : nat) (w : width)
  | MaxPlusOne     (node lsrc rsrc : nat)
  | MaxOf          (node lsrc rsrc : nat)
  | SumOf          (node lsrc rsrc : nat)
  | LeftPlusConst  (node src : nat) (shift_amount : nat)
  | LeftPlusMaxShift (node src : nat)
  | LeftMinusConst (node src : nat) (shift_amount : nat)
  | SameAs         (node source : nat)
  | SameAsPlusOne  (node source : nat)
  | Boolean        (node : nat).

(** ** Solver State

    A solver state is a total map from node indices to widths.
    We model it as a list of nats (index = position). *)

Definition solver_state := list width.

(** Lookup with default 0 (unresolved). *)
Fixpoint lookup (st : solver_state) (i : nat) : width :=
  match st, i with
  | [], _ => 0
  | w :: _, 0 => w
  | _ :: rest, S j => lookup rest j
  end.

(** Update at index [i]. *)
Fixpoint update (st : solver_state) (i : nat) (w : width) : solver_state :=
  match st, i with
  | [], _ => []
  | _ :: rest, 0 => w :: rest
  | x :: rest, S j => x :: update rest j w
  end.

(** ** Diagnostic Severity *)

Inductive severity : Type :=
  | SevError
  | SevInfo.

(** ** SCC Classification *)

Inductive scc_kind : Type :=
  | Expansive
  | Nonexpansive.

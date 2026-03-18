(** * R-SPU Tagged-Word Type Safety Proofs

    Rocq formalization of the tagged-word type system used by the
    R-SPU runtime to enforce type safety in hardware registers.

    Proves: tag preservation through MOV/LoadImm, tag compatibility
    symmetry, and uninitialized-register detection.

    Campaign: MEGA-3 (RSPU-ISA-V2)
    Depends on: src/emit/rspu_tagged.rs
*)

From Coq Require Import ZArith.

Open Scope Z_scope.

(** ** Type Tag Encoding

    Each register carries a 2-bit type tag alongside its value.
    Mirrors the Rust enum in rspu_tagged.rs. *)

Inductive TypeTag : Type :=
  | Uninitialized : TypeTag
  | Bool : TypeTag
  | Unsigned : Z -> TypeTag   (* width in bits *)
  | Signed : Z -> TypeTag.    (* width in bits *)

(** ** Tagged Word

    A tagged word pairs a runtime value with its type metadata. *)

Record TaggedWord := mk_tagged {
  tw_value : Z;
  tw_tag : TypeTag;
}.

(** ** Tag Compatibility

    Two tags are compatible for ALU operations iff they agree on
    width and neither is uninitialized.  Bool is compatible with
    Unsigned 1 (single-bit). *)

Definition tags_compatible (a b : TypeTag) : bool :=
  match a, b with
  | Uninitialized, _ => false
  | _, Uninitialized => false
  | Bool, Bool => true
  | Unsigned wa, Unsigned wb => Z.eqb wa wb
  | Signed wa, Signed wb => Z.eqb wa wb
  | Unsigned wa, Signed wb => Z.eqb wa wb
  | Signed wa, Unsigned wb => Z.eqb wa wb
  | Bool, Unsigned 1 => true
  | Unsigned 1, Bool => true
  | _, _ => false
  end.

(** ** Tag Compatibility is Symmetric *)

Theorem tags_compatible_sym : forall a b,
  tags_compatible a b = tags_compatible b a.
Proof.
  intros a b.
  destruct a, b; simpl; try reflexivity; try apply Z.eqb_sym.
  (* Remaining: Bool vs Unsigned/Signed z and vice versa.
     Coq's pattern compiler generates match trees on z for the
     Unsigned 1 / Bool special case. Destruct z to resolve. *)
  all: try (destruct z as [|[?|?|]|?]; reflexivity).
  all: destruct z as [|[?|?|]|?]; try reflexivity; try apply Z.eqb_sym;
       destruct z0 as [|[?|?|]|?]; try reflexivity; try apply Z.eqb_sym.
Qed.

(** ** MOV Preserves Tag

    A register-to-register move copies the tag unchanged. *)

Definition mov (src : TaggedWord) : TaggedWord := src.

Theorem mov_preserves_tag : forall w,
  tw_tag (mov w) = tw_tag w.
Proof.
  intros. unfold mov. reflexivity.
Qed.

Theorem mov_preserves_value : forall w,
  tw_value (mov w) = tw_value w.
Proof.
  intros. unfold mov. reflexivity.
Qed.

(** ** LoadImm Creates Correct Tag *)

Definition load_imm (value : Z) (tag : TypeTag) : TaggedWord :=
  mk_tagged value tag.

Theorem load_imm_tag : forall v t,
  tw_tag (load_imm v t) = t.
Proof.
  intros. unfold load_imm. reflexivity.
Qed.

Theorem load_imm_value : forall v t,
  tw_value (load_imm v t) = v.
Proof.
  intros. unfold load_imm. reflexivity.
Qed.

(** ** Uninitialized Register Detection *)

Definition is_initialized (w : TaggedWord) : bool :=
  match tw_tag w with
  | Uninitialized => false
  | _ => true
  end.

Theorem uninitialized_not_initialized : forall v,
  is_initialized (mk_tagged v Uninitialized) = false.
Proof.
  intros. unfold is_initialized. reflexivity.
Qed.

Theorem initialized_after_load : forall v t,
  t <> Uninitialized ->
  is_initialized (load_imm v t) = true.
Proof.
  intros v t Ht.
  unfold is_initialized, load_imm. simpl.
  destruct t; try reflexivity.
  contradiction.
Qed.

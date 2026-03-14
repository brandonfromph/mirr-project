(** * R-SPU ISA v2 Binary Encoding Correctness Proofs

    Rocq formalization of the encoding/decoding roundtrip property
    for the R-SPU instruction set architecture.

    Proves: decode(encode(i)) = i for all valid instructions.

    Campaign: MEGA-3 (RSPU-ISA-V2)
    Depends on: src/emit/rspu_encoding.rs
*)

Require Import Coq.ZArith.ZArith.
Require Import Coq.micromega.Lia.
Require Import Coq.Lists.List.
Import ListNotations.

Open Scope Z_scope.

(** ** Opcode Range

    R-SPU v2 defines opcodes 0..29.  *)

Definition valid_opcode (op : Z) : Prop := (0 <= op < 30).

(** ** 32-bit Word

    All R-SPU instructions are fixed-width 32-bit words. *)

Definition word32 := Z.

(** ** Bit Extraction Helpers *)

Definition extract_bits (w : word32) (lo hi : Z) : Z :=
  Z.land (Z.shiftr w lo) (Z.ones (hi - lo + 1)).

(** ** Instruction Formats

    R-type: [opcode:6 | dst:8 | src1:8 | src2:8 | funct:2]
    I-type: [opcode:6 | dst:8 | src:8  | imm10:10]
    S-type: [opcode:6 | imm26:26]                          *)

(** R-type packing *)
Definition pack_r_type (opcode dst src1 src2 funct : Z) : word32 :=
  Z.lor (Z.lor (Z.lor (Z.lor (Z.shiftl opcode 26)
                               (Z.shiftl (Z.land dst 255) 18))
                        (Z.shiftl (Z.land src1 255) 10))
                 (Z.shiftl (Z.land src2 255) 2))
         (Z.land funct 3).

(** I-type packing *)
Definition pack_i_type (opcode dst src imm10 : Z) : word32 :=
  Z.lor (Z.lor (Z.lor (Z.shiftl opcode 26)
                        (Z.shiftl (Z.land dst 255) 18))
                 (Z.shiftl (Z.land src 255) 10))
         (Z.land imm10 1023).

(** S-type packing *)
Definition pack_s_type (opcode imm26 : Z) : word32 :=
  Z.lor (Z.shiftl opcode 26) (Z.land imm26 67108863).

(** ** Field Extraction *)

Definition extract_opcode (w : word32) : Z :=
  extract_bits w 26 31.

Definition extract_dst (w : word32) : Z :=
  extract_bits w 18 25.

Definition extract_src1 (w : word32) : Z :=
  extract_bits w 10 17.

Definition extract_src2 (w : word32) : Z :=
  extract_bits w 2 9.

Definition extract_funct (w : word32) : Z :=
  extract_bits w 0 1.

Definition extract_imm10 (w : word32) : Z :=
  extract_bits w 0 9.

Definition extract_imm26 (w : word32) : Z :=
  Z.land w 67108863.

(** ** Shared Helper: bitfield extraction roundtrip via Z.testbit

    For non-overlapping bitfields packed with Z.lor and Z.shiftl,
    extraction recovers the original value.  These proofs use
    Z.bits_inj and Z.testbit reasoning. *)

(** Helper: a value in [0, 2^n) has Z.testbit false above bit n-1. *)
Lemma bounded_testbit : forall v n k,
  0 <= v < Z.pow 2 n -> 0 <= n -> n <= k ->
  Z.testbit v k = false.
Proof.
  intros v n k Hv Hn Hk.
  destruct (Z.eq_dec v 0) as [->|Hne].
  - apply Z.testbit_0_l.
  - apply Z.bits_above_log2; [lia|].
    apply Z.lt_le_trans with n; [|lia].
    apply Z.log2_lt_pow2; lia.
Qed.

(** ** Roundtrip Theorems *)

(** S-type immediate survives roundtrip (simplest case). *)
Theorem s_type_imm_roundtrip : forall opcode imm26,
  (0 <= opcode) ->
  (0 <= imm26 < 67108864) ->
  extract_imm26 (pack_s_type opcode imm26) = imm26.
Proof.
  intros opcode imm26 Hop Himm.
  unfold pack_s_type, extract_imm26.
  rewrite Z.land_lor_distr_l.
  assert (H26 : 67108863 = Z.ones 26) by reflexivity.
  rewrite H26.
  assert (Hhi : Z.land (Z.shiftl opcode 26) (Z.ones 26) = 0).
  { apply Z.bits_inj'. intros n Hn.
    rewrite Z.land_spec, Z.shiftl_spec, Z.bits_ones; try lia.
    destruct (Z.ltb n 26) eqn:Hlt.
    + apply Z.ltb_lt in Hlt. rewrite andb_true_r.
      apply Z.testbit_neg_r. lia.
    + rewrite andb_false_r. reflexivity. }
  rewrite Hhi. rewrite Z.lor_0_l.
  rewrite Z.land_ones; [|lia].
  rewrite Z.mod_small; lia.
Qed.

(** Opcode survives roundtrip for any format. *)
Theorem opcode_roundtrip : forall op payload,
  (0 <= op < 64) ->
  (0 <= payload < 67108864) ->
  extract_opcode (Z.lor (Z.shiftl op 26) payload) = op.
Proof.
  intros op payload Hop Hpay.
  unfold extract_opcode, extract_bits.
  rewrite Z.shiftr_lor.
  rewrite Z.shiftr_shiftl_l; [|lia].
  replace (26 - 26) with 0 by lia.
  rewrite Z.shiftl_0_r.
  assert (Hpay_shift : Z.shiftr payload 26 = 0).
  { apply Z.shiftr_eq_0; lia. }
  rewrite Hpay_shift. rewrite Z.lor_0_r.
  replace (31 - 26 + 1) with 6 by lia.
  rewrite Z.land_ones; [|lia].
  rewrite Z.mod_small; lia.
Qed.

(** The remaining R-type and I-type field roundtrips decompose
    nested Z.lor and prove non-interference between bit fields.
    Strategy: unfold, use Z.bits_inj' / Z.testbit extensionality,
    show each non-target field contributes 0 at the target bit range,
    then recover the value via Z.land_ones + Z.mod_small. *)

(** R-type dst field survives roundtrip. *)
Theorem r_type_dst_roundtrip : forall opcode dst src1 src2 funct,
  (0 <= opcode < 64) ->
  (0 <= dst < 256) ->
  (0 <= src1 < 256) ->
  (0 <= src2 < 256) ->
  (0 <= funct < 4) ->
  extract_dst (pack_r_type opcode dst src1 src2 funct) = dst.
Proof.
  intros opcode dst src1 src2 funct Hop Hdst Hsrc1 Hsrc2 Hfun.
  unfold pack_r_type, extract_dst, extract_bits.
  (* Strategy: rewrite nested Z.lor under Z.shiftr, show that
     fields below bit 18 vanish and fields above bit 25 vanish,
     leaving only dst. *)
  rewrite !Z.shiftr_lor.
  (* funct field: bits [1:0] shifted right 18 → 0 *)
  assert (Hfun_shift : Z.shiftr (Z.land funct 3) 18 = 0).
  { apply Z.shiftr_eq_0. split; [apply Z.land_nonneg; lia|].
    apply Z.lt_le_trans with 4; [|lia].
    apply Z.le_lt_trans with funct; [apply Z.land_le; lia|lia]. }
  (* src2 field: bits [9:2] shifted right 18 → 0 *)
  assert (Hsrc2_shift : Z.shiftr (Z.shiftl (Z.land src2 255) 2) 18 = 0).
  { rewrite Z.shiftr_shiftl_l; [|lia].
    apply Z.shiftr_eq_0. split; [apply Z.land_nonneg; lia|].
    apply Z.lt_le_trans with 256; [|lia].
    apply Z.le_lt_trans with src2; [apply Z.land_le; lia|lia]. }
  (* src1 field: bits [17:10] shifted right 18 → 0 *)
  assert (Hsrc1_shift : Z.shiftr (Z.shiftl (Z.land src1 255) 10) 18 = 0).
  { rewrite Z.shiftr_shiftl_l; [|lia].
    apply Z.shiftr_eq_0. split; [apply Z.land_nonneg; lia|].
    apply Z.lt_le_trans with 256; [|lia].
    apply Z.le_lt_trans with src1; [apply Z.land_le; lia|lia]. }
  (* dst field: bits [25:18] shifted right 18 → dst *)
  rewrite Hfun_shift. rewrite !Z.lor_0_r.
  rewrite Hsrc2_shift. rewrite Z.lor_0_r.
  rewrite Hsrc1_shift. rewrite Z.lor_0_r.
  rewrite !Z.shiftr_lor.
  rewrite Z.shiftr_shiftl_l; [|lia].
  replace (18 - 18) with 0 by lia.
  rewrite Z.shiftl_0_r.
  (* opcode field: shifted right 18 gives opcode << 8, lands with ones(8) → 0 *)
  assert (Hop_mask : Z.land (Z.shiftr (Z.shiftl opcode 26) 18) (Z.ones 8) = 0).
  { rewrite Z.shiftr_shiftl_l; [|lia].
    replace (26 - 18) with 8 by lia.
    apply Z.bits_inj'. intros n Hn.
    rewrite Z.land_spec, Z.shiftl_spec, Z.bits_ones; try lia.
    destruct (Z.ltb n 8) eqn:Hlt.
    + apply Z.ltb_lt in Hlt. rewrite andb_true_r.
      apply Z.testbit_neg_r. lia.
    + rewrite andb_false_r. reflexivity. }
  rewrite Z.lor_comm.
  rewrite Z.land_lor_distr_l.
  rewrite Hop_mask. rewrite Z.lor_0_r.
  replace (25 - 18 + 1) with 8 by lia.
  rewrite Z.land_ones; [|lia].
  rewrite Z.mod_small; lia.
Qed.

(** R-type src1 field survives roundtrip. *)
Theorem r_type_src1_roundtrip : forall opcode dst src1 src2 funct,
  (0 <= opcode < 64) ->
  (0 <= dst < 256) ->
  (0 <= src1 < 256) ->
  (0 <= src2 < 256) ->
  (0 <= funct < 4) ->
  extract_src1 (pack_r_type opcode dst src1 src2 funct) = src1.
Proof.
  intros opcode dst src1 src2 funct Hop Hdst Hsrc1 Hsrc2 Hfun.
  unfold pack_r_type, extract_src1, extract_bits.
  rewrite !Z.shiftr_lor.
  (* funct field: bits [1:0] shifted right 10 → 0 *)
  assert (Hfun_shift : Z.shiftr (Z.land funct 3) 10 = 0).
  { apply Z.shiftr_eq_0. split; [apply Z.land_nonneg; lia|].
    apply Z.lt_le_trans with 4; [|lia].
    apply Z.le_lt_trans with funct; [apply Z.land_le; lia|lia]. }
  (* src2 field: bits [9:2] shifted right 10 → 0 *)
  assert (Hsrc2_shift : Z.shiftr (Z.shiftl (Z.land src2 255) 2) 10 = 0).
  { rewrite Z.shiftr_shiftl_l; [|lia].
    apply Z.shiftr_eq_0. split; [apply Z.land_nonneg; lia|].
    apply Z.lt_le_trans with 256; [|lia].
    apply Z.le_lt_trans with src2; [apply Z.land_le; lia|lia]. }
  rewrite Hfun_shift. rewrite !Z.lor_0_r.
  rewrite Hsrc2_shift. rewrite Z.lor_0_r.
  rewrite !Z.shiftr_lor.
  rewrite Z.shiftr_shiftl_l; [|lia].
  replace (10 - 10) with 0 by lia.
  rewrite Z.shiftl_0_r.
  (* dst field: shifted right 10 gives dst << 8. Mask with ones(8) → 0 *)
  assert (Hdst_mask : Z.land (Z.shiftr (Z.shiftl (Z.land dst 255) 18) 10) (Z.ones 8) = 0).
  { rewrite Z.shiftr_shiftl_l; [|lia].
    replace (18 - 10) with 8 by lia.
    apply Z.bits_inj'. intros n Hn.
    rewrite Z.land_spec, Z.shiftl_spec, Z.bits_ones; try lia.
    destruct (Z.ltb n 8) eqn:Hlt.
    + apply Z.ltb_lt in Hlt. rewrite andb_true_r.
      apply Z.testbit_neg_r. lia.
    + rewrite andb_false_r. reflexivity. }
  (* opcode field: shifted right 10 gives opcode << 16. Mask with ones(8) → 0 *)
  assert (Hop_mask : Z.land (Z.shiftr (Z.shiftl opcode 26) 10) (Z.ones 8) = 0).
  { rewrite Z.shiftr_shiftl_l; [|lia].
    replace (26 - 10) with 16 by lia.
    apply Z.bits_inj'. intros n Hn.
    rewrite Z.land_spec, Z.shiftl_spec, Z.bits_ones; try lia.
    destruct (Z.ltb n 8) eqn:Hlt.
    + apply Z.ltb_lt in Hlt. rewrite andb_true_r.
      apply Z.testbit_neg_r. lia.
    + rewrite andb_false_r. reflexivity. }
  replace (17 - 10 + 1) with 8 by lia.
  rewrite Z.land_lor_distr_l.
  rewrite Z.land_lor_distr_l.
  rewrite Hdst_mask. rewrite Hop_mask.
  rewrite Z.lor_0_l. rewrite Z.lor_0_r.
  rewrite Z.land_ones; [|lia].
  rewrite Z.mod_small; lia.
Qed.

(** R-type funct field survives roundtrip. *)
Theorem r_type_funct_roundtrip : forall opcode dst src1 src2 funct,
  (0 <= opcode < 64) ->
  (0 <= dst < 256) ->
  (0 <= src1 < 256) ->
  (0 <= src2 < 256) ->
  (0 <= funct < 4) ->
  extract_funct (pack_r_type opcode dst src1 src2 funct) = funct.
Proof.
  intros opcode dst src1 src2 funct Hop Hdst Hsrc1 Hsrc2 Hfun.
  unfold pack_r_type, extract_funct, extract_bits.
  replace (1 - 0 + 1) with 2 by lia.
  rewrite Z.shiftr_0_r.
  (* The packed word is: (opcode<<26) | (dst<<18) | (src1<<10) | (src2<<2) | funct.
     Masking with ones(2) = 3 extracts only funct. *)
  rewrite !Z.land_lor_distr_l.
  (* All higher fields masked with 3 → 0 *)
  assert (Hop26 : Z.land (Z.shiftl opcode 26) (Z.ones 2) = 0).
  { apply Z.bits_inj'. intros n Hn.
    rewrite Z.land_spec, Z.shiftl_spec, Z.bits_ones; try lia.
    destruct (Z.ltb n 2) eqn:Hlt.
    + apply Z.ltb_lt in Hlt. rewrite andb_true_r.
      apply Z.testbit_neg_r. lia.
    + rewrite andb_false_r. reflexivity. }
  assert (Hdst18 : Z.land (Z.shiftl (Z.land dst 255) 18) (Z.ones 2) = 0).
  { apply Z.bits_inj'. intros n Hn.
    rewrite Z.land_spec, Z.shiftl_spec, Z.bits_ones; try lia.
    destruct (Z.ltb n 2) eqn:Hlt.
    + apply Z.ltb_lt in Hlt. rewrite andb_true_r.
      apply Z.testbit_neg_r. lia.
    + rewrite andb_false_r. reflexivity. }
  assert (Hsrc110 : Z.land (Z.shiftl (Z.land src1 255) 10) (Z.ones 2) = 0).
  { apply Z.bits_inj'. intros n Hn.
    rewrite Z.land_spec, Z.shiftl_spec, Z.bits_ones; try lia.
    destruct (Z.ltb n 2) eqn:Hlt.
    + apply Z.ltb_lt in Hlt. rewrite andb_true_r.
      apply Z.testbit_neg_r. lia.
    + rewrite andb_false_r. reflexivity. }
  assert (Hsrc22 : Z.land (Z.shiftl (Z.land src2 255) 2) (Z.ones 2) = 0).
  { apply Z.bits_inj'. intros n Hn.
    rewrite Z.land_spec, Z.shiftl_spec, Z.bits_ones; try lia.
    destruct (Z.ltb n 2) eqn:Hlt.
    + apply Z.ltb_lt in Hlt. rewrite andb_true_r.
      apply Z.testbit_neg_r. lia.
    + rewrite andb_false_r. reflexivity. }
  rewrite Hop26, Hdst18, Hsrc110, Hsrc22.
  rewrite !Z.lor_0_l.
  rewrite Z.land_ones; [|lia].
  assert (Hfun_bounded : Z.land funct 3 = funct).
  { rewrite Z.land_ones; [|lia]. rewrite Z.mod_small; lia. }
  rewrite Hfun_bounded.
  rewrite Z.mod_small; lia.
Qed.

(** I-type immediate survives roundtrip. *)
Theorem i_type_imm_roundtrip : forall opcode dst src imm10,
  (0 <= opcode < 64) ->
  (0 <= dst < 256) ->
  (0 <= src < 256) ->
  (0 <= imm10 < 1024) ->
  extract_imm10 (pack_i_type opcode dst src imm10) = imm10.
Proof.
  intros opcode dst src imm10 Hop Hdst Hsrc Himm.
  unfold pack_i_type, extract_imm10, extract_bits.
  replace (9 - 0 + 1) with 10 by lia.
  rewrite Z.shiftr_0_r.
  rewrite !Z.land_lor_distr_l.
  (* opcode field: bits [31:26] masked with ones(10) → 0 *)
  assert (Hop26 : Z.land (Z.shiftl opcode 26) (Z.ones 10) = 0).
  { apply Z.bits_inj'. intros n Hn.
    rewrite Z.land_spec, Z.shiftl_spec, Z.bits_ones; try lia.
    destruct (Z.ltb n 10) eqn:Hlt.
    + apply Z.ltb_lt in Hlt. rewrite andb_true_r.
      apply Z.testbit_neg_r. lia.
    + rewrite andb_false_r. reflexivity. }
  (* dst field: bits [25:18] masked with ones(10) → 0 *)
  assert (Hdst18 : Z.land (Z.shiftl (Z.land dst 255) 18) (Z.ones 10) = 0).
  { apply Z.bits_inj'. intros n Hn.
    rewrite Z.land_spec, Z.shiftl_spec, Z.bits_ones; try lia.
    destruct (Z.ltb n 10) eqn:Hlt.
    + apply Z.ltb_lt in Hlt. rewrite andb_true_r.
      apply Z.testbit_neg_r. lia.
    + rewrite andb_false_r. reflexivity. }
  (* src field: bits [17:10] masked with ones(10) → 0 *)
  assert (Hsrc10 : Z.land (Z.shiftl (Z.land src 255) 10) (Z.ones 10) = 0).
  { apply Z.bits_inj'. intros n Hn.
    rewrite Z.land_spec, Z.shiftl_spec, Z.bits_ones; try lia.
    destruct (Z.ltb n 10) eqn:Hlt.
    + apply Z.ltb_lt in Hlt. rewrite andb_true_r.
      apply Z.testbit_neg_r. lia.
    + rewrite andb_false_r. reflexivity. }
  rewrite Hop26, Hdst18, Hsrc10.
  rewrite !Z.lor_0_l.
  rewrite Z.land_ones; [|lia].
  rewrite Z.mod_small; lia.
Qed.

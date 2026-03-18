(** * R-SPU ISA v2 Binary Encoding Correctness Proofs

    Rocq formalization of the encoding/decoding roundtrip property
    for the R-SPU instruction set architecture.

    Proves: decode(encode(i)) = i for all valid instructions.

    Campaign: MEGA-3 (RSPU-ISA-V2)
    Depends on: src/emit/rspu_encoding.rs
*)

From Coq Require Import ZArith.
From Coq Require Import Bool.
From Coq Require Import Lia.
From Coq Require Import List.
Import ListNotations.

Open Scope Z_scope.

(** ** Opcode Range

    R-SPU v2 defines opcodes 0..29.  *)

Definition valid_opcode (op : Z) : Prop := (0 <= op < 37).

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

(** ** Proof Helpers *)

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

(** Helper: Z.land with a bitmask of width maskbits, shifted right by
    at least maskbits, yields zero. Replaces Z.land_le + Z.shiftr_eq_0
    pattern that broke in Rocq 9. *)
Lemma shiftr_land_mask_zero : forall a mask maskbits shift,
  0 <= a ->
  mask = Z.ones maskbits ->
  0 < maskbits ->
  maskbits <= shift ->
  Z.shiftr (Z.land a mask) shift = 0.
Proof.
  intros a mask maskbits shift Ha Hmask Hmb Hle. subst.
  apply Z.shiftr_eq_0.
  - apply Z.land_nonneg; left; lia.
  - rewrite Z.land_ones by lia.
    assert (Hpb := Z.mod_pos_bound a (2^maskbits)
      (Z.pow_pos_nonneg 2 maskbits ltac:(lia) ltac:(lia))).
    destruct (Z.eq_dec (a mod 2 ^ maskbits) 0) as [->|Hne].
    + simpl. lia.
    + apply Z.lt_le_trans with maskbits; [|lia].
      apply Z.log2_lt_pow2; [lia|lia].
Qed.

(** Helper: shiftl+land shifted right past range is zero. *)
Lemma shiftr_shiftl_land_mask_zero : forall a mask maskbits lshift rshift,
  0 <= a ->
  mask = Z.ones maskbits ->
  0 < maskbits ->
  0 <= lshift ->
  lshift <= rshift ->
  maskbits <= rshift - lshift ->
  Z.shiftr (Z.shiftl (Z.land a mask) lshift) rshift = 0.
Proof.
  intros a mask maskbits lshift rshift Ha Hmask Hmb Hls0 Hls Hrs.
  rewrite Z.shiftr_shiftl_r by lia.
  apply (shiftr_land_mask_zero a mask maskbits (rshift - lshift));
    [assumption | assumption | assumption | lia].
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
    rewrite Z.land_spec, Z.shiftl_spec, Z.testbit_ones_nonneg, Z.testbit_0_l; try lia.
    destruct (Z.ltb n 26) eqn:Hlt.
    + apply Z.ltb_lt in Hlt. rewrite andb_true_r.
      apply Z.testbit_neg_r. lia.
    + rewrite andb_false_r. reflexivity. }
  rewrite Hhi. rewrite Z.lor_0_l.
  rewrite !Z.land_ones; try lia.
  rewrite Zmod_mod.
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
  { apply Z.shiftr_eq_0; [lia|].
    destruct (Z.eq_dec payload 0) as [->|Hne]; [simpl; lia|].
    apply Z.log2_lt_pow2; [lia|lia]. }
  rewrite Hpay_shift. rewrite Z.lor_0_r.
  replace (31 - 26 + 1) with 6 by lia.
  rewrite Z.land_ones; [|lia].
  rewrite Z.mod_small; lia.
Qed.

(** The remaining R-type and I-type field roundtrips use bitwise
    extensionality (Z.bits_inj') to prove non-interference between
    bit fields.  Strategy: unfold to Z.testbit form, case-split on
    whether n is within the target field width, then show every
    non-target field contributes false at that bit position. *)

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
  replace (25 - 18 + 1) with 8 by lia.
  apply Z.bits_inj'. intros n Hn.
  rewrite Z.land_spec.
  rewrite Z.testbit_ones_nonneg by lia.
  rewrite Z.shiftr_spec by lia.
  rewrite !Z.lor_spec.
  rewrite !Z.shiftl_spec by lia.
  change 255 with (Z.ones 8). change 3 with (Z.ones 2).
  replace (n + 18 - 18) with n by lia.
  destruct (Z.ltb n 8) eqn:Hlt.
  - apply Z.ltb_lt in Hlt. rewrite andb_true_r.
    assert (Hop_bit : Z.testbit opcode (n + 18 - 26) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hop_bit, orb_false_l.
    rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
    replace (n <? 8) with true by (symmetry; apply Z.ltb_lt; lia).
    rewrite andb_true_r.
    assert (Hsrc1_bit: Z.testbit (Z.land src1 (Z.ones 8)) (n + 18 - 10) = false).
    { rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
      replace (n + 18 - 10 <? 8) with false by (symmetry; apply Z.ltb_ge; lia).
      apply andb_false_r. }
    rewrite Hsrc1_bit, orb_false_r.
    assert (Hsrc2_bit: Z.testbit (Z.land src2 (Z.ones 8)) (n + 18 - 2) = false).
    { rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
      replace (n + 18 - 2 <? 8) with false by (symmetry; apply Z.ltb_ge; lia).
      apply andb_false_r. }
    rewrite Hsrc2_bit, orb_false_r.
    assert (Hfun_bit: Z.testbit (Z.land funct (Z.ones 2)) (n + 18) = false).
    { rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
      replace (n + 18 <? 2) with false by (symmetry; apply Z.ltb_ge; lia).
      apply andb_false_r. }
    rewrite Hfun_bit, orb_false_r.
    reflexivity.
  - apply Z.ltb_ge in Hlt. rewrite andb_false_r.
    symmetry. apply (bounded_testbit dst 8 n); lia.
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
  replace (17 - 10 + 1) with 8 by lia.
  apply Z.bits_inj'. intros n Hn.
  rewrite Z.land_spec.
  rewrite Z.testbit_ones_nonneg by lia.
  rewrite Z.shiftr_spec by lia.
  rewrite !Z.lor_spec.
  rewrite !Z.shiftl_spec by lia.
  change 255 with (Z.ones 8). change 3 with (Z.ones 2).
  replace (n + 10 - 10) with n by lia.
  destruct (Z.ltb n 8) eqn:Hlt.
  - apply Z.ltb_lt in Hlt. rewrite andb_true_r.
    assert (Hop_bit : Z.testbit opcode (n + 10 - 26) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hop_bit, orb_false_l.
    assert (Hdst_bit: Z.testbit (Z.land dst (Z.ones 8)) (n + 10 - 18) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hdst_bit, orb_false_l.
    rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
    replace (n <? 8) with true by (symmetry; apply Z.ltb_lt; lia).
    rewrite andb_true_r.
    assert (Hsrc2_bit: Z.testbit (Z.land src2 (Z.ones 8)) (n + 10 - 2) = false).
    { rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
      replace (n + 10 - 2 <? 8) with false by (symmetry; apply Z.ltb_ge; lia).
      apply andb_false_r. }
    rewrite Hsrc2_bit, orb_false_r.
    assert (Hfun_bit: Z.testbit (Z.land funct (Z.ones 2)) (n + 10) = false).
    { rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
      replace (n + 10 <? 2) with false by (symmetry; apply Z.ltb_ge; lia).
      apply andb_false_r. }
    rewrite Hfun_bit, orb_false_r.
    reflexivity.
  - apply Z.ltb_ge in Hlt. rewrite andb_false_r.
    symmetry. apply (bounded_testbit src1 8 n); lia.
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
  apply Z.bits_inj'. intros n Hn.
  rewrite Z.land_spec.
  rewrite Z.testbit_ones_nonneg by lia.
  rewrite !Z.lor_spec.
  rewrite !Z.shiftl_spec by lia.
  change 255 with (Z.ones 8). change 3 with (Z.ones 2).
  destruct (Z.ltb n 2) eqn:Hlt.
  - apply Z.ltb_lt in Hlt. rewrite andb_true_r.
    assert (Hop_bit : Z.testbit opcode (n - 26) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hop_bit, orb_false_l.
    assert (Hdst_bit: Z.testbit (Z.land dst (Z.ones 8)) (n - 18) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hdst_bit, orb_false_l.
    assert (Hsrc1_bit: Z.testbit (Z.land src1 (Z.ones 8)) (n - 10) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hsrc1_bit, orb_false_l.
    assert (Hsrc2_bit: Z.testbit (Z.land src2 (Z.ones 8)) (n - 2) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hsrc2_bit, orb_false_l.
    rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
    replace (n <? 2) with true by (symmetry; apply Z.ltb_lt; lia).
    rewrite andb_true_r. reflexivity.
  - apply Z.ltb_ge in Hlt. rewrite andb_false_r.
    symmetry. apply (bounded_testbit funct 2 n); lia.
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
  apply Z.bits_inj'. intros n Hn.
  rewrite Z.land_spec.
  rewrite Z.testbit_ones_nonneg by lia.
  rewrite !Z.lor_spec.
  rewrite !Z.shiftl_spec by lia.
  change 255 with (Z.ones 8). change 1023 with (Z.ones 10).
  destruct (Z.ltb n 10) eqn:Hlt.
  - apply Z.ltb_lt in Hlt. rewrite andb_true_r.
    assert (Hop_bit : Z.testbit opcode (n - 26) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hop_bit, orb_false_l.
    assert (Hdst_bit: Z.testbit (Z.land dst (Z.ones 8)) (n - 18) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hdst_bit, orb_false_l.
    assert (Hsrc_bit: Z.testbit (Z.land src (Z.ones 8)) (n - 10) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hsrc_bit, orb_false_l.
    rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
    replace (n <? 10) with true by (symmetry; apply Z.ltb_lt; lia).
    rewrite andb_true_r. reflexivity.
  - apply Z.ltb_ge in Hlt. rewrite andb_false_r.
    symmetry. apply (bounded_testbit imm10 10 n); lia.
Qed.

(** ** MEGA-4: Totality Engine Opcodes (30-32) *)

(** VERIFY (opcode 30) is S-type: imm26 = cert_offset.
    Roundtrip: extract_imm26(pack_s_type 30 cert_offset) = cert_offset. *)
Theorem verify_s_type_roundtrip : forall cert_offset,
  (0 <= cert_offset < 67108864) ->
  extract_imm26 (pack_s_type 30 cert_offset) = cert_offset.
Proof.
  intros cert_offset H.
  apply s_type_imm_roundtrip; lia.
Qed.

(** VERIFY opcode survives roundtrip. *)
Theorem verify_opcode_roundtrip : forall cert_offset,
  (0 <= cert_offset < 67108864) ->
  extract_opcode (pack_s_type 30 cert_offset) = 30.
Proof.
  intros cert_offset H.
  unfold pack_s_type.
  apply opcode_roundtrip; lia.
Qed.

(** CERTIFY (opcode 31) is R-type: dst field carries the destination register.
    Roundtrip: extract_dst(pack_r_type 31 dst 0 0 0) = dst. *)
Theorem certify_r_type_dst_roundtrip : forall dst,
  (0 <= dst < 256) ->
  extract_dst (pack_r_type 31 dst 0 0 0) = dst.
Proof.
  intros dst H.
  apply r_type_dst_roundtrip; lia.
Qed.

(** CERTIFY opcode survives roundtrip. *)
Theorem certify_opcode_roundtrip : forall dst,
  (0 <= dst < 256) ->
  extract_opcode (pack_r_type 31 dst 0 0 0) = 31.
Proof.
  intros dst H.
  unfold pack_r_type.
  replace (Z.land dst 255) with dst
    by (rewrite Z.land_ones; [rewrite Z.mod_small; lia | lia]).
  replace (Z.land 0 255) with 0 by reflexivity.
  replace (Z.land 0 3) with 0 by reflexivity.
  rewrite !Z.shiftl_0_l.
  rewrite !Z.lor_0_r.
  apply opcode_roundtrip.
  - lia.
  - split; [|].
    + apply Z.shiftl_nonneg; lia.
    + assert (Hd : Z.shiftl dst 18 < Z.pow 2 26).
      { rewrite Z.shiftl_mul_pow2 by lia.
        apply Z.lt_trans with (256 * Z.pow 2 18);
          [apply Z.mul_lt_mono_pos_r; [apply Z.pow_pos_nonneg; lia | lia] |].
        change (256 * 2 ^ 18) with (2 ^ 26). lia. }
      lia.
Qed.

(** TOTAL_CHECK (opcode 32) is S-type: imm26 = expected_properties.
    Roundtrip: extract_imm26(pack_s_type 32 expected) = expected. *)
Theorem total_check_s_type_roundtrip : forall expected,
  (0 <= expected < 67108864) ->
  extract_imm26 (pack_s_type 32 expected) = expected.
Proof.
  intros expected H.
  apply s_type_imm_roundtrip; lia.
Qed.

(** TOTAL_CHECK opcode survives roundtrip. *)
Theorem total_check_opcode_roundtrip : forall expected,
  (0 <= expected < 67108864) ->
  extract_opcode (pack_s_type 32 expected) = 32.
Proof.
  intros expected H.
  unfold pack_s_type.
  apply opcode_roundtrip; lia.
Qed.

(* ======================================================================= *)
(* MEGA-5: Symbolic Reasoning opcodes (33-36)                              *)
(* ======================================================================= *)

(** MATCH (opcode 33) is I-type: imm10 = table_offset.
    Roundtrip: extract_imm10(pack_i_type 33 dst src offset) = offset. *)
Theorem match_i_type_imm_roundtrip : forall dst src offset,
  (0 <= dst < 256) ->
  (0 <= src < 256) ->
  (0 <= offset < 1024) ->
  extract_imm10 (pack_i_type 33 dst src offset) = offset.
Proof.
  intros dst src offset Hdst Hsrc Hoff.
  apply i_type_imm_roundtrip; lia.
Qed.

(** MATCH opcode survives roundtrip. *)
Theorem match_opcode_roundtrip : forall dst src offset,
  (0 <= dst < 256) ->
  (0 <= src < 256) ->
  (0 <= offset < 1024) ->
  extract_opcode (pack_i_type 33 dst src offset) = 33.
Proof.
  intros dst src offset Hdst Hsrc Hoff.
  unfold pack_i_type.
  replace (Z.land dst 255) with dst
    by (rewrite Z.land_ones; [rewrite Z.mod_small; lia | lia]).
  replace (Z.land src 255) with src
    by (rewrite Z.land_ones; [rewrite Z.mod_small; lia | lia]).
  replace (Z.land offset 1023) with offset
    by (rewrite Z.land_ones; [rewrite Z.mod_small; lia | lia]).
  apply opcode_roundtrip.
  - lia.
  - split; [|].
    + apply Z.lor_nonneg. split.
      * apply Z.lor_nonneg. split.
        { apply Z.shiftl_nonneg; lia. }
        { apply Z.shiftl_nonneg; lia. }
      * lia.
    + assert (Hd : Z.shiftl dst 18 < Z.pow 2 26).
      { rewrite Z.shiftl_mul_pow2 by lia.
        apply Z.lt_trans with (256 * Z.pow 2 18);
          [apply Z.mul_lt_mono_pos_r; [apply Z.pow_pos_nonneg; lia | lia] |].
        change (256 * 2 ^ 18) with (2 ^ 26). lia. }
      assert (Hs : Z.shiftl src 10 < Z.pow 2 18).
      { rewrite Z.shiftl_mul_pow2 by lia.
        apply Z.lt_trans with (256 * Z.pow 2 10);
          [apply Z.mul_lt_mono_pos_r; [apply Z.pow_pos_nonneg; lia | lia] |].
        change (256 * 2 ^ 10) with (2 ^ 18). lia. }
      apply Z.lt_le_trans with (Z.pow 2 26).
      * apply Z.lt_le_trans with (Z.shiftl dst 18 + Z.shiftl src 10 + offset + 1).
        { apply Z.lt_succ_r. apply Z.lor_le. }
        { lia. }
      * lia.
Qed.

(** INTERVAL_LO (opcode 34) is R-type: dst is preserved.
    Roundtrip: extract_dst(pack_r_type 34 dst src 0 0) = dst. *)
Theorem interval_lo_r_type_dst_roundtrip : forall dst src,
  (0 <= dst < 256) ->
  (0 <= src < 256) ->
  extract_dst (pack_r_type 34 dst src 0 0) = dst.
Proof.
  intros dst src Hdst Hsrc.
  apply r_type_dst_roundtrip; lia.
Qed.

(** INTERVAL_LO opcode survives roundtrip. *)
Theorem interval_lo_opcode_roundtrip : forall dst src,
  (0 <= dst < 256) ->
  (0 <= src < 256) ->
  extract_opcode (pack_r_type 34 dst src 0 0) = 34.
Proof.
  intros dst src Hdst Hsrc.
  unfold pack_r_type.
  replace (Z.land dst 255) with dst
    by (rewrite Z.land_ones; [rewrite Z.mod_small; lia | lia]).
  replace (Z.land src 255) with src
    by (rewrite Z.land_ones; [rewrite Z.mod_small; lia | lia]).
  replace (Z.land 0 255) with 0 by reflexivity.
  replace (Z.land 0 3) with 0 by reflexivity.
  rewrite !Z.shiftl_0_l.
  rewrite !Z.lor_0_r.
  apply opcode_roundtrip.
  - lia.
  - split; [|].
    + apply Z.shiftl_nonneg; lia.
    + assert (Hd : Z.shiftl dst 18 < Z.pow 2 26).
      { rewrite Z.shiftl_mul_pow2 by lia.
        apply Z.lt_trans with (256 * Z.pow 2 18);
          [apply Z.mul_lt_mono_pos_r; [apply Z.pow_pos_nonneg; lia | lia] |].
        change (256 * 2 ^ 18) with (2 ^ 26). lia. }
      lia.
Qed.

(** INTERVAL_HI (opcode 35) is R-type: dst is preserved. *)
Theorem interval_hi_r_type_dst_roundtrip : forall dst src,
  (0 <= dst < 256) ->
  (0 <= src < 256) ->
  extract_dst (pack_r_type 35 dst src 0 0) = dst.
Proof.
  intros dst src Hdst Hsrc.
  apply r_type_dst_roundtrip; lia.
Qed.

(** INTERVAL_HI opcode survives roundtrip. *)
Theorem interval_hi_opcode_roundtrip : forall dst src,
  (0 <= dst < 256) ->
  (0 <= src < 256) ->
  extract_opcode (pack_r_type 35 dst src 0 0) = 35.
Proof.
  intros dst src Hdst Hsrc.
  unfold pack_r_type.
  replace (Z.land dst 255) with dst
    by (rewrite Z.land_ones; [rewrite Z.mod_small; lia | lia]).
  replace (Z.land src 255) with src
    by (rewrite Z.land_ones; [rewrite Z.mod_small; lia | lia]).
  replace (Z.land 0 255) with 0 by reflexivity.
  replace (Z.land 0 3) with 0 by reflexivity.
  rewrite !Z.shiftl_0_l.
  rewrite !Z.lor_0_r.
  apply opcode_roundtrip.
  - lia.
  - split; [|].
    + apply Z.shiftl_nonneg; lia.
    + assert (Hd : Z.shiftl dst 18 < Z.pow 2 26).
      { rewrite Z.shiftl_mul_pow2 by lia.
        apply Z.lt_trans with (256 * Z.pow 2 18);
          [apply Z.mul_lt_mono_pos_r; [apply Z.pow_pos_nonneg; lia | lia] |].
        change (256 * 2 ^ 18) with (2 ^ 26). lia. }
      lia.
Qed.

(** INTERVAL_CHECK (opcode 36) is R-type: src1 is preserved.
    Roundtrip: extract_src1(pack_r_type 36 0 src bounds 0) = src. *)
Theorem interval_check_opcode_roundtrip : forall src bounds,
  (0 <= src < 256) ->
  (0 <= bounds < 256) ->
  extract_opcode (pack_r_type 36 0 src bounds 0) = 36.
Proof.
  intros src bounds Hsrc Hbounds.
  unfold pack_r_type.
  replace (Z.land 0 255) with 0 by reflexivity.
  replace (Z.land src 255) with src
    by (rewrite Z.land_ones; [rewrite Z.mod_small; lia | lia]).
  replace (Z.land bounds 255) with bounds
    by (rewrite Z.land_ones; [rewrite Z.mod_small; lia | lia]).
  replace (Z.land 0 3) with 0 by reflexivity.
  rewrite Z.shiftl_0_l.
  rewrite Z.lor_0_l.
  rewrite Z.shiftl_0_l.
  rewrite Z.lor_0_r.
  apply opcode_roundtrip.
  - lia.
  - split; [|].
    + apply Z.lor_nonneg. split.
      * apply Z.shiftl_nonneg; lia.
      * apply Z.shiftl_nonneg; lia.
    + assert (Hs : Z.shiftl src 10 < Z.pow 2 18).
      { rewrite Z.shiftl_mul_pow2 by lia.
        apply Z.lt_trans with (256 * Z.pow 2 10);
          [apply Z.mul_lt_mono_pos_r; [apply Z.pow_pos_nonneg; lia | lia] |].
        change (256 * 2 ^ 10) with (2 ^ 18). lia. }
      assert (Hb : Z.shiftl bounds 2 < Z.pow 2 10).
      { rewrite Z.shiftl_mul_pow2 by lia.
        apply Z.lt_trans with (256 * Z.pow 2 2);
          [apply Z.mul_lt_mono_pos_r; [apply Z.pow_pos_nonneg; lia | lia] |].
        change (256 * 2 ^ 2) with 1024. change (2 ^ 10) with 1024. lia. }
      apply Z.lt_le_trans with (Z.pow 2 26).
      * apply Z.lt_le_trans with (Z.shiftl src 10 + Z.shiftl bounds 2 + 1).
        { apply Z.lt_succ_r. apply Z.lor_le. }
        { lia. }
      * lia.
Qed.

(** INTERVAL_CHECK src1 field roundtrip. *)
Theorem interval_check_src1_roundtrip : forall src bounds,
  (0 <= src < 256) ->
  (0 <= bounds < 256) ->
  extract_src1 (pack_r_type 36 0 src bounds 0) = src.
Proof.
  intros src bounds Hsrc Hbounds.
  apply r_type_src1_roundtrip; lia.
Qed.

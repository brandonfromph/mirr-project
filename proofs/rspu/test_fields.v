From Coq Require Import ZArith.
From Coq Require Import Bool.
From Coq Require Import Lia.
Open Scope Z_scope.

Definition word32 := Z.
Definition extract_bits (w : word32) (lo hi : Z) : Z :=
  Z.land (Z.shiftr w lo) (Z.ones (hi - lo + 1)).

Definition pack_r_type (opcode dst src1 src2 funct : Z) : word32 :=
  Z.lor (Z.lor (Z.lor (Z.lor (Z.shiftl opcode 26)
                               (Z.shiftl (Z.land dst 255) 18))
                        (Z.shiftl (Z.land src1 255) 10))
                 (Z.shiftl (Z.land src2 255) 2))
         (Z.land funct 3).

Definition pack_i_type (opcode dst src imm10 : Z) : word32 :=
  Z.lor (Z.lor (Z.lor (Z.shiftl opcode 26)
                        (Z.shiftl (Z.land dst 255) 18))
                 (Z.shiftl (Z.land src 255) 10))
         (Z.land imm10 1023).

Definition extract_dst (w : word32) : Z := extract_bits w 18 25.
Definition extract_src1 (w : word32) : Z := extract_bits w 10 17.
Definition extract_src2 (w : word32) : Z := extract_bits w 2 9.
Definition extract_funct (w : word32) : Z := extract_bits w 0 1.
Definition extract_imm10 (w : word32) : Z := extract_bits w 0 9.

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

(** Tactic: after unfolding pack/extract/extract_bits and
    rewriting with Z.land_spec, Z.testbit_ones_nonneg, Z.shiftr_spec,
    Z.lor_spec, Z.shiftl_spec, we get a goal of the form:

      (... || ... || ... || ...) && (n <? width) = Z.testbit target n

    Strategy: case split on (n <? width), then in the true case
    show each non-target field contributes false via Z.testbit_neg_r
    or bounded_testbit, simplify boolean, get reflexivity.
    In the false case, bounded_testbit closes it. *)

(** R-type dst: bits [18..25] *)
Theorem r_type_dst_roundtrip : forall opcode dst src1 src2 funct,
  (0 <= opcode < 64) -> (0 <= dst < 256) -> (0 <= src1 < 256) ->
  (0 <= src2 < 256) -> (0 <= funct < 4) ->
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
  (* Don't expand Z.land_spec for everything. Instead case split on n. *)
  replace (n + 18 - 18) with n by lia.
  destruct (Z.ltb n 8) eqn:Hlt.
  - apply Z.ltb_lt in Hlt. rewrite andb_true_r.
    (* opcode: n+18-26 < 0 *)
    assert (Hop_bit : Z.testbit opcode (n + 18 - 26) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hop_bit, orb_false_l.
    (* dst: Z.testbit (Z.land dst (Z.ones 8)) n = Z.testbit dst n *)
    rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
    replace (n <? 8) with true by (symmetry; apply Z.ltb_lt; lia).
    rewrite andb_true_r.
    (* src1: Z.testbit (Z.land src1 (Z.ones 8)) (n+18-10) where n+18-10 = n+8 >= 8 *)
    assert (Hsrc1_bit: Z.testbit (Z.land src1 (Z.ones 8)) (n + 18 - 10) = false).
    { rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
      replace (n + 18 - 10 <? 8) with false by (symmetry; apply Z.ltb_ge; lia).
      apply andb_false_r. }
    rewrite Hsrc1_bit, orb_false_r.
    (* src2: n+18-2 = n+16 >= 8 *)
    assert (Hsrc2_bit: Z.testbit (Z.land src2 (Z.ones 8)) (n + 18 - 2) = false).
    { rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
      replace (n + 18 - 2 <? 8) with false by (symmetry; apply Z.ltb_ge; lia).
      apply andb_false_r. }
    rewrite Hsrc2_bit, orb_false_r.
    (* funct: n+18 >= 2 *)
    assert (Hfun_bit: Z.testbit (Z.land funct (Z.ones 2)) (n + 18) = false).
    { rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
      replace (n + 18 <? 2) with false by (symmetry; apply Z.ltb_ge; lia).
      apply andb_false_r. }
    rewrite Hfun_bit, orb_false_r.
    reflexivity.
  - apply Z.ltb_ge in Hlt. rewrite andb_false_r.
    symmetry. apply (bounded_testbit dst 8 n); lia.
Qed.

(** R-type src1: bits [10..17] *)
Theorem r_type_src1_roundtrip : forall opcode dst src1 src2 funct,
  (0 <= opcode < 64) -> (0 <= dst < 256) -> (0 <= src1 < 256) ->
  (0 <= src2 < 256) -> (0 <= funct < 4) ->
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
    (* opcode: n+10-26 < 0 *)
    assert (Hop_bit : Z.testbit opcode (n + 10 - 26) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hop_bit, orb_false_l.
    (* dst: n+10-18 = n-8 < 0 for n < 8 *)
    assert (Hdst_bit: Z.testbit (Z.land dst (Z.ones 8)) (n + 10 - 18) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hdst_bit, orb_false_l.
    (* src1: Z.testbit (Z.land src1 (Z.ones 8)) n *)
    rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
    replace (n <? 8) with true by (symmetry; apply Z.ltb_lt; lia).
    rewrite andb_true_r.
    (* src2: n+10-2 = n+8 >= 8 *)
    assert (Hsrc2_bit: Z.testbit (Z.land src2 (Z.ones 8)) (n + 10 - 2) = false).
    { rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
      replace (n + 10 - 2 <? 8) with false by (symmetry; apply Z.ltb_ge; lia).
      apply andb_false_r. }
    rewrite Hsrc2_bit, orb_false_r.
    (* funct: n+10 >= 2 *)
    assert (Hfun_bit: Z.testbit (Z.land funct (Z.ones 2)) (n + 10) = false).
    { rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
      replace (n + 10 <? 2) with false by (symmetry; apply Z.ltb_ge; lia).
      apply andb_false_r. }
    rewrite Hfun_bit, orb_false_r.
    reflexivity.
  - apply Z.ltb_ge in Hlt. rewrite andb_false_r.
    symmetry. apply (bounded_testbit src1 8 n); lia.
Qed.

(** R-type funct: bits [0..1] *)
Theorem r_type_funct_roundtrip : forall opcode dst src1 src2 funct,
  (0 <= opcode < 64) -> (0 <= dst < 256) -> (0 <= src1 < 256) ->
  (0 <= src2 < 256) -> (0 <= funct < 4) ->
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
    (* opcode: n-26 < 0 *)
    assert (Hop_bit : Z.testbit opcode (n - 26) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hop_bit, orb_false_l.
    (* dst: n-18 < 0 *)
    assert (Hdst_bit: Z.testbit (Z.land dst (Z.ones 8)) (n - 18) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hdst_bit, orb_false_l.
    (* src1: n-10 < 0 *)
    assert (Hsrc1_bit: Z.testbit (Z.land src1 (Z.ones 8)) (n - 10) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hsrc1_bit, orb_false_l.
    (* src2: n-2 < 0 *)
    assert (Hsrc2_bit: Z.testbit (Z.land src2 (Z.ones 8)) (n - 2) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hsrc2_bit, orb_false_l.
    (* funct: Z.testbit (Z.land funct (Z.ones 2)) n *)
    rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
    replace (n <? 2) with true by (symmetry; apply Z.ltb_lt; lia).
    rewrite andb_true_r. reflexivity.
  - apply Z.ltb_ge in Hlt. rewrite andb_false_r.
    symmetry. apply (bounded_testbit funct 2 n); lia.
Qed.

(** I-type imm10: bits [0..9] *)
Theorem i_type_imm_roundtrip : forall opcode dst src imm10,
  (0 <= opcode < 64) -> (0 <= dst < 256) -> (0 <= src < 256) ->
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
    (* opcode: n-26 < 0 *)
    assert (Hop_bit : Z.testbit opcode (n - 26) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hop_bit, orb_false_l.
    (* dst: n-18 < 0 *)
    assert (Hdst_bit: Z.testbit (Z.land dst (Z.ones 8)) (n - 18) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hdst_bit, orb_false_l.
    (* src: n-10 < 0 *)
    assert (Hsrc_bit: Z.testbit (Z.land src (Z.ones 8)) (n - 10) = false)
      by (apply Z.testbit_neg_r; lia).
    rewrite Hsrc_bit, orb_false_l.
    (* imm10: Z.testbit (Z.land imm10 (Z.ones 10)) n *)
    rewrite Z.land_spec, Z.testbit_ones_nonneg by lia.
    replace (n <? 10) with true by (symmetry; apply Z.ltb_lt; lia).
    rewrite andb_true_r. reflexivity.
  - apply Z.ltb_ge in Hlt. rewrite andb_false_r.
    symmetry. apply (bounded_testbit imm10 10 n); lia.
Qed.

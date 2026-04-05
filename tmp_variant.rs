enum E { A } fn main(){ let x=E::A; let _=matches!(x, E::A { .. }); }

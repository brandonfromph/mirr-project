#![forbid(unsafe_code)]
//! Master registry file for the 14 stress audit bug hunts.
//! This activates previously unreferenced/dead integration tests under `tests/stress_audit/`.

#[path = "stress_audit/bug_hunt_01.rs"]
mod bug_hunt_01;

#[path = "stress_audit/bug_hunt_02.rs"]
mod bug_hunt_02;

#[path = "stress_audit/bug_hunt_03.rs"]
mod bug_hunt_03;

#[path = "stress_audit/bug_hunt_04.rs"]
mod bug_hunt_04;

#[path = "stress_audit/bug_hunt_05.rs"]
mod bug_hunt_05;

#[path = "stress_audit/bug_hunt_06.rs"]
mod bug_hunt_06;

#[path = "stress_audit/bug_hunt_07.rs"]
mod bug_hunt_07;

#[path = "stress_audit/bug_hunt_08.rs"]
mod bug_hunt_08;

#[path = "stress_audit/bug_hunt_09.rs"]
mod bug_hunt_09;

#[path = "stress_audit/bug_hunt_10.rs"]
mod bug_hunt_10;

#[path = "stress_audit/bug_hunt_11.rs"]
mod bug_hunt_11;

#[path = "stress_audit/bug_hunt_12.rs"]
mod bug_hunt_12;

#[path = "stress_audit/bug_hunt_13.rs"]
mod bug_hunt_13;

#[path = "stress_audit/bug_hunt_14.rs"]
mod bug_hunt_14;

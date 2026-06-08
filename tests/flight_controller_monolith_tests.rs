#[cfg(test)]
mod tests {
    use mirrc::pipeline::{run_pipeline, PipelineConfig};

    #[test]
    fn test_flight_controller_monolith() {
        let src = r#"
module autonomous_flight_controller {
    signal alt_sensor_a:     in u16;
    signal alt_sensor_b:     in u16;
    signal alt_sensor_c:     in u16;
    
    signal alt_a_ok:         in bool;
    signal alt_b_ok:         in bool;
    signal alt_c_ok:         in bool;
    
    signal pitch_angle:      in u16;
    signal roll_angle:       in u16;
    signal radio_heartbeat:  in bool;
    
    signal throttle_cut:     out bool;
    signal parachute_deploy: out bool;
    signal stabilise_mode:   out bool;
    signal safe_altitude:    out u16;

    guard radio_lost {
        when !radio_heartbeat
        for 50 cycles;
    }

    guard radio_stable {
        when radio_heartbeat
        for 5 cycles;
    }

    guard excessive_pitch_dive {
        when pitch_angle > 45
        for 10 cycles;
    }

    guard ground_proximity {
        when (alt_a_ok && alt_sensor_a < 100) || (alt_b_ok && alt_sensor_b < 100) || (alt_c_ok && alt_sensor_c < 100)
        for 5 cycles;
    }

    guard total_sensor_failure {
        when !alt_a_ok && !alt_b_ok && !alt_c_ok
        for 2 cycles;
    }

    reflex alt_vote_a {
        on radio_stable {
            safe_altitude = alt_sensor_a;
        }
    }

    reflex auto_stabilise {
        on excessive_pitch_dive {
            stabilise_mode = true;
        }
    }

    reflex emergency_parachute {
        on total_sensor_failure {
            parachute_deploy = true;
            throttle_cut = true;
        }
    }
    
    reflex failsafe_descent {
        on radio_lost {
            stabilise_mode = true;
        }
    }

    property deploy_on_total_failure {
        always (total_sensor_failure -> parachute_deploy);
    }

    property no_throttle_with_parachute {
        always (parachute_deploy -> throttle_cut);
    }

    property auto_stabilize_on_radio_loss {
        always (radio_lost -> stabilise_mode);
    }
    
    property radio_state_mutually_exclusive {
        always (!(radio_lost && radio_stable));
    }
}
        "#;

        let config = PipelineConfig {
            typecheck: true,
            temporal: true,
            width: true,
            simplify: true,
            ..Default::default()
        };

        let result = run_pipeline(src, &config);

        match &result {
            Ok(_) => println!("Compilation successful!"),
            Err(e) => {
                for err in &e.errors {
                    println!("ERROR: {:?}", err);
                }
                panic!("Compilation failed!");
            }
        }
    }
}

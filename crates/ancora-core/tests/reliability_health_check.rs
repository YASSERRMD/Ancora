// Reliability: health check endpoint returns correct status per subsystem.

#[derive(Debug, PartialEq, Clone)]
enum Health {
    Ok,
    Degraded(String),
    Down(String),
}

struct Subsystem {
    name: &'static str,
    health: Health,
}

fn aggregate_health(subsystems: &[Subsystem]) -> Health {
    let down: Vec<&str> = subsystems
        .iter()
        .filter(|s| matches!(s.health, Health::Down(_)))
        .map(|s| s.name)
        .collect();
    let degraded: Vec<&str> = subsystems
        .iter()
        .filter(|s| matches!(s.health, Health::Degraded(_)))
        .map(|s| s.name)
        .collect();

    if !down.is_empty() {
        Health::Down(format!("down: {}", down.join(",")))
    } else if !degraded.is_empty() {
        Health::Degraded(format!("degraded: {}", degraded.join(",")))
    } else {
        Health::Ok
    }
}

#[test]
fn test_all_ok_returns_ok() {
    let subs = vec![
        Subsystem {
            name: "db",
            health: Health::Ok,
        },
        Subsystem {
            name: "cache",
            health: Health::Ok,
        },
    ];
    assert_eq!(aggregate_health(&subs), Health::Ok);
}

#[test]
fn test_one_down_returns_down() {
    let subs = vec![
        Subsystem {
            name: "db",
            health: Health::Down("conn refused".to_string()),
        },
        Subsystem {
            name: "cache",
            health: Health::Ok,
        },
    ];
    assert!(matches!(aggregate_health(&subs), Health::Down(_)));
}

#[test]
fn test_degraded_only_returns_degraded() {
    let subs = vec![
        Subsystem {
            name: "db",
            health: Health::Ok,
        },
        Subsystem {
            name: "cache",
            health: Health::Degraded("slow".to_string()),
        },
    ];
    assert!(matches!(aggregate_health(&subs), Health::Degraded(_)));
}

#[test]
fn test_down_takes_priority_over_degraded() {
    let subs = vec![
        Subsystem {
            name: "db",
            health: Health::Degraded("slow".to_string()),
        },
        Subsystem {
            name: "cache",
            health: Health::Down("crash".to_string()),
        },
    ];
    assert!(matches!(aggregate_health(&subs), Health::Down(_)));
}

#[test]
fn test_down_message_includes_subsystem_name() {
    let subs = vec![Subsystem {
        name: "journal-store",
        health: Health::Down("io error".to_string()),
    }];
    if let Health::Down(msg) = aggregate_health(&subs) {
        assert!(msg.contains("journal-store"));
    } else {
        panic!("expected Down");
    }
}

#[test]
fn test_empty_subsystem_list_returns_ok() {
    let subs: Vec<Subsystem> = vec![];
    assert_eq!(aggregate_health(&subs), Health::Ok);
}

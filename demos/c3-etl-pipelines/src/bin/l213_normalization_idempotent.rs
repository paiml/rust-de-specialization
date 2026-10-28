fn normalize_name(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

fn normalize_category(s: &str) -> String {
    s.trim().to_lowercase()
}

fn main() {
    assert_eq!(normalize_name("  Green   APPLE "), "green apple");
    assert_eq!(normalize_category(" Tropical\t"), "tropical");
    // Idempotence: normalizing twice equals normalizing once.
    for raw in ["  Green   APPLE ", "banana", "  ", "MiXeD  Case"] {
        let once = normalize_name(raw);
        assert_eq!(
            normalize_name(&once),
            once,
            "normalize must be idempotent for {raw:?}"
        );
    }
    for raw in [" Tropical\t", "pome", "", "  MIXED case "] {
        let once = normalize_category(raw);
        assert_eq!(
            normalize_category(&once),
            once,
            "normalize must be idempotent for {raw:?}"
        );
    }
    println!("normalize: idempotent");
}

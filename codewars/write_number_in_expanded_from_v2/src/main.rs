fn main() {
    println!("{}", expanded_form(209603948.5293))
}

fn expanded_form(num: f64) -> String {
    num.to_string()
        .split('.')
        .collect::<Vec<_>>()
        .iter()
        .enumerate()
        .map(|(i, &j)| {
            j.chars()
                .enumerate()
                .filter(|&(_, c)| c != '0')
                .map(|(k, l)| match i {
                    0 => format!("{}{}", l, "0".repeat(j.len() - k - 1)),
                    _ => format!("{}{}{}", l, "/1", "0".repeat(k + 1)),
                })
                .collect::<Vec<_>>()
                .join(" + ")
        })
        .collect::<Vec<_>>()
        .join(" + ")
}

#[test]
fn test_expanded_from() {
    assert_eq!(
        expanded_form(1568.156),
        "1000 + 500 + 60 + 8 + 1/10 + 5/100 + 6/1000"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            expanded_form(1568.156),
            "1000 + 500 + 60 + 8 + 1/10 + 5/100 + 6/1000"
        );
        assert_eq!(
            expanded_form(1278.8766),
            "1000 + 200 + 70 + 8 + 8/10 + 7/100 + 6/1000 + 6/10000"
        );
        assert_eq!(
            expanded_form(4982.342),
            "4000 + 900 + 80 + 2 + 3/10 + 4/100 + 2/1000"
        );
    }

    #[test]
    fn edge_case_zeros() {
        assert_eq!(
            expanded_form(42037.0022),
            "40000 + 2000 + 30 + 7 + 2/1000 + 2/10000"
        );
        assert_eq!(expanded_form(70.304), "70 + 3/10 + 4/1000");
        assert_eq!(expanded_form(9000.000), "9000");
    }

    #[test]
    fn edge_case_big_numbers() {
        assert_eq!(
            expanded_form(92093403.034573),
            "90000000 + 2000000 + 90000 + 3000 + 400 + 3 + 3/100 + 4/1000 + 5/10000 + 7/100000 + 3/1000000"
        );
        assert_eq!(
            expanded_form(209603948.5293),
            "200000000 + 9000000 + 600000 + 3000 + 900 + 40 + 8 + 5/10 + 2/100 + 9/1000 + 3/10000"
        );
    }

    use rand::{thread_rng, Rng};

    #[test]
    fn random() {
        let mut rng = thread_rng();

        for _ in 0..100 {
            let n = rng.gen_range(1.0f64..=100_000_000_000_000.0f64);

            assert_eq!(expanded_form(n), expanded_form_solution(n));
        }
    }

    fn expanded_form_solution(n: f64) -> String {
        n.to_string()
        .split('.')
        .collect::<Vec<_>>()
        .iter()
        .enumerate()
        .map(|(i, &j)| {
            j.chars()
                .enumerate()
                .filter(|&(_, c)| c != '0')
                .map(|(k, l)| match i {
                    0 => format!("{}{}", l, "0".repeat(j.len() - k - 1)),
                    _ => format!("{}{}{}", l, "/1", "0".repeat(k + 1)),
                })
                .collect::<Vec<_>>()
                .join(" + ")
        })
        .collect::<Vec<_>>()
        .join(" + ")
    }
}

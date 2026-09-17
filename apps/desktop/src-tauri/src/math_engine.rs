use regex::Regex;

use crate::error::{ScoopError, ScoopResult};

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MathResult {
    pub expression: String,
    pub normalized: String,
    pub answer: String,
}

pub fn evaluate(raw: &str) -> ScoopResult<MathResult> {
    let expression = raw.trim().lines().next().unwrap_or("").trim().to_string();
    if expression.is_empty() {
        return Err(ScoopError::msg("Empty expression"));
    }

    let normalized = normalize_expression(&expression)?;
    let value = meval::eval_str(&normalized)
        .map_err(|e| ScoopError::msg(format!("Could not evaluate: {e}")))?;

    let answer = format_answer(value, &expression);
    Ok(MathResult {
        expression,
        normalized,
        answer,
    })
}

fn normalize_expression(expr: &str) -> ScoopResult<String> {
    let mut s = expr.to_string();

    // Strip currency symbols for evaluation
    s = s.replace(['₹', '$', '€', '£'], "");

    // Indian / western grouping commas
    s = s.replace(',', "");

    // Operators
    s = s.replace('×', "*").replace('·', "*").replace('÷', "/");
    s = s.replace('^', "**"); // meval uses **? Actually meval uses ^ for power
    s = s.replace("**", "^");

    // sqrt( )
    let sqrt_re = Regex::new(r"(?i)sqrt\s*\(([^)]*)\)").unwrap();
    s = sqrt_re.replace_all(&s, "sqrt($1)").to_string();

    // "15% of 4500" → (15/100)*4500
    let pct_of = Regex::new(r"(?i)([\d.]+)\s*%\s*of\s*([\d.]+)").unwrap();
    if let Some(c) = pct_of.captures(&s) {
        let a = &c[1];
        let b = &c[2];
        s = format!("({a}/100)*{b}");
        return Ok(s);
    }

    // "Y × X%" or "Y * X%" → Y * (X/100)
    let times_pct = Regex::new(r"([\d.]+)\s*\*\s*([\d.]+)\s*%").unwrap();
    s = times_pct.replace_all(&s, "$1*($2/100)").to_string();

    // Trailing standalone percent after a number in a product chain: e.g. 12%
    // Convert remaining N% to (N/100) when not already rewritten
    let lone_pct = Regex::new(r"([\d.]+)\s*%").unwrap();
    s = lone_pct.replace_all(&s, "($1/100)").to_string();

    // Whitespace cleanup
    s = s.split_whitespace().collect::<Vec<_>>().join("");

    if s.is_empty() {
        return Err(ScoopError::msg("Could not normalize expression"));
    }
    Ok(s)
}

fn format_answer(value: f64, original: &str) -> String {
    let rounded = if (value - value.round()).abs() < 1e-9 {
        format!("{}", value.round() as i64)
    } else {
        format!("{:.4}", value)
            .trim_end_matches('0')
            .trim_end_matches('.')
            .to_string()
    };

    if original.contains('₹') {
        format!("₹{rounded}")
    } else if original.contains('$') {
        format!("${rounded}")
    } else {
        rounded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power() {
        let r = evaluate("2^10").unwrap();
        assert_eq!(r.answer, "1024");
    }

    #[test]
    fn sqrt_case() {
        let r = evaluate("sqrt(144)").unwrap();
        assert_eq!(r.answer, "12");
    }

    #[test]
    fn percent_of() {
        let r = evaluate("15% of 4500").unwrap();
        assert_eq!(r.answer, "675");
    }

    #[test]
    fn simple_div() {
        let r = evaluate("(12 × 45) / 3").unwrap();
        assert_eq!(r.answer, "180");
    }

    #[test]
    fn currency_chain() {
        let r = evaluate("₹2,40,000 × 8.5% × 4").unwrap();
        assert_eq!(r.answer, "₹81600");
    }
}

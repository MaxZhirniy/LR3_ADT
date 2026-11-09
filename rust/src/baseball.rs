use crate::stack::Stack;

pub fn baseball(args: &[&str]) -> Result<String, String> {
    if args.len() < 2 { return Err("missing operations".into()); }
    let mut scores = Stack::default();
    let mut size = 0usize;
    let mut sum = 0i64;
    for action in &args[1..] {
        let score = match *action {
            "C" => {
                if size == 0 { return Err("not enough scores for C".into()); }
                let removed = scores.pop()?.parse::<i64>().map_err(|e| e.to_string())?;
                size -= 1;
                sum -= removed;
                continue;
            }
            "D" => {
                if size == 0 { return Err("not enough scores for D".into()); }
                2 * scores.top()?.parse::<i64>().map_err(|e| e.to_string())?
            }
            "+" => {
                if size < 2 { return Err("not enough scores for +".into()); }
                let top = scores.pop()?;
                let second = scores.top()?.parse::<i64>().map_err(|e| e.to_string())?;
                let score = top.parse::<i64>().map_err(|e| e.to_string())? + second;
                scores.push(top);
                score
            }
            _ => {
                let value = action.parse::<i64>().map_err(|_| format!("invalid integer: {action}"))?;
                if value <= 0 { return Err("score must be positive".into()); }
                value
            }
        };
        scores.push(score.to_string());
        size += 1;
        sum += score;
    }
    Ok(sum.to_string())
}

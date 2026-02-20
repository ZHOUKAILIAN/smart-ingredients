pub fn conclusion_label(score: i32) -> &'static str {
    match score {
        75..=100 => "可吃",
        50..=74 => "谨慎",
        _ => "不推荐",
    }
}

#[cfg(test)]
mod tests {
    use super::conclusion_label;

    #[test]
    fn conclusion_label_ranges() {
        assert_eq!(conclusion_label(80), "可吃");
        assert_eq!(conclusion_label(60), "谨慎");
        assert_eq!(conclusion_label(20), "不推荐");
    }
}

pub fn split_params(s: &str) -> Vec<String> {
  let mut params = Vec::new();
  let mut depth = 0;
  let mut start = 0;

  for (i, c) in s.char_indices() {
    match c {
      '(' => depth += 1,
      ')' => {
        if depth > 0 {
          depth -= 1;
        }
      }
      ',' if depth == 0 => {
        params.push(s[start..i].trim().to_string());
        start = i + 1;
      }
      _ => {}
    }
  }

  // Push the last param if any
  if start < s.len() {
    params.push(s[start..].trim().to_string());
  }

  params
}

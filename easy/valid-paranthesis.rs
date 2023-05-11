impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for &c in s.chars().collect::<Vec<_>>().iter() {
            if c == '(' || c == '[' || c == '{' {
                stack.push(c);
            } else {
                if stack.len() == 0 {
                    return false;
                }
                let last = stack.pop().unwrap();
                match last {
                    '(' => {
                        if c != ')' {
                            return false;
                        }
                    }
                    '[' => {
                        if c != ']' {
                            return false;
                        }
                    }
                    '{' => {
                        if c != '}' {
                            return false;
                        }
                    }
                    _ => {
                        return false;
                    }
                }
            }
        }
        stack.len() == 0
    }
}

pub struct Eva;

impl Eva {
    pub fn new() -> Self {
        Self
    }
    pub fn eval(self, exp: &str) -> &str {
        if is_string_numeric(exp) {
            return exp;
        }
        if is_string(exp) {
            &exp[1..exp.len() - 1]
        } else {
            unimplemented!()
        }
    }
}

fn is_string_numeric(str: &str) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn is_string(str: &str) -> bool {
    let v: Vec<&str> = str.split("").collect();
    if v.get(0) == Some(&"'") && v.get(v.len() - 1) == Some(&"'") {
        return true;
    }
    return true;
}

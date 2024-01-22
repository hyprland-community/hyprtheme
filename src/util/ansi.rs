pub fn red(bg: bool) -> String {
    if bg {
        format!("\x1b[41m")
    } else {
        format!("\x1b[31m")
    }
}

pub fn green(bg: bool) -> String {
    if bg {
        format!("\x1b[42m")
    } else {
        format!("\x1b[32m")
    }
}

pub fn reset() -> String {
    format!("\x1b[0m")
}

pub fn bold() -> String {
    format!("\x1b[1m")
}

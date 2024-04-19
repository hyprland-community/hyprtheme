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

// pub fn yellow(bg: bool) -> String {
//     if bg {
//         format!("\x1b[43m")
//     } else {
//         format!("\x1b[33m")
//     }
// }

// pub fn blue(bg: bool) -> String {
//     if bg {
//         format!("\x1b[44m")
//     } else {
//         format!("\x1b[34m")
//     }
// }

// pub fn magenta(bg: bool) -> String {
//     if bg {
//         format!("\x1b[45m")
//     } else {
//         format!("\x1b[35m")
//     }
// }

// pub fn cyan(bg: bool) -> String {
//     if bg {
//         format!("\x1b[46m")
//     } else {
//         format!("\x1b[36m")
//     }
// }

// pub fn white(bg: bool) -> String {
//     if bg {
//         format!("\x1b[47m")
//     } else {
//         format!("\x1b[37m")
//     }
// }

// pub fn black(bg: bool) -> String {
//     if bg {
//         format!("\x1b[40m")
//     } else {
//         format!("\x1b[30m")
//     }
// }

// pub fn rgb(r: u8, g: u8, b: u8) -> String {
//     format!("\x1b[38;2;{};{};{}m", r, g, b)
// }

// pub fn rgb_bg(r: u8, g: u8, b: u8) -> String {
//     format!("\x1b[48;2;{};{};{}m", r, g, b)
// }

// pub fn italic() -> String {
//     format!("\x1b[3m")
// }

/* pub fn underline() -> String {
    format!("\x1b[4m")
} */

// pub fn blink() -> String {
//     format!("\x1b[5m")
// }

// pub fn reverse() -> String {
//     format!("\x1b[7m")
// }

// pub fn conceal() -> String {
//     format!("\x1b[8m")
// }

// pub fn strike() -> String {
//     format!("\x1b[9m")
// }

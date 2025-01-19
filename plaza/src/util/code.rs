use rand::seq::SliceRandom;

use crate::Result;

const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &[u8] = b"0123456789";
const SYMBOL: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?/~`";

/// 生成指定长度的码
pub fn generate_code(
    length: usize,
    upper: bool,
    lower: bool,
    num: bool,
    symbol: bool,
) -> Result<String> {
    if length < 1 {
        return Err(anyhow::anyhow!("密码长度至少为1").into());
    }

    let mut code = Vec::with_capacity(length as usize + 4); // 4是为了保证密码中至少包含一个指定类型
    let mut chars = vec![];
    let mut rng = rand::thread_rng();

    if upper {
        chars.extend_from_slice(UPPERCASE);
        // 保证密码中至少包含一个大写字母
        code.push(*UPPERCASE.choose(&mut rng).unwrap());
    }

    if lower {
        chars.extend_from_slice(LOWERCASE);

        code.push(*LOWERCASE.choose(&mut rng).unwrap());
    }

    if num {
        chars.extend_from_slice(NUMBERS);
        code.push(*NUMBERS.choose(&mut rng).unwrap());
    }

    if symbol {
        chars.extend_from_slice(SYMBOL);
        code.push(*SYMBOL.choose(&mut rng).unwrap());
    }

    if chars.is_empty() {
        return Err(anyhow::anyhow!("至少指定一种类型").into());
    }

    if code.len() < length as usize {
        for _ in 0..(length - code.len()) {
            let c = *chars.choose(&mut rng).unwrap();
            code.push(c);
        }
    }

    code.shuffle(&mut rng);

    let code = String::from_utf8(code)?[..length as usize].to_string();

    Ok(code)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_code() {
        let code = super::generate_code(6, false, false, true, false).unwrap();
        assert_eq!(code.len(), 6);
        assert_eq!("", code);
    }
}

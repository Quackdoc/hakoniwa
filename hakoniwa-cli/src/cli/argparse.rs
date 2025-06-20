use std::env;

pub(crate) fn contains_arg(arg: &str) -> bool {
    for a in env::args() {
        match a.as_str() {
            "--" => return false,
            a if a == arg => return true,
            _ => {}
        }
    }
    false
}

pub(crate) fn contains_arg_landlock() -> bool {
    for a in env::args() {
        match a.as_str() {
            "--" => return false,
            a if a.contains("--landlock") => return true,
            _ => {}
        }
    }
    false
}

pub(crate) fn contains_arg_raw() -> bool {
    match env::args().position(|arg| arg == "--") {
        Some(pos) => pos + 1 != env::args().len(),
        None => false,
    }
}

pub(crate) fn parse_rootdir<T, U>(
    s: &str,
) -> Result<(T, U), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    match s.find(':') {
        Some(pos) => Ok((s[..pos].parse()?, s[pos + 1..].parse()?)),
        None => Ok((s.parse()?, "".parse()?)),
    }
}

pub(crate) fn parse_network<T, U>(
    s: &str,
) -> Result<(T, U), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    match s.find(':') {
        Some(pos) => Ok((s[..pos].parse()?, s[pos + 1..].parse()?)),
        None => Ok((s.parse()?, "".parse()?)),
    }
}

pub(crate) fn parse_network_options(s: &str) -> anyhow::Result<Vec<String>> {
    if s.is_empty() {
        Ok(vec![])
    } else {
        Ok(s.split(',').map(|s| s.to_string()).collect())
    }
}

pub(crate) fn parse_bindmount<T, U>(
    s: &str,
) -> Result<(T, U), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    match s.find(':') {
        Some(pos) => Ok((s[..pos].parse()?, s[pos + 1..].parse()?)),
        None => Ok((s.parse()?, s.parse()?)),
    }
}

pub(crate) fn parse_symlink<T, U>(
    s: &str,
) -> Result<(T, U), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    match s.find(':') {
        Some(pos) => Ok((s[..pos].parse()?, s[pos + 1..].parse()?)),
        None => Ok((s.parse()?, s.parse()?)),
    }
}

pub(crate) fn parse_setenv<T, U>(
    s: &str,
) -> Result<(T, U), Box<dyn std::error::Error + Send + Sync + 'static>>
where
    T: std::str::FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
    U: std::str::FromStr,
    U::Err: std::error::Error + Send + Sync + 'static,
{
    match s.find(['=']) {
        Some(pos) => Ok((s[..pos].parse()?, s[pos + 1..].parse()?)),
        None => match env::var(s) {
            Ok(v) => Ok((s.parse()?, v.parse()?)),
            Err(_) => Ok((s.parse()?, "".parse()?)),
        },
    }
}

pub(crate) fn parse_landlock_net_ports(s: &str) -> anyhow::Result<Vec<u16>> {
    if s.is_empty() {
        Ok(vec![])
    } else {
        Ok(s.split(',')
            .map(|e| e.to_string().parse::<u16>().unwrap_or(0))
            .filter(|e| *e != 0)
            .collect())
    }
}

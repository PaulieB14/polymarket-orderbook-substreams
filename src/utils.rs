use substreams::scalar::BigInt;

/// base units (6 decimals) -> decimal string
pub fn scale6(v: &BigInt) -> String {
    let s = BigInt::from(1_000_000u64);
    let int = v.clone() / s.clone();
    let frac = (v.clone() - int.clone() * s).to_u64();
    format!("{}.{:06}", int, frac)
}

pub fn unix_to_timestamp(secs: i64) -> String {
    let days = secs.div_euclid(86400);
    let tod = secs.rem_euclid(86400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", y, m, d, tod / 3600, (tod % 3600) / 60, tod % 60)
}

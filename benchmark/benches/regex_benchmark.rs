use criterion::{criterion_group, criterion_main, Criterion};
use regex::Regex;

const MATCH_STR: &'static str = 
    "wwww.google.com fsdjflkadjf.com https://www.google.com http://google.com fjdsklfjasljflsajflsajf fjsdalkfjsldjf https://meta.com fdsjfklsdjfaljlj https://3213@.com fjsdaklfjsladjflsjadflkjsdafjslajflasjflaksjflkasjflksajf fdsjfklsdjfaljlj https://3213@.com fjsdaklfjsladjflsjadflkjsdafjslajflasjflaksjflkasjflksajf xxx.xx@gmail.com @^#,@gmailc.om fdsfdsf.com fsdf123@23.com fdsjfklsdjfaljlj https://3213@.com fjsdaklfjsladjflsjadflkjsdafjslajflasjflaksjflkasjflksajf fdfsf@gds.com";

fn test_regex(regex: &Regex) {
    let caps = regex.captures(MATCH_STR);
    assert!(caps.unwrap().len() > 0);
}

fn test_pcre2(regex: &pcre2::bytes::Regex) {
    let caps = regex.captures(MATCH_STR.as_bytes()).unwrap();
    assert!(caps.unwrap().len() > 0);
}

pub fn criterion_values(c: &mut Criterion) {
    let regex = Regex::new(r"([a-zA-z]+://[^\s]*)|(^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$)").unwrap();
    let pcre2 = pcre2::bytes::Regex::new(r"([a-zA-z]+://[^\s]*)|(^\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*$)").unwrap();

    c.bench_function("regex-regex-test", |b| b.iter(|| test_regex(&regex)));
    c.bench_function("regex-pcre2-test", |b| b.iter(|| test_pcre2(&pcre2)));
}

criterion_group!(benches, criterion_values);
criterion_main!(benches);

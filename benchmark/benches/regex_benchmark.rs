use criterion::{criterion_group, criterion_main, Criterion};
use regex::Regex;

const MATCH_STR: &'static str = 
    "wwww.google.com fsdjflkadjf.com https://www.google.com http://google.com fjdsklfjasljflsajflsajf fjsdalkfjsldjf https://meta.com fdsjfklsdjfaljlj https://3213@.com fjsdaklfjsladjflsjadflkjsdafjslajflasjflaksjflkasjflksajf fdsjfklsdjfaljlj https://3213@.com fjsdaklfjsladjflsjadflkjsdafjslajflasjflaksjflkasjflksajf xxx.xx@gmail.com @^#,@gmailc.om fdsfdsf.com fsdf123@23.com fdsjfklsdjfaljlj https://3213@.com fjsdaklfjsladjflsjadflkjsdafjslajflasjflaksjflkasjflksajf fdfsf@gds.com";

fn test_regex(regex: &Regex) {
    let mut captured_text = vec![];
    for cap in regex.captures_iter(MATCH_STR) {
        for matched in cap.iter() {
            if let Some(m) = matched {
                captured_text.push(m.as_str().to_string());
            }
        }
    }
    assert!(captured_text.len() > 0)
}

fn test_regex_clone(regex: &Regex) {
    let _ = regex.clone();
}

pub fn criterion_values(c: &mut Criterion) {
    let regex = Regex::new(r"([a-zA-z]+://[^\s]*)|([\w!#$%&'*+/=?^_`{|}~-]+(?:\.[\w!#$%&'*+/=?^_`{|}~-]+)*@(?:[\w](?:[\w-]*[\w])?\.)+[\w](?:[\w-]*[\w])?)").unwrap();

    c.bench_function("regex-regex-test", |b| b.iter(|| test_regex(&regex)));
    c.bench_function("regex-clone-test", |b| b.iter(|| test_regex_clone(&regex)));
}

criterion_group!(benches, criterion_values);
criterion_main!(benches);

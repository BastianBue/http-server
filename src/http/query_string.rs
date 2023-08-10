use std::collections::HashMap;

// example
// a=1&b=2&c&d=&e===$d=7&d=abc

#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, QueryStringValue<'buf>>,
}

#[derive(Debug)]
pub enum QueryStringValue<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&QueryStringValue> {
        self.data.get(key)
    }
}

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self {
        let mut data = HashMap::new();
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";

            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i + 1..];
            }

            data.entry(key)
                .and_modify(|existing: &mut QueryStringValue| match existing {
                    QueryStringValue::Single(previous_val) => {
                        *existing = QueryStringValue::Multiple(vec![previous_val, val]);
                    }
                    QueryStringValue::Multiple(vec) => { vec.push(val) }
                })
                .or_insert(QueryStringValue::Single(val));
        }

        QueryString { data }
    }
}
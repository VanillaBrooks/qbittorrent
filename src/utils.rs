use super::data::Hash;

pub trait QueryConcat {
    fn query_concat(&self, sep: char) -> String;
}

impl QueryConcat for Hash {
    fn query_concat(&self, sep:char) -> String {
        return self.hash.to_string();
    }
}

impl QueryConcat for &str {
    fn query_concat(&self, sep:char) -> String {
        return self.to_string();
    }
}

impl QueryConcat for &[String] {
    fn query_concat(&self, sep:char) -> String {
        let mut hash_url = self
            .iter()
            .map(|x| {
                let mut cln = x.clone();
                cln.push(sep);
                cln
            })
            .collect::<String>();

        // remove the final | from the string
        hash_url.remove(hash_url.len() - 1);
        hash_url
    }
}

impl QueryConcat for &[Hash] {
    fn query_concat(&self, sep:char) -> String {
        let mut hash_url = self
            .iter()
            .map(|x| {
                let mut cln = x.hash.clone();
                cln.push(sep);
                cln
            })
            .collect::<String>();

        // remove the final | from the string
        hash_url.remove(hash_url.len() - 1);
        hash_url
    }
}

impl QueryConcat for [&Hash] {
    fn query_concat(&self, sep:char) -> String {
        let mut hash_url = self
            .iter()
            .map(|x| {
                let mut cln = x.hash.clone();
                cln.push(sep);
                cln
            })
            .collect::<String>();

        // remove the final | from the string
        hash_url.remove(hash_url.len() - 1);
        hash_url
    }
}

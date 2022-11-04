const VCARD_HEADER: &str = "BEGIN:VCARD\nVERSION:4.0\n";
const VCARD_FOOTER: &str = "END:VCARD";

pub struct VCard {
    buf: String,
}

impl VCard {
    pub fn new() -> Self {
        VCard {
            buf: VCARD_HEADER.into(),
        }
    }

    pub fn push<P, V>(&mut self, property: P, value: V)
    where
        P: AsRef<str>,
        V: AsRef<str>,
    {
        self.buf
            .push_str(&format!("{}:{}\n", property.as_ref(), value.as_ref()))
    }

    pub fn optional_push<P, V>(&mut self, property: P, value: V)
    where
        P: AsRef<str>,
        V: AsRef<str>,
    {
        if !value.as_ref().is_empty() {
            self.push(property, value)
        }
    }

    pub fn push_explicit<T>(&mut self, data: T)
    where
        T: AsRef<str>,
    {
        self.buf.push_str(&format!("{}\n", data.as_ref()))
    }

    pub fn finalize(mut self) -> String {
        self.push_explicit(VCARD_FOOTER);
        self.buf.trim().to_owned()
    }
}

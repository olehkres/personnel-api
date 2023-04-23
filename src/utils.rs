use std::collections::HashMap;

pub (super) struct XmlVariables {
    content: HashMap<String, String>,
}

impl XmlVariables {
    pub fn new() -> Self {
        XmlVariables {
            content: HashMap::<String, String>::new(),
        }
    }

    /**
    This function take key and value and push it to variables list.

    # Arguments
    * `str` - string to find matches.

    # Examples
    * Simplistic.
        ```
        let mut variables = XmlVariables::new();
        variables.insert("name", "value");

        assert_eq!(variables.replace("{name}"), "value");
        ```
    <br/>
    * You can use set value of existing value to new one.
        ```
        let mut variables = XmlVariables::new();
        variables.insert("name1", "value");
        variables.insert("name2", "{name1}");

        assert_eq!(variables.replace("{name2}"), "value");
        ```

    NOTE: This function can cost really bad if you pass not `mut String`.
    */
    pub fn insert(&mut self, k: impl Into<String>, v: impl Into<String>) {
        self.content.insert(k.into(), self.replace(v.into()));
    }

    /**
    This function take string and match all variables. If find any it fill replace it in it.

    # Arguments
    * `str` - string to find matches.

    NOTE: This function can cost really bad if you pass not `mut String`.
    */
    pub fn replace(&self, str: impl Into<String>) -> String {
        let mut str = str.into();

        for (field, value) in &self.content {
            str = str.replace(format!("{{{}}}", field).as_str(), value.as_str());
        }

        str
    }

    /// Reroute to HashMap extend.
    pub fn extend(&mut self, iter: HashMap<String, String>) {
        self.content.extend(iter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable() {
        let mut variables = XmlVariables::new();
        variables.insert("name", "value");

        assert_eq!(variables.replace("{name}"), "value");
    }

    #[test]
    fn variable_recursive() {
        let mut variables = XmlVariables::new();
        variables.insert("name1", "value");
        variables.insert("name2", "{name1}");

        assert_eq!(variables.replace("{name2}"), "value");
    }
}
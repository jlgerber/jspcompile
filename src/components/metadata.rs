/// Potential Metadata associated with a `Node` in the `JGraph`.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MetadataComponent {
    Volume,
    Permissions(String),
    EnvVarName(String),
    Owner(String),
    Separator, // needed for parsing purposes
    //NavAlias(String), 
    //AutoCreate
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Metadata {
    volume: bool,
    permissions: Option<String>,
    varname: Option<String>,
    owner: Option<String>
}

impl Metadata {
    /// new up metadata
    pub fn new() -> Self {
        Self {
            volume: false,
            permissions: None,
            varname: None,
            owner: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.volume == false && self.permissions.is_none() && self.varname.is_none() && self.owner.is_none()
    }

    /// Set volume and get back moved self. This is designed to be used in 
    /// a fluent api. Otherwise, you must assign back. 
    /// 
    /// # Examples
    /// 
    /// ```
    /// let metdata = Metadata.new()
    ///                 .set_volume(true)
    ///                 .set_owner(Some("jgerber".to_string()));
    /// ```
    pub fn set_volume(mut self, is: bool) -> Self {
        self.volume = is;
        self
    }

    pub fn is_volume(&self) -> bool {
        self.volume
    }
    /// Retrieve whether the metadata has volume set
    pub fn volume(&self) -> bool {
        self.volume
    }

    /// Set permissions, passing in an Option of a type which we 
    /// can get a string from (via into)
    pub fn set_permissions<T>(mut self, perms: Option<T>) -> Self 
    where 
        T: Into<String> 
    {
        self.permissions = perms.map(|x| x.into());
        self
    }

    pub fn permissions(&self) -> Option<&str> {
        self.permissions.as_ref().map(|x| &**x)
    }

    pub fn set_varname<T>(mut self, varname: Option<T>) -> Self 
    where 
        T: Into<String>
    {
        self.varname = varname.map(|x| x.into());
        self
    }

    pub fn varname(&self) -> Option<&str> {
        self.varname.as_ref().map(|x| &**x)
    }
    
    pub fn set_owner<T>(mut self, owner: Option<T>) -> Self 
    where
        T: Into<String>
    {
        self.owner = owner.map(|x| x.into());
        self
    }

    pub fn owner(&self) -> Option<&str> {
        self.owner.as_ref().map(|x| &**x)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_metadata() {
        let md = Metadata::new();
        let expect = Metadata {
            volume: false,
            permissions: None,
            varname: None,
            owner: None,
        };
        assert_eq!(md, expect);
    }

     #[test]
    fn can_create_metadata_and_set_volume() {
        let md = Metadata::new().set_volume(true);
        let expect = Metadata {
            volume: true,
            permissions: None,
            varname: None,
            owner: None,
        };
        assert_eq!(md, expect);
    }

     #[test]
    fn can_create_metadata_and_set_owner() {
        let md = Metadata::new().set_volume(true).set_owner(Some("jgerber"));
        let expect = Metadata {
            volume: true,
            permissions: None,
            varname: None,
            owner: Some("jgerber".to_string()),
        };
        assert_eq!(md, expect);
    }

     #[test]
    fn can_create_metadata_and_set_varname() {
        let md = Metadata::new().set_volume(true).set_owner(Some("jgerber")).set_varname(Some("jg_show"));
        let expect = Metadata {
            volume: true,
            permissions: None,
            varname: Some("jg_show".to_string()),
            owner: Some("jgerber".to_string()),
        };
        assert_eq!(md, expect);
    }

     #[test]
    fn can_create_metadata_and_set_perms() {
        let md = Metadata::new()
                    .set_volume(true)
                    .set_owner(Some("jgerber"))
                    .set_varname(Some("jg_show"))
                    .set_permissions(Some("777"));

        let expect = Metadata {
            volume: true,
            permissions: Some("777".to_string()),
            varname: Some("jg_show".to_string()),
            owner: Some("jgerber".to_string()),
        };
        assert_eq!(md, expect);
    }

    #[test]
    fn can_get_volume() {
        let md = Metadata::new().set_volume(true);
        assert_eq!(md.volume(), true);
    }

    #[test]
    fn can_get_owner() {
        let md = Metadata::new().set_volume(true).set_owner(Some("jgerber"));
        assert_eq!(md.owner(), Some("jgerber"));
    }

    #[test]
    fn can_get_varname() {
        let md = Metadata::new()
                    .set_volume(true)
                    .set_owner(Some("jgerber"))
                    .set_varname(Some("jg_foo"));
        assert_eq!(md.varname(), Some("jg_foo"));
    }

    #[test]
    fn can_get_permissions() {
        let md = Metadata::new()
                    .set_volume(true)
                    .set_owner(Some("jgerber"))
                    .set_varname(Some("jg_foo"))
                    .set_permissions(Some("777"));
        assert_eq!(md.permissions(), Some("777"));
    }
}
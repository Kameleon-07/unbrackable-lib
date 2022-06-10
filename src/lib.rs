use rand::{thread_rng, Rng};

#[derive(Default)]
pub struct PasswordBuilder {
    length: i32,
    include_special_characters: bool,
    use_numbers: bool,
    use_uppercase: bool,
    use_underlines: bool,
}

impl PasswordBuilder {
    fn default() -> Self {
        PasswordBuilder {
            length: 20,
            include_special_characters: false,
            use_numbers: false,
            use_uppercase: false,
            use_underlines: false,
        }
    }

    /// Makes new PasswordBuilder instance using default values
    /// 
    /// # Examples
    /// 
    /// ```
    /// let password_builder = PasswordBuilder::new();
    /// ```
    /// 
    /// ```
    /// let mut password_builder = PasswordBuilder::new()
    ///     .set_length(35)
    ///     .include_special_characters();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Builds a password based on set parameters
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut password_builder = PasswordBuilder::new()
    ///     .set_length(25)
    ///     .use_numbers();
    /// 
    /// let password = password_builder::build();
    /// println!("{}", password);
    /// ```
    pub fn build(self) -> String {
        let mut password = String::new();

        let mut allowed_characters = String::from("qwertyuiopasdfghjklzxcvbnm");

        if self.include_special_characters {
            allowed_characters += "!\"$#%'()*+`-./:;<=>?@[\\]^{|}~&";
        }
        if self.use_numbers {
            allowed_characters += "1234567890";
        }
        if self.use_uppercase {
            allowed_characters += "QWERTYUIOPASDFGHJKLZXCVBNM";
        }
        if self.use_underlines {
            allowed_characters += "_";
        }

        let mut rng = thread_rng();
        for _ in 0..self.length {
            let rand_num = rng.gen_range(0..allowed_characters.len());

            password.push(allowed_characters.as_bytes()[rand_num] as char);
        }

        password
    }

    /// Sets password length parameter
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut password_builder = PasswordBuilder::new() // length 20
    ///     .set_length(15); // length 15
    /// ```
    pub fn set_length(mut self, length: i32) -> Self {
        self.length = length;
        self
    }

    /// Sets include_special_characters to true
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut password_builder = PasswordBuilder::new() // false
    ///     .include_special_characters(); // true
    /// ```
    pub fn include_special_characters(mut self) -> Self {
        self.include_special_characters = true;
        self
    }

    /// Sets use_numbers to true
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut password_builder = PasswordBuilder::new() // false
    ///     .use_numbers(); // true
    /// ```
    pub fn use_numbers(mut self) -> Self {
        self.use_numbers = true;
        self
    }

    /// Sets use_uppercase to true
    ///
    /// # Examples
    /// 
    /// ```
    /// let mut password_builder = PasswordBuilder::new() // false
    ///     .use_uppercase(); // true
    /// ```
    pub fn use_uppercase(mut self) -> Self {
        self.use_uppercase = true;
        self
    }

    /// Sets use_underlines to true
    /// 
    /// # Examples
    /// 
    /// ```
    /// let mut password_builder = PasswordBuilder::new() // false
    ///     .use_underlines(); // true
    /// ```
    pub fn use_underlines(mut self) -> Self {
        self.use_underlines = true;
        self
    }
}
// Define a macro named 'basic_type' that creates a custom type.
macro_rules! basic_type {
    // Accepts input attributes, a name for the type, and its underlying type.
    ($(#[$attr:meta])* $name:ident, $type:ty) => {
        // Apply provided attributes to the struct.
        $(#[$attr])*
        // Derive various traits for the new struct.
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash, PartialOrd, Ord
        )]
        // Define a public struct with the provided name, wrapping the specified type.
        pub struct $name(pub $type);

        // Implement methods for the new struct.
        impl $name {
            // Method 'next' increments the value of the struct by 1.
            pub fn next(self) -> $name {
                $name(self.0 + 1)
            }
        }

        // Implement dereferencing behavior for the struct.
        impl Deref for $name {
            type Target = $type;

            // Define dereferencing to the target type.
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        // Implement mutable dereferencing behavior for the struct.
        impl DerefMut for $name {
            // Define mutable dereferencing to the target type.
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        // Allow conversion from a string to the custom type.
        impl FromStr for $name {
            type Err = ParseIntError;

            // Parse the string value into the specified type.
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let value = s.parse::<$type>()?;
                Ok(Self(value))
            }
        }

        // Implement how the custom type should be displayed when formatted as a string.
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        // Implement addition operations between the custom type and its underlying type.
        impl Add<$type> for $name {
            type Output = Self;

            fn add(self, other: $type) -> Self {
                Self(self.0 + other)
            }
        }

        // Implement compound assignment operations for addition.
        impl std::ops::AddAssign<$type> for $name {
            fn add_assign(&mut self, other: $type) {
                self.0 += other;
            }
        }

        // Implement subtraction operations between the custom type and its underlying type.
        impl Sub<$type> for $name {
            type Output = Self;

            fn sub(self, other: $type) -> Self {
                Self(self.0 - other)
            }
        }

        // Implement compound assignment operations for subtraction.
        impl std::ops::SubAssign<$type> for $name {
            fn sub_assign(&mut self, other: $type) {
                self.0 -= other;
            }
        }

        // Allow conversion from the underlying type to the custom type.
        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }
    };
}


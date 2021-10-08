use std::ffi::OsStr;
use std::path::{Component, Path, PathBuf};

use convert_case::{Case, Casing};

use crate::error::PathConvertError;
use std::convert::TryFrom;

/// Describes the supported file naming conventions.
///
/// Converting to and from some of these cases is "lossy" and you may lose information regarding word boundaries. In
/// these cases, it will be impossible to revert to the original case once converted. Note that not all word boundaries
/// are necessarily lost, especially since it is not always possible to determine if a number is the start of a word,
/// the end of a word, or a word itself.
#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Convention {
    /// In title case convention describes strings where the first letter of each word is capitalized and spaces are
    /// preserved.
    TitleCase,

    /// In flat case all letters are undercase and there is no whitespace between words.
    FlatCase,

    /// In flat case all letters are uppercase and there is no whitespace between words.
    UpperFlatCase,

    /// In camel case all white space is removed between words, and the first letter o each worked except the first is
    /// capitalized.
    CamelCase,

    /// Upper camel cse and pascal case are the same as camel case, except the first letter o all words is capitalized.
    UpperCamelCase,

    /// In snake case words are separated by underscores (ie '_') and all are lowercase.
    SnakeCase,

    /// Upper snake case, screaming snake, and pascal snake case are the same as snake case, but all letters are
    /// capitalized.
    UpperSnakeCase,

    /// In kebab case words are separated by underscores (ie '-') and all are lowercase.
    KebabCase,
}

impl From<Convention> for Case {
    fn from(convention: Convention) -> Self {
        match convention {
            Convention::TitleCase => Case::Title,
            Convention::FlatCase => Case::Flat,
            Convention::UpperFlatCase => Case::UpperFlat,
            Convention::CamelCase => Case::Camel,
            Convention::UpperCamelCase => Case::UpperCamel,
            Convention::SnakeCase => Case::Snake,
            Convention::UpperSnakeCase => Case::UpperSnake,
            Convention::KebabCase => Case::Kebab,
        }
    }
}

impl TryFrom<&str> for Convention {
    type Error = String;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "title" => Ok(Convention::TitleCase),
            "flat" => Ok(Convention::FlatCase),
            "FLAT" => Ok(Convention::UpperFlatCase),
            "camel" => Ok(Convention::CamelCase),
            "CAMEL" => Ok(Convention::UpperCamelCase),
            "snake" => Ok(Convention::SnakeCase),
            "SNAKE" => Ok(Convention::UpperSnakeCase),
            "kebab" => Ok(Convention::KebabCase),
            _ => Err(format!(
                "Unsupported naming convention '{}'",
                <str as AsRef<str>>::as_ref(s)
            )),
        }
    }
}

/// Convert a component of a path into the desired case.
fn convert_component(
    component: &OsStr,
    from_convention: Option<Convention>,
    to_convention: Convention,
) -> Result<String, PathConvertError> {
    let path: &Path = component.as_ref();

    // allow remaining code to safely call 'OsStr::toStr' without checks for valid utf-8
    if path.to_str().is_none() {
        return Err(PathConvertError::InvalidUtf8Path);
    }

    let stem = path.file_stem();
    let ext = path.extension();

    if stem.is_none() && ext.is_none() {
        Err(PathConvertError::InvalidPath)
    } else if stem.is_none() {
        Ok(String::from(ext.unwrap().to_str().unwrap()))
    } else {
        let new_stem = if from_convention.is_some() {
            stem.unwrap()
                .to_str()
                .unwrap()
                .from_case(from_convention.unwrap().into())
                .to_case(to_convention.into())
        } else {
            stem.unwrap()
                .to_str()
                .unwrap()
                .to_case(to_convention.into())
        };

        match ext {
            Some(ext) => Ok(String::from(format!(
                "{}.{}",
                new_stem,
                ext.to_str().unwrap()
            ))),
            None => Ok(new_stem),
        }
    }
}

/// Convert just the filename portion to the desired case.
pub fn convert_basename<P: AsRef<Path>>(
    path: P,
    from_convention: Option<Convention>,
    to_convention: Convention,
) -> Result<PathBuf, PathConvertError> {
    let parent = path.as_ref().parent();
    let basename = path.as_ref().file_name();

    // if the path is either the root path or '..'
    if parent.is_none() || basename.is_none() {
        Ok(path.as_ref().to_path_buf())
    } else {
        match convert_component(basename.unwrap(), from_convention, to_convention) {
            Ok(base) => {
                let mut path = path.as_ref().to_path_buf();
                path.pop();
                path.push(base);

                Ok(path)
            }
            Err(err) => Err(err),
        }
    }
}

/// Convert the entire path  to the desired case.
pub fn convert_full<P: AsRef<Path>>(
    path: P,
    from_convention: Option<Convention>,
    to_convention: Convention,
) -> Result<PathBuf, PathConvertError> {
    let mut converted_path: PathBuf = PathBuf::new();

    for component in path.as_ref().components() {
        match component {
            Component::Normal(path) => {
                let converted_component: String =
                    match convert_component(path, from_convention, to_convention) {
                        Ok(s) => s,
                        Err(err) => return Err(err),
                    };

                converted_path.push(converted_component);
            }
            _ => converted_path.push(component),
        }
    }

    Ok(converted_path)
}

#[cfg(test)]
mod test {
    use std::ffi::OsStr;

    use crate::convert_path::{convert_basename, convert_component, convert_full, Convention};
    use std::path::{Path, PathBuf};

    #[test]
    fn test_convert_component_kebab_to_snake_no_from_case() {
        let expected = Ok(String::from("some_file.jpg"));

        let actual = convert_component(OsStr::new("some-file.jpg"), None, Convention::SnakeCase);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_convert_component_upper_camel_to_flat_with_from_case() {
        let expected = Ok(String::from("somefile.jpg"));

        let actual = convert_component(
            OsStr::new("SomeFile.jpg"),
            Some(Convention::UpperCamelCase),
            Convention::FlatCase,
        );

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_convert_basename_title_to_camel_no_from_case() {
        let expected = Ok(PathBuf::from("/An Absolute/Path To/someFile.jpg"));

        let actual = convert_basename(
            Path::new("/An Absolute/Path To/Some File.jpg"),
            None,
            Convention::CamelCase,
        );

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_convert_basename_upper_snake_to_kebab_no_from_case() {
        let expected = Ok(PathBuf::from("/An Absolute/Path To/some-file.jpg"));

        let actual = convert_basename(
            Path::new("/An Absolute/Path To/SOME_FILE.jpg"),
            Some(Convention::UpperSnakeCase),
            Convention::KebabCase,
        );

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_convert_full_camel_to_snake() {
        let expected = Ok(PathBuf::from("/an_absolute/path_to/some_file.jpg"));

        let actual = convert_full(
            Path::new("/anAbsolute/pathTo/someFile.jpg"),
            None,
            Convention::SnakeCase,
        );

        assert_eq!(expected, actual)
    }

    #[test]
    fn test_convert_full_mixed_to_upper_snake_case() {
        let expected = Ok(PathBuf::from("/AN_ABSOLUTE/PATH_TO/SOME_FILE.jpg"));

        let actual = convert_full(
            Path::new("/An Absolute/path-to/someFile.jpg"),
            None,
            Convention::UpperSnakeCase,
        );

        assert_eq!(expected, actual)
    }
}

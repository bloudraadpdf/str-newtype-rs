use std::borrow::Cow;

use str_newtype::StrNewType;

#[derive(StrNewType)]
#[newtype(owned(FooBuf))]
pub struct Foo(str);

impl Foo {
	pub const fn validate_bytes(bytes: &[u8]) -> bool {
		matches!(bytes, [b'f', b'o', b'o'])
	}

	pub const fn validate_str(value: &str) -> bool {
		Self::validate_bytes(value.as_bytes())
	}
}

#[test]
fn generated_constructors_retain_validation_errors() {
	assert_eq!(
		Foo::new("foo").expect("valid borrowed value").as_bytes(),
		b"foo"
	);
	assert_eq!(
		FooBuf::new(String::from("foo"))
			.expect("valid owned value")
			.as_str(),
		"foo"
	);
	let error = FooBuf::new(String::from("bar")).expect_err("invalid owned value");
	assert_eq!(error.0, "bar");
	let error: &dyn std::error::Error = &error;
	assert!(!error.to_string().is_empty());
	assert!(Foo::from_bytes(&[0xff]).is_err());
}

#[test]
fn generated_cow_conversions_preserve_ownership() {
	let borrowed = Foo::new("foo").expect("valid borrowed value");
	let value = Cow::<Foo>::from(borrowed);
	assert!(matches!(value, Cow::Borrowed(_)));
	assert_eq!(value.as_bytes(), b"foo");

	let owned = FooBuf::new(String::from("foo")).expect("valid owned value");
	let value = Cow::<Foo>::from(owned);
	assert!(matches!(value, Cow::Owned(_)));
	assert_eq!(value.as_bytes(), b"foo");
}

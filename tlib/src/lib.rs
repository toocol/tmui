mod object;
mod prelude;
mod types;
mod values;

#[cfg(test)]
mod tests {
    use macros::extends_object;

    use crate::{
        object::{Object, ObjectImpl, ObjectSubclass},
        prelude::*,
    };

    #[extends_object]
    pub struct TestObject {}

    impl ObjectSubclass for TestObject {
        const NAME: &'static str = "TestObject";

        type Type = TestObject;

        type ParentType = Object;
    }

    impl ObjectImpl for TestObject {}

    #[test]
    fn test_object() {
        let obj: TestObject = Object::new();
        assert_eq!("TestObject", obj.type_().name());
        assert!(obj.is::<TestObject>());
        test_is_a(obj)
    }

    fn test_is_a<T: IsA<Object>>(obj: T) {
        let obj = obj.downcast_ref::<TestObject>().unwrap();
        assert_eq!("TestObject", obj.type_().name());
        assert!(obj.is::<TestObject>());
    }

    #[test]
    fn test_value() {
        let val = true.to_value();
        let get = val.get::<bool>();
        assert_eq!(true, get);

        let val = (-8i8).to_value();
        let get = val.get::<i8>();
        assert_eq!(-8, get);

        let val = 8u8.to_value();
        let get = val.get::<u8>();
        assert_eq!(8, get);

        let val = (-16i16).to_value();
        let get = val.get::<i16>();
        assert_eq!(-16, get);

        let val = 16u16.to_value();
        let get = val.get::<u16>();
        assert_eq!(16, get);

        let val = (-32i32).to_value();
        let get = val.get::<i32>();
        assert_eq!(-32, get);

        let val = 32u32.to_value();
        let get = val.get::<u32>();
        assert_eq!(32, get);

        let val = (-64i64).to_value();
        let get = val.get::<i64>();
        assert_eq!(-64, get);

        let val = 64u64.to_value();
        let get = val.get::<u64>();
        assert_eq!(64, get);

        let val = (-128i128).to_value();
        let get = val.get::<i128>();
        assert_eq!(-128, get);

        let val = 128u128.to_value();
        let get = val.get::<u128>();
        assert_eq!(128, get);

        let val = 0.32f32.to_value();
        let get = val.get::<f32>();
        assert_eq!(0.32, get);

        let val = 0.64f64.to_value();
        let get = val.get::<f64>();
        assert_eq!(0.64, get);

        let vec = vec![12, 12];
        let val = vec.clone().to_value();
        let get = val.get::<Vec<i32>>();
        assert_eq!(vec, get);

        let vec = vec!["Hello", "World"];
        let val = vec.clone().to_value();
        let get = val.get::<Vec<String>>();
        assert_eq!(vec, get);

        let tuple = ("Hello".to_string(), 1024);
        let val = tuple.clone().to_value();
        let get = val.get::<(String, i32)>();
        assert_eq!(tuple, get);

        let tuple = (
            "Hello".to_string(),
            vec!["World".to_string(), "Hello".to_string(), "Rust".to_string()],
        );
        let val = tuple.clone().to_value();
        let get = val.get::<(String, Vec<String>)>();
        assert_eq!(tuple, get);
    }
}

pub trait BinarySerializable {
    fn serialize(&self) -> Vec<u8>;
    fn deserialize(data: &[u8], byte_offset: &mut usize) -> Result<Self, String> where Self: Sized;
}

// #[cfg(test)]
// mod tests {
//     use binser_derive::BinarySerializable;

//     use super::*;

//     #[derive(Default, BinarySerializable)]
//     pub struct Test {
//         id: u32,
//         description: String
//     }

//     #[derive(Default, BinarySerializable)]
//     pub struct TestTwo {
//         first: u32,
//         second: u32,
//         third: u32
//     }

//     #[test]
//     fn can_build() {
//         let trace = Test {
//             id: 1,
//             description: String::from("This is a test trace"),
//         };

//         assert_eq!(trace.id, 1);
//         assert_eq!(trace.description, "This is a test trace");
//     }

//     #[test]
//     fn can_serialize() {
//         let trace = Test {
//             id: 5,
//             description: String::from("This is a test trace"),
//         };

//         let serialized = trace.serialize();
//         assert!(!serialized.is_empty());
//         println!("{:?}", serialized);

//         let mut byte_offset: usize = 0;
//         let deserialized = Test::deserialize(&serialized, &mut byte_offset).unwrap();
//         assert_eq!(deserialized.id, trace.id);
//         assert_eq!(deserialized.description, trace.description);
//     }

//     #[test]
//     fn can_serialize_test() {
//         let test = TestTwo {
//             first: 10,
//             second: 15,
//             third: 3
//         };

//         let serialized = test.serialize();
//         assert!(!serialized.is_empty());
//         println!("{:?}", serialized);
//         let mut byte_offset: usize = 0;
//         let deserialized = TestTwo::deserialize(&serialized, &mut byte_offset).unwrap();
//         assert_eq!(deserialized.first, test.first);
//         assert_eq!(deserialized.second, test.second);
//         assert_eq!(deserialized.third, test.third);
//     }
// }
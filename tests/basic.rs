#[derive(PartialEq, Debug)]
struct TestData {
       id: u64,
}

#[test]
fn test_basic_union() {

   let data1 = TestData {
       id: 1,
   };
   let data2 = TestData {
       ..data1
   };

   assert_eq!(data1, data2);

   //this is where I need to actually use some code! I don't know how yet


}
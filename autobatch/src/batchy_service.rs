pub trait BatchyService: Clone + Send {
    type Request: Send + 'static;
    type Response: Send + Sync + 'static;
    fn batch_call(&self, items: &[Self::Request]) -> Vec<Self::Response>;
}

#[derive(Clone)]
pub struct Doubler {}
impl BatchyService for Doubler {
    type Request = u32;
    type Response = u32;
    fn batch_call(&self, items: &[Self::Request]) -> Vec<Self::Response> {
        items.iter().map(|i| *i * 2).collect()
    }
}

#[test]
fn basic_batchy_service_test() {
    let service = Doubler {};
    let input: Vec<u32> = (1..30).collect();
    let expected_output: Vec<_> = input.iter().map(|x| x * 2).collect();
    assert_eq!(expected_output, service.batch_call(&input));
}

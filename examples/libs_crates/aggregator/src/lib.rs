pub trait Summarizable {
    fn summary(&self) -> String;
    // OR with default implementation
    //fn summary(&self) -> String {
    //    String::from("(Read more...)")
    //}
}

pub trait GetElementAt<Element, Index> {
    fn get_element_at(&self, at: Index) -> Option<Element>;
}

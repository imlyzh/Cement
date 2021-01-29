trait FindName {
    type Output;
    fn find_name(i: &str) -> Self::Output;
}

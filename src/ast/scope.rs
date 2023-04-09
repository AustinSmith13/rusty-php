pub struct Scope {
    pub id: usize,
    pub parent: usize,
    pub symbols: HashMap<String, Symbol>,
}

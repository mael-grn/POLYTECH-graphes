use std::collections::HashMap;
use crate::models;
#[derive(Clone)]
pub struct Graphe {
    pub sommets: Vec<models::sommet::Sommet>,
    pub index_sommets: HashMap<i64, usize>,
    pub nom: String,
    pub commentaires: String,
    pub oriente: bool,
    pub nb_sommets: i64,
    pub nb_arretes: i64,
}
use crate::models::arrete::Arrete;
#[derive(Clone)]
pub struct Sommet {
    pub arretes: Vec<Arrete>,
    pub nom: String,
    pub id: i64,
    pub valeurs: Vec<f64>
}
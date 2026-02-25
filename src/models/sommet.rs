use crate::models::arrete::Arrete;
#[derive(Clone)]
pub struct Sommet {
    pub arretes: Vec<Arrete>,
    pub nom: String,
    pub id: i64,
    pub d_moins: i64,
    pub d_plus: i64,
    pub valeurs: Vec<f64>
}
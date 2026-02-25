use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::prelude::IteratorRandom;
use rand::Rng;
use crate::models;
use crate::models::arrete::Arrete;
enum ContexteLecture {
    Donnees,
    Sommets,
    Arretes,
}

/// Permet d'afficher un objet graph. Je sais pas exactement ce que ça fait, c'est gemini.
pub fn afficher_graph(_graphe: &models::graphe::Graphe) {
    println!("=== GRAPHE : {} ===", _graphe.nom);
    println!("Commentaires : {}", _graphe.commentaires);
    println!("Type : {}", if _graphe.oriente { "Orienté" } else { "Non-orienté" });
    println!("Stats : {} sommets, {} arêtes", _graphe.nb_sommets, _graphe.nb_arretes);
    println!("---------------------------------------");

    for sommet in &_graphe.sommets {
        print!("Sommet #{} ({}) d+ : {}, d- : {}", sommet.id, sommet.nom, sommet.d_plus, sommet.d_moins);
        if !sommet.valeurs.is_empty() {
            print!(" [Valeurs: {:?}]", sommet.valeurs);
        }
        println!();

        if sommet.arretes.is_empty() {
            println!("  (Pas d'arêtes sortantes)");
        } else {
            for arrete in &sommet.arretes {
                println!(
                    "  --> Destination: Sommet #{} | Valeurs arête: {:?}",
                    arrete.destination,
                    arrete.valeurs
                );
            }
        }
        println!();
    }
    println!("=======================================");
}

/// Permet de lire un fichier ligne par ligne et de traiter le contenu de sorte à créer un objet graph.
pub fn lire_un_fichier_ligne_par_ligne_et_traiter_le_contenu_de_sorte_a_creer_un_objet_graph(
    _chemin: &str
) -> Result<models::graphe::Graphe, String> {

    let fichier = File::open(_chemin).map_err(|_| "Erreur lors de l'ouverture du fichier".to_string())?;
    let lecteur = BufReader::new(fichier);

    let mut graphe = models::graphe::Graphe {
        sommets: Vec::new(),
        index_sommets: HashMap::new(),
        nom: String::from("Aucun nom"),
        commentaires: String::from("Aucun commentaire"),
        nb_arretes: 0,
        nb_sommets: 0,
        oriente: false
    };

    let mut contexte_lecture_actuel = ContexteLecture::Donnees;

    for (_, ligne) in lecteur.lines().enumerate() {
        let l = ligne.map_err(|_| "Erreur lors de la lecture d'une ligne".to_string())?;

        traiter_une_ligne_d_un_fichier_gra_en_fonction_du_contexte_de_lecture(&l, &mut graphe, &mut contexte_lecture_actuel)?;
    }

    Ok(graphe)
}

/// Permet de traiter une ligne d'un fichier gra en fonction du contexte de lecture
fn traiter_une_ligne_d_un_fichier_gra_en_fonction_du_contexte_de_lecture(_ligne: &String, _graphe: &mut models::graphe::Graphe, _contexte: &mut ContexteLecture) -> Result<(), String> {

    // Un sépare la ligne en une liste de "mots"
    let mots = _ligne.split_whitespace().collect::<Vec<&str>>();
    if mots.is_empty() {
        return Ok(());
    }
    match _contexte {
        ContexteLecture::Donnees => traiter_une_ligne_d_un_fichier_gra_dans_un_contexte_de_traitement_des_donnees_initiales(&mots, _graphe, _contexte),
        ContexteLecture::Sommets => traiter_une_ligne_d_un_fichier_gra_dans_un_contexte_de_lecture_des_vertices_ou_autrement_dit_des_sommets(&mots, _graphe, _contexte),
        ContexteLecture::Arretes => traiter_une_ligne_d_un_fichier_gra_dans_un_contexte_de_lecture_des_edges_ou_autrement_dit_des_arretes(&mots, _graphe, _contexte),
    }
}

fn traiter_une_ligne_d_un_fichier_gra_dans_un_contexte_de_traitement_des_donnees_initiales(_mots: &Vec<&str>, _graphe: &mut models::graphe::Graphe, _contexte: &mut ContexteLecture) -> Result<(), String> {
    if _mots.len() < 2 {
        Err(String::from("Il manque des données dans la ligne"))
    } else {
        match _mots[0] {
            "NAME:" => {
                _graphe.nom = _mots[1].to_string();
                Ok(())
            },
            "COMMENT:" => {
                _graphe.commentaires = _mots[2..].join(" ").to_string();
                Ok(())
            },
            "ORIENTED:" => {
                match _mots[1] {
                    "true" => _graphe.oriente = true,
                    "false" => _graphe.oriente = false,
                    _ => return Err(String::from("Valeur pour ORIENTED doit être 'true' ou 'false'")),
                }
                Ok(())
            },
            "NB_VERTICES:" => {
                match _mots[1].parse::<i64>() {
                    Ok(nb) => _graphe.nb_sommets = nb,
                    Err(_) => return Err(String::from("Valeur pour NB_VERTICES doit être un nombre entier")),
                }
                Ok(())
            },
            "NB_EDGES:" => {
                match _mots[1].parse::<i64>() {
                    Ok(nb) => _graphe.nb_arretes = nb,
                    Err(_) => return Err(String::from("Valeur pour NB_EDGES doit être un nombre entier")),
                }
                Ok(())
            },
            "VERTICES" => {
                *_contexte = ContexteLecture::Sommets;
                Ok(())
            },
            _ => Ok(()), // Si donnée inconnue, on ignore la ligne
        }
    }
}

fn traiter_une_ligne_d_un_fichier_gra_dans_un_contexte_de_lecture_des_vertices_ou_autrement_dit_des_sommets(_mots: &Vec<&str>, _graphe: &mut models::graphe::Graphe, _contexte: &mut ContexteLecture) -> Result<(), String> {
    if _mots.len() < 2 {
        Err(String::from("L'une des lignes de sommet (VERICES) n'est pas complète. Celle-ci doit avoir au moins 2 colonnes."))
    } else {
        match _mots[0] {
            "EDGES" => {
                *_contexte = ContexteLecture::Arretes;
                Ok(())
            },
            _ => {
                let id = match _mots[0].parse::<i64>() {
                    Ok(nb) => nb,
                    Err(_) => return Err(String::from("Valeur pour ID pour un sommet doit être un nombre entier")),
                };
                let sommet = models::sommet::Sommet {
                    id,
                    nom: _mots[1].to_string(),
                    valeurs: if _mots.len() > 2 {
                        _mots[2..]
                            .iter()
                            .map(|val| val.parse::<f64>().unwrap_or(0.0))
                            .collect()
                    } else {
                        Vec::new()
                    },
                    arretes: Vec::new(),
                    d_moins: 0,
                    d_plus: 0,
                };

                _graphe.sommets.push(sommet);
                _graphe.index_sommets.insert(id, _graphe.sommets.len() - 1);
                Ok(())
            },
        }
    }
}

fn traiter_une_ligne_d_un_fichier_gra_dans_un_contexte_de_lecture_des_edges_ou_autrement_dit_des_arretes(_mots: &Vec<&str>, _graphe: &mut models::graphe::Graphe, _contexte: &mut ContexteLecture) -> Result<(), String> {
    if _mots.len() < 2 {
        Err(String::from("Il manque des données dans une ligne de sommet"))
    } else {
        let id_depart =  match _mots[0].parse::<i64>() {
            Ok(nb) => nb,
            Err(_) => return Err(String::from("Valeur pour ID pour un depart d'une arrete (id de sommet) doit être un nombre entier")),
        };
        let id_arrive =  match _mots[1].parse::<i64>() {
            Ok(nb) => nb,
            Err(_) => return Err(String::from("Valeur pour ID pour une arrivée d'une arrete (id de sommet) doit être un nombre entier")),
        };
        let pos_depart = match _graphe.index_sommets.get(&id_depart) {
            Some(pos) => *pos,
            None => return Err(format!("Le sommet de départ avec l'id {} n'existe pas dans le graphe", id_depart)),
        };
        let pos_arrive = match _graphe.index_sommets.get(&id_arrive) {
            Some(pos) => *pos,
            None => return Err(format!("Le sommet de arrivée avec l'id {} n'existe pas dans le graphe", id_depart)),
        };
        _graphe.sommets[pos_arrive].d_moins += 1;
        let depart = &mut _graphe.sommets[pos_depart];
        depart.d_plus += 1;
        depart.arretes.push(Arrete {
            destination: id_arrive,
            valeurs: if _mots.len() > 2 {
                _mots[2..]
                    .iter()
                    .map(|val| val.parse::<f64>().unwrap_or(0.0))
                    .collect()
            } else {
                Vec::new()
            },
        });

        Ok(())
    }
}

/// Implementation de l'algo dans le cm4
/// Je sais pas si ca fonctionne, ca retourne des valeurs mais je sais pas comment tester
pub fn verifier_si_le_graphe_en_question_est_un_circuit(_graphe: &models::graphe::Graphe) -> bool {
    let mut file: HashMap<i64, models::sommet::Sommet> = HashMap::new();
    let mut p = 0;
    for sommet in _graphe.sommets.iter() {
        if (sommet.d_moins == 0) {
            file.insert(sommet.id, sommet.clone());
            p += 1;
        }
    }

    while !file.is_empty() {
        let sommet_id = *file.keys().next().unwrap();
        while let Some(arete) = file.get_mut(&sommet_id).unwrap().arretes.pop() {
            let dest_id = arete.destination;
            if let Some(suivant) = file.get_mut(&dest_id) {
                suivant.d_moins -= 1;
                if suivant.d_moins <= 0 {
                    p += 1;
                }
            }
        }
        file.remove(&sommet_id);
    }

    return p != _graphe.sommets.len();
}

pub fn on_va_calculer_une_numerotation_topologique_pour_le_graphe_en_parametre_sachant_qu_il_est_oriente(_graphe: &models::graphe::Graphe) -> HashMap<i64, i64> {
    let mut file: HashMap<i64, models::sommet::Sommet> = HashMap::new();
    let mut topo: HashMap<i64, i64> = HashMap::new(); //id sommet / topo
    let mut p = 1;
    for sommet in _graphe.sommets.iter() {
        if (sommet.d_moins == 0) {
            file.insert(sommet.id, sommet.clone());
            topo.insert(sommet.id, p);
            p += 1;
        }
    }

    while !file.is_empty() {
        let sommet_id = *file.keys().next().unwrap();
        while let Some(arete) = file.get_mut(&sommet_id).unwrap().arretes.pop() {
            let dest_id = arete.destination;
            if let Some(suivant) = file.get_mut(&dest_id) {
                suivant.d_moins -= 1;
                if suivant.d_moins <= 0 {
                    topo.insert(sommet_id, p);
                    p += 1;
                }
            }
        }
        file.remove(&sommet_id);
    }

    return topo;
}
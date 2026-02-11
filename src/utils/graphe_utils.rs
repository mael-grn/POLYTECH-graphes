use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::models;
use crate::models::arrete::Arrete;
enum ContexteLecture {
    Donnees,
    Sommets,
    Arretes,
}

pub fn afficher_graph(_graphe: &models::graphe::Graphe) {
    println!("=== GRAPHE : {} ===", _graphe.nom);
    println!("Commentaires : {}", _graphe.commentaires);
    println!("Type : {}", if _graphe.oriente { "Orienté" } else { "Non-orienté" });
    println!("Stats : {} sommets, {} arêtes", _graphe.nb_sommets, _graphe.nb_arretes);
    println!("---------------------------------------");

    for sommet in &_graphe.sommets {
        // Affichage du sommet et de ses valeurs
        print!("Sommet #{} ({})", sommet.id, sommet.nom);
        if !sommet.valeurs.is_empty() {
            print!(" [Valeurs: {:?}]", sommet.valeurs);
        }
        println!();

        // Affichage des arêtes partant de ce sommet
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
        println!(); // Saut de ligne pour la clarté entre sommets
    }
    println!("=======================================");
}


pub fn recuperer_graphe_depuis_fichier(
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

        traiter_ligne(&l, &mut graphe, &mut contexte_lecture_actuel)?;
    }

    Ok(graphe)
}

fn traiter_ligne(_ligne: &String, _graphe: &mut models::graphe::Graphe, _contexte: &mut ContexteLecture) -> Result<(), String> {
    let mots = _ligne.split_whitespace().collect::<Vec<&str>>();
    if mots.is_empty() {
        return Ok(());
    }
    match _contexte {
        ContexteLecture::Donnees => {
            if mots.len() < 2 {
                Err(String::from("Il manque des données dans la ligne"))
            } else {
                match mots[0] {
                    "NAME:" => {
                        _graphe.nom = mots[1].to_string();
                        Ok(())
                    },
                    "COMMENT:" => {
                        _graphe.commentaires = mots[2..].join(" ").to_string();
                        Ok(())
                    },
                    "ORIENTED:" => {
                        match mots[1] {
                            "true" => _graphe.oriente = true,
                            "false" => _graphe.oriente = false,
                            _ => return Err(String::from("Valeur pour ORIENTED doit être 'true' ou 'false'")),
                        }
                        Ok(())
                    },
                    "NB_VERTICES:" => {
                        match mots[1].parse::<i64>() {
                            Ok(nb) => _graphe.nb_sommets = nb,
                            Err(_) => return Err(String::from("Valeur pour NB_VERTICES doit être un nombre entier")),
                        }
                        Ok(())
                    },
                    "NB_EDGES:" => {
                        match mots[1].parse::<i64>() {
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
        },
        ContexteLecture::Sommets => {
            if mots.len() < 2 {
                Err(String::from("L'une des lignes de sommet (VERICES) n'est pas complète. Celle-ci doit avoir au moins 2 colonnes."))
            } else {
                match mots[0] {
                    "EDGES" => {
                        *_contexte = ContexteLecture::Arretes;
                        Ok(())
                    },
                    _ => {
                        let id = match mots[0].parse::<i64>() {
                            Ok(nb) => nb,
                            Err(_) => return Err(String::from("Valeur pour ID pour un sommet doit être un nombre entier")),
                        };
                        let sommet = models::sommet::Sommet {
                            id,
                            nom: mots[1].to_string(),
                            valeurs: if mots.len() > 2 {
                                mots[2..]
                                    .iter()
                                    .map(|val| val.parse::<f64>().unwrap_or(0.0))
                                    .collect()
                            } else {
                                Vec::new()
                            },
                            arretes: Vec::new(),
                        };

                        _graphe.sommets.push(sommet);
                        _graphe.index_sommets.insert(id, _graphe.sommets.len() - 1);
                        Ok(())
                    },
                }
            }
        },
        ContexteLecture::Arretes => {
            if mots.len() < 2 {
                Err(String::from("Il manque des données dans une ligne de sommet"))
            } else {
                let id_depart =  match mots[0].parse::<i64>() {
                    Ok(nb) => nb,
                    Err(_) => return Err(String::from("Valeur pour ID pour un depart d'une arrete (id de sommet) doit être un nombre entier")),
                };
                let id_arrive =  match mots[1].parse::<i64>() {
                    Ok(nb) => nb,
                    Err(_) => return Err(String::from("Valeur pour ID pour une arrivée d'une arrete (id de sommet) doit être un nombre entier")),
                };
                let pos = match _graphe.index_sommets.get(&id_depart) {
                    Some(pos) => *pos,
                    None => return Err(format!("Le sommet de départ avec l'id {} n'existe pas dans le graphe", id_depart)),
                };
                let depart = &mut _graphe.sommets[pos];

                depart.arretes.push(Arrete {
                    destination: id_arrive,
                    valeurs: if mots.len() > 2 {
                        mots[2..]
                            .iter()
                            .map(|val| val.parse::<f64>().unwrap_or(0.0))
                            .collect()
                    } else {
                        Vec::new()
                    },
                });

                Ok(())
            }

        },
    }
}


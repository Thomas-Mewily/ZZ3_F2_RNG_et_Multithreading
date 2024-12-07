#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
use std::{fs::File, io::Write, time::Instant};

use criterion::Criterion;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use serde_binary::binary_stream::Endian;
use vrd::random::Random as Mt32;
use rand::{RngCore, SeedableRng};

pub mod save_to_file;
pub use save_to_file::*;

fn q2_archiver_plusieur_tirages()
{
    println!("q2_archiver_plusieur_tirages");

    let seed = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut rng = Mt32::from_seed(seed); // initialisation de mersenne twister

    for status in 0..5
    {
        let file = &format!("./mersenne_twister_etat/q2_status_{}", status);
        rng.save_to_file_ron(file).expect("impossible de sauvegarder le status du générateur dans un fichier");
        let old_rng = rng.clone();

        let tirages : Vec<f64> = (0..10).map(|_| rng.f64()).collect();
        println!("les 10 tirages sont : {:?}", tirages);
        println!("l'ancien état du générateur a été sauvegardé dans {}", file);
    
        // Vérification que le générateur importé depuis le fichier est bien le même que le générateur sauvegardé
        let rng_from_file = Mt32::load_from_file_ron(file).expect("impossible d'ouvrir le fichier");
        assert_eq!(rng_from_file, old_rng);
    }
}

const NB_STATUS : u64 = 10;
// Pour tomber sur un multiple de 3, car chaque point nécéssite 3 coordonnées
const NB_TIRAGE_PER_STATUS : u64 = 2_000_000_000 / 3 * 3;

fn q3_archiver_10_status()
{
    println!("q3_archiver_10_status");

    let seed= [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut rng = Mt32::from_seed(seed);

    for status in 0..NB_STATUS
    {
        let path = &format!("./mersenne_twister_etat/q3_status_{}", status);
        rng.save_to_file_ron(path).unwrap();
        println!("l'état du générateur après {} tirages a été sauvegardé dans {}", status * NB_TIRAGE_PER_STATUS, path);

        // exemple sauvegarde du fichier en binaire pure histoire
        // rng.save_to_file_bin(path).unwrap();

        for _ in 0..NB_TIRAGE_PER_STATUS { rng.f64(); }
    }
}

fn nb_point_dans_sphere_rayon_1(rng : &mut Mt32, nb_tirage : u64) -> u64
{
    let mut count : u64 = 0;
    
    for _ in 0..nb_tirage
    {
        let x = rng.f64();
        let y = rng.f64();
        let z = rng.f64();
        if x * x + y * y + z * z <= 1.0 {
            count += 1; // On est dans la sphère
        }
    }
    count
}

fn q4_volume_sphere_rayon_1_non_parallelise() -> f64
{
    println!("q4_volume_sphere_rayon_1_non_parallelisé");

    // Divisé par 3 car il faut 3 points par simulation pour tirer un point
    let nb_point_par_status = NB_TIRAGE_PER_STATUS / 3;

    let seed = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut rng = Mt32::from_seed(seed);

    let in_sphere : u64 = (0..NB_STATUS)
        .into_iter() // pour générer un itérateur à partir de la range (0..NB_STATUS)
        .map(|_| 
        {
            nb_point_dans_sphere_rayon_1(&mut rng, nb_point_par_status)
        })
        .sum();

    let aire = 8. * (in_sphere as f64) / ((nb_point_par_status * NB_STATUS) as f64);
    println!(
        "(non_paralleliser). L'aire d'une sphère de rayon 1 est d'approximativement {}",
        aire
    );
    aire
}

fn q5_volume_sphere_rayon_1_parallelise() -> f64
{
    println!("q5_volume_sphere_rayon_1_parallelisé");

    // Divisé par 3 car il faut 3 points par simulation pour tirer un point
    let nb_point_par_status = NB_TIRAGE_PER_STATUS / 3;

    // itération en parralèle avec into_par_iter
    let in_sphere : u64 = (0..NB_STATUS)
        .into_par_iter() // le `par` dans `into_par_iter()` au lieu de `into_iter` se charge de faire la parallélisation
        .map(|status| 
            {
            let mut rng = Mt32::load_from_file_ron(&format!("./mersenne_twister_etat/q3_status_{}", status)).expect("peut pas ouvrir le fichier");
            nb_point_dans_sphere_rayon_1(&mut rng, nb_point_par_status)
        })
        .sum();

    let aire = 8. * (in_sphere as f64) / ((nb_point_par_status * NB_STATUS) as f64);
    println!(
        "(paralleliser). L'aire d'une sphère de rayon 1 est d'approximativement {}",
        aire
    );
    aire
}


// Juste pour se donner un ordre de grandeur
fn benchmark_simple_1_repetition(f : fn() -> f64) -> f64
{
    let begin = Instant::now();
    let result = f();
    let end = Instant::now();

    println!("fini en {}s", (end-begin).as_secs_f64());
    println!();
    result
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BaseNucleique { A, C, G, T }

impl BaseNucleique
{
    pub const fn len() -> usize { 4 }
}

fn q6_trouver<const N : usize>(target_sequence : &[BaseNucleique; N])
{
    use BaseNucleique::*;

    let possibilite = (BaseNucleique::len() as u128).checked_pow(u32::try_from(target_sequence.len()).expect("la séquence est super longue !")).map(|v| v.to_string()).unwrap_or(format!("beaucoup (> {})", u128::MAX));
    println!("Il y a {possibilite} possibilitées pour trouver la séquence de base nucléique: {target_sequence:?} (recherche en cours)");

    let mut generated_seq = [BaseNucleique::A; N];
    const BASE : [BaseNucleique; 4] = [A, C, G, T];

    let seed = [0u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
    let mut rng = Mt32::from_seed(seed); // initialisation de mersenne twister
    let mut nb_it = 0;

    let begin = Instant::now();

    #[allow(unused_labels)]
    'search_loop : loop
    {
        nb_it +=1 ;

        for i in 0..N
        {
            generated_seq[i] = BASE[rng.next_u32() as usize % 4];
            // pour aller plus vite, mais c'est à l'encontre de l'idée de l'exercice
            //if generated_seq[i] != target_sequence[i] { continue 'search_loop; }
        }

        if &generated_seq == target_sequence
        {
            break;
        }
    }

    let end = Instant::now();
    println!("la séquence {:?} a été trouvé en {} itérations en {} s en non paralleliser", generated_seq, nb_it, (end-begin).as_secs_f64());
    println!();
}

fn q6b_gattaca()
{
    println!("q6_gattaca");

    use BaseNucleique::*;
    q6_trouver(&[G,A,T,T,A,C,A]);
    q6_trouver(&[A,A,G,G,T,T,C,C]);
    q6_trouver(&[A,A,A,G,G,G,T,T,T,C,C,C]);
    q6_trouver(&[A,A,A,G,G,G,T,T,T,C,C,C]);
    //q6_trouver(&[A,A,A,T,T,T,G,C,G,T,T,C,G,A,T,T,A,G]); //trop long
}

fn main()
{
    let begin = Instant::now();
    
    q2_archiver_plusieur_tirages();
    println!();

    q3_archiver_10_status();
    println!();

    let resultat_parallelise = benchmark_simple_1_repetition(q4_volume_sphere_rayon_1_non_parallelise);
    println!();

    let resultat_non_parallelise = benchmark_simple_1_repetition(q5_volume_sphere_rayon_1_parallelise);
    println!();

    assert_eq!(resultat_parallelise, resultat_non_parallelise, "Le résulat entre la version parallelise et non parallelise devrait être exactement le même");
    println!("les résultats entre la versions parallelisé et non parallelisé sont strictement identique");

    q6b_gattaca();
    println!();

    println!("Calcul et génération du rapport de benchmark pour comparer les versions parallélisées et non parallélisées du calcul du volume d'une sphère de rayon 1...");
    println!("(Cela va prendre plusieurs et longues minutes...)");
    println!();

    let mut criterion: Criterion = Criterion::default().nresamples(20).sample_size(15);
    criterion.bench_function("q5 calculer volume sphere parallelisé", |b| b.iter(|| q5_volume_sphere_rayon_1_parallelise()));
    criterion.bench_function("q4 calculer volume sphere non parallelisé", |b| b.iter(|| q4_volume_sphere_rayon_1_non_parallelise()));
    
    println!("Le programme à durée {} s", (Instant::now()-begin).as_secs_f64());
}
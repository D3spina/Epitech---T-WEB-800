## Normes de Code Rust pour le Projet

## Introduction

Ce document établit les normes de code pour le développement du projet en utilisant le langage de programmation Rust. Ces normes sont conçues pour promouvoir des pratiques de codage qui sont à la fois sécurisées et efficaces, conformément aux meilleures pratiques de la communauté Rust.
[Rusftfmt](https://github.com/rust-lang/rustfmt) est utilisé pour vérifier le respect des normes de code.

## Formatage du code

- Indentation : Utilisez 4 espaces plutôt qu'une tabulation.
- Longueur de ligne : Limitez les lignes à 100 caractères.
- Style de parenthèse : Utilisez le style "ouverture sur la même ligne" pour les blocs de code.

## Conventions de nommage

- Type : Utilisez la casse Pascal pour les noms de types, par exemple StructExemple.
- Fonctions et variables : Utilisez la casse snake pour les fonctions et les variables, par exemple ma_variable.
- Constantes : Utilisez la casse SCREAMING_SNAKE pour les constantes, par exemple CONSTANTE_EXEMPLE.

## Gestion des erreurs

- Privilégiez l'utilisation de Result<T, E> pour la gestion des erreurs plutôt que d'utiliser des panic! ou des unwrap() sauf dans les cas où le panic est le comportement attendu.
- Utilisez des messages d'erreurs clairs et descriptifs.

## Tests

- Les tests doivent être écrits avant le code qu'ils testent afin de respecter la méthode "Test Driven Development".
- Utilisez assert!, assert_eq!, et assert_ne! pour vérifier les conditions dans les tests.

## Documentation

- Documentez toutes les fonctions publiques avec des commentaires de documentation (///).
- Incluez des exemples dans la documentation lorsque cela est pertinent.

## Sécurité

- Évitez l'utilisation de unsafe autant que possible. Lorsque son utilisation est inévitable, documentez clairement pourquoi elle est nécessaire et assurez-vous qu'elle est correctement sécurisée.

## Gestion des Dépendances

- Soyez prudent lors de l'ajout de dépendances. Privilégiez les bibliothèques bien maintenues et largement utilisées.

## Formatage d'un fichier

Utiliser la commande suivante pour formater un fichier :

```bash
cargo fmt chemin_du_fichier/fichier.rs
```

Utiliser la commande suivante pour formater tout le projet :

```bash
cargo fmt
```

## Intégration Continue

Une pipeline CI a été mise en place pour vérifier le respect des normes de code avec l'éxécution de :
```bash
cargo fmt
```
puis vérifier avec :
```bash
cargo fmt --all -- --check
```
Les tests seront également réalisés automatiquement avec 
```bash
cargo build
```
Le PUSH sera réalisé uniquement si tout est validé.

# Memory-Allocator
Memory allocator in no_std Rust
Marjorie Bécot, 4SI4

## Choix techniques -

Je pensais d'abord utiliser un tableau afin de répertorier les chunks réutilisables.
Ensuite j'ai pensé à utiliser des hashmaps afin de mettre une clé et une valeur
Exemple : clé (adresse) et valeur (taille) car il faut ces deux champs.
Finalement, j'utilise un tableau de structures puisque la structure contient autant de champs que souhaité, et je peux facilement la mettre à jour pour ajouter d'autres champs si besoin.

## Idée -

La heap contiendra les variables sur lesquelles on utilise l'allocateur. L'idée est de déplacer progressivement brk afin de créer et d'agrandir la heap.
Chaque chunk créé a une adresse dans la heap et une taille. Lorsqu'un chunk est libéré, on met son adresse et sa taille dans un tableau à part.
Ces chunks libérés devraient pouvoir être réutilisés quand on rappelle l'allocateur.
Lorsque l'allocateur est appelé, on parcourt le tableau de chunks libérés et on regarde la taille.

Est-ce que la taille du chunk n est suffisant ?
* Si la taille est plus petite, passer au suivant
* Si la taille est égale, on récupère l'adresse pour réutiliser ce chunk
* Si la taille est plus grande que ce dont j'ai besoin, je garde l'adresse sous le coude, et je continue à parcourir au cas où une taille plus petite et suffisante est disponible.
Si j'ai fini de parcourir le tableau et qu'il n'y a aucune place assez grande, alors on bouge brk pour agrandir la heap et avoir de l'espace pour la nouvelle variable.

## Bump Allocator

Le Bump Allocator est plus rapide et plus simple que le free-list allocator.
Il ne permet pas de désallouer des parties isolées, la seule désallocation possible est une désallocation totale.
Le pointeur "next" qui pointe vers la prochaine zone mémoire à allouer est unidirectionnel puisqu'il n'y a pas de free partiel.
J'ai mis un algorithme qui sélectionne le meilleur chunk dans l'idée d'améliorer ce bump allocator, car pour un kernel, un bump allocator n'est pas assez malléable.

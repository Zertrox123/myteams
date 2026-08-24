# MyTeams

Plateforme de messagerie collaborative distribuée client/serveur en C avec protocole binaire TCP.

## Présentation

MyTeams (G-NWP-400) reproduit les fonctionnalités essentielles d'outils collaboratifs (Slack / Microsoft Teams) à travers une architecture réseau asynchrone.

Composants :
- **Serveur TCP** : Multiplexage d'E/S non-bloquant (`select`), gestion des équipes, canaux, threads de discussion et messages privés (DMs).
- **Client CLI** : Interface en ligne de commande interactive avec formatage des notifications temps réel.
- **Persistance** : Sauvegarde et rechargement automatique des données sur disque.

## Prérequis

- GCC
- Make

## Compilation et Lancement

```bash
# Compiler le serveur et le client
make

# Lancer le serveur
./myteams_server 4242

# Dans un autre terminal, lancer le client
./myteams_cli 127.0.0.1 4242
```

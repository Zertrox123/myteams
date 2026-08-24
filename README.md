# MyTeams

Plateforme de messagerie collaborative (style Slack / Teams) en C avec architecture client-serveur et protocole binaire TCP

- Serveur non-bloquant avec `select`
- Gestion des teams, channels, threads et messages privés
- Client en ligne de commande interactif
- Sauvegarde et chargement des données sur disque

## Build et lancement

```bash
make

# Lancer le serveur dans un terminal
./myteams_server 4242

# Lancer le client dans un autre
./myteams_cli 127.0.0.1 4242
```

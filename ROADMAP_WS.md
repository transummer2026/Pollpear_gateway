# Roadmap — proxy WS (branche feat/ws-proxy)

## Relai (`proxy_upstream` / `proxy_downstream` dans `proxy.rs`)

- Boucle de forward dans chaque fonction (actuellement corps vides), avec conversion entre
  `axum::extract::ws::Message` et `tokio_tungstenite::tungstenite::Message` (pas le même type
  des deux côtés) :
  - `proxy_upstream` : `receiver_front` → conversion → `sender_back`
  - `proxy_downstream` : `receiver_back` → conversion → `sender_front`
- **Close** : propager le `Close` reçu d'un côté vers l'autre, puis couper la tâche opposée
  (les deux tâches tournent indépendamment via `tokio::spawn` dans `handle_socket` — s'assurer
  qu'un close d'un côté ne laisse pas l'autre tâche orpheline indéfiniment).
- **Ping/Pong** : décider si on laisse axum répondre nativement aux pings front (comportement
  par défaut, le ping ne remonte pas dans la boucle) ou si on les relaie explicitement jusqu'au
  backend pour un keepalive de bout en bout.
- **Erreur/déconnexion d'un côté** ⇒ fermer proprement l'autre côté.

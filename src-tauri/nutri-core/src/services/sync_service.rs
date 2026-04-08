use crate::models::backup::AppBackup;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SyncAction {
    /// El servidor tiene datos más nuevos, el cliente debe actualizarlos (PULL).
    PullRemote,
    /// El cliente tiene datos más nuevos, el servidor debe actualizarlos (PUSH).
    PushLocal,
    /// Ambos están en el mismo estado, no hay nada que hacer.
    NoAction,
    /// Conflicto: No se puede determinar automáticamente (futuro uso).
    Conflict,
}

pub struct SyncService;

impl SyncService {
    /// Compara un backup local con uno remoto y determina la acción necesaria
    /// basándose en la estrategia LWW (Last Write Wins).
    pub fn resolve_conflict(local: &AppBackup, remote: &AppBackup) -> (SyncAction, String) {
        let local_ts = Self::parse_timestamp(&local.last_updated);
        let remote_ts = Self::parse_timestamp(&remote.last_updated);

        match (local_ts, remote_ts) {
            (Some(l), Some(r)) => {
                if r > l {
                    (SyncAction::PullRemote, "El servidor tiene cambios más recientes.".to_string())
                } else if l > r {
                    (SyncAction::PushLocal, "La app tiene cambios más recientes locales.".to_string())
                } else {
                    (SyncAction::NoAction, "Ya estás sincronizado (timestamps idénticos).".to_string())
                }
            }
            (None, Some(_)) => {
                (SyncAction::PullRemote, "No hay timestamp local, descargando del servidor.".to_string())
            }
            (Some(_), None) => {
                (SyncAction::PushLocal, "El servidor no tiene timestamp, subiendo local.".to_string())
            }
            (None, None) => {
                (SyncAction::NoAction, "No hay datos de sincronización en ningún lado.".to_string())
            }
        }
    }

    fn parse_timestamp(ts: &str) -> Option<DateTime<Utc>> {
        if ts.is_empty() {
            return None;
        }
        DateTime::parse_from_rfc3339(ts)
            .ok()
            .map(|dt| dt.with_timezone(&Utc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::backup::AppBackup;

    fn create_backup(ts: &str) -> AppBackup {
        let mut b = AppBackup::default();
        b.last_updated = ts.to_string();
        b
    }

    #[test]
    fn test_remote_wins() {
        let local = create_backup("2024-03-20T10:00:00Z");
        let remote = create_backup("2024-03-20T11:00:00Z");
        let (action, _) = SyncService::resolve_conflict(&local, &remote);
        assert_eq!(action, SyncAction::PullRemote);
    }

    #[test]
    fn test_local_wins() {
        let local = create_backup("2024-03-20T12:00:00Z");
        let remote = create_backup("2024-03-20T11:00:00Z");
        let (action, _) = SyncService::resolve_conflict(&local, &remote);
        assert_eq!(action, SyncAction::PushLocal);
    }

    #[test]
    fn test_no_action() {
        let local = create_backup("2024-03-20T11:00:00Z");
        let remote = create_backup("2024-03-20T11:00:00Z");
        let (action, _) = SyncService::resolve_conflict(&local, &remote);
        assert_eq!(action, SyncAction::NoAction);
    }

    #[test]
    fn test_empty_local() {
        let local = create_backup("");
        let remote = create_backup("2024-03-20T11:00:00Z");
        let (action, _) = SyncService::resolve_conflict(&local, &remote);
        assert_eq!(action, SyncAction::PullRemote);
    }
}

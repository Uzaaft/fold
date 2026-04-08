use crate::{
    FoldError, repos::RepositoryStore, sessions::SessionStore, workspaces::WorkspaceStore,
};

pub struct PruneStore {
    repos: RepositoryStore,
    sessions: SessionStore,
    workspaces: WorkspaceStore,
}

impl PruneStore {
    pub async fn open() -> Result<Self, FoldError> {
        Ok(Self {
            repos: RepositoryStore::open().await?,
            sessions: SessionStore::open().await?,
            workspaces: WorkspaceStore::open().await?,
        })
    }

    pub async fn sessions(&self) -> Result<usize, FoldError> {
        self.sessions.prune_terminal_sessions().await
    }

    pub async fn workspaces(&self) -> Result<usize, FoldError> {
        let mut pruned = 0;

        for repo in self.repos.list().await? {
            pruned += self.workspaces.prune(&repo.canonical()).await?.len();
        }

        Ok(pruned)
    }
}

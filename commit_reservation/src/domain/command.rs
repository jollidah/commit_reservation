#[allow(dead_code)]
pub enum GitCommand {
    // TODO merge
    Add{
        path: String
    },
    Pull,
    Commit{
        message: String
    },
    Push,
    Show,
    Status,
    ResetCommit,
}

#[allow(dead_code)]
pub enum DBCommand{
    Push,
    Pull,
    Update
}
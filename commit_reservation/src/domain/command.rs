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
    GetTodaysCommit
}

#[allow(dead_code)]
pub enum DBCommand{
    Push,
    Pull,
    Update
}
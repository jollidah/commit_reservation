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
    GetCommitLog
}

#[allow(dead_code)]
pub enum DBCommand{
    Push,
    Pull,
    Update
}
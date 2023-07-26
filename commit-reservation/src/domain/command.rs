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


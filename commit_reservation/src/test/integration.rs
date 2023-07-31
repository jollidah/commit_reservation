#[cfg(test)]
mod integration_test{
    use crate::test::command_handler_test::command_handler_test;

    
    #[test]
    fn test_git_pull(){
        assert_eq!(command_handler_test::git_pull(), true);
    }

    #[test]
    fn test_git_add(){
        assert_eq!(command_handler_test::git_add(), true);
    }

    #[test]
    fn test_git_commit(){
        assert_eq!(command_handler_test::git_commit(), true);
    }

    #[test]
    fn test_git_push(){
        assert_eq!(command_handler_test::git_push(), true);
    }
}
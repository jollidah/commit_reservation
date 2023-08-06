#[cfg(test)]
mod a_integration_test{
    use crate::test::command_handler_test::command_handler_test;

    
    #[test]
    fn a_test_git_pull(){
        assert_eq!(command_handler_test::git_pull(), true);
    }

    #[test]
    fn b_test_git_add(){
        assert_eq!(command_handler_test::git_add(), true);
    }

    #[test]
    fn c_test_git_commit(){
        assert_eq!(command_handler_test::git_commit(), true);
    }

    #[test]
    fn d_test_git_get_todays_commit(){
        assert_eq!(command_handler_test::git_get_todays_commit(), true);
    }

    #[test]
    fn e_test_git_push(){
        assert_eq!(command_handler_test::git_show_push(), true);
    }
}

// cargo test -- --test-threads=1
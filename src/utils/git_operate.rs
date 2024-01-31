use std::process::Command;

use crate::errors::ApplicationError;



pub fn git_exist() -> Result<(),ApplicationError>{
    match Command::new("git").arg("--version").output() {
        Ok(output) => 
        {
            let out = String::from_utf8(output.stdout).unwrap();
            println!("输出 {:?} ",out);
            Ok(())
        },
        Err(_) => Err(ApplicationError::NotFoundGitError),
    }
}

pub fn git_clone(url: &str, path: &str) -> Result<(),ApplicationError>{
    match Command::new("git").arg("clone").arg(url).arg(path).output() {
        Ok(output) => {
            let out = String::from_utf8(output.stdout).unwrap();
            println!("输出 {:?} ",out);
            Ok(())
        },
        Err(_) => Err(ApplicationError::GitCloneError),
        
    }
}
#[cfg(test)]
mod test{
    use crate::utils::git_operate::git_exist;


    #[test]
    fn test_exist_git(){
        assert!(git_exist());
    }
}
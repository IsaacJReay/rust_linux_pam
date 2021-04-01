use std::os::unix::process::CommandExt;
use std::process::Command;
use pam::Authenticator;
use users::get_user_by_name;

pub fn complexpam(username: &str, password: &str){

    // Now, setup the authenticator, we require the basic "system-auth" service
    let mut client = Authenticator::with_password("system-auth")
        .expect("Failed to init PAM client!");
    
    // Now, give username password to be authenticated
    client
        .get_handler()
        .set_credentials(username, password);
    

    // Now, Authenticate and get returned Error if any
    let login = client.authenticate();


    // Error Handling if Authentication Fails
    let mut loginstatus: bool = false;
    match login {
        Err(err) => println!("{:?}", err),
        Ok(()) => loginstatus = true,
    }

    
    // Error Handling if Authentication fails and cause session opening to fails
    let sessionstatus: Result<(), pam::PamError>;
    match loginstatus {
        true => sessionstatus = client.open_session(),
        false => sessionstatus = client.authenticate(), // Borrow Error Status from Authentication because I don't know how to set default Error for it
    }


    // we now try to spawn `/bin/bash` as this user
    // note that setting the uid/gid is likely to fail if this program is not already run as the
    // proper user or as root
    match sessionstatus {
        Ok(()) => {
            println!("This get run");
            let user = get_user_by_name(username).unwrap();
            let error = Command::new("/bin/bash")
            .uid(user.uid())
            .gid(user.primary_group_id())
            .exec();
        // if exec() returned, this means there was an error:
            println!("Error spawning bash: {:?}", error);
        },
        Err(_err) => (),
    }

}


pub fn simplepam<'longevity>(username: &str, password: &str) -> &'longevity str{

    // setup authenticator with system-auth
    let service = "system-auth";
    let mut auth = pam::Authenticator::with_password(&service)
        .unwrap();

    // Now, give username password to be authenticated 
    auth.get_handler()
        .set_credentials(username, password);

    // Now, Authenticate and Listen for feedback
    if  auth.authenticate()
            .is_ok() && 
        auth
            .open_session()
            .is_ok() {
        "Authentication Succeed!"
    }
    else {
        "Authentication failed"
    }
}
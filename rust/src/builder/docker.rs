use std::process;
use Builder;

pub struct DockerBuilder{}

impl Builder for DockerBuilder {
    fn placeholder(&self) {
        println!("Docker builder!");
    }
    fn build(&self, dir: String, image: String) {}
    fn push(&self, image: String) {}
    fn logs(&self, name: String) {}
    fn cancel(&self, name: String) {}
}

pub fn build(dir: String, image: String) {
    let mut child = process::Command::new("docker")
        .arg("build")
        .arg(image)
        .arg(dir)
        .spawn()
        .expect("failed to execute 'docker build'");
    
    child.wait();
}

pub fn push(image: String) {
    let mut child = process::Command::new("docker")
        .arg("push")
        .arg(image)
        .spawn()
        .expect("failed to execute 'docker push'");
    
    child.wait();

}

pub fn logs(name: String) {
    let mut child = process::Command::new("docker")
        .arg("logs")
        .arg("-f")
        .arg(name)
        .spawn()
        .expect("failed to execute 'docker logs'");
    
    child.wait();
}

pub fn cancel(name: String) {
    let mut child = process::Command::new("docker")
        .arg("stop")
        .arg(name.clone())
        .spawn()
        .expect("failed to execute 'docker stop'");
    
    child.wait();

    child = process::Command::new("docker")
        .arg("rm")
        .arg("-f")
        .arg(name.clone())
        .spawn()
        .expect("failed to execute 'docker rm'");
    
    child.wait();
}

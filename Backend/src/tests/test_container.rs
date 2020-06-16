use testcontainers::clients::Cli;
use testcontainers::{Container, Docker, clients};
use std::env;
use testcontainers::images::generic::{GenericImage, WaitFor};

pub struct TestContainer {
  docker: Cli,
  full_db: bool,
}

impl TestContainer {
  pub fn new(full_db: bool) -> Self {
    TestContainer {
      docker: clients::Cli::default(),
      full_db,
    }
  }

  pub fn run(&self) -> (String, Container<'_, Cli, GenericImage>) {
    let node = self.docker.run(self.get_test_image());
    let dns = format!("mysql://root:vagrant@localhost:{}/", node.get_host_port(3306).unwrap());
    (dns, node)
  }

  fn get_test_image(&self) -> GenericImage {
    let mut container = GenericImage::new("mariadb:10.5.3")
      .with_env_var("MYSQL_USER", "mysql")
      .with_env_var("MYSQL_PASSWORD", "vagrant")
      .with_env_var("MYSQL_ROOT_PASSWORD", "vagrant")
      .with_wait_for(WaitFor::message_on_stderr("port: 3306"));

    let repo_path: String = env::var("PWD").unwrap();
    if self.full_db {
      container = if repo_path.contains("Backend") {
        container.with_volume(format!("{}/../Database/patches", repo_path), "/docker-entrypoint-initdb.d")
      } else {
        container.with_volume(format!("{}/Database/patches", repo_path), "/docker-entrypoint-initdb.d")
      };
    } else {
      container = if repo_path.contains("Backend") {
        container.with_volume(format!("{}/../Database/test", repo_path), "/docker-entrypoint-initdb.d")
      } else {
        container.with_volume(format!("{}/Database/test", repo_path), "/docker-entrypoint-initdb.d")
      }
    }

    container
  }
}
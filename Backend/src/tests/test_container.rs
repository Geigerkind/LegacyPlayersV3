#[macro_export]
macro_rules! start_test_db {
  ( $x:expr, $dns:expr ) => {
    use testcontainers::{clients, Docker};
    use std::env;
    use testcontainers::images::generic::{GenericImage, WaitFor};

    let docker = clients::Cli::default();
    let mut container = GenericImage::new("mariadb:10.5.3")
      .with_env_var("MYSQL_USER", "mysql")
      .with_env_var("MYSQL_PASSWORD", "vagrant")
      .with_env_var("MYSQL_ROOT_PASSWORD", "vagrant")
      .with_wait_for(WaitFor::message_on_stderr("port: 3306"));

    let repo_path = env::var("PWD").unwrap();
    if $x {
      container = container
      .with_volume(format!("{}/../Database/patches", repo_path), "/docker-entrypoint-initdb.d");
    } else {
      container = container
      .with_volume(format!("{}/../Database/test", repo_path), "/docker-entrypoint-initdb.d");
    }

    let node = docker.run(container);
    let port = node.get_host_port(3306).unwrap();
    $dns = format!("mysql://root:vagrant@localhost:{}/", port);
  };
}

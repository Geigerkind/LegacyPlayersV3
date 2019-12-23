REPOSITORY_NAME='LegacyPlayersV3'
REPOSITORY='https://github.com/Geigerkind/LegacyPlayersV3'
DOMAIN='beta.legacyplayers.com'
HOST_USER='root'
BACKEND_USER='rpll'
HOST_IP='51.38.99.189'
DB_PASSWORD=$(cat /root/Keys/db_password)

function fixCertificates {
  pacman -S --noconfirm ca-certificates
  if [ ! -f "/etc/ssl/certs/ca-certificates.crt" ]; then
    cd /etc/ssl/certs
    cat *.pem >> ca-certificates.crt
    cd ~
  fi
}

function initCertificates {
  pacman -S --noconfirm certbot python certbot-dns-ovh
  # See: https://certbot-dns-ovh.readthedocs.io/en/stable/
  chmod -R 600 ~/Keys/ovh.ini
  # Requires user input
  certbot certonly --dns-ovh --dns-ovh-credentials ~/Keys/ovh.ini -d ${DOMAIN} -d smtp.${DOMAIN}
  echo "0 0,12 * * * root python -c 'import random; import time; time.sleep(random.random() * 3600)' && certbot renew" | tee -a /etc/crontab > /dev/null
}

function installZopfli {
  git clone https://github.com/google/zopfli
  cd zopfli
  make zopflipng
  cp zopflipng /usr/bin/
  cd ..
  rm -rf zopfli
}

function initNginx {
  pacman -S --noconfirm nginx nginx-mod-brotli
  cp ~/${REPOSITORY_NAME}/Deploy/conf/nginx.conf /etc/nginx/
  mkdir -p /var/www/html
  systemctl enable nginx
  systemctl start nginx
}

function initMariaDb {
  pacman -S --noconfirm mariadb
  mysql_install_db --user=mysql --basedir=/usr --datadir=/var/lib/mysql
  cp ~/${REPOSITORY_NAME}/Deploy/conf/my.conf /etc/
  systemctl enable mysqld
  systemctl start mysqld
  mysql -u root mysql -e "ALTER USER 'root'@'localhost' IDENTIFIED BY '${DB_PASSWORD}'"
  systemctl restart mysqld
  cd ~/${REPOSITORY_NAME}/Database
  bash merger.sh
  mysql -uroot -p${DB_PASSWORD} < merge.sql
  rm merge.sql
  cd ~
  systemctl restart mysqld
}

function initPostfix {
  pacman -S --noconfirm postfix
  cp ~/${REPOSITORY_NAME}/Deploy/conf/virtual /etc/postfix/
  cp ~/${REPOSITORY_NAME}/Deploy/conf/main.cf /etc/postfix/
  postmap /etc/postfix/virtual
  systemctl enable postfix
  systemctl start postfix
}

function initSSH {
  mkdir /home/${BACKEND_USER}/.ssh
  touch /home/${BACKEND_USER}/.ssh/authorized_keys
  for filename in /root/${REPOSITORY_NAME}/Deploy/ssh/*.pub; do
    if [ ! -f "${filename}" ]; then
      continue
    fi
    cat ${filename} >> /home/${BACKEND_USER}/.ssh/authorized_keys
  done

  # Adjusting Configuration
  sed -i "s/#MaxAuthTries 6/MaxAuthTries 4/g" /etc/ssh/sshd_config
  sed -i "s/#Port 22/Port 2222/g" /etc/ssh/sshd_config
  sed -i "s/#PasswordAuthentication yes/PasswordAuthentication no/g" /etc/ssh/sshd_config
  sed -i "s/#PermitRootLogin prohibit-password/PermitRootLogin no/g" /etc/ssh/sshd_config
  echo "AllowUsers ${BACKEND_USER}" >> /etc/ssh/sshd_config
  systemctl restart sshd
}

function installRust {
  pacman -S --noconfirm rustup
  rustup toolchain install nightly
  rustup default nightly
}

function initPrometheus {
  pacman -S --noconfirm prometheus prometheus-node-exporter
  cp /root/${REPOSITORY_NAME}/Deploy/conf/prometheus.yml /etc/prometheus/
  systemctl enable prometheus.service
  systemctl enable prometheus-node-exporter.service
  systemctl start prometheus-node-exporter.service
  systemctl start prometheus.service
}

function initGrafana {
  pacman -S --noconfirm grafana
  mkdir /var/lib/grafana/provisioning
  mkdir /var/lib/grafana/dashboards
  cp -r /root/${REPOSITORY_NAME}/Deploy/conf/Grafana/provisioning/* /var/lib/grafana/provisioning/
  # Replacing discord webhook
  WEBHOOK_URL=$(cat /root/Keys/discord_webhook | sed -e 's/[\/&]/\\&/g')
  sed -i "s/\{\{DISCORD_WEBHOOK\}\}/${WEBHOOK_URL}/g" /var/lib/grafana/provisioning/notifiers/discord.yml

  cp /root/${REPOSITORY_NAME}/Deploy/conf/Grafana/dashboards/* /var/lib/grafana/dashboards/
  sed -i "s/;provisioning = conf\/provisioning/provisioning = \/var\/lib\/grafana\/provisioning/g" /etc/grafana.ini
  sed -i "s/;domain = localhost/domain = ${DOMAIN}/g" /etc/grafana.ini
  sed -i "s/;reporting_enabled = true/reporting_enabled = false/g" /etc/grafana.ini
  sed -i "s/;check_for_updates = true/check_for_updates = false/g" /etc/grafana.ini
  sed -i "s/;enabled = false/enabled = true/g" /etc/grafana.ini
  sed -i "s/;root_url = http:\/\/localhost:3000/root_url = https:\/\/${DOMAIN}\/grafana\//g" /etc/grafana.ini
  systemctl enable grafana
  systemctl start grafana
}

function initUfw {
  pacman -S --noconfirm ufw
  ufw default deny incoming
  ufw allow 2222
  ufw allow 443
  ufw allow 5000
  systemctl enable ufw
  yes | ufw enable
}

function initServer {
  sed -i "s/# %wheel ALL=(ALL) ALL/%wheel ALL=(ALL) ALL/g" /etc/sudoers

  # Requires user input
  useradd -m -G wheel ${BACKEND_USER}
  passwd ${BACKEND_USER}
  passwd -l root

  pacman -Sy
  pacman -S --noconfirm git npm guetzli zopfli libwebp htop clang openssl pkg-config python python-werkzeug make fail2ban unzip

  # Fail2Ban configuration
  sed -i "s/maxretry = 5/maxretry = 3/g" /etc/sudoers

  fixCertificates
  installRust
  installZopfli
  npm install -g html-minifier
  # Requires user input
  npm i -g @angular/cli
  # See: https://git-scm.com/book/de/v2/Git-Tools-Credential-Storage
  git config --global credential.helper
  cp ~/Keys/.git-credentials ~/
  git clone ${REPOSITORY}
  cd /root/${REPOSITORY_NAME}/Webclient
  # Requires user input
  npm install
  cd /root
  cp /root/${REPOSITORY_NAME}/Deploy/conf/backend.service /etc/systemd/system/
  cp /root/${REPOSITORY_NAME}/Deploy/conf/deploy.service /etc/systemd/system/
  systemctl daemon-reload
  systemctl enable backend.service
  systemctl enable deploy.service
  systemctl enable deploy.service
  systemctl enable fail2ban
  systemctl start fail2ban
  systemctl start deploy

  initSSH
  initCertificates
  initNginx
  initMariaDb
  initPostfix
  initPrometheus
  initGrafana
  initUfw
}

initServer
bash /root/${REPOSITORY_NAME}/Deploy/deploy.sh 1
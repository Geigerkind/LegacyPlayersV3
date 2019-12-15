## Rusty Template
Rusty Template is a clean slate for REST based projects. The goal is to take care of all
tedious infrastructural problems which slow down down the initial establishment of a project.
With this in mind the application is designed to scale with the amount of cores while attempting
to maintain a small memory footprint.

## Features
#### Infrastructure
- Continues Integration using Github Actions with integration tests
- Automatic deployment to the production server
- Database changes management
- NGINX hardening (h2 and Brotli compression enabled)
- Clean and maintainable architecture

#### Backend
- Account management system 
    - APIToken management
    - Creation/Deletion
    - Modification nickname, mail, password
    - Reset password
    - Token based Authentication
- API is exposed in a verbose way

#### Webclient
- GDPR ready cookie banner (Does not include ADs), privacy (and imprint for Germany)
- Google Analytics integration
- Global configuration
- PWA ready (Prompt and manifest provided)
- Lazy loading
- HTML minimization
- i18n
- Account integration (see Backend)
- Service worker configuration (including caching system for the API)
- Notification system
- Responsive design

#### Tools
- The Webclient uses a custom naming convention. To make the creation of such components easier, a
script is provided in Webclient/tools/generate.sh <(component|module|service)> <PATH> <SNAKE_CASE_NAME>

and much more...

## Disclaimer GDPR
I have written the cookie banner to my best understanding of the current GDPR laws. I am a 
computer scientist and not a lawyer. The privacy and imprint are generated from sites of law 
companies (links in the respective page). Use on your own risk. Consult a lawyer for confirmation.

## Operating systems
This project has only been tested on UNIX systems.  
Non-UNIX users are on their own!

## Installation
1. Install **docker**, **docker-compose**, **rustup**
2. Using rustup, install the **nightly** toolchain and set it to default
3. Make sure that no service is running on the following ports: 3306, 443, 80, 25, 4200 and 8000
4. Go into the Environment directory and start it using **docker-compose up**. (If you want to run it as daemon, append -d)
5. Go into the Backend directory and start the server using **cargo run**
6. Go into the Webclient directory and install packages **npm i**
7. Start the webclient using **npm run start**

## Enabling HTTPS locally
1. Import **Environment/nginx/ca.pem** as **authority** into your browser
2. Append the entry **jaylapp.dev 127.0.0.1** to your **hosts file**
3. Access the website using **jaylapp.dev** in your browser

## Deploying for the first time
1. Adjust shell variables in the Deploy folder
2. Adjust configuration files
3. Adjust scheduler.py
4. Execute bootstrap.sh
5. Add WebHook to github for the scheduler url

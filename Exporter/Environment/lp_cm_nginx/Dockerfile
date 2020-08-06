FROM archlinux:latest as builder

RUN pacman -Sy --noconfirm
RUN pacman -S --noconfirm npm
RUN npm i -g @angular/cli

ADD ./Webclient /Webclient

WORKDIR /Webclient

RUN npm i
RUN ng build --prod

FROM nginx:stable

RUN mkdir -p /var/www/html

COPY --from=builder /Webclient/dist/* /var/www/html/
COPY ./Environment/lp_cm_nginx/nginx.conf /etc/nginx/
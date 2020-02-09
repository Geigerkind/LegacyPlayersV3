FROM ranadeeppolavarapu/nginx-http3

RUN mkdir -p /etc/nginx/cert

COPY nginx.conf /etc/nginx/
copy dhparam.pem /etc/nginx/cert/
copy jaylapp.crt /etc/nginx/cert/
copy jaylapp.key.pem /etc/nginx/cert/
FROM nginx:1.17.8-alpine
COPY ./nginx.conf.template /nginx.conf.template
COPY ./proxy_params /etc/nginx/proxy_params
CMD ["/bin/sh" , "-c" , "envsubst < /nginx.conf.template > /etc/nginx/nginx.conf && exec nginx -g 'daemon off;'"]
EXPOSE 80

FROM nginx:alpine

COPY nginx/nginx.conf /etc/nginx/nginx.conf

COPY build /var/www/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]

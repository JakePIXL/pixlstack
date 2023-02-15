# Use the official Nginx image as the base image
FROM nginx:alpine

# Copy the configuration file and the HTML file to the image
COPY conf/nginx.conf /etc/nginx/nginx.conf
# COPY html/ /usr/share/nginx/html/

# Start Nginx when the container is run
CMD ["nginx", "-g", "daemon off;"]

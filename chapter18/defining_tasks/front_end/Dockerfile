# # pull official base image
FROM node:17.0.0

# set working directory
WORKDIR .

# add app
COPY . ./

RUN npm install -g serve
# RUN yarn add node-sass
RUN npm install
RUN npm run react-build
# RUN serve -s build
EXPOSE 4000

CMD ["serve", "-s", "build", "-l", "4000"]

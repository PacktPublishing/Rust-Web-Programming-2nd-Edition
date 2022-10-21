FROM postgres 

RUN apt-get update \
  && apt-get install -y wget \
  && wget -O - https://raw.githubusercontent.com/yellow-bird-consult/build_tools/develop/scripts/install.sh | bash \
  && cp ~/yb_tools/database.sh ./database.sh

WORKDIR .
ADD . .

CMD ["bash", "./database.sh", "db", "rollup"]

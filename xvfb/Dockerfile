FROM --platform=linux/amd64 ubuntu:24.04

RUN apt-get update && apt-get install -y xvfb

EXPOSE 6000

ENTRYPOINT ["Xvfb",":0","-listen","tcp","-ac","-screen","0","1280x800x24","-fakescreenfps","60"]
CMD []

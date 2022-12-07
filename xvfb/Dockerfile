FROM ubuntu:22.04

RUN apt-get update && apt-get install -y xvfb

EXPOSE 6000

CMD []
ENTRYPOINT ["Xvfb", ":0", "-listen", "tcp", "-ac", "-screen", "0", "1280x800x24"]

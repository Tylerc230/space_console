FROM ubuntu
ARG DEBIAN_FRONTEND=noninteractive
ENV TZ=america/los_angeles
RUN apt update
RUN apt-get install -y lsb-release
RUN apt-get install -y gnupg2
RUN sh -c 'echo "deb http://packages.ros.org/ros/ubuntu $(lsb_release -sc) main" > /etc/apt/sources.list.d/ros-latest.list'
RUN apt-key adv --keyserver 'hkp://keyserver.ubuntu.com:80' --recv-key C1CF6E31E6BADE8868B172B4F42ED6FBAB17C654
RUN apt update
RUN apt install -y ros-noetic-ros-base
RUN echo "source /opt/ros/noetic/setup.bash" >> ~/.bashrc
ENTRYPOINT bash && source /opt/ros/noetic/setup.bash

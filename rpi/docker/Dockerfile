#FROM osrf/ros:noetic-desktop-full
#FROM arm32v7/ros:noetic-robot
FROM arm64v8/ros:noetic-robot
RUN apt-get update && apt-get -y install python3-pip git
#RUN pip3 install -U catkin_tools
RUN pip3 install git+https://github.com/catkin/catkin_tools.git
RUN echo "source /opt/ros/noetic/setup.bash" >> ~/.bashrc
RUN echo "source /workspace/devel/setup.bash" >> ~/.bashrc
WORKDIR /workspace

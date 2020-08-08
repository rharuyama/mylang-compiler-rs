FROM ubuntu

#RUN apt-get update

#RUN apt-get install -y gcc cargo
#RUN cargo build
#RUN cargo run > tmp.s

RUN gcc -o /home/mylang-compiler-rs/src/tmp /home/mylang-compiler-rs/src/tmp.s

RUN ./home/mylang-compiler-rs/src/tmp

RUN echo $?
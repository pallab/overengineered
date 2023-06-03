This is an overengineered piece of system that serves no purpose at all.
I built it to expereriment with different technologies. 
Every piece of tech used in this React, Next, Rust, Actix, Go, Kafka, Docker were unknown to me before this. 

Heres's how it works :

### Webserver:
At the center of the system there is an Actix Webserver that serves a web app built using Next. 
I would try to build a few forms and dashboards using the apis served by this webserver. 

### Next Web App
The frontend is built using Next. I have no plans for learning it well just wanted to learn how to serve a modern web app via actix. An also learn enough Next to build dashboards, forms etc. 

### Database:
I am using a MySql db just to experiment with diesel in rust and dockerization of db. 
It does stoire the user table, so not so pointless after all.

### Go RPC
There is small go app which pretends to be the stock market and sends price data of stocks to the Rust Webserver over gRPC. As you might have guessed I added this to experiment with gRPC. I am using the Tonic library in the webserver to build the client. 

### Kafka 
The stock prices that the webserver receives go to kafka to be further processed. I understand how kafka works but never used in any app so here it goes. Using the librdkafka library to talk to kafka. 

### Docker
All the pieces in this are dockerized. There's a docker compose file to run it all. I never used docker before. Man, I was missing out. 

### Kubernetes
Umm. Maybe. :)

### Actors
I love Akka actors. So I thought I'll find something similar for rust. I found a couple, there is riker which is heavily inspired by Akka but seems almost dead. And there is Actix, which is still actively developed. Compared to Akka actix seems limited in feature and very hard to work with. Specially the async rpc call I made in the handler pushed my stress level through the roof for a good part of the day till midnight when I found a workable compromise. 

### UI
https://s4ch1.hashnode.dev/how-to-create-pixel-art-with-css
https://github.com/fayez-nazzal/ace-pixels
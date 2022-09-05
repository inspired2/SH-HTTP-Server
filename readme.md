# Description

this repo is for educational purposes only.
it contains homework projects for rust course, provided by otus.ru

## to test:

### 1. run server
Server runs at "127.0.0.1:8080"

cargo run --example server



## 2. send http requests via postman, insomnia, etc.
### Routes
#### Add room:
POST    /add_room
##### payload:

{
    "name": "roomName"
}

#### Add device:
POST     /add_device
##### payload:

{
    "device": "deviceName",
    "room": "roomName"
}

#### Get report:
GET     /report

#### Get rooms: 
GET     /rooms

#### Get devices in room:
GET     /devices/{room}

#### Delete room:
DELETE     /{room}

#### Delete device: 
DELETE     /{room}/{device}
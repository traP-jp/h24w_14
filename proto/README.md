# proto

```mermaid
graph LR
    World[world.proto]
    User[user.proto]
    Auth[auth.proto]
    SpeakerPhone[speaker_phone.proto]
    Reaction[reaction.proto]
    Message[message.proto]
    Explore[explore.proto]

    SpeakerPhone --> World
    Reaction --> World
    Message --> World
    Explore --> World
```

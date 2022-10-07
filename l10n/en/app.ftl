greeting = Hello { $username }! It's nice day out here, right? Thanks for using { -app-name(case: "lowercase") } 🙂

menu =
    .home = Home
    .settings = Settings

status =
    .online = Online
    .offline = Offline
    .busy = Busy ({ $reason })
    .busy-for = Busy for { TIME($time) } ({ $reason })

launch-timer = Launch a { TIME($seconds) } timer.

order-a-pop = Make yourself comfortable on the couch, your pop is coming!

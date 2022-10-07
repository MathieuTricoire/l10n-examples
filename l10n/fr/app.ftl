greeting = Bonjour { $username } ! Merci d'utiliser { -app-name(case: "lowercase") } 🙂

menu =
    .home = Home
    .settings = Settings

status =
    .online = En ligne
    .offline = Hors ligne
    .busy = { $gender ->
        [male] Occupé
        [female] Occupée
       *[other] Non disponible
    } ({ $reason })
    .busy-for = { $gender ->
        [male] Occupé
        [female] Occupée
       *[other] Non disponible
    } pour { TIME($time) } ({ $reason })

launch-timer = Lancer un minuteur de { TIME($seconds) }.

order-a-pop = Installez-vous confortablement dans le canapé, votre soda arrive !
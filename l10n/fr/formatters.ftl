time =
    .seconds = { $seconds ->
        [one] 1 seconde
       *[other] { $seconds } secondes
    }
    .minutes = { $minutes ->
        [one] 1 minute
       *[other] { $minutes } minutes
    }
    .minutes-seconds = { $minutes ->
        [one] 1 minute { $seconds }
       *[other] { $minutes } minutes { $seconds }
    }
    .hours = { $hours ->
        [one] 1 heure
       *[other] { $hours } heures
    }
    .hours-minutes = { $hours ->
        [one] 1 heure { $minutes }
       *[other] { $hours } heures { $minutes }
    }

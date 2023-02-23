time =
    .seconds = { $seconds ->
        [one] 1 second
       *[other] { $seconds } seconds
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
        [one] 1 hour
       *[other] { $hours } hours
    }
    .hours-minutes = { $hours ->
        [one] 1 hour { $minutes }
       *[other] { $hours } hours { $minutes }
    }
